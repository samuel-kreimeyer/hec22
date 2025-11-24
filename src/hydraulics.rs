//! Hydraulic calculations for drainage network analysis
//!
//! This module implements the hydraulic calculation methods from HEC-22 Chapter 9,
//! including Manning's equation, energy grade line (EGL) and hydraulic grade line (HGL)
//! calculations, and various energy loss computations.
//!
//! ## Key Concepts
//!
//! - **HGL (Hydraulic Grade Line)**: Water surface elevation in open channels;
//!   in closed conduits, the height water would rise in a piezometer
//! - **EGL (Energy Grade Line)**: Total energy line = HGL + velocity head (V²/2g)
//! - **Energy Losses**: Friction, entrance, exit, bend, and junction losses
//!
//! ## References
//!
//! FHWA HEC-22 (4th Edition, 2024), Chapter 9: Storm Drain Conduits

use std::f64::consts::PI;

/// Gravitational acceleration constant
pub const GRAVITY_US: f64 = 32.17; // ft/s²
pub const GRAVITY_SI: f64 = 9.81;  // m/s²

/// Manning's equation constants
pub const MANNING_CONST_US: f64 = 1.486; // US customary
pub const MANNING_CONST_SI: f64 = 1.0;   // SI metric

/// Pipe flow result
#[derive(Debug, Clone, PartialEq)]
pub struct PipeFlowResult {
    /// Flow rate (cfs or cms)
    pub flow: f64,
    /// Flow depth (ft or m)
    pub depth: f64,
    /// Flow area (sq ft or sq m)
    pub area: f64,
    /// Wetted perimeter (ft or m)
    pub perimeter: f64,
    /// Hydraulic radius (ft or m)
    pub hydraulic_radius: f64,
    /// Velocity (ft/s or m/s)
    pub velocity: f64,
    /// Velocity head V²/(2g) (ft or m)
    pub velocity_head: f64,
    /// Depth ratio (y/D)
    pub depth_ratio: f64,
    /// Whether pipe is flowing full
    pub is_full_flow: bool,
}

/// Flow regime classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowRegime {
    /// Subcritical flow (Fr < 1)
    Subcritical,
    /// Critical flow (Fr ≈ 1)
    Critical,
    /// Supercritical flow (Fr > 1)
    Supercritical,
}

/// Manning's equation calculations
pub struct ManningsEquation {
    /// Manning's constant (1.486 for US, 1.0 for SI)
    pub k: f64,
}

impl ManningsEquation {
    /// Create for US customary units
    pub fn us_customary() -> Self {
        Self { k: MANNING_CONST_US }
    }

    /// Create for SI metric units
    pub fn si_metric() -> Self {
        Self { k: MANNING_CONST_SI }
    }

    /// Calculate full pipe flow capacity using Manning's equation
    ///
    /// Q = (K/n) × A × R^(2/3) × S^(1/2)
    ///
    /// # Arguments
    /// * `diameter` - Pipe diameter (ft or m)
    /// * `slope` - Pipe slope (ft/ft or m/m)
    /// * `manning_n` - Manning's roughness coefficient
    ///
    /// # Returns
    /// Flow capacity (cfs or cms)
    pub fn full_pipe_capacity(
        &self,
        diameter: f64,
        slope: f64,
        manning_n: f64,
    ) -> f64 {
        let area = PI * diameter.powi(2) / 4.0;
        let perimeter = PI * diameter;
        let hydraulic_radius = area / perimeter; // For circle: D/4

        (self.k / manning_n) * area * hydraulic_radius.powf(2.0 / 3.0) * slope.sqrt()
    }

    /// Calculate velocity in full pipe
    ///
    /// V = Q / A
    pub fn full_pipe_velocity(&self, diameter: f64, flow: f64) -> f64 {
        let area = PI * diameter.powi(2) / 4.0;
        flow / area
    }

