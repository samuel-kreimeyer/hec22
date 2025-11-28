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
    /// **HEC-22 Equation 9.2: Flow Rate in Full Flow Pipe**
    ///
    /// ```text
    /// Q = (K_Q/n)D^2.67 S_o^0.5
    /// ```
    ///
    /// Where:
    /// - Q = Rate of flow, ft³/s (m³/s)
    /// - K_Q = Unit conversion constant, 0.46 in CU (0.312 in SI)
    /// - n = Manning's roughness coefficient
    /// - D = Storm drain diameter, ft (m)
    /// - S_o = Slope of the energy grade line, ft/ft (m/m)
    ///
    /// Note: This implementation uses the standard form Q = (K/n) × A × R^(2/3) × S^(1/2),
    /// which is mathematically equivalent to Equation 9.2.
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
    /// **HEC-22 Equation 9.1: Mean Velocity in Full Flow Pipe**
    ///
    /// ```text
    /// V = (K_V/n)D^0.67 S_o^0.5
    /// ```
    ///
    /// Where:
    /// - V = Mean velocity, ft/s (m/s)
    /// - K_V = Unit conversion constant, 0.59 in CU (0.397 in SI)
    /// - n = Manning's roughness coefficient
    /// - D = Storm drain diameter, ft (m)
    /// - S_o = Slope of the energy grade line, ft/ft (m/m)
    ///
    /// Note: This implementation calculates velocity as V = Q / A.
    ///
    /// # Arguments
    /// * `diameter` - Pipe diameter (ft or m)
    /// * `flow` - Flow rate (cfs or cms)
    ///
    /// # Returns
    /// Velocity (ft/s or m/s)
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
    /// **HEC-22 Equation 9.3: Head Loss Due to Friction**
    ///
    /// ```text
    /// h_f = S_f L
    /// ```
    ///
    /// Where:
    /// - h_f = Friction loss, ft (m)
    /// - S_f = Friction slope, ft/ft (m/m)
    /// - L = Length of pipe, ft (m)
    ///
    /// **HEC-22 Equation 9.4: Friction Slope for Full Flow**
    ///
    /// ```text
    /// S_f = (h_f/L) = (Qn/(K_Q D^2.67))^2
    /// ```
    ///
    /// Where:
    /// - Q = Rate of flow, ft³/s (m³/s)
    /// - n = Manning's roughness coefficient
    /// - K_Q = Unit conversion constant, 0.46 in CU (0.312 in SI)
    /// - D = Storm drain diameter, ft (m)
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
    /// **HEC-22 Equation 9.15: Entrance Loss Coefficient**
    ///
    /// ```text
    /// H_i = K_i V^2/2g
    /// ```
    ///
    /// Where:
    /// - H_i = Entrance loss, ft (m)
    /// - K_i = Entrance loss coefficient = 0.2 (Kerenyi et al. 2006)
    /// - V = Velocity in pipe, ft/s (m/s)
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// Typical K values for different entrance types:
    /// - Outlet control (FHWA): 0.2
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
    /// **HEC-22 Equation 9.5: Exit Loss at Storm Drain Outlet**
    ///
    /// ```text
    /// H_o = 1.0[V_o^2/2g - V_d^2/2g]
    /// ```
    ///
    /// Where:
    /// - H_o = Exit loss, ft (m)
    /// - V_o = Average outlet velocity, ft/s (m/s)
    /// - V_d = Channel velocity downstream of outlet in direction of pipe flow, ft/s (m/s)
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// Note: For sudden expansion at end wall, coefficient is 1.0. When V_d = 0 (reservoir),
    /// exit loss equals one velocity head.
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
    /// **HEC-22 Equation 9.6: Bend Loss Coefficient**
    ///
    /// ```text
    /// H_b = 0.0033(Δ)V^2/2g
    /// ```
    ///
    /// Where:
    /// - H_b = Bend loss, ft (m)
    /// - Δ = Angle of bend, degrees
    /// - V = Velocity in pipe, ft/s (m/s)
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// Note: Coefficient 0.0033 is derived from AASHTO 2014. Used when pipes change
    /// direction without access holes.
    ///
    /// # Arguments
    /// * `velocity` - Velocity (ft/s or m/s)
    /// * `bend_angle` - Bend angle in degrees
    pub fn bend_loss(&self, velocity: f64, bend_angle: f64) -> f64 {
        let k_bend = 0.0033 * bend_angle;
        k_bend * velocity.powi(2) / (2.0 * self.gravity)
    }

    /// Calculate expansion loss
    ///
    /// **HEC-22 Equation 9.7: Expansion Loss**
    ///
    /// ```text
    /// H_e = K_e[V_2^2/2g - V_1^2/2g]
    /// ```
    ///
    /// Where:
    /// - H_e = Expansion loss, ft (m)
    /// - K_e = Expansion coefficient (see Table 9.3)
    /// - V_1 = Velocity upstream of transition, ft/s (m/s)
    /// - V_2 = Velocity downstream of transition, ft/s (m/s)
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// Note: Energy losses in expansions expressed in terms of kinetic energy at two ends.
    /// K_e values depend on D2/D1 ratio and angle of cone (Table 9.3). Typically designers
    /// use access holes when pipe size increases.
    ///
    /// # Arguments
    /// * `v_upstream` - Velocity upstream of transition (ft/s or m/s)
    /// * `v_downstream` - Velocity downstream of transition (ft/s or m/s)
    /// * `k_expansion` - Expansion coefficient
    ///
    /// # Returns
    /// Expansion loss (ft or m)
    pub fn expansion_loss(
        &self,
        v_upstream: f64,
        v_downstream: f64,
        k_expansion: f64,
    ) -> f64 {
        let vh_upstream = v_upstream.powi(2) / (2.0 * self.gravity);
        let vh_downstream = v_downstream.powi(2) / (2.0 * self.gravity);
        k_expansion * (vh_downstream - vh_upstream).max(0.0)
    }

    /// Calculate contraction loss
    ///
    /// **HEC-22 Equation 9.8: Contraction Loss**
    ///
    /// ```text
    /// H_c = K_c[V_2^2/2g - V_1^2/2g]
    /// ```
    ///
    /// Where:
    /// - H_c = Contraction loss, ft (m)
    /// - K_c = Contraction coefficient
    /// - V_1 = Velocity upstream of transition, ft/s (m/s)
    /// - V_2 = Velocity downstream of transition, ft/s (m/s)
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// Note: Analogous energy loss for contractions. However, designers don't use contractions
    /// in storm drains because of potential for clogging and safety hazards when transitioning
    /// to smaller pipe size.
    ///
    /// # Arguments
    /// * `v_upstream` - Velocity upstream of transition (ft/s or m/s)
    /// * `v_downstream` - Velocity downstream of transition (ft/s or m/s)
    /// * `k_contraction` - Contraction coefficient
    ///
    /// # Returns
    /// Contraction loss (ft or m)
    pub fn contraction_loss(
        &self,
        v_upstream: f64,
        v_downstream: f64,
        k_contraction: f64,
    ) -> f64 {
        let vh_upstream = v_upstream.powi(2) / (2.0 * self.gravity);
        let vh_downstream = v_downstream.powi(2) / (2.0 * self.gravity);
        k_contraction * (vh_downstream - vh_upstream).max(0.0)
    }

    /// Calculate junction loss using momentum equation
    ///
    /// **HEC-22 Equation 9.9: Junction Loss (Momentum Equation)**
    ///
    /// ```text
    /// H_j = [(Q_o V_o) - (Q_i V_i) - (Q_l V_l cos θ_j)] / [0.5g(A_o + A_i)] + h_i - h_o
    /// ```
    ///
    /// Where:
    /// - H_j = Junction loss, ft (m)
    /// - Q_o = Outlet flow, ft³/s (m³/s)
    /// - Q_i = Inlet flow, ft³/s (m³/s)
    /// - Q_l = Lateral flow, ft³/s (m³/s)
    /// - V_o = Outlet velocity, ft/s (m/s)
    /// - V_i = Inlet velocity, ft/s (m/s)
    /// - V_l = Lateral velocity, ft/s (m/s)
    /// - h_o = Outlet velocity head, ft (m)
    /// - h_i = Inlet velocity head, ft (m)
    /// - A_o = Outlet cross-sectional area, ft² (m²)
    /// - A_i = Inlet cross-sectional area, ft² (m²)
    /// - θ_j = Angle between inflow trunk pipe and inflow lateral pipe
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// This is the preferred method from HEC-22 Chapter 9.1.6.5 for calculating
    /// energy losses at pipe junctions where flows combine.
    ///
    /// # Arguments
    /// * `q_outlet` - Outlet flow rate (cfs or cms)
    /// * `q_inlet` - Inlet (main trunk) flow rate (cfs or cms)
    /// * `q_lateral` - Lateral pipe flow rate (cfs or cms)
    /// * `v_outlet` - Outlet pipe velocity (ft/s or m/s)
    /// * `v_inlet` - Inlet pipe velocity (ft/s or m/s)
    /// * `v_lateral` - Lateral pipe velocity (ft/s or m/s)
    /// * `a_outlet` - Outlet cross-sectional area (sq ft or sq m)
    /// * `a_inlet` - Inlet cross-sectional area (sq ft or sq m)
    /// * `theta_j` - Angle between inflow trunk pipe and lateral pipe (degrees)
    ///
    /// # Returns
    /// Junction loss (ft or m)
    pub fn junction_loss(
        &self,
        q_outlet: f64,
        q_inlet: f64,
        q_lateral: f64,
        v_outlet: f64,
        v_inlet: f64,
        v_lateral: f64,
        a_outlet: f64,
        a_inlet: f64,
        theta_j: f64,
    ) -> f64 {
        // Convert angle from degrees to radians
        let theta_rad = theta_j.to_radians();

        // Calculate velocity heads
        let h_outlet = v_outlet.powi(2) / (2.0 * self.gravity);
        let h_inlet = v_inlet.powi(2) / (2.0 * self.gravity);

        // Momentum term: [(Q_o·V_o) - (Q_i·V_i) - (Q_l·V_l·cos θ_j)]
        let momentum_term = (q_outlet * v_outlet) - (q_inlet * v_inlet) - (q_lateral * v_lateral * theta_rad.cos());

        // Denominator: 0.5g(A_o + A_i)
        let denominator = 0.5 * self.gravity * (a_outlet + a_inlet);

        // HEC-22 Equation 9.9
        let junction_loss = (momentum_term / denominator) + h_inlet - h_outlet;

        junction_loss
    }

    /// Calculate junction loss using K-method (approximate)
    ///
    /// H_j = K × (V_outlet²/2g)
    ///
    /// Note: This is an approximate method. For more accurate results,
    /// use the `junction_loss` method which implements HEC-22 Equation 9.9.
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

    /// Calculate approximate access hole loss
    ///
    /// **HEC-22 Equation 9.10: Approximate Access Hole Loss**
    ///
    /// ```text
    /// H_ah = K_ah V_o^2/2g
    /// ```
    ///
    /// Where:
    /// - H_ah = Head loss across access hole, ft (m)
    /// - K_ah = Head loss coefficient (see Table 9.4)
    /// - V_o = Outlet pipe velocity, ft/s (m/s)
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// **WARNING: For preliminary design only!**
    ///
    /// This is the simplest method for estimating losses across access hole by multiplying
    /// velocity head of outflow pipe by coefficient. Coefficients from Table 9.4 vary with
    /// structure configuration and angle. Does NOT apply to EGL calculations - use only for
    /// establishing initial pipe invert elevations.
    ///
    /// For accurate analysis, use the FHWA Access Hole Method (Equations 9.11-9.31).
    ///
    /// # Arguments
    /// * `v_outlet` - Outlet pipe velocity (ft/s or m/s)
    /// * `k_access_hole` - Access hole loss coefficient (see HEC-22 Table 9.4)
    ///
    /// # Returns
    /// Access hole loss (ft or m)
    pub fn approximate_access_hole_loss(&self, v_outlet: f64, k_access_hole: f64) -> f64 {
        k_access_hole * v_outlet.powi(2) / (2.0 * self.gravity)
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

/// Benching configuration for access holes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BenchingType {
    /// Flat floor (level)
    Flat,
    /// Depressed floor
    Depressed,
    /// Improved benching
    Improved,
}

