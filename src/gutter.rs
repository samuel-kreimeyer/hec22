//! Gutter flow and spread calculations
//!
//! This module implements gutter spread calculations from HEC-22 Chapter 5,
//! including uniform cross-slope gutters, composite gutter sections, and
//! parabolic crowned sections.
//!
//! ## Gutter Types
//!
//! 1. **Uniform Cross-Slope**: Simple triangular section with constant slope (Section 5.3.2.1)
//! 2. **Composite Section**: Gutter section + roadway with different slopes (Section 5.3.2.2)
//! 3. **Parabolic Crown**: Curved roadway surface (Section 5.3.2.3)
//!
//! ## Key Equations
//!
//! ### Equation 5.2: Gutter Flow (Izzard's Modified Manning's Equation)
//! ```
//! Q = (Ku/n) × Sx^1.67 × SL^0.5 × T^2.67
//! ```
//!
//! Where:
//! - Q = flow rate (cfs or m³/s)
//! - Ku = 0.56 (US customary), 0.376 (SI metric)
//! - n = Manning's roughness coefficient
//! - Sx = cross slope (ft/ft or m/m)
//! - SL = longitudinal slope (ft/ft or m/m)
//! - T = spread (width of flow) (ft or m)
//!
//! ### Equation 5.4: Spread for Given Flow
//! ```
//! T = [(Qn)/(Ku × Sx^1.67 × SL^0.5)]^0.375
//! ```

/// Gutter section type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GutterSectionType {
    /// Uniform cross-slope (triangular section)
    UniformSlope,
    /// Composite section (gutter + roadway)
    Composite,
    /// Parabolic crown
    ParabolicCrown,
}

/// Gutter flow result
#[derive(Debug, Clone, PartialEq)]
pub struct GutterFlowResult {
    /// Total spread (ft or m)
    pub spread: f64,
    /// Flow rate (cfs or cms)
    pub flow: f64,
    /// Flow depth at curb (ft or m)
    pub depth_at_curb: f64,
    /// Average velocity (ft/s or m/s)
    pub velocity: f64,
    /// Flow area (sq ft or sq m)
    pub area: f64,
    /// Frontal flow in gutter (cfs or cms) - for composite sections
    pub frontal_flow: Option<f64>,
    /// Side flow on roadway (cfs or cms) - for composite sections
    pub side_flow: Option<f64>,
}

/// Uniform cross-slope gutter calculator
///
/// For simple triangular gutter sections with uniform cross slope
pub struct UniformGutter {
    /// Manning's roughness coefficient
    pub manning_n: f64,
    /// Cross slope S_x (ft/ft or m/m)
    pub cross_slope: f64,
    /// Longitudinal slope S_L (ft/ft or m/m)
    pub longitudinal_slope: f64,
    /// Gutter width W (ft or m) - optional, for width-limited calculations
    pub gutter_width: Option<f64>,
}

impl UniformGutter {
    /// Create a new uniform gutter calculator
    pub fn new(
        manning_n: f64,
        cross_slope: f64,
        longitudinal_slope: f64,
        gutter_width: Option<f64>,
    ) -> Self {
        Self {
            manning_n,
            cross_slope,
            longitudinal_slope,
            gutter_width,
        }
    }

    /// Calculate flow capacity for a given spread
    ///
    /// **HEC-22 Equation 5.2:**
    /// ```
    /// Q = (Ku/n) × Sx^1.67 × SL^0.5 × T^2.67
    /// ```
    ///
    /// # Arguments
    /// * `spread` - Spread from curb (ft or m)
    /// * `k` - Unit constant Ku (0.56 for US customary, 0.376 for SI)
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Equation 5.2, Example 5.1
    pub fn flow_capacity(&self, spread: f64, k: f64) -> f64 {
        (k / self.manning_n)
            * self.cross_slope.powf(1.67)
            * self.longitudinal_slope.powf(0.5)
            * spread.powf(2.67)
    }

    /// Calculate spread for a given flow rate
    ///
    /// **HEC-22 Equation 5.4:**
    /// ```
    /// T = [(Qn)/(Ku × Sx^1.67 × SL^0.5)]^0.375
    /// ```
    ///
    /// # Arguments
    /// * `flow` - Flow rate (cfs or cms)
    /// * `k` - Unit constant Ku (0.56 for US customary, 0.376 for SI)
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Equation 5.4, Example 5.1
    pub fn spread_for_flow(&self, flow: f64, k: f64) -> f64 {
        let numerator = flow * self.manning_n;
        let denominator = k * self.cross_slope.powf(1.67) * self.longitudinal_slope.powf(0.5);
        (numerator / denominator).powf(0.375)
    }

