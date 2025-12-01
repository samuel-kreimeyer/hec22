//! Inlet capacity and interception calculations
//!
//! This module implements inlet design procedures from HEC-22 Chapter 7,
//! including capacity calculations for different inlet types and locations.
//!
//! ## Inlet Types
//!
//! - **Grate Inlets**: P-grates, curved vane grates, etc.
//! - **Curb Opening Inlets**: Horizontal, vertical, or inclined throat
//! - **Combination Inlets**: Both grate and curb opening
//! - **Slotted Drains**: Continuous slot along gutter
//!
//! ## Inlet Locations
//!
//! - **On-Grade**: Continuous longitudinal slope (has bypass flow)
//! - **Sag**: Low point in vertical profile (captures all flow)

use crate::gutter::{CompositeGutter, GutterFlowResult, UniformGutter, GUTTER_K_US};

/// Inlet interception result
#[derive(Debug, Clone, PartialEq)]
pub struct InletInterceptionResult {
    /// Total flow approaching the inlet (cfs)
    pub approach_flow: f64,
    /// Flow intercepted by the inlet (cfs)
    pub intercepted_flow: f64,
    /// Bypass flow continuing downstream (cfs)
    pub bypass_flow: f64,
    /// Interception efficiency (0.0 to 1.0)
    pub efficiency: f64,
    /// Spread at inlet (ft)
    pub spread: f64,
    /// Velocity at inlet (ft/s)
    pub velocity: f64,
}

/// Grate inlet on grade
///
/// Follows HEC-22 Section 7.4 procedures for grate inlets on continuous grade
pub struct GrateInletOnGrade {
    /// Grate length parallel to flow (ft)
    pub length: f64,
    /// Grate width perpendicular to flow (ft)
    pub width: f64,
    /// Bar configuration
    pub bar_configuration: BarConfiguration,
    /// Clogging factor (0.0 to 1.0, typically 0.15-0.50)
    pub clogging_factor: f64,
    /// Local depression depth (in)
    pub local_depression: f64,
}

/// Bar configuration for grates
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BarConfiguration {
    /// Bars parallel to flow direction
    Parallel,
    /// Bars perpendicular to flow direction
    Perpendicular,
}

/// Standard grate types with opening ratios from HEC-22 Table 7.5
///
/// Opening ratio is the ratio of clear opening area to total grate area.
/// These values are from FHWA HEC-22 Chapter 7, Table 7.5.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrateType {
    /// P-1-7/8-4: Parallel bar grate, 1-7/8" spacing, 4" width, 80% open
    P1_7_8_4,
    /// P-1-7/8: Parallel bar grate, 1-7/8" spacing, 90% open
    P1_7_8,
    /// P-1-1/8: Parallel bar grate, 1-1/8" spacing, 60% open
    P1_1_8,
    /// Reticuline grate, 80% open
    Reticuline,
    /// Curved vane grate, 35% open
    CurvedVane,
    /// 30° tilt-bar grate, 34% open
    TiltBar30,
    /// Custom grate with specified opening ratio
    Custom(f64),
}

impl GrateType {
    /// Get the opening ratio (clear opening / total area) for this grate type
    ///
    /// Values from HEC-22 Table 7.5
    pub fn opening_ratio(&self) -> f64 {
        match self {
            GrateType::P1_7_8_4 => 0.80,    // P-1-7/8-4: 80% open
            GrateType::P1_7_8 => 0.90,      // P-1-7/8: 90% open
            GrateType::P1_1_8 => 0.60,      // P-1-1/8: 60% open
            GrateType::Reticuline => 0.80,  // Reticuline: 80% open
            GrateType::CurvedVane => 0.35,  // Curved vane: 35% open
            GrateType::TiltBar30 => 0.34,   // 30° tilt-bar: 34% open
            GrateType::Custom(ratio) => *ratio,
        }
    }

    /// Get the designation string for this grate type
    pub fn designation(&self) -> &str {
        match self {
            GrateType::P1_7_8_4 => "P-1-7/8-4",
            GrateType::P1_7_8 => "P-1-7/8",
            GrateType::P1_1_8 => "P-1-1/8",
            GrateType::Reticuline => "Reticuline",
            GrateType::CurvedVane => "Curved Vane",
            GrateType::TiltBar30 => "30° Tilt-Bar",
            GrateType::Custom(_) => "Custom",
        }
    }
}