/// Inflow pipe configuration for access hole analysis
#[derive(Debug, Clone)]
pub struct InflowPipe {
    /// Flow rate (cfs or cms)
    pub flow: f64,
    /// Velocity (ft/s or m/s)
    pub velocity: f64,
    /// Diameter (ft or m)
    pub diameter: f64,
    /// Cross-sectional area (sq ft or sq m)
    pub area: f64,
    /// Angle from outlet pipe (degrees, 180° is straight through)
    pub angle: f64,
    /// Invert elevation difference from access hole invert (ft or m)
    pub invert_offset: f64,
}

/// FHWA Access Hole Method analysis results
#[derive(Debug, Clone)]
pub struct AccessHoleResult {
    /// Initial access hole energy level, ft (m) - Equation 9.13
    pub initial_energy_level: f64,
    /// Outlet control energy level, ft (m) - Equation 9.14
    pub outlet_control_energy: f64,
    /// Submerged inlet control energy level, ft (m) - Equation 9.17
    pub submerged_inlet_energy: f64,
    /// Unsubmerged inlet control energy level, ft (m) - Equation 9.18
    pub unsubmerged_inlet_energy: f64,
    /// Benching loss coefficient - Equation 9.20
    pub benching_coefficient: f64,
    /// Angled inflow coefficient - Equation 9.22
    pub angle_coefficient: f64,
    /// Plunging flow coefficient - Equation 9.25
    pub plunging_coefficient: f64,
    /// Total additional loss, ft (m) - Equation 9.27
    pub additional_loss: f64,
    /// Final access hole energy level, ft (m) - Equation 9.28
    pub final_energy_level: f64,
    /// Access hole energy grade line elevation, ft (m) - Equation 9.29
    pub egl_elevation: f64,
}

