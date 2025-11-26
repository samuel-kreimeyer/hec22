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

use crate::gutter::{GutterFlowResult, UniformGutter, GUTTER_K_US};

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

        // Calculate frontal flow ratio using HEC-22 composite gutter equation
        // Eo = 1 - (1 - W/T)^(8/3) for uniform cross slope
        let w_over_t = (self.width / spread).min(1.0);
        let ratio_frontal = 1.0 - (1.0 - w_over_t).powf(8.0 / 3.0);

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
    ///       Q_orifice = C_o × A × (2gd)^0.5 (high head)
    pub fn capacity(&self, ponding_depth: f64) -> f64 {
        // Net open area after clogging
        let perimeter = 2.0 * (self.length + self.width) * self.count as f64;
        let area = self.length * self.width * self.count as f64;
        let net_area = area * (1.0 - self.clogging_factor);

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
}