impl GrateInletOnGrade {
    /// Create a new grate inlet
    pub fn new(
        length: f64,
        width: f64,
        bar_configuration: BarConfiguration,
        clogging_factor: f64,
        local_depression: f64,
    ) -> Self {
        Self {
            length,
            width,
            bar_configuration,
            clogging_factor,
            local_depression,
        }
    }

    /// Calculate frontal flow interception efficiency
    ///
    /// E_f = R_f for V < V_0
    /// E_f = 1 - (1 - R_f)(V/V_0 - 1) for V >= V_0
    ///
    /// where V_0 = 1.79 ft/s for perpendicular bars
    ///       V_0 = 0.49 ft/s for parallel bars (splash-over threshold)
    fn frontal_efficiency(&self, velocity: f64, ratio_frontal: f64) -> f64 {
        let v0 = match self.bar_configuration {
            BarConfiguration::Perpendicular => 1.79,
            BarConfiguration::Parallel => 0.49,
        };

        if velocity < v0 {
            ratio_frontal
        } else {
            let splash_over = (1.0 - ratio_frontal) * (velocity / v0 - 1.0);
            1.0 - splash_over
        }
    }

    /// Calculate side flow interception efficiency
    ///
    /// E_s = K_x × (L/T)^1.8
    ///
    /// where K_x = 0.15 for perpendicular bars
    ///       K_x = 0.09 for parallel bars
    fn side_efficiency(&self, spread: f64) -> f64 {
        let kx = match self.bar_configuration {
            BarConfiguration::Perpendicular => 0.15,
            BarConfiguration::Parallel => 0.09,
        };

        let ratio = (self.length / spread).min(1.0);
        kx * ratio.powf(1.8)
    }

    /// Calculate interception capacity
    ///
    /// Uses composite gutter approach with frontal and side flow
    pub fn interception(
        &self,
        approach_flow: f64,
        gutter_result: &GutterFlowResult,
    ) -> InletInterceptionResult {
        let spread = gutter_result.spread;
        let velocity = gutter_result.velocity;

        // Calculate frontal flow ratio based on gutter type
        // Composite gutters provide frontal_flow, uniform gutters do not
        let ratio_frontal = if let Some(frontal) = gutter_result.frontal_flow {
            // Composite gutter - use the calculated frontal flow ratio
            // This accounts for depression and different slopes
            if gutter_result.flow > 0.0 {
                frontal / gutter_result.flow
            } else {
                0.0
            }
        } else {
            // Uniform cross slope - use HEC-22 equation
            // Eo = 1 - (1 - W/T)^(8/3) for uniform cross slope
            let w_over_t = (self.width / spread).min(1.0);
            1.0 - (1.0 - w_over_t).powf(8.0 / 3.0)
        };

        // Frontal flow efficiency
        let ef = self.frontal_efficiency(velocity, ratio_frontal);

        // Side flow efficiency
        let es = self.side_efficiency(spread);

        // Total efficiency (conservative approach)
        let efficiency_gross = ef + es - ef * es;

        // Apply clogging factor
        let efficiency = efficiency_gross * (1.0 - self.clogging_factor);

        let intercepted_flow = approach_flow * efficiency;
        let bypass_flow = approach_flow - intercepted_flow;

        InletInterceptionResult {
            approach_flow,
            intercepted_flow,
            bypass_flow,
            efficiency,
            spread,
            velocity,
        }
    }

    /// Calculate required length for 100% interception
    ///
    /// L_T = 0.6 × Q^0.42 × S_L^0.3 / (n × S_x^0.6)
    ///
    /// HEC-22 Equation 7-11
    pub fn length_for_total_interception(
        flow: f64,
        manning_n: f64,
        cross_slope: f64,
        longitudinal_slope: f64,
    ) -> f64 {
        0.6 * flow.powf(0.42) * longitudinal_slope.powf(0.3)
            / (manning_n * cross_slope.powf(0.6))
    }
}