/// FHWA Access Hole Method for detailed energy loss calculations
///
/// Implements HEC-22 Equations 9.11 through 9.31 for accurate analysis of
/// energy losses at access holes (manholes) in storm drain systems.
///
/// This is the preferred method for final design and EGL calculations.
pub struct FhwaAccessHoleMethod {
    /// Gravitational constant
    pub gravity: f64,
}

impl FhwaAccessHoleMethod {
    /// Create for US customary units
    pub fn us_customary() -> Self {
        Self { gravity: GRAVITY_US }
    }

    /// Create for SI metric units
    pub fn si_metric() -> Self {
        Self { gravity: GRAVITY_SI }
    }

    /// Calculate outflow pipe energy head from components
    ///
    /// **HEC-22 Equation 9.11: Total Energy Head Components**
    ///
    /// ```text
    /// E_i = y + (P/γ) + (V^2/2g)
    /// ```
    ///
    /// Where:
    /// - E_i = Outflow pipe energy head, ft (m)
    /// - y = Outflow pipe depth (potential head), ft (m)
    /// - P/γ = Outflow pipe pressure head, ft (m)
    /// - V²/2g = Outflow pipe velocity head, ft (m)
    ///
    /// # Arguments
    /// * `depth` - Flow depth (ft or m)
    /// * `pressure_head` - Pressure head (ft or m)
    /// * `velocity_head` - Velocity head (ft or m)
    pub fn energy_head_from_components(
        &self,
        depth: f64,
        pressure_head: f64,
        velocity_head: f64,
    ) -> f64 {
        depth + pressure_head + velocity_head
    }

    /// Calculate outflow pipe energy head from EGL and invert
    ///
    /// **HEC-22 Equation 9.12: Energy Head from EGL and Invert**
    ///
    /// ```text
    /// E_i = EGL_i - Z_i
    /// ```
    ///
    /// Where:
    /// - E_i = Outflow pipe energy head, ft (m)
    /// - EGL_i = Outflow pipe energy grade line, ft (m)
    /// - Z_i = Outflow pipe invert elevation, ft (m)
    ///
    /// This alternative method avoids problems with solving Equation 9.11 directly.
    ///
    /// # Arguments
    /// * `egl` - Energy grade line elevation (ft or m)
    /// * `invert_elevation` - Pipe invert elevation (ft or m)
    pub fn energy_head_from_egl(&self, egl: f64, invert_elevation: f64) -> f64 {
        egl - invert_elevation
    }

    /// Calculate outlet control energy level
    ///
    /// **HEC-22 Equation 9.14: Outlet Control Energy Level**
    ///
    /// ```text
    /// E_aio = E_i + H_i
    /// ```
    ///
    /// Where:
    /// - E_aio = Access hole energy level for outlet control, ft (m)
    /// - E_i = Outflow pipe energy head, ft (m)
    /// - H_i = Entrance loss assuming outlet control, ft (m)
    ///
    /// **HEC-22 Equation 9.15: Entrance Loss Coefficient**
    ///
    /// ```text
    /// H_i = K_i V^2/2g
    /// ```
    ///
    /// Where K_i = 0.2 (Kerenyi et al. 2006)
    ///
    /// # Arguments
    /// * `outflow_energy_head` - Outflow pipe energy head E_i (ft or m)
    /// * `outflow_velocity` - Outflow pipe velocity (ft/s or m/s)
    pub fn outlet_control_energy(&self, outflow_energy_head: f64, outflow_velocity: f64) -> f64 {
        let k_entrance = 0.2; // HEC-22 recommended value
        let entrance_loss = k_entrance * outflow_velocity.powi(2) / (2.0 * self.gravity);
        outflow_energy_head + entrance_loss
    }

    /// Calculate discharge intensity
    ///
    /// **HEC-22 Equation 9.16: Discharge Intensity**
    ///
    /// ```text
    /// DI = Q / [A(gD_o)^0.5]
    /// ```
    ///
    /// Where:
    /// - DI = Discharge intensity (dimensionless)
    /// - Q = Discharge, ft³/s (m³/s)
    /// - A = Area of outflow pipe, ft² (m²)
    /// - D_o = Diameter of outflow pipe, ft (m)
    /// - g = Gravitational acceleration, 32.2 ft/s² (9.81 m/s²)
    ///
    /// This dimensionless ratio is key for inlet control calculations.
    ///
    /// # Arguments
    /// * `flow` - Discharge (cfs or cms)
    /// * `area` - Outflow pipe area (sq ft or sq m)
    /// * `diameter` - Outflow pipe diameter (ft or m)
    pub fn discharge_intensity(&self, flow: f64, area: f64, diameter: f64) -> f64 {
        flow / (area * (self.gravity * diameter).sqrt())
    }

    /// Calculate submerged inlet control energy level (orifice analogy)
    ///
    /// **HEC-22 Equation 9.17: Submerged Inlet Control (Orifice)**
    ///
    /// ```text
    /// E_ais = D_o(DI)^2
    /// ```
    ///
    /// Where:
    /// - E_ais = Access hole energy level for inlet control (submerged), ft (m)
    /// - D_o = Diameter of outflow pipe, ft (m)
    /// - DI = Discharge intensity (dimensionless)
    ///
    /// Applies when opening in access hole structure to outlet pipe is limiting and
    /// water depth is sufficiently high. Derived from laboratory data with DI ≤ 1.6.
    ///
    /// # Arguments
    /// * `diameter` - Outflow pipe diameter (ft or m)
    /// * `discharge_intensity` - DI from Equation 9.16
    pub fn submerged_inlet_control(&self, diameter: f64, discharge_intensity: f64) -> f64 {
        diameter * discharge_intensity.powi(2)
    }