    /// Calculate partial flow in circular pipe given depth
    ///
    /// Uses geometric relationships for circular sections
    ///
    /// # Arguments
    /// * `diameter` - Pipe diameter (ft or m)
    /// * `depth` - Flow depth (ft or m)
    /// * `slope` - Pipe slope (ft/ft or m/m)
    /// * `manning_n` - Manning's roughness coefficient
    /// * `gravity` - Gravitational constant (32.17 ft/s² or 9.81 m/s²)
    pub fn partial_pipe_flow(
        &self,
        diameter: f64,
        depth: f64,
        slope: f64,
        manning_n: f64,
        gravity: f64,
    ) -> PipeFlowResult {
        let radius = diameter / 2.0;
        let depth_ratio = depth / diameter;

        // Handle edge cases
        if depth <= 0.0 {
            return self.empty_pipe_result(diameter);
        }
        if depth >= diameter {
            return self.full_pipe_flow_result(diameter, slope, manning_n, gravity);
        }

        // Central angle (radians)
        let theta = 2.0 * ((radius - depth) / radius).acos();

        // Flow area
        let area = (radius.powi(2) / 2.0) * (theta - theta.sin());

        // Wetted perimeter
        let perimeter = radius * theta;

        // Hydraulic radius
        let hydraulic_radius = area / perimeter;

        // Flow rate using Manning's equation
        let flow = (self.k / manning_n) * area * hydraulic_radius.powf(2.0 / 3.0) * slope.sqrt();

        // Velocity
        let velocity = flow / area;

        // Velocity head
        let velocity_head = velocity.powi(2) / (2.0 * gravity);

        PipeFlowResult {
            flow,
            depth,
            area,
            perimeter,
            hydraulic_radius,
            velocity,
            velocity_head,
            depth_ratio,
            is_full_flow: false,
        }
    }

    /// Helper: Create result for empty pipe
    fn empty_pipe_result(&self, diameter: f64) -> PipeFlowResult {
        PipeFlowResult {
            flow: 0.0,
            depth: 0.0,
            area: 0.0,
            perimeter: 0.0,
            hydraulic_radius: 0.0,
            velocity: 0.0,
            velocity_head: 0.0,
            depth_ratio: 0.0,
            is_full_flow: false,
        }
    }

    /// Helper: Create result for full pipe
    fn full_pipe_flow_result(
        &self,
        diameter: f64,
        slope: f64,
        manning_n: f64,
        gravity: f64,
    ) -> PipeFlowResult {
        let area = PI * diameter.powi(2) / 4.0;
        let perimeter = PI * diameter;
        let hydraulic_radius = diameter / 4.0;
        let flow = self.full_pipe_capacity(diameter, slope, manning_n);
        let velocity = flow / area;
        let velocity_head = velocity.powi(2) / (2.0 * gravity);

        PipeFlowResult {
            flow,
            depth: diameter,
            area,
            perimeter,
            hydraulic_radius,
            velocity,
            velocity_head,
            depth_ratio: 1.0,
            is_full_flow: true,
        }
    }

    /// Calculate normal depth for given flow in circular pipe
    ///
    /// Iteratively solves Manning's equation for depth that produces the given flow.
    /// Uses bisection method.
    ///
    /// # Arguments
    /// * `flow` - Target flow rate (cfs or cms)
    /// * `diameter` - Pipe diameter (ft or m)
    /// * `slope` - Pipe slope (ft/ft or m/m)
    /// * `manning_n` - Manning's roughness coefficient
    /// * `gravity` - Gravitational constant
    ///
    /// # Returns
    /// Normal depth (ft or m), or None if no solution exists
    pub fn normal_depth(
        &self,
        flow: f64,
        diameter: f64,
        slope: f64,
        manning_n: f64,
        gravity: f64,
    ) -> Option<f64> {
        // Check if flow exceeds full pipe capacity
        let q_full = self.full_pipe_capacity(diameter, slope, manning_n);
        if flow > q_full {
            return Some(diameter); // Pressurized flow
        }

        // Bisection method
        let mut y_low = 0.0001 * diameter;
        let mut y_high = diameter;
        let tolerance = 0.0001;
        let max_iterations = 50;

        for _ in 0..max_iterations {
            let y_mid = (y_low + y_high) / 2.0;
            let result = self.partial_pipe_flow(diameter, y_mid, slope, manning_n, gravity);
            let q_mid = result.flow;

            if (q_mid - flow).abs() < tolerance {
                return Some(y_mid);
            }

            if q_mid < flow {
                y_low = y_mid;
            } else {
                y_high = y_mid;
            }

            if (y_high - y_low) < tolerance {
                return Some(y_mid);
            }
        }

        Some((y_low + y_high) / 2.0)
    }