/// Curb opening inlet on grade
///
/// Follows HEC-22 Section 7.5 for curb opening inlets
pub struct CurbOpeningInletOnGrade {
    /// Opening length (ft)
    pub length: f64,
    /// Opening height (ft)
    pub height: f64,
    /// Throat type
    pub throat_type: ThroatType,
    /// Clogging factor (0.0 to 1.0)
    pub clogging_factor: f64,
}

/// Throat configuration for curb openings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThroatType {
    /// Horizontal throat (standard)
    Horizontal,
    /// Inclined throat (45 degrees)
    Inclined,
    /// Vertical throat (90 degrees)
    Vertical,
}

impl CurbOpeningInletOnGrade {
    /// Create a new curb opening inlet
    pub fn new(
        length: f64,
        height: f64,
        throat_type: ThroatType,
        clogging_factor: f64,
    ) -> Self {
        Self {
            length,
            height,
            throat_type,
            clogging_factor,
        }
    }

    /// Calculate interception efficiency
    ///
    /// Uses weir flow equation for low flow, orifice for high flow
    pub fn interception(
        &self,
        approach_flow: f64,
        gutter_result: &GutterFlowResult,
    ) -> InletInterceptionResult {
        let depth = gutter_result.depth_at_curb;
        let velocity = gutter_result.velocity;

        // Efficiency based on length and flow conditions
        // Simplified approach - full HEC-22 includes detailed weir/orifice calcs

        // Length efficiency (HEC-22 Figure 7-8)
        let l_t = Self::length_for_total_interception(approach_flow, velocity);
        let efficiency_gross = if self.length >= l_t {
            1.0
        } else {
            1.0 - (1.0 - self.length / l_t).powf(1.8)
        };

        // Apply clogging factor
        let efficiency = efficiency_gross * (1.0 - self.clogging_factor);

        let intercepted_flow = approach_flow * efficiency;
        let bypass_flow = approach_flow - intercepted_flow;

        InletInterceptionResult {
            approach_flow,
            intercepted_flow,
            bypass_flow,
            efficiency,
            spread: gutter_result.spread,
            velocity,
        }
    }

    /// Calculate required length for 100% interception
    ///
    /// L_T = K_u × Q^0.42 / S_L^0.3
    ///
    /// HEC-22 Equation 7-15
    pub fn length_for_total_interception(flow: f64, velocity: f64) -> f64 {
        // Simplified - actual equation depends on throat type
        let ku = 0.6; // Coefficient varies by throat type
        ku * flow.powf(0.42) / velocity.powf(0.3)
    }
}

/// Combination inlet on grade (grate + curb opening)
pub struct CombinationInletOnGrade {
    /// Grate component
    pub grate: GrateInletOnGrade,
    /// Curb opening component
    pub curb_opening: CurbOpeningInletOnGrade,
}

impl CombinationInletOnGrade {
    /// Create a new combination inlet
    pub fn new(grate: GrateInletOnGrade, curb_opening: CurbOpeningInletOnGrade) -> Self {
        Self {
            grate,
            curb_opening,
        }
    }

    /// Calculate interception for combination inlet
    ///
    /// Grate intercepts first, then curb opening intercepts from bypass
    pub fn interception(
        &self,
        approach_flow: f64,
        gutter_result: &GutterFlowResult,
    ) -> InletInterceptionResult {
        // Grate intercepts first
        let grate_result = self.grate.interception(approach_flow, gutter_result);

        // Curb opening intercepts from grate bypass
        if grate_result.bypass_flow > 0.0 {
            let curb_result = self.curb_opening.interception(
                grate_result.bypass_flow,
                gutter_result,
            );

            let total_intercepted = grate_result.intercepted_flow + curb_result.intercepted_flow;
            let total_bypass = curb_result.bypass_flow;
            let total_efficiency = total_intercepted / approach_flow;

            InletInterceptionResult {
                approach_flow,
                intercepted_flow: total_intercepted,
                bypass_flow: total_bypass,
                efficiency: total_efficiency,
                spread: gutter_result.spread,
                velocity: gutter_result.velocity,
            }
        } else {
            grate_result
        }
    }
}