    /// Calculate unsubmerged inlet control energy level (weir analogy)
    ///
    /// **HEC-22 Equation 9.18: Unsubmerged Inlet Control (Weir)**
    ///
    /// ```text
    /// E_aiu = 1.6D_o(DI)^0.67
    /// ```
    ///
    /// Where:
    /// - E_aiu = Access hole energy level for inlet control (unsubmerged), ft (m)
    /// - D_o = Diameter of outflow pipe, ft (m)
    /// - DI = Discharge intensity (dimensionless)
    ///
    /// Applies when flow control is limited by opening but water level involves treating
    /// opening as weir. Laboratory data shows DI range of 0.0 to 0.5.
    ///
    /// # Arguments
    /// * `diameter` - Outflow pipe diameter (ft or m)
    /// * `discharge_intensity` - DI from Equation 9.16
    pub fn unsubmerged_inlet_control(&self, diameter: f64, discharge_intensity: f64) -> f64 {
        1.6 * diameter * discharge_intensity.powf(0.67)
    }

    /// Calculate initial access hole energy level
    ///
    /// **HEC-22 Equation 9.13: Initial Access Hole Energy Level**
    ///
    /// ```text
    /// E_ai = max(E_aio, E_ais, E_aiu)
    /// ```
    ///
    /// Where:
    /// - E_ai = Initial access hole energy level, ft (m)
    /// - E_aio = Estimated access hole energy level for outlet control, ft (m)
    /// - E_ais = Estimated access hole energy level for inlet control (submerged), ft (m)
    /// - E_aiu = Estimated access hole energy level for inlet control (unsubmerged), ft (m)
    ///
    /// # Arguments
    /// * `outlet_control` - E_aio from Equation 9.14
    /// * `submerged_inlet` - E_ais from Equation 9.17
    /// * `unsubmerged_inlet` - E_aiu from Equation 9.18
    pub fn initial_energy_level(
        &self,
        outlet_control: f64,
        submerged_inlet: f64,
        unsubmerged_inlet: f64,
    ) -> f64 {
        outlet_control.max(submerged_inlet).max(unsubmerged_inlet)
    }

    /// Calculate benching energy loss coefficient
    ///
    /// **HEC-22 Equation 9.20: Benching Energy Loss**
    ///
    /// ```text
    /// H_B = C_B(E_ai - E_i)
    /// ```
    ///
    /// Where:
    /// - H_B = Additional energy loss for benching, ft (m)
    /// - C_B = Energy loss coefficient for benching (see Table 9.5)
    /// - E_ai = Initial access hole energy level, ft (m)
    /// - E_i = Outflow pipe energy head, ft (m)
    ///
    /// Benching tends to direct flow through access hole, reducing energy losses.
    /// C_B values depend on whether bench is submerged (E_ai/D_o > 2.5) or
    /// unsubmerged (E_ai/D_o < 1.0). Negative values indicate water depth reduction.
    ///
    /// # Arguments
    /// * `benching_type` - Type of benching (Flat, Depressed, Improved)
    /// * `initial_energy` - E_ai (ft or m)
    /// * `outflow_diameter` - D_o (ft or m)
    ///
    /// # Returns
    /// C_B coefficient (dimensionless)
    pub fn benching_coefficient(
        &self,
        benching_type: BenchingType,
        initial_energy: f64,
        outflow_diameter: f64,
    ) -> f64 {
        let ratio = initial_energy / outflow_diameter;

        // Coefficients from HEC-22 Table 9.5
        match benching_type {
            BenchingType::Flat => {
                if ratio > 2.5 {
                    0.0 // Submerged - no effect
                } else if ratio < 1.0 {
                    0.0 // Unsubmerged - no effect
                } else {
                    0.0 // Transition
                }
            }
            BenchingType::Depressed => {
                if ratio > 2.5 {
                    0.5 // Submerged - increased loss
                } else if ratio < 1.0 {
                    0.3 // Unsubmerged
                } else {
                    0.4 // Transition - interpolated
                }
            }
            BenchingType::Improved => {
                if ratio > 2.5 {
                    -0.3 // Submerged - reduced loss (negative)
                } else if ratio < 1.0 {
                    -0.5 // Unsubmerged - reduced loss
                } else {
                    -0.4 // Transition - interpolated
                }
            }
        }
    }

    /// Calculate flow-weighted angle from multiple inflows
    ///
    /// **HEC-22 Equation 9.21: Flow-Weighted Angle**
    ///
    /// ```text
    /// θ_w = Σ(Q_j θ_j) / ΣQ_j
    /// ```
    ///
    /// Where:
    /// - θ_w = Flow-weighted angle, degrees
    /// - Q_j = Contributing flow from inflow pipe j, ft³/s (m³/s)
    /// - θ_j = Angle measured from outlet pipe (180° is straight pipe), degrees
    ///
    /// Addresses effect of skewed inflows by considering momentum vectors.
    /// If all flows plunging, set θ_w = 180°.
    ///
    /// # Arguments
    /// * `inflow_pipes` - Non-plunging inflow pipes with flows and angles
    pub fn flow_weighted_angle(&self, inflow_pipes: &[InflowPipe]) -> f64 {
        let sum_q_theta: f64 = inflow_pipes.iter()
            .map(|pipe| pipe.flow * pipe.angle)
            .sum();
        let sum_q: f64 = inflow_pipes.iter()
            .map(|pipe| pipe.flow)
            .sum();

        if sum_q > 0.0 {
            sum_q_theta / sum_q
        } else {
            180.0 // All flows plunging
        }
    }

    /// Calculate angled inflow coefficient
    ///
    /// **HEC-22 Equation 9.22: Angled Inflow Coefficient**
    ///
    /// ```text
    /// C_θ = 4.5(ΣQ_j/Q_o)cos(θ_w/2)
    /// ```
    ///
    /// Where:
    /// - C_θ = Angled inflow coefficient (dimensionless)
    /// - ΣQ_j = Sum of non-plunging contributing flows, ft³/s (m³/s)
    /// - Q_o = Flow in outflow pipe, ft³/s (m³/s)
    /// - θ_w = Flow-weighted angle, degrees
    ///
    /// Coefficient approaches zero as θ_w approaches 180° and as relative inflow approaches zero.
    ///
    /// # Arguments
    /// * `total_inflow` - Sum of non-plunging inflow (cfs or cms)
    /// * `outflow` - Outflow rate (cfs or cms)
    /// * `flow_weighted_angle` - θ_w from Equation 9.21 (degrees)
    pub fn angled_inflow_coefficient(
        &self,
        total_inflow: f64,
        outflow: f64,
        flow_weighted_angle: f64,
    ) -> f64 {
        if outflow > 0.0 {
            let angle_rad = (flow_weighted_angle / 2.0).to_radians();
            4.5 * (total_inflow / outflow) * angle_rad.cos()
        } else {
            0.0
        }
    }