    /// Calculate complete flow result for given spread
    ///
    /// # Arguments
    /// * `spread` - Spread from curb (ft or m)
    /// * `k` - Unit constant (0.56 for US, 0.376 for SI)
    pub fn flow_result(&self, spread: f64, k: f64) -> GutterFlowResult {
        // Flow capacity
        let flow = self.flow_capacity(spread, k);

        // Depth at curb
        let depth_at_curb = spread * self.cross_slope;

        // Flow area (triangular)
        let area = 0.5 * spread * depth_at_curb;

        // Velocity
        let velocity = if area > 0.0 { flow / area } else { 0.0 };

        GutterFlowResult {
            spread,
            flow,
            depth_at_curb,
            velocity,
            area,
            frontal_flow: None,
            side_flow: None,
        }
    }

    /// Calculate spread for given flow (inverse of flow_capacity)
    ///
    /// # Arguments
    /// * `flow` - Flow rate (cfs or cms)
    /// * `k` - Unit constant (0.56 for US, 0.376 for SI)
    pub fn result_for_flow(&self, flow: f64, k: f64) -> GutterFlowResult {
        let spread = self.spread_for_flow(flow, k);
        self.flow_result(spread, k)
    }
}

/// Composite gutter section calculator
///
/// For sections with a depressed gutter section and roadway with different slopes.
/// The gutter section has a base cross slope (Sg), and the depression adds additional
/// slope according to Equation 5.8: Sw = Sg + a/W
pub struct CompositeGutter {
    /// Manning's roughness coefficient
    pub manning_n: f64,
    /// Gutter section base cross slope Sg (ft/ft) - slope before depression is added
    pub gutter_slope: f64,
    /// Roadway cross slope Sx (ft/ft) - slope of the undepressed pavement section
    pub roadway_slope: f64,
    /// Longitudinal slope SL (ft/ft)
    pub longitudinal_slope: f64,
    /// Gutter width W (ft)
    pub gutter_width: f64,
    /// Local depression a (in or mm)
    pub local_depression: f64,
}

impl CompositeGutter {
    /// Create a new composite gutter calculator
    pub fn new(
        manning_n: f64,
        gutter_slope: f64,
        roadway_slope: f64,
        longitudinal_slope: f64,
        gutter_width: f64,
        local_depression: f64,
    ) -> Self {
        Self {
            manning_n,
            gutter_slope,
            roadway_slope,
            longitudinal_slope,
            gutter_width,
            local_depression,
        }
    }

    /// Calculate cross slope in depressed section
    ///
    /// **HEC-22 Equation 5.8:**
    /// ```
    /// Sw = Sg + a/W
    /// ```
    /// Where Sg is the gutter section base cross slope (before depression is added)
    ///
    /// # Arguments
    /// * `depression_ft` - Gutter depression depth (ft or m)
    ///
    /// # Returns
    /// Cross slope in the depressed gutter section (ft/ft or m/m)
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Equation 5.8, Example 5.2
    fn depressed_section_slope(&self, depression_ft: f64) -> f64 {
        self.gutter_slope + (depression_ft / self.gutter_width)
    }

    /// Calculate ratio of flow in depressed section to total flow
    ///
    /// **HEC-22 Equation 5.7:**
    /// ```
    /// Eo = 1 / [1 + (Sw/Sx) / [(1 + (Sw/Sx)/(T/W-1))^2.67 - 1]]
    /// ```
    ///
    /// # Arguments
    /// * `sw` - Cross slope in depressed section (ft/ft or m/m)
    /// * `sx` - Cross slope of pavement (ft/ft or m/m)
    /// * `t` - Total spread (ft or m)
    /// * `w` - Width of depressed section (ft or m)
    ///
    /// # Returns
    /// Ratio Eo = Qw/Q (flow in depressed section / total flow)
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Equation 5.7, Example 5.2
    fn flow_ratio_depressed(&self, sw: f64, sx: f64, t: f64, w: f64) -> f64 {
        let sw_over_sx = sw / sx;
        let t_over_w = t / w;

        // Handle edge case where T = W (no spread beyond gutter)
        if (t_over_w - 1.0).abs() < 0.001 {
            return 1.0; // All flow in gutter section
        }

        let denominator_term = (1.0 + sw_over_sx / (t_over_w - 1.0)).powf(2.67) - 1.0;

        // Handle edge case where denominator_term is very small
        if denominator_term.abs() < 1e-10 {
            return 1.0;
        }

        1.0 / (1.0 + sw_over_sx / denominator_term)
    }