/// Grate inlet in sag (low point)
///
/// At sag locations, all flow ponds and enters the inlet
pub struct GrateInletSag {
    /// Grate length (ft)
    pub length: f64,
    /// Grate width (ft)
    pub width: f64,
    /// Number of grates
    pub count: usize,
    /// Clogging factor
    pub clogging_factor: f64,
}

impl GrateInletSag {
    /// Create a new sag grate inlet
    pub fn new(length: f64, width: f64, count: usize, clogging_factor: f64) -> Self {
        Self {
            length,
            width,
            count,
            clogging_factor,
        }
    }

    /// Calculate capacity using weir and orifice equations
    ///
    /// Q = min(Q_weir, Q_orifice)
    ///
    /// where Q_weir = C_w × P × d^1.5 (low head)
    ///       Q_orifice = C_o × A_net × (2gd)^0.5 (high head)
    ///
    /// # Arguments
    /// * `ponding_depth` - Water depth above grate (ft)
    /// * `opening_ratio` - Grate opening ratio (0.0 to 1.0), typically 0.35-0.90
    ///                     For P-grates: ~0.5-0.6, Curved vane: ~0.80-0.90
    ///                     If None, assumes 100% open area (conservative)
    pub fn capacity_with_opening_ratio(&self, ponding_depth: f64, opening_ratio: Option<f64>) -> f64 {
        // Gross area
        let gross_area = self.length * self.width * self.count as f64;
        let perimeter = 2.0 * (self.length + self.width) * self.count as f64;

        // Apply opening ratio (grate bar reduction)
        let opening_ratio = opening_ratio.unwrap_or(1.0);
        let open_area = gross_area * opening_ratio;

        // Apply clogging factor to open area
        let net_area = open_area * (1.0 - self.clogging_factor);

        // Weir flow (low head)
        let cw = 3.0; // Weir coefficient
        let q_weir = cw * perimeter * ponding_depth.powf(1.5);

        // Orifice flow (high head)
        let co = 0.67; // Orifice coefficient
        let g = 32.17; // ft/s²
        let q_orifice = co * net_area * (2.0 * g * ponding_depth).sqrt();

        // Capacity is minimum of weir and orifice
        q_weir.min(q_orifice)
    }

    /// Calculate capacity using weir and orifice equations (simplified version)
    ///
    /// This version assumes 100% grate opening ratio. For more accurate results
    /// with specific grate types, use `capacity_with_opening_ratio()`.
    pub fn capacity(&self, ponding_depth: f64) -> f64 {
        self.capacity_with_opening_ratio(ponding_depth, None)
    }

    /// Check if flooding occurs (capacity exceeded)
    pub fn check_flooding(&self, design_flow: f64, rim_elevation: f64, invert_elevation: f64) -> (bool, f64) {
        // Iterate to find ponding depth
        let max_depth = rim_elevation - invert_elevation;
        let mut depth = 0.1;
        let increment = 0.1;

        while depth <= max_depth {
            let capacity = self.capacity(depth);
            if capacity >= design_flow {
                return (false, depth); // No flooding
            }
            depth += increment;
        }

        // Flow exceeds capacity even at rim - flooding occurs
        (true, max_depth)
    }