    /// Calculate relative plunge height for a plunging pipe
    ///
    /// **HEC-22 Equation 9.24: Relative Plunge Height**
    ///
    /// ```text
    /// h_k = (z_k - E_ai) / D_o
    /// ```
    ///
    /// Where:
    /// - h_k = Relative plunge height (dimensionless)
    /// - z_k = Difference between access hole invert and inflow pipe k invert, ft (m)
    /// - E_ai = Initial access hole energy level, ft (m)
    /// - D_o = Diameter of outflow pipe, ft (m)
    ///
    /// Plunging inflow occurs where inflow pipe invert (z_k) is greater than estimated
    /// structure water depth (approximated by E_ai). Only applies when z_k < 10D_o;
    /// if z_k > 10D_o, set it to 10D_o.
    ///
    /// # Arguments
    /// * `invert_offset` - z_k (ft or m)
    /// * `initial_energy` - E_ai (ft or m)
    /// * `outflow_diameter` - D_o (ft or m)
    pub fn relative_plunge_height(
        &self,
        invert_offset: f64,
        initial_energy: f64,
        outflow_diameter: f64,
    ) -> f64 {
        let z_k = invert_offset.min(10.0 * outflow_diameter);
        (z_k - initial_energy) / outflow_diameter
    }

    /// Calculate plunging flow coefficient
    ///
    /// **HEC-22 Equation 9.25: Plunging Flow Coefficient**
    ///
    /// ```text
    /// C_P = Σ(Q_k h_k) / Q_o
    /// ```
    ///
    /// Where:
    /// - C_P = Plunging flow coefficient (dimensionless)
    /// - Q_k = Flow from plunging pipe k, ft³/s (m³/s)
    /// - h_k = Relative plunge height for pipe k (dimensionless)
    /// - Q_o = Flow in outflow pipe, ft³/s (m³/s)
    ///
    /// As proportion of plunging flows approaches zero, C_P approaches zero.
    ///
    /// # Arguments
    /// * `plunging_pipes` - Plunging inflow pipes
    /// * `initial_energy` - E_ai (ft or m)
    /// * `outflow` - Q_o (cfs or cms)
    /// * `outflow_diameter` - D_o (ft or m)
    pub fn plunging_flow_coefficient(
        &self,
        plunging_pipes: &[InflowPipe],
        initial_energy: f64,
        outflow: f64,
        outflow_diameter: f64,
    ) -> f64 {
        if outflow > 0.0 {
            let sum: f64 = plunging_pipes.iter()
                .map(|pipe| {
                    let h_k = self.relative_plunge_height(
                        pipe.invert_offset,
                        initial_energy,
                        outflow_diameter,
                    );
                    pipe.flow * h_k
                })
                .sum();
            sum / outflow
        } else {
            0.0
        }
    }

    /// Calculate total additional loss from benching, angles, and plunging
    ///
    /// **HEC-22 Equation 9.27: Combined Additional Loss**
    ///
    /// ```text
    /// H_a = (C_B + C_θ + C_P)(E_ai - E_i)
    /// ```
    ///
    /// Where:
    /// - H_a = Total additional loss, ft (m)
    /// - C_B = Energy loss coefficient for benching (dimensionless)
    /// - C_θ = Angled inflow coefficient (dimensionless)
    /// - C_P = Plunging flow coefficient (dimensionless)
    /// - E_ai = Initial access hole energy level, ft (m)
    /// - E_i = Outflow pipe energy head, ft (m)
    ///
    /// Value should always be positive; if negative, set H_a = 0.
    ///
    /// # Arguments
    /// * `c_benching` - C_B coefficient
    /// * `c_angle` - C_θ coefficient
    /// * `c_plunging` - C_P coefficient
    /// * `initial_energy` - E_ai (ft or m)
    /// * `outflow_energy` - E_i (ft or m)
    pub fn total_additional_loss(
        &self,
        c_benching: f64,
        c_angle: f64,
        c_plunging: f64,
        initial_energy: f64,
        outflow_energy: f64,
    ) -> f64 {
        let h_a = (c_benching + c_angle + c_plunging) * (initial_energy - outflow_energy);
        h_a.max(0.0)
    }

    /// Calculate final access hole energy level
    ///
    /// **HEC-22 Equation 9.28: Final Access Hole Energy Level**
    ///
    /// ```text
    /// E_a = E_ai + H_a
    /// ```
    ///
    /// Where:
    /// - E_a = Revised access hole energy level, ft (m)
    /// - E_ai = Initial access hole energy level, ft (m)
    /// - H_a = Total additional loss, ft (m)
    ///
    /// If computed E_a < E_i, use higher value (E_i).
    ///
    /// # Arguments
    /// * `initial_energy` - E_ai (ft or m)
    /// * `additional_loss` - H_a (ft or m)
    /// * `outflow_energy` - E_i (ft or m)
    pub fn final_energy_level(
        &self,
        initial_energy: f64,
        additional_loss: f64,
        outflow_energy: f64,
    ) -> f64 {
        (initial_energy + additional_loss).max(outflow_energy)
    }

    /// Calculate access hole energy grade line elevation
    ///
    /// **HEC-22 Equation 9.29: Access Hole Energy Grade Line**
    ///
    /// ```text
    /// EGL_a = E_a + Z_a
    /// ```
    ///
    /// Where:
    /// - EGL_a = Access hole energy grade line elevation, ft (m)
    /// - E_a = Revised access hole energy level, ft (m)
    /// - Z_a = Access hole invert elevation (same as outflow pipe invert), ft (m)
    ///
    /// # Arguments
    /// * `final_energy` - E_a (ft or m)
    /// * `access_hole_invert` - Z_a elevation (ft or m)
    pub fn access_hole_egl(&self, final_energy: f64, access_hole_invert: f64) -> f64 {
        final_energy + access_hole_invert
    }

    /// Calculate inflow pipe energy head for non-plunging pipes
    ///
    /// **HEC-22 Equation 9.30: Inflow Pipe Energy Head (Non-Plunging)**
    ///
    /// ```text
    /// EGL_o = E_a + H_o
    /// ```
    ///
    /// Where:
    /// - EGL_o = Inflow pipe energy head, ft (m)
    /// - E_a = Revised access hole energy grade line, ft (m)
    /// - H_o = Inflow pipe exit loss, ft (m)
    ///
    /// **HEC-22 Equation 9.31: Inflow Pipe Exit Loss**
    ///
    /// ```text
    /// H_o = K_o V^2/2g
    /// ```
    ///
    /// Where K_o = 0.4 (Kerenyi et al. 2006)
    ///
    /// For non-plunging inflow pipes with hydraulic connection to water in access hole
    /// (when E_a > inflow pipe invert).
    ///
    /// # Arguments
    /// * `final_energy` - E_a (ft or m)
    /// * `inflow_velocity` - V (ft/s or m/s)
    pub fn inflow_pipe_egl(&self, final_energy: f64, inflow_velocity: f64) -> f64 {
        let k_exit = 0.4; // HEC-22 recommended value
        let exit_loss = k_exit * inflow_velocity.powi(2) / (2.0 * self.gravity);
        final_energy + exit_loss
    }