    /// Calculate critical depth for circular pipe
    ///
    /// Critical depth occurs when Froude number = 1
    /// At critical flow: Q²/g = A³/T, where T is top width
    ///
    /// # Arguments
    /// * `flow` - Flow rate (cfs or cms)
    /// * `diameter` - Pipe diameter (ft or m)
    /// * `gravity` - Gravitational constant
    ///
    /// # Returns
    /// Critical depth (ft or m)
    pub fn critical_depth(
        &self,
        flow: f64,
        diameter: f64,
        gravity: f64,
    ) -> Option<f64> {
        // Iteratively solve for yc where Fr = 1
        let radius = diameter / 2.0;
        let mut y_low = 0.0001 * diameter;
        let mut y_high = diameter;
        let tolerance = 0.0001;
        let max_iterations = 50;

        for _ in 0..max_iterations {
            let y_mid = (y_low + y_high) / 2.0;

            // Calculate area and top width
            let theta = 2.0 * ((radius - y_mid) / radius).acos();
            let area = (radius.powi(2) / 2.0) * (theta - theta.sin());
            let top_width = 2.0 * (radius.powi(2) - (radius - y_mid).powi(2)).sqrt();

            // Critical flow condition: Q² = g * A³ / T
            let lhs = flow.powi(2);
            let rhs = gravity * area.powi(3) / top_width;

            if (lhs - rhs).abs() < tolerance * lhs {
                return Some(y_mid);
            }

            if lhs > rhs {
                y_low = y_mid;
            } else {
                y_high = y_mid;
            }
        }

        Some((y_low + y_high) / 2.0)
    }

    /// Calculate Froude number
    ///
    /// Fr = V / sqrt(g * D_h)
    /// where D_h = hydraulic depth = A / T (area / top width)
    ///
    /// # Returns
    /// Froude number (dimensionless)
    pub fn froude_number(
        &self,
        velocity: f64,
        area: f64,
        top_width: f64,
        gravity: f64,
    ) -> f64 {
        let hydraulic_depth = area / top_width;
        velocity / (gravity * hydraulic_depth).sqrt()
    }

    /// Classify flow regime based on Froude number
    pub fn flow_regime(&self, froude_number: f64) -> FlowRegime {
        const CRITICAL_TOLERANCE: f64 = 0.05;

        if (froude_number - 1.0).abs() < CRITICAL_TOLERANCE {
            FlowRegime::Critical
        } else if froude_number < 1.0 {
            FlowRegime::Subcritical
        } else {
            FlowRegime::Supercritical
        }
    }
}

/// Energy loss calculations
pub struct EnergyLoss {
    /// Gravitational constant
    pub gravity: f64,
}

impl EnergyLoss {
    /// Create for US customary units
    pub fn us_customary() -> Self {
        Self { gravity: GRAVITY_US }
    }

    /// Create for SI metric units
    pub fn si_metric() -> Self {
        Self { gravity: GRAVITY_SI }
    }