    /// Design a grate size for given flow and allowable ponding depth
    ///
    /// This function follows the HEC-22 Example 7.5 procedure for sizing grates in sag conditions.
    /// The method evaluates both weir and orifice flow regimes and uses the more conservative result
    /// (whichever produces higher depth).
    ///
    /// # Procedure (HEC-22 Example 7.5)
    /// 1. Calculate required perimeter for weir flow (Equation 7.14) without clogging
    /// 2. Apply clogging factor to find effective perimeter needed
    /// 3. Select grate size and verify depth using Equation 7.14
    /// 4. Calculate required area for orifice flow (Equation 7.15) without clogging
    /// 5. Apply clogging factor to find effective area needed
    /// 6. Select grate size and verify depth using Equation 7.15
    /// 7. Use whichever equation produces HIGHER depth (more conservative)
    ///
    /// # Arguments
    /// * `design_flow` - Required flow capacity (cfs)
    /// * `max_ponding_depth` - Maximum allowable ponding depth at curb (ft)
    /// * `grate_width` - Standard grate width (ft), typically 2 or 3 ft
    /// * `clogging_factor` - Expected clogging from curb (0.0 to 1.0), e.g., 0.5 for 50%
    /// * `grate_type` - Grate type with opening ratio from HEC-22 Table 7.5
    ///                  If None, assumes 100% open (overly conservative)
    ///
    /// # Returns
    /// Tuple of (length, count) where:
    /// - length: grate length in feet (whole number)
    /// - count: number of grates needed
    ///
    /// # Example
    /// ```
    /// use hec22::inlet::{GrateInletSag, GrateType};
    ///
    /// // Design for 6.71 cfs with max 0.49 ft ponding, P-1-7/8-4 grate
    /// let (length, count) = GrateInletSag::design_for_sag(
    ///     6.71,                        // design flow
    ///     0.49,                        // max ponding depth at curb
    ///     2.0,                         // grate width
    ///     0.5,                         // 50% clogging from curb
    ///     Some(GrateType::P1_7_8_4)    // P-1-7/8-4 grate (80% open)
    /// );
    /// // Expected: Double 2x3 ft grate (per Example 7.5)
    /// ```
    pub fn design_for_sag(
        design_flow: f64,
        max_ponding_depth: f64,
        grate_width: f64,
        clogging_factor: f64,
        grate_type: Option<GrateType>,
    ) -> (f64, usize) {
        // Constants from HEC-22
        let cw = 3.0;  // Weir coefficient (approximates 0.37 × √(2g))
        let co = 0.67; // Orifice coefficient
        let g = 32.17; // Gravity (ft/s²)

        // Get opening ratio from grate type
        let opening_ratio = grate_type.map(|gt| gt.opening_ratio()).unwrap_or(1.0);

        // Standard grate lengths to try
        let standard_lengths = vec![2.0, 3.0, 4.0, 5.0];

        // ========== STEP 1-2: WEIR FLOW ANALYSIS (Equation 7.14) ==========
        // Equation 7.14: Qi = Cw × P × d^1.5
        // Rearrange to solve for P: P = Qi / (Cw × d^1.5)

        let required_perimeter_unclogged = design_flow / (cw * max_ponding_depth.powf(1.5));

        // Apply clogging to perimeter (clogging reduces effective perimeter from curb side)
        // P_effective = L_total + (1 - clogging_factor) × 2W
        // Rearrange: L_total = P_effective - (1 - clogging_factor) × 2W
        let effective_side_perimeter = (1.0 - clogging_factor) * 2.0 * grate_width;
        let required_total_length_weir = required_perimeter_unclogged - effective_side_perimeter;

        // Select standard grate configuration that meets required total length
        let mut weir_grate: Option<(f64, usize, f64)> = None;

        'weir_search: for count in 1..=10 {
            for &length in &standard_lengths {
                let total_length = length * count as f64;
                if total_length >= required_total_length_weir {
                    // This configuration meets the minimum length requirement
                    let perimeter_effective = total_length + effective_side_perimeter;

                    // Step 3: Verify depth using Equation 7.14 (rearranged)
                    // d = [Qi / (Cw × P)]^(2/3)
                    let depth_weir = (design_flow / (cw * perimeter_effective)).powf(2.0/3.0);

                    if depth_weir <= max_ponding_depth {
                        weir_grate = Some((length, count, depth_weir));
                        break 'weir_search;
                    }
                }
            }
        }

        // ========== STEP 4-5: ORIFICE FLOW ANALYSIS (Equation 7.15) ==========
        // Equation 7.15: Qi = Co × Ag × √(2gd)
        // Rearrange to solve for Ag: Ag = Qi / [Co × √(2gd)]

        let required_area_unclogged = design_flow / (co * (2.0 * g * max_ponding_depth).sqrt());

        // Apply clogging and opening ratio to area
        // Ag_effective = L_total × W × opening_ratio × (1 - clogging_factor)
        // Rearrange: L_total = Ag_effective / [W × opening_ratio × (1 - clogging_factor)]
        let area_factor = grate_width * opening_ratio * (1.0 - clogging_factor);
        let required_total_length_orifice = required_area_unclogged / area_factor;