    /// Perform complete FHWA access hole analysis
    ///
    /// This method implements the complete FHWA access hole methodology from
    /// HEC-22 Equations 9.11 through 9.31.
    ///
    /// # Arguments
    /// * `outflow_egl` - Outflow pipe energy grade line (ft or m)
    /// * `outflow_invert` - Outflow pipe invert elevation (ft or m)
    /// * `outflow_velocity` - Outflow pipe velocity (ft/s or m/s)
    /// * `outflow_flow` - Outflow rate (cfs or cms)
    /// * `outflow_diameter` - Outflow pipe diameter (ft or m)
    /// * `outflow_area` - Outflow pipe area (sq ft or sq m)
    /// * `inflow_pipes` - All inflow pipes (both plunging and non-plunging)
    /// * `benching` - Benching type
    /// * `access_hole_invert` - Access hole invert elevation (ft or m)
    ///
    /// # Returns
    /// Complete access hole analysis results
    pub fn analyze_access_hole(
        &self,
        outflow_egl: f64,
        outflow_invert: f64,
        outflow_velocity: f64,
        outflow_flow: f64,
        outflow_diameter: f64,
        outflow_area: f64,
        inflow_pipes: &[InflowPipe],
        benching: BenchingType,
        access_hole_invert: f64,
    ) -> AccessHoleResult {
        // Equation 9.12: Outflow energy head
        let outflow_energy = self.energy_head_from_egl(outflow_egl, outflow_invert);

        // Equation 9.14-9.15: Outlet control
        let outlet_control = self.outlet_control_energy(outflow_energy, outflow_velocity);

        // Equation 9.16: Discharge intensity
        let di = self.discharge_intensity(outflow_flow, outflow_area, outflow_diameter);

        // Equation 9.17: Submerged inlet control
        let submerged_inlet = self.submerged_inlet_control(outflow_diameter, di);

        // Equation 9.18: Unsubmerged inlet control
        let unsubmerged_inlet = self.unsubmerged_inlet_control(outflow_diameter, di);

        // Equation 9.13: Initial energy level
        let initial_energy = self.initial_energy_level(
            outlet_control,
            submerged_inlet,
            unsubmerged_inlet,
        );

        // Separate plunging and non-plunging pipes
        let (plunging, non_plunging): (Vec<_>, Vec<_>) = inflow_pipes.iter()
            .partition(|pipe| pipe.invert_offset > initial_energy);

        // Clone into owned vectors for use in methods
        let non_plunging_owned: Vec<InflowPipe> = non_plunging.into_iter().cloned().collect();
        let plunging_owned: Vec<InflowPipe> = plunging.into_iter().cloned().collect();

        // Equation 9.20: Benching coefficient
        let c_benching = self.benching_coefficient(benching, initial_energy, outflow_diameter);

        // Equations 9.21-9.23: Angled inflow
        let theta_w = self.flow_weighted_angle(&non_plunging_owned);
        let total_non_plunging_flow: f64 = non_plunging_owned.iter().map(|p| p.flow).sum();
        let c_angle = self.angled_inflow_coefficient(
            total_non_plunging_flow,
            outflow_flow,
            theta_w,
        );

        // Equations 9.24-9.26: Plunging flow
        let c_plunging = self.plunging_flow_coefficient(
            &plunging_owned,
            initial_energy,
            outflow_flow,
            outflow_diameter,
        );

        // Equation 9.27: Total additional loss
        let additional_loss = self.total_additional_loss(
            c_benching,
            c_angle,
            c_plunging,
            initial_energy,
            outflow_energy,
        );

        // Equation 9.28: Final energy level
        let final_energy = self.final_energy_level(
            initial_energy,
            additional_loss,
            outflow_energy,
        );

        // Equation 9.29: EGL elevation
        let egl_elevation = self.access_hole_egl(final_energy, access_hole_invert);

        AccessHoleResult {
            initial_energy_level: initial_energy,
            outlet_control_energy: outlet_control,
            submerged_inlet_energy: submerged_inlet,
            unsubmerged_inlet_energy: unsubmerged_inlet,
            benching_coefficient: c_benching,
            angle_coefficient: c_angle,
            plunging_coefficient: c_plunging,
            additional_loss,
            final_energy_level: final_energy,
            egl_elevation,
        }
    }
}

/// Design calculations for storm drain systems
///
/// Implements design-related equations from HEC-22 Chapter 9, including
/// time of concentration adjustments and minimum slope calculations.
pub struct DesignCalculations {
    /// Manning's constant for unit system
    pub k: f64,
}

impl DesignCalculations {
    /// Create for US customary units
    pub fn us_customary() -> Self {
        Self { k: MANNING_CONST_US }
    }

    /// Create for SI metric units
    pub fn si_metric() -> Self {
        Self { k: MANNING_CONST_SI }
    }

    /// Calculate contributing area for shorter time of concentration
    ///
    /// **HEC-22 Equation 9.32: Contributing Area for Shorter Time of Concentration**
    ///
    /// ```text
    /// A_c = A(t_c1 / t_c2)
    /// ```
    ///
    /// Where:
    /// - A_c = Part of larger primary area contributing during shorter time of concentration, ac (ha)
    /// - A = Area of larger primary area, ac (ha)
    /// - t_c1 = Time of concentration of smaller, less pervious area, min
    /// - t_c2 = Time of concentration of larger primary area, min
    ///
    /// Used when highly impervious sub-area might dominate design flow. Estimates portion
    /// of area relevant to shorter time of concentration. Second calculation uses weighted C
    /// value combining smaller less pervious area and area A_c. Designer uses larger of two
    /// discharge calculations.
    ///
    /// # Arguments
    /// * `total_area` - Area of larger primary area (acres or hectares)
    /// * `tc_smaller` - Time of concentration of smaller, less pervious area (minutes)
    /// * `tc_larger` - Time of concentration of larger primary area (minutes)
    ///
    /// # Returns
    /// Contributing area during shorter time of concentration (acres or hectares)
    pub fn contributing_area_for_shorter_tc(
        &self,
        total_area: f64,
        tc_smaller: f64,
        tc_larger: f64,
    ) -> f64 {
        if tc_larger > 0.0 {
            total_area * (tc_smaller / tc_larger)
        } else {
            0.0
        }
    }