    /// Calculate spread width ratio W/T
    #[allow(dead_code)]
    fn width_ratio(&self, spread: f64) -> f64 {
        self.gutter_width / spread
    }

    /// Calculate frontal flow Q_w (flow in depressed gutter section)
    ///
    /// **HEC-22 Equation 5.5:**
    /// ```
    /// Qw = Q × Eo
    /// ```
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Equation 5.5, Example 5.2
    pub fn frontal_flow(&self, total_flow: f64, spread: f64, depression_ft: f64, _k: f64) -> f64 {
        let sw = self.depressed_section_slope(depression_ft);
        let sx = self.roadway_slope;
        let eo = self.flow_ratio_depressed(sw, sx, spread, self.gutter_width);
        total_flow * eo
    }

    /// Calculate side flow Q_s (flow on roadway beyond gutter)
    ///
    /// **HEC-22 Equation 5.6:**
    /// ```
    /// Qs = Q × (1 - Eo)
    /// ```
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Equation 5.6, Example 5.2
    pub fn side_flow(&self, total_flow: f64, spread: f64, depression_ft: f64, _k: f64) -> f64 {
        let sw = self.depressed_section_slope(depression_ft);
        let sx = self.roadway_slope;
        let eo = self.flow_ratio_depressed(sw, sx, spread, self.gutter_width);
        total_flow * (1.0 - eo)
    }

    /// Calculate total flow capacity for composite section
    ///
    /// **Algorithm from HEC-22 Example 5.2, Part A:**
    /// 1. Compute Sw using Equation 5.8
    /// 2. Compute Ts = T - W
    /// 3. Compute Qs using Equation 5.2 with Sx and Ts
    /// 4. Compute Eo using Equation 5.7
    /// 5. Compute Q = Qs / (1 - Eo)
    ///
    /// # Arguments
    /// * `spread` - Total spread T (ft or m)
    /// * `k` - Unit constant Ku (0.56 for US customary, 0.376 for SI)
    ///
    /// # Returns
    /// Total gutter flow capacity (cfs or cms)
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Example 5.2 Part A
    pub fn flow_capacity(&self, spread: f64, k: f64) -> f64 {
        // Convert depression to feet if in inches
        let depression_ft = if self.local_depression < 1.0 {
            self.local_depression // Already in feet
        } else {
            self.local_depression / 12.0 // Convert inches to feet
        };

        // Step 1: Compute cross slope in depressed section (Equation 5.8)
        let sw = self.depressed_section_slope(depression_ft);

        // Step 2: Compute width of spread beyond gutter section
        let ts = spread - self.gutter_width;

        // If spread doesn't extend beyond gutter, use simple calculation
        if ts <= 0.0 {
            // All flow within depressed gutter section
            return (k / self.manning_n)
                * sw.powf(1.67)
                * self.longitudinal_slope.powf(0.5)
                * spread.powf(2.67);
        }

        // Step 3: Compute flow in side section (roadway) using Equation 5.2
        let qs = (k / self.manning_n)
            * self.roadway_slope.powf(1.67)
            * self.longitudinal_slope.powf(0.5)
            * ts.powf(2.67);

        // Step 4: Compute flow ratio Eo using Equation 5.7
        let eo = self.flow_ratio_depressed(sw, self.roadway_slope, spread, self.gutter_width);

        // Step 5: Compute total flow Q = Qs / (1 - Eo)
        if (1.0 - eo).abs() < 1e-10 {
            // All flow in gutter section
            return (k / self.manning_n)
                * sw.powf(1.67)
                * self.longitudinal_slope.powf(0.5)
                * self.gutter_width.powf(2.67);
        }

        qs / (1.0 - eo)
    }