        // Select standard grate configuration that meets required total length
        let mut orifice_grate: Option<(f64, usize, f64)> = None;

        'orifice_search: for count in 1..=10 {
            for &length in &standard_lengths {
                let total_length = length * count as f64;
                if total_length >= required_total_length_orifice {
                    // This configuration meets the minimum length requirement
                    let total_area = total_length * grate_width;
                    let area_effective = total_area * opening_ratio * (1.0 - clogging_factor);

                    // Step 6: Verify depth using Equation 7.15 (rearranged)
                    // d = [Qi / (Co × Ag)]² / (2g)
                    let depth_orifice = (design_flow / (co * area_effective)).powi(2) / (2.0 * g);

                    if depth_orifice <= max_ponding_depth {
                        orifice_grate = Some((length, count, depth_orifice));
                        break 'orifice_search;
                    }
                }
            }
        }

        // ========== STEP 7: USE MORE CONSERVATIVE RESULT (HIGHER DEPTH) ==========
        match (weir_grate, orifice_grate) {
            (Some((l_w, c_w, d_w)), Some((l_o, c_o, d_o))) => {
                // Both solutions exist - use the one with higher depth (more conservative)
                if d_w > d_o {
                    (l_w, c_w) // Weir is more conservative
                } else {
                    (l_o, c_o) // Orifice is more conservative
                }
            },
            (Some((l_w, c_w, _)), None) => {
                // Only weir solution works
                (l_w, c_w)
            },
            (None, Some((l_o, c_o, _))) => {
                // Only orifice solution works
                (l_o, c_o)
            },
            (None, None) => {
                // No standard size works - calculate custom length using more restrictive equation
                // Use the equation that requires longer total length (more conservative)
                let length_required = required_total_length_weir.max(required_total_length_orifice);
                (length_required.ceil(), 1)
            }
        }
    }
}

/// Curb opening inlet in sag
pub struct CurbOpeningInletSag {
    /// Opening length (ft)
    pub length: f64,
    /// Opening height (ft)
    pub height: f64,
    /// Throat type
    pub throat_type: ThroatType,
    /// Clogging factor
    pub clogging_factor: f64,
}

impl CurbOpeningInletSag {
    /// Create a new sag curb opening inlet
    pub fn new(
        length: f64,
        height: f64,
        throat_type: ThroatType,
        clogging_factor: f64,
    ) -> Self {
        Self {
            length,
            height,
            throat_type,
            clogging_factor,
        }
    }