    /// Calculate minimum slope required for design velocity
    ///
    /// **HEC-22 Equation 9.33: Minimum Slope for Design Velocity**
    ///
    /// ```text
    /// S = K_u[nV / D^0.67]^2
    /// ```
    ///
    /// Where:
    /// - S = Minimum slope, ft/ft (m/m)
    /// - K_u = Unit conversion constant, 2.87 in CU (6.35 in SI)
    /// - n = Manning's roughness coefficient
    /// - V = Design velocity (typically 3 ft/s minimum), ft/s (m/s)
    /// - D = Diameter, ft (m)
    ///
    /// Maintains self-cleaning velocity in storm drain system to prevent sediment deposition
    /// and capacity loss. Typically develop storm drains to maintain full flow velocities of
    /// 3 ft/s or greater. Computed from Manning's equation rearranged to solve for slope.
    ///
    /// # Arguments
    /// * `diameter` - Pipe diameter (ft or m)
    /// * `manning_n` - Manning's roughness coefficient
    /// * `design_velocity` - Minimum design velocity (ft/s or m/s, typically 3 ft/s)
    ///
    /// # Returns
    /// Minimum slope (ft/ft or m/m)
    pub fn minimum_slope_for_velocity(
        &self,
        diameter: f64,
        manning_n: f64,
        design_velocity: f64,
    ) -> f64 {
        // K_u values: 2.87 for US customary, 6.35 for SI
        let k_u = if (self.k - MANNING_CONST_US).abs() < 0.01 {
            2.87 // US customary
        } else {
            6.35 // SI metric
        };

        let numerator = manning_n * design_velocity;
        let denominator = diameter.powf(0.67);
        k_u * (numerator / denominator).powi(2)
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

    #[test]
    fn test_junction_loss() {
        let energy_loss = EnergyLoss::us_customary();

        // Test case: 90-degree junction with lateral inflow
        // Outlet: 24" pipe, Q=10 cfs
        // Inlet: 18" pipe, Q=6 cfs
        // Lateral: 18" pipe, Q=4 cfs, 90 degrees

        let q_outlet = 10.0; // cfs
        let q_inlet = 6.0;   // cfs
        let q_lateral = 4.0; // cfs

        // Calculate areas
        let d_outlet: f64 = 2.0; // ft (24 inches)
        let d_inlet: f64 = 1.5;  // ft (18 inches)
        let a_outlet = std::f64::consts::PI * d_outlet.powi(2) / 4.0;
        let a_inlet = std::f64::consts::PI * d_inlet.powi(2) / 4.0;

        // Calculate velocities
        let v_outlet = q_outlet / a_outlet;
        let v_inlet = q_inlet / a_inlet;
        let v_lateral = q_lateral / a_inlet;

        let theta_j = 90.0; // degrees

        let hj = energy_loss.junction_loss(
            q_outlet, q_inlet, q_lateral,
            v_outlet, v_inlet, v_lateral,
            a_outlet, a_inlet,
            theta_j
        );

        // Junction loss should be positive for this configuration
        assert!(hj > 0.0, "Junction loss should be positive, got {}", hj);

        // For this case, loss should be reasonable (not excessive)
        assert!(hj < 2.0, "Junction loss seems excessive: {}", hj);
    }

    #[test]
    fn test_junction_loss_straight_through() {
        let energy_loss = EnergyLoss::us_customary();

        // Test case: straight through junction (180 degrees), no lateral
        // This should result in minimal loss

        let q_outlet = 5.0;
        let q_inlet = 5.0;
        let q_lateral = 0.0;

        let d: f64 = 1.5; // Same diameter
        let a = std::f64::consts::PI * d.powi(2) / 4.0;

        let v_outlet = q_outlet / a;
        let v_inlet = q_inlet / a;
        let v_lateral = 0.0;

        let theta_j = 180.0; // Straight through

        let hj = energy_loss.junction_loss(
            q_outlet, q_inlet, q_lateral,
            v_outlet, v_inlet, v_lateral,
            a, a,
            theta_j
        );

        // For straight through with same diameter, loss should be minimal/near zero
        assert!(hj.abs() < 0.01, "Straight through junction loss should be near zero, got {}", hj);
    }

    #[test]
    fn test_expansion_loss() {
        let energy_loss = EnergyLoss::us_customary();

        // Test case: 18" to 24" expansion
        let v_upstream = 5.0; // ft/s in 18" pipe
        let v_downstream = 2.8; // ft/s in 24" pipe (same Q, larger A)
        let k_expansion = 0.3; // Typical expansion coefficient

        let h_e = energy_loss.expansion_loss(v_upstream, v_downstream, k_expansion);

        // Should have positive loss for expansion
        assert!(h_e >= 0.0, "Expansion loss should be non-negative, got {}", h_e);
        // Loss should be reasonable (less than 1 ft for these velocities)
        assert!(h_e < 1.0, "Expansion loss seems excessive: {}", h_e);
    }

    #[test]
    fn test_contraction_loss() {
        let energy_loss = EnergyLoss::us_customary();

        // Test case: 24" to 18" contraction (though not recommended in practice)
        let v_upstream = 2.8; // ft/s in 24" pipe
        let v_downstream = 5.0; // ft/s in 18" pipe (same Q, smaller A)
        let k_contraction = 0.5; // Typical contraction coefficient

        let h_c = energy_loss.contraction_loss(v_upstream, v_downstream, k_contraction);

        // Should have positive loss for contraction
        assert!(h_c >= 0.0, "Contraction loss should be non-negative, got {}", h_c);
        // Loss should be reasonable
        assert!(h_c < 1.0, "Contraction loss seems excessive: {}", h_c);
    }

    #[test]
    fn test_approximate_access_hole_loss() {
        let energy_loss = EnergyLoss::us_customary();

        // Test case: typical access hole with 90° bend
        let v_outlet = 4.0; // ft/s
        let k_ah = 1.2; // Typical for 90° bend (from Table 9.4)

        let h_ah = energy_loss.approximate_access_hole_loss(v_outlet, k_ah);

        // H_ah = 1.2 × 4²/(2×32.17) ≈ 0.30 ft
        assert!((h_ah - 0.30).abs() < 0.05, "Access hole loss = {}", h_ah);
    }

    #[test]
    fn test_fhwa_discharge_intensity() {
        let fhwa = FhwaAccessHoleMethod::us_customary();

        let flow = 10.0; // cfs
        let diameter = 2.0; // ft (24 inches)
        let area = std::f64::consts::PI * diameter.powi(2) / 4.0;

        let di = fhwa.discharge_intensity(flow, area, diameter);

        // DI = Q / [A(gD)^0.5]
        // Should be dimensionless and reasonable
        assert!(di > 0.0, "DI should be positive");
        assert!(di < 2.0, "DI should be reasonable, got {}", di);
    }

    #[test]
    fn test_fhwa_inlet_control() {
        let fhwa = FhwaAccessHoleMethod::us_customary();

        let diameter = 2.0; // ft
        let di = 0.5; // Discharge intensity

        // Equation 9.17: Submerged inlet control
        let e_ais = fhwa.submerged_inlet_control(diameter, di);
        assert!(e_ais > 0.0, "Submerged inlet control energy should be positive");

        // Equation 9.18: Unsubmerged inlet control
        let e_aiu = fhwa.unsubmerged_inlet_control(diameter, di);
        assert!(e_aiu > 0.0, "Unsubmerged inlet control energy should be positive");

        // For same DI, unsubmerged should be greater than submerged
        assert!(e_aiu > e_ais, "Unsubmerged control should be greater than submerged");
    }

    #[test]
    fn test_fhwa_flow_weighted_angle() {
        let fhwa = FhwaAccessHoleMethod::us_customary();

        // Test case: Two inflow pipes
        let inflow_pipes = vec![
            InflowPipe {
                flow: 3.0,
                velocity: 4.0,
                diameter: 1.5,
                area: std::f64::consts::PI * 1.5_f64.powi(2) / 4.0,
                angle: 180.0, // Straight through
                invert_offset: 0.0,
            },
            InflowPipe {
                flow: 2.0,
                velocity: 3.0,
                diameter: 1.0,
                area: std::f64::consts::PI * 1.0_f64.powi(2) / 4.0,
                angle: 90.0, // 90° lateral
                invert_offset: 0.0,
            },
        ];

        let theta_w = fhwa.flow_weighted_angle(&inflow_pipes);

        // θ_w = (3.0×180 + 2.0×90) / (3.0 + 2.0) = 720/5 = 144°
        assert!((theta_w - 144.0).abs() < 1.0, "Flow-weighted angle = {}", theta_w);
    }

    #[test]
    fn test_fhwa_complete_analysis() {
        let fhwa = FhwaAccessHoleMethod::us_customary();

        // Simple test case: single straight-through inflow
        let outflow_diameter = 2.0; // ft (24 inches)
        let outflow_area = std::f64::consts::PI * outflow_diameter.powi(2) / 4.0;
        let outflow_flow = 10.0; // cfs
        let outflow_velocity = outflow_flow / outflow_area;
        let outflow_invert = 100.0; // ft elevation
        let outflow_egl = 105.0; // ft elevation

        let inflow = InflowPipe {
            flow: 10.0,
            velocity: outflow_velocity,
            diameter: 2.0,
            area: outflow_area,
            angle: 180.0, // Straight through
            invert_offset: 0.0,
        };

        let result = fhwa.analyze_access_hole(
            outflow_egl,
            outflow_invert,
            outflow_velocity,
            outflow_flow,
            outflow_diameter,
            outflow_area,
            &[inflow],
            BenchingType::Improved,
            outflow_invert,
        );

        // Verify all results are reasonable
        assert!(result.initial_energy_level > 0.0, "Initial energy should be positive");
        assert!(result.final_energy_level > 0.0, "Final energy should be positive");
        assert!(result.egl_elevation > outflow_invert, "EGL should be above invert");

        // For straight through with improved benching, losses should be minimal
        assert!(result.additional_loss >= 0.0, "Additional loss should be non-negative");
    }

    #[test]
    fn test_contributing_area_for_tc() {
        let design = DesignCalculations::us_customary();

        // Test case: 10-acre area, smaller tc = 5 min, larger tc = 10 min
        let total_area = 10.0; // acres
        let tc_smaller = 5.0; // minutes
        let tc_larger = 10.0; // minutes

        let a_c = design.contributing_area_for_shorter_tc(total_area, tc_smaller, tc_larger);

        // A_c = 10 × (5/10) = 5 acres
        assert!((a_c - 5.0).abs() < 0.01, "Contributing area = {}", a_c);
    }

    #[test]
    fn test_minimum_slope_for_velocity() {
        let design = DesignCalculations::us_customary();

        // Test case: 18" pipe, maintain 3 ft/s minimum velocity
        let diameter = 1.5; // ft
        let n = 0.013; // RCP
        let v_min = 3.0; // ft/s (self-cleaning velocity)

        let s_min = design.minimum_slope_for_velocity(diameter, n, v_min);

        // Slope should be positive and reasonable
        assert!(s_min > 0.0, "Minimum slope should be positive");
        assert!(s_min < 0.1, "Minimum slope should be reasonable, got {}", s_min);

        // For typical pipe, minimum slope should be in range 0.001 to 0.01
        assert!(s_min >= 0.0001 && s_min <= 0.02,
            "Minimum slope {} outside expected range", s_min);
    }

    #[test]
    fn test_benching_coefficients() {
        let fhwa = FhwaAccessHoleMethod::us_customary();

        let initial_energy = 3.0; // ft
        let outflow_diameter = 2.0; // ft
        let ratio = initial_energy / outflow_diameter; // 1.5

        // Test all benching types
        let c_flat = fhwa.benching_coefficient(BenchingType::Flat, initial_energy, outflow_diameter);
        let c_depressed = fhwa.benching_coefficient(BenchingType::Depressed, initial_energy, outflow_diameter);
        let c_improved = fhwa.benching_coefficient(BenchingType::Improved, initial_energy, outflow_diameter);

        // Flat should be zero (no effect)
        assert_eq!(c_flat, 0.0, "Flat benching coefficient should be 0");

        // Depressed should be positive (increased loss)
        assert!(c_depressed > 0.0, "Depressed benching should increase losses");

        // Improved should be negative (reduced loss)
        assert!(c_improved < 0.0, "Improved benching should reduce losses");
    }

    #[test]
    fn test_plunging_flow() {
        let fhwa = FhwaAccessHoleMethod::us_customary();

        let initial_energy = 2.0; // ft
        let outflow_diameter = 2.0; // ft
        let outflow_flow = 10.0; // cfs

        // Create plunging inflow (invert above water surface)
        let plunging_pipe = InflowPipe {
            flow: 4.0,
            velocity: 5.0,
            diameter: 1.5,
            area: std::f64::consts::PI * 1.5_f64.powi(2) / 4.0,
            angle: 90.0,
            invert_offset: 3.0, // Above initial energy level (plunging)
        };

        let c_p = fhwa.plunging_flow_coefficient(
            &[plunging_pipe],
            initial_energy,
            outflow_flow,
            outflow_diameter,
        );

        // Plunging coefficient should be positive
        assert!(c_p > 0.0, "Plunging coefficient should be positive for plunging flow");
    }
}