    /// Calculate spread for a given flow rate (iterative)
    ///
    /// Uses bisection method on spread T to find the spread that produces
    /// the target flow. This is more robust than the nested iteration approach
    /// shown in HEC-22 Example 5.2 Part B, but produces equivalent results.
    ///
    /// # Arguments
    /// * `flow` - Total gutter flow Q (cfs or cms)
    /// * `k` - Unit constant Ku (0.56 for US customary, 0.376 for SI)
    ///
    /// # Returns
    /// Total spread T (ft or m)
    ///
    /// # Reference
    /// HEC-22 Chapter 5, Example 5.2 Part B (algorithm simplified for robustness)
    pub fn spread_for_flow(&self, flow: f64, k: f64) -> f64 {
        // Use direct bisection on spread T
        // This is simpler and more robust than the nested iteration in HEC-22
        let mut t_low = self.gutter_width;
        let mut t_high = 50.0;
        let tolerance = 0.01;

        for _ in 0..50 {
            let t_mid = (t_low + t_high) / 2.0;
            let q_mid = self.flow_capacity(t_mid, k);

            if (q_mid - flow).abs() < tolerance {
                return t_mid;
            }

            if q_mid < flow {
                t_low = t_mid;
            } else {
                t_high = t_mid;
            }

            // Check if interval is small enough
            if (t_high - t_low) < 0.01 {
                return t_mid;
            }
        }

        (t_low + t_high) / 2.0
    }

    /// Calculate complete flow result for given spread
    pub fn flow_result(&self, spread: f64, k: f64) -> GutterFlowResult {
        let depression_ft = if self.local_depression < 1.0 {
            self.local_depression
        } else {
            self.local_depression / 12.0
        };

        let flow = self.flow_capacity(spread, k);
        let frontal = self.frontal_flow(flow, spread, depression_ft, k);
        let side = self.side_flow(flow, spread, depression_ft, k);

        // Depth at curb (including depression)
        let depth_at_curb = spread * self.gutter_slope + depression_ft;

        // Approximate area (simplified)
        let area = 0.5 * spread * depth_at_curb;

        // Velocity
        let velocity = if area > 0.0 { flow / area } else { 0.0 };

        GutterFlowResult {
            spread,
            flow,
            depth_at_curb,
            velocity,
            area,
            frontal_flow: Some(frontal),
            side_flow: Some(side),
        }
    }

    /// Calculate spread for given flow
    pub fn result_for_flow(&self, flow: f64, k: f64) -> GutterFlowResult {
        let spread = self.spread_for_flow(flow, k);
        self.flow_result(spread, k)
    }
}

/// Parabolic crown section calculator
///
/// For roadways with parabolic cross-section
pub struct ParabolicCrown {
    /// Manning's roughness coefficient
    pub manning_n: f64,
    /// Crown height h_c (ft)
    pub crown_height: f64,
    /// Width to crown T_c (ft)
    pub width_to_crown: f64,
    /// Longitudinal slope S_L (ft/ft)
    pub longitudinal_slope: f64,
}

impl ParabolicCrown {
    /// Create a new parabolic crown calculator
    pub fn new(
        manning_n: f64,
        crown_height: f64,
        width_to_crown: f64,
        longitudinal_slope: f64,
    ) -> Self {
        Self {
            manning_n,
            crown_height,
            width_to_crown,
            longitudinal_slope,
        }
    }

    /// Calculate equivalent slope at spread T
    ///
    /// For parabolic section: S_x(T) = 2 × h_c × T / T_c²
    fn equivalent_slope_at_spread(&self, spread: f64) -> f64 {
        2.0 * self.crown_height * spread / self.width_to_crown.powi(2)
    }

    /// Calculate flow capacity using parabolic section equation
    ///
    /// This is an approximation using equivalent triangular section
    pub fn flow_capacity(&self, spread: f64, k: f64) -> f64 {
        let sx_equiv = self.equivalent_slope_at_spread(spread);

        (k / self.manning_n)
            * sx_equiv.powf(5.0 / 3.0)
            * self.longitudinal_slope.sqrt()
            * spread.powf(8.0 / 3.0)
    }

    /// Calculate spread for given flow (iterative)
    pub fn spread_for_flow(&self, flow: f64, k: f64) -> f64 {
        let mut t_low = 0.1;
        let mut t_high = self.width_to_crown;
        let tolerance = 0.001;
        let max_iterations = 50;

        for _ in 0..max_iterations {
            let t_mid = (t_low + t_high) / 2.0;
            let q_mid = self.flow_capacity(t_mid, k);

            if (q_mid - flow).abs() < tolerance {
                return t_mid;
            }

            if q_mid < flow {
                t_low = t_mid;
            } else {
                t_high = t_mid;
            }

            if (t_high - t_low) < tolerance {
                return t_mid;
            }
        }

        (t_low + t_high) / 2.0
    }