    /// Calculate friction loss using Manning's equation
    ///
    /// h_f = S_f × L
    /// where S_f = [(Q × n) / (K × A × R^(2/3))]²
    ///
    /// # Arguments
    /// * `flow` - Flow rate (cfs or cms)
    /// * `length` - Conduit length (ft or m)
    /// * `area` - Flow area (sq ft or sq m)
    /// * `hydraulic_radius` - Hydraulic radius (ft or m)
    /// * `manning_n` - Manning's roughness coefficient
    /// * `k` - Manning's constant (1.486 for US, 1.0 for SI)
    ///
    /// # Returns
    /// Friction loss (ft or m)
    pub fn friction_loss(
        &self,
        flow: f64,
        length: f64,
        area: f64,
        hydraulic_radius: f64,
        manning_n: f64,
        k: f64,
    ) -> f64 {
        let sf = ((flow * manning_n) / (k * area * hydraulic_radius.powf(2.0 / 3.0))).powi(2);
        sf * length
    }

    /// Calculate entrance loss
    ///
    /// H_e = K_e × (V²/2g)
    ///
    /// Typical K_e values:
    /// - Square edge: 0.5
    /// - Bell mouth: 0.05
    /// - Projecting: 0.9
    ///
    /// # Arguments
    /// * `velocity` - Velocity (ft/s or m/s)
    /// * `k_entrance` - Entrance loss coefficient
    pub fn entrance_loss(&self, velocity: f64, k_entrance: f64) -> f64 {
        k_entrance * velocity.powi(2) / (2.0 * self.gravity)
    }

    /// Calculate exit loss
    ///
    /// H_exit = K_exit × (V_upstream²/2g - V_downstream²/2g)
    ///
    /// Typically K_exit = 1.0 (sudden expansion)
    ///
    /// # Arguments
    /// * `v_upstream` - Upstream velocity (ft/s or m/s)
    /// * `v_downstream` - Downstream velocity (ft/s or m/s)
    /// * `k_exit` - Exit loss coefficient (typically 1.0)
    pub fn exit_loss(
        &self,
        v_upstream: f64,
        v_downstream: f64,
        k_exit: f64,
    ) -> f64 {
        let vh_up = v_upstream.powi(2) / (2.0 * self.gravity);
        let vh_down = v_downstream.powi(2) / (2.0 * self.gravity);
        k_exit * (vh_up - vh_down).max(0.0)
    }

    /// Calculate bend loss
    ///
    /// H_b = K_b × (V²/2g)
    /// where K_b = 0.0033 × Δ (for Δ in degrees)
    ///
    /// # Arguments
    /// * `velocity` - Velocity (ft/s or m/s)
    /// * `bend_angle` - Bend angle in degrees
    pub fn bend_loss(&self, velocity: f64, bend_angle: f64) -> f64 {
        let k_bend = 0.0033 * bend_angle;
        k_bend * velocity.powi(2) / (2.0 * self.gravity)
    }

    /// Calculate junction loss using K-method (approximate)
    ///
    /// H_j = K × (V_outlet²/2g)
    ///
    /// Typical K values:
    /// - Straight through: 0.05 - 0.15
    /// - 45° bend: 0.25 - 0.50
    /// - 90° bend: 1.00 - 1.50
    ///
    /// # Arguments
    /// * `v_outlet` - Outlet pipe velocity (ft/s or m/s)
    /// * `k_junction` - Junction loss coefficient
    pub fn junction_loss_k_method(&self, v_outlet: f64, k_junction: f64) -> f64 {
        k_junction * v_outlet.powi(2) / (2.0 * self.gravity)
    }