    /// Calculate capacity
    ///
    /// Uses weir and orifice equations similar to grate
    pub fn capacity(&self, ponding_depth: f64) -> f64 {
        let net_length = self.length * (1.0 - self.clogging_factor);

        // Weir flow
        let cw = 2.3; // Weir coefficient for curb opening
        let q_weir = cw * net_length * ponding_depth.powf(1.5);

        // Orifice flow
        let area = net_length * self.height;
        let co = 0.67;
        let g = 32.17;
        let q_orifice = co * area * (2.0 * g * ponding_depth).sqrt();

        q_weir.min(q_orifice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grate_inlet_on_grade() {
        let inlet = GrateInletOnGrade::new(
            3.0,  // 3 ft length
            2.0,  // 2 ft width
            BarConfiguration::Perpendicular,
            0.15, // 15% clogging
            2.0,  // 2 inch depression
        );

        // Create gutter result
        let gutter = UniformGutter::new(0.016, 0.02, 0.01, None);
        let gutter_result = gutter.result_for_flow(4.0, GUTTER_K_US);

        let result = inlet.interception(4.0, &gutter_result);

        // Should intercept some flow
        assert!(result.intercepted_flow > 0.0);
        assert!(result.bypass_flow >= 0.0);
        assert!((result.intercepted_flow + result.bypass_flow - 4.0).abs() < 0.01);
        assert!(result.efficiency > 0.0 && result.efficiency <= 1.0);
    }

    #[test]
    fn test_curb_opening_on_grade() {
        let inlet = CurbOpeningInletOnGrade::new(
            5.0,  // 5 ft length
            0.5,  // 6 inch height
            ThroatType::Horizontal,
            0.10,
        );

        let gutter = UniformGutter::new(0.016, 0.02, 0.01, None);
        let gutter_result = gutter.result_for_flow(3.0, GUTTER_K_US);

        let result = inlet.interception(3.0, &gutter_result);

        assert!(result.intercepted_flow > 0.0);
        assert!(result.bypass_flow >= 0.0);
        assert!((result.intercepted_flow + result.bypass_flow - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_combination_inlet() {
        let grate = GrateInletOnGrade::new(
            2.0,
            1.5,
            BarConfiguration::Perpendicular,
            0.15,
            2.0,
        );

        let curb = CurbOpeningInletOnGrade::new(
            3.0,
            0.5,
            ThroatType::Horizontal,
            0.10,
        );

        let combo = CombinationInletOnGrade::new(grate, curb);

        let gutter = UniformGutter::new(0.016, 0.02, 0.01, None);
        let gutter_result = gutter.result_for_flow(5.0, GUTTER_K_US);

        let result = combo.interception(5.0, &gutter_result);

        // Combination should intercept more than either alone
        assert!(result.efficiency > 0.0);
        assert!(result.bypass_flow < 5.0);
    }

    #[test]
    fn test_grate_inlet_sag() {
        let inlet = GrateInletSag::new(
            3.0,  // 3 ft length
            2.0,  // 2 ft width
            1,    // 1 grate
            0.50, // 50% clogging
        );

        // Test capacity at different depths
        let capacity_6in = inlet.capacity(0.5);
        let capacity_12in = inlet.capacity(1.0);

        assert!(capacity_12in > capacity_6in);
        assert!(capacity_6in > 0.0);
    }

    #[test]
    fn test_100_percent_interception_length() {
        let flow = 5.0;
        let n = 0.016;
        let sx = 0.02;
        let sl = 0.01;

        let lt = GrateInletOnGrade::length_for_total_interception(flow, n, sx, sl);

        // Length should be positive and reasonable
        // At mild slopes and moderate flows, this can be 200-500 ft
        assert!(lt > 0.0);
        assert!(lt < 1000.0); // Sanity check
    }

    #[test]
    fn test_grate_inlet_with_composite_gutter() {
        // Test that grate inlet correctly uses composite gutter frontal flow
        let inlet = GrateInletOnGrade::new(
            3.0,  // 3 ft length
            2.0,  // 2 ft width
            BarConfiguration::Perpendicular,
            0.15, // 15% clogging
            2.0,  // 2 inch depression
        );

        // Create composite gutter result
        let composite_gutter = CompositeGutter::new(
            0.016,  // Manning's n
            0.04,   // 4% gutter slope
            0.02,   // 2% roadway slope
            0.01,   // 1% longitudinal slope
            2.0,    // 2 ft gutter width
            2.0,    // 2 inch local depression
        );

        let gutter_result = composite_gutter.result_for_flow(4.0, GUTTER_K_US);

        // Verify that the composite gutter result has frontal flow populated
        assert!(gutter_result.frontal_flow.is_some());
        assert!(gutter_result.side_flow.is_some());

        let result = inlet.interception(4.0, &gutter_result);

        // Should intercept some flow
        assert!(result.intercepted_flow > 0.0);
        assert!(result.bypass_flow >= 0.0);
        assert!((result.intercepted_flow + result.bypass_flow - 4.0).abs() < 0.01);
        assert!(result.efficiency > 0.0 && result.efficiency <= 1.0);

        // For composite gutters, the frontal flow ratio should be higher than
        // the simple W/T ratio due to flow concentration near the curb
        let frontal = gutter_result.frontal_flow.unwrap();
        let simple_ratio = inlet.width / gutter_result.spread;
        let composite_ratio = frontal / gutter_result.flow;
        assert!(composite_ratio > simple_ratio,
            "Composite gutter frontal ratio ({}) should be > simple W/T ({})",
            composite_ratio, simple_ratio);
    }
}
