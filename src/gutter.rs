//! Gutter flow and spread calculations
//!
//! This module implements gutter spread calculations from HEC-22 Chapter 5,
//! including uniform cross-slope gutters, composite gutter sections, and
//! parabolic crowned sections.
//!
//! ## Gutter Types
//!
//! 1. **Uniform Cross-Slope**: Simple triangular section with constant slope
//! 2. **Composite Section**: Gutter section + roadway with different slopes
//! 3. **Parabolic Crown**: Curved roadway surface
//!
//! ## Key Equations
//!
//! For triangular sections:
//! - Q = (0.56/n) × S_x^(5/3) × S_L^(1/2) × T^(8/3)
//!
//! Where:
//! - Q = flow rate (cfs)
//! - n = Manning's roughness
//! - S_x = cross slope (ft/ft)
//! - S_L = longitudinal slope (ft/ft)
//! - T = spread (ft)

use std::f64::consts::PI;

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
    /// Q = (K/n) × S_x^(5/3) × S_L^(1/2) × T^(8/3)
    /// where K = 0.56 for US customary, 0.376 for SI
    ///
    /// # Arguments
    /// * `spread` - Spread from curb (ft or m)
    /// * `k` - Unit constant (0.56 for US, 0.376 for SI)
    pub fn flow_capacity(&self, spread: f64, k: f64) -> f64 {
        (k / self.manning_n)
            * self.cross_slope.powf(5.0 / 3.0)
            * self.longitudinal_slope.sqrt()
            * spread.powf(8.0 / 3.0)
    }

    /// Calculate spread for a given flow rate
    ///
    /// Solves: T = [Q × n / (K × S_x^(5/3) × S_L^(1/2))]^(3/8)
    ///
    /// # Arguments
    /// * `flow` - Flow rate (cfs or cms)
    /// * `k` - Unit constant (0.56 for US, 0.376 for SI)
    pub fn spread_for_flow(&self, flow: f64, k: f64) -> f64 {
        let numerator = flow * self.manning_n;
        let denominator = k * self.cross_slope.powf(5.0 / 3.0) * self.longitudinal_slope.sqrt();
        (numerator / denominator).powf(3.0 / 8.0)
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
/// For sections with a depressed gutter section and roadway with different slopes
pub struct CompositeGutter {
    /// Manning's roughness coefficient
    pub manning_n: f64,
    /// Gutter cross slope S_x (ft/ft)
    pub gutter_slope: f64,
    /// Roadway cross slope S_w (ft/ft)
    pub roadway_slope: f64,
    /// Longitudinal slope S_L (ft/ft)
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

    /// Calculate equivalent cross slope S_x'
    ///
    /// S_x' = S_x + a/W
    /// where a is local depression (converted to ft) and W is gutter width
    fn equivalent_cross_slope(&self, depression_ft: f64) -> f64 {
        self.gutter_slope + (depression_ft / self.gutter_width)
    }

    /// Calculate flow efficiency ratio E_o
    ///
    /// E_o = (1 + S_w/S_x')^(8/3) / [1 + (S_w/S_x')^(8/3)]
    fn flow_efficiency_ratio(&self, sx_prime: f64) -> f64 {
        let ratio = self.roadway_slope / sx_prime;
        let term = (1.0 + ratio).powf(8.0 / 3.0);
        term / (1.0 + ratio.powf(8.0 / 3.0))
    }

    /// Calculate spread width ratio W/T
    fn width_ratio(&self, spread: f64) -> f64 {
        self.gutter_width / spread
    }

    /// Calculate frontal flow Q_w (flow in gutter section)
    ///
    /// Q_w = Q × E_o
    pub fn frontal_flow(&self, total_flow: f64, spread: f64, depression_ft: f64, k: f64) -> f64 {
        let sx_prime = self.equivalent_cross_slope(depression_ft);
        let eo = self.flow_efficiency_ratio(sx_prime);
        total_flow * eo
    }

    /// Calculate side flow Q_s (flow on roadway)
    ///
    /// Q_s = Q × (1 - E_o)
    pub fn side_flow(&self, total_flow: f64, spread: f64, depression_ft: f64, k: f64) -> f64 {
        let sx_prime = self.equivalent_cross_slope(depression_ft);
        let eo = self.flow_efficiency_ratio(sx_prime);
        total_flow * (1.0 - eo)
    }

    /// Calculate total flow capacity for composite section
    ///
    /// Uses modified gutter equation accounting for composite geometry
    pub fn flow_capacity(&self, spread: f64, k: f64) -> f64 {
        // Convert depression to feet if in inches
        let depression_ft = if self.local_depression < 1.0 {
            self.local_depression // Already in feet
        } else {
            self.local_depression / 12.0 // Convert inches to feet
        };

        let sx_prime = self.equivalent_cross_slope(depression_ft);

        // Calculate spread ratio
        let w_over_t = self.width_ratio(spread);

        // Calculate efficiency for spread ratio
        let sw_over_sx = self.roadway_slope / sx_prime;

        // Total flow using composite section equation
        let q_total = (k / self.manning_n)
            * sx_prime.powf(5.0 / 3.0)
            * self.longitudinal_slope.sqrt()
            * spread.powf(8.0 / 3.0)
            * (1.0 + sw_over_sx.powf(8.0 / 3.0) - (w_over_t).powf(8.0 / 3.0) * sw_over_sx.powf(8.0 / 3.0));

        q_total
    }

    /// Calculate spread for a given flow rate (iterative)
    pub fn spread_for_flow(&self, flow: f64, k: f64) -> f64 {
        // Iterative solution using bisection
        let mut t_low = self.gutter_width;
        let mut t_high = 50.0; // Maximum spread assumption
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