    /// Calculate total head loss for a conduit
    ///
    /// H_total = H_friction + H_entrance + H_exit + H_bend
    pub fn total_conduit_loss(
        &self,
        friction: f64,
        entrance: f64,
        exit: f64,
        bend: f64,
    ) -> f64 {
        friction + entrance + exit + bend
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 0.01;

    #[test]
    fn test_full_pipe_capacity() {
        let mannings = ManningsEquation::us_customary();

        // 18-inch RCP pipe, 1% slope, n=0.013
        let diameter = 1.5; // ft
        let slope = 0.01;
        let n = 0.013;

        let q_full = mannings.full_pipe_capacity(diameter, slope, n);

        // Expected: approximately 10.5 cfs for 18" pipe at 1% slope
        assert!((q_full - 10.5).abs() < 0.5, "Q_full = {}", q_full);
    }

    #[test]
    fn test_full_pipe_velocity() {
        let mannings = ManningsEquation::us_customary();

        let diameter = 1.5; // ft (18 inches)
        let flow = 10.5; // cfs

        let velocity = mannings.full_pipe_velocity(diameter, flow);

        // V = Q/A = 10.5 / (π × 1.5²/4) ≈ 5.9 ft/s
        assert!((velocity - 5.9).abs() < 0.2, "V = {}", velocity);
    }

    #[test]
    fn test_partial_pipe_flow() {
        let mannings = ManningsEquation::us_customary();

        let diameter = 1.5; // ft
        let depth = 0.75; // ft (half full)
        let slope = 0.01;
        let n = 0.013;

        let result = mannings.partial_pipe_flow(
            diameter,
            depth,
            slope,
            n,
            GRAVITY_US,
        );

        // Half-full pipe flows at about 50% of full capacity
        assert!(result.depth_ratio - 0.5 < TOLERANCE);
        assert!(!result.is_full_flow);
        assert!(result.flow > 0.0);
    }

    #[test]
    fn test_normal_depth() {
        let mannings = ManningsEquation::us_customary();

        let flow = 2.0; // cfs
        let diameter = 1.5; // ft
        let slope = 0.01;
        let n = 0.013;

        let yn = mannings.normal_depth(flow, diameter, slope, n, GRAVITY_US);

        assert!(yn.is_some());
        let depth = yn.unwrap();

        // Normal depth should be less than diameter for partial flow
        assert!(depth < diameter);
        assert!(depth > 0.0);

        // Verify the depth produces approximately the desired flow
        let check = mannings.partial_pipe_flow(diameter, depth, slope, n, GRAVITY_US);
        assert!((check.flow - flow).abs() < 0.01,
            "Expected flow {}, got {}", flow, check.flow);
    }

    #[test]
    fn test_critical_depth() {
        let mannings = ManningsEquation::us_customary();

        let flow = 2.0; // cfs
        let diameter = 1.5; // ft

        let yc = mannings.critical_depth(flow, diameter, GRAVITY_US);

        assert!(yc.is_some());
        let depth = yc.unwrap();

        // Critical depth should be positive and less than diameter
        assert!(depth > 0.0);
        assert!(depth < diameter);
    }

    #[test]
    fn test_friction_loss() {
        let energy_loss = EnergyLoss::us_customary();

        let flow = 3.0; // cfs
        let length = 100.0; // ft
        let area = 1.767; // sq ft (18-inch pipe)
        let hydraulic_radius = 0.375; // ft (D/4)
        let n = 0.013;

        let hf = energy_loss.friction_loss(
            flow,
            length,
            area,
            hydraulic_radius,
            n,
            MANNING_CONST_US,
        );

        // Friction loss should be positive
        assert!(hf > 0.0);
        assert!(hf < 10.0); // Reasonable upper bound
    }

    #[test]
    fn test_entrance_loss() {
        let energy_loss = EnergyLoss::us_customary();

        let velocity = 3.0; // ft/s
        let k_entrance = 0.5; // Square edge

        let he = energy_loss.entrance_loss(velocity, k_entrance);

        // H_e = 0.5 × 3²/(2×32.17) ≈ 0.07 ft
        assert!((he - 0.07).abs() < 0.01, "H_e = {}", he);
    }

    #[test]
    fn test_flow_regime_classification() {
        let mannings = ManningsEquation::us_customary();

        assert_eq!(
            mannings.flow_regime(0.5),
            FlowRegime::Subcritical
        );

        assert_eq!(
            mannings.flow_regime(1.0),
            FlowRegime::Critical
        );

        assert_eq!(
            mannings.flow_regime(2.0),
            FlowRegime::Supercritical
        );
    }
}