    /// Calculate complete flow result
    pub fn flow_result(&self, spread: f64, k: f64) -> GutterFlowResult {
        let flow = self.flow_capacity(spread, k);
        let sx_equiv = self.equivalent_slope_at_spread(spread);
        let depth_at_curb = spread * sx_equiv / 2.0; // Approximate

        // Parabolic area: A = (2/3) × T × d
        let area = (2.0 / 3.0) * spread * depth_at_curb;

        let velocity = if area > 0.0 { flow / area } else { 0.0 };

        GutterFlowResult {
            spread,
            flow,
            depth_at_curb,
            velocity,
            area,
            frontal_flow: None,
            side_flow: None,
        }
    }

    /// Calculate spread for given flow
    pub fn result_for_flow(&self, flow: f64, k: f64) -> GutterFlowResult {
        let spread = self.spread_for_flow(flow, k);
        self.flow_result(spread, k)
    }
}

/// Unit constants for gutter equations
pub const GUTTER_K_US: f64 = 0.56;  // US customary units
pub const GUTTER_K_SI: f64 = 0.376; // SI metric units

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 0.01;

    #[test]
    fn test_uniform_gutter_flow_capacity() {
        let gutter = UniformGutter::new(
            0.016,  // Manning's n
            0.02,   // 2% cross slope
            0.01,   // 1% longitudinal slope
            None,
        );

        // Calculate flow for 8 ft spread
        let result = gutter.flow_result(8.0, GUTTER_K_US);

        // Flow should be positive
        assert!(result.flow > 0.0);
        assert!(result.spread - 8.0 < TOLERANCE);

        // Depth at curb = T × Sx = 8 × 0.02 = 0.16 ft
        assert!((result.depth_at_curb - 0.16).abs() < TOLERANCE);
    }

    #[test]
    fn test_uniform_gutter_spread_for_flow() {
        let gutter = UniformGutter::new(
            0.016,
            0.02,
            0.01,
            None,
        );

        // Calculate spread for 3 cfs
        let result = gutter.result_for_flow(3.0, GUTTER_K_US);

        // Verify roundtrip
        let check = gutter.flow_capacity(result.spread, GUTTER_K_US);
        assert!((check - 3.0).abs() < 0.1);
    }

    #[test]
    fn test_composite_gutter() {
        let gutter = CompositeGutter::new(
            0.016,  // Manning's n
            0.04,   // 4% gutter slope
            0.02,   // 2% roadway slope
            0.01,   // 1% longitudinal slope
            2.0,    // 2 ft gutter width
            2.0,    // 2 inch local depression
        );

        let result = gutter.flow_result(10.0, GUTTER_K_US);

        // Should have both frontal and side flow
        assert!(result.frontal_flow.is_some());
        assert!(result.side_flow.is_some());

        let frontal = result.frontal_flow.unwrap();
        let side = result.side_flow.unwrap();

        // Frontal + side should equal total
        assert!((frontal + side - result.flow).abs() < 0.1);

        // Frontal flow should be greater than side flow (steeper slope)
        assert!(frontal > side);
    }

    #[test]
    fn test_parabolic_crown() {
        let crown = ParabolicCrown::new(
            0.016,  // Manning's n
            0.10,   // 0.10 ft crown height
            12.0,   // 12 ft width to crown
            0.01,   // 1% longitudinal slope
        );

        let result = crown.flow_result(8.0, GUTTER_K_US);

        assert!(result.flow > 0.0);
        assert!(result.spread - 8.0 < TOLERANCE);
    }

    #[test]
    fn test_composite_gutter_spread_for_flow() {
        let gutter = CompositeGutter::new(
            0.016,
            0.04,
            0.02,
            0.01,
            2.0,
            2.0,
        );

        // Calculate spread for given flow
        let target_flow = 4.0;
        let result = gutter.result_for_flow(target_flow, GUTTER_K_US);

        // Verify the computed spread produces approximately the target flow
        let check = gutter.flow_capacity(result.spread, GUTTER_K_US);
        assert!((check - target_flow).abs() < 0.1,
            "Expected flow {}, got {}", target_flow, check);
    }
}
