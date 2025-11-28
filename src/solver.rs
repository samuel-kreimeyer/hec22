//! HGL/EGL solver for drainage networks
//!
//! This module implements the 9-step HGL evaluation procedure from HEC-22 Chapter 9.
//!
//! ## Procedure Overview (from Chapter 9)
//!
//! 1. Determine tailwater elevation at outfall
//! 2. Estimate HGL/EGL at downstream end of each pipe
//! 3. Estimate HGL/EGL at upstream end of pipe
//! 4. Calculate EGL/HGL at each structure
//! 5-8. Repeat for all pipes and structures working upstream
//! 9. Compare EGL elevations to ground surface
//!
//! The procedure starts at the outfall and works upstream through the network.

use crate::analysis::{
    Analysis, AnalysisMethod, ConduitResult, DrainageAreaResult, HeadLoss, NodeResult,
    Violation, ViolationType, Severity,
};
use crate::conduit::{Conduit, ConduitType};
use crate::drainage::DrainageArea;
use crate::gutter::{UniformGutter, GUTTER_K_US, GUTTER_K_SI};
use crate::hydraulics::{
    EnergyLoss, FlowRegime, ManningsEquation, PipeFlowResult,
    FhwaAccessHoleMethod, InflowPipe, BenchingType, AccessHoleResult,
};
use crate::inlet::{
    BarConfiguration as InletBarConfig, CombinationInletOnGrade, CurbOpeningInletOnGrade,
    GrateInletOnGrade, InletInterceptionResult, ThroatType as InletThroatType,
};
use crate::network::Network;
use crate::node::{BoundaryCondition, Node, NodeType, InletLocation};
use crate::project::UnitSystem;
use std::collections::HashMap;

/// HGL/EGL solver configuration
pub struct SolverConfig {
    /// Unit system
    pub unit_system: UnitSystem,
    /// Gravitational constant (32.17 for US, 9.81 for SI)
    pub gravity: f64,
    /// Manning's constant (1.486 for US, 1.0 for SI)
    pub manning_k: f64,
    /// Maximum iterations for iterative solvers
    pub max_iterations: usize,
    /// Convergence tolerance
    pub tolerance: f64,
}

impl SolverConfig {
    /// Create configuration for US customary units
    pub fn us_customary() -> Self {
        Self {
            unit_system: UnitSystem::US,
            gravity: 32.17,
            manning_k: 1.486,
            max_iterations: 50,
            tolerance: 0.001,
        }
    }

    /// Create configuration for SI metric units
    pub fn si_metric() -> Self {
        Self {
            unit_system: UnitSystem::SI,
            gravity: 9.81,
            manning_k: 1.0,
            max_iterations: 50,
            tolerance: 0.001,
        }
    }
}

/// HGL/EGL solver
pub struct HglSolver {
    config: SolverConfig,
    mannings: ManningsEquation,
    energy_loss: EnergyLoss,
    fhwa_access_hole: FhwaAccessHoleMethod,
}

impl HglSolver {
    /// Create a new solver with the given configuration
    pub fn new(config: SolverConfig) -> Self {
        let mannings = ManningsEquation { k: config.manning_k };
        let energy_loss = EnergyLoss { gravity: config.gravity };
        let fhwa_access_hole = FhwaAccessHoleMethod { gravity: config.gravity };

        Self {
            config,
            mannings,
            energy_loss,
            fhwa_access_hole,
        }
    }

    /// Solve the network for HGL/EGL
    ///
    /// Implements the 9-step procedure from HEC-22 Chapter 9:
    /// - Starts at outfall with tailwater condition
    /// - Works upstream through each conduit
    /// - Calculates energy losses at each structure
    /// - Checks for violations of design criteria
    ///
    /// # Arguments
    /// * `network` - The drainage network to solve
    /// * `flows` - Flow rates at each node (from hydrologic analysis)
    /// * `design_storm_id` - ID of the design storm being analyzed
    ///
    /// # Returns
    /// Analysis results with computed HGL/EGL values
    pub fn solve(
        &self,
        network: &Network,
        flows: &HashMap<String, f64>,
        design_storm_id: String,
    ) -> Result<Analysis, String> {
        // Initialize analysis
        let mut analysis = Analysis::new(AnalysisMethod::Rational, design_storm_id);

        // Storage for computed values
        let mut node_hgls: HashMap<String, f64> = HashMap::new();
        let mut node_egls: HashMap<String, f64> = HashMap::new();
        let mut node_depths: HashMap<String, f64> = HashMap::new();
        let mut node_velocities: HashMap<String, f64> = HashMap::new();
        let mut node_junction_losses: HashMap<String, f64> = HashMap::new();

        // Step 1: Determine tailwater at outfall(s)
        let outfalls = network.outfalls();
        if outfalls.is_empty() {
            return Err("Network has no outfall nodes".to_string());
        }

        for outfall in outfalls {
            let tailwater = self.get_tailwater_elevation(outfall)?;
            node_hgls.insert(outfall.id.clone(), tailwater);

            // For outfall, EGL = HGL (assume minimal velocity)
            node_egls.insert(outfall.id.clone(), tailwater);
        }

        // Build network traversal order (topological sort from outfalls upstream)
        let traversal_order = self.topological_sort(network)?;

        // Storage for conduit flow results (needed for junction loss calculation)
        let mut conduit_velocities: HashMap<String, f64> = HashMap::new();
        let mut conduit_areas: HashMap<String, f64> = HashMap::new();

        // Process each conduit in order
        for conduit_id in traversal_order {
            let conduit = network
                .find_conduit(&conduit_id)
                .ok_or_else(|| format!("Conduit {} not found", conduit_id))?;

            // Get flow in this conduit
            let flow = flows.get(&conduit.id).cloned().unwrap_or(0.0);

            // Get downstream HGL/EGL
            let downstream_hgl = node_hgls
                .get(&conduit.to_node)
                .ok_or_else(|| format!("HGL not computed for node {}", conduit.to_node))?;

            // Solve for upstream HGL/EGL
            let (upstream_hgl, upstream_egl, conduit_result) = self.solve_conduit(
                conduit,
                flow,
                *downstream_hgl,
                network,
            )?;

            // Store results
            node_hgls.insert(conduit.from_node.clone(), upstream_hgl);
            node_egls.insert(conduit.from_node.clone(), upstream_egl);

            // Store velocity and area for junction loss calculations
            if let Some(velocity) = conduit_result.velocity {
                conduit_velocities.insert(conduit.id.clone(), velocity);
            }
            if let Some(depth) = conduit_result.depth {
                // Calculate area from depth for circular pipe
                if let Some(ref pipe) = conduit.pipe {
                    if let Some(diameter) = pipe.diameter {
                        let d = diameter / 12.0; // Convert inches to feet
                        let area = self.circular_pipe_area(d, depth);
                        conduit_areas.insert(conduit.id.clone(), area);
                    }
                }
            }

            if let Some(ref mut results) = analysis.conduit_results {
                results.push(conduit_result);
            }
        }

        // Apply access hole/junction losses using FHWA Access Hole Method
        // (HEC-22 Equations 9.11-9.31)
        //
        // For junctions and manholes, HEC-22 recommends the comprehensive FHWA
        // Access Hole Method which accounts for:
        // - Outlet control vs inlet control conditions (Equations 9.13-9.18)
        // - Benching configuration effects (Equation 9.20)
        // - Angled inflow effects (Equations 9.21-9.23)
        // - Plunging flow effects (Equations 9.24-9.26)
        //
        // This method is more accurate than the simple Equation 9.9 and provides
        // realistic energy losses at access holes. For simple junctions without
        // access holes, falls back to Equation 9.9.
        for node in &network.nodes {
            if !node.is_junction() {
                continue;
            }

            let upstream_conduits = network.upstream_conduits(&node.id);
            let downstream_conduits = network.downstream_conduits(&node.id);

            // Skip if no converging flows
            if upstream_conduits.is_empty() || downstream_conduits.is_empty() {
                continue;
            }

            // Get the outlet conduit (typically only one)
            let outlet_conduit = &downstream_conduits[0];
            let q_outlet = flows.get(&outlet_conduit.id).cloned().unwrap_or(0.0);

            if q_outlet <= 0.0 {
                continue;
            }

            // Determine if this is a manhole/access hole (use FHWA method) or simple junction
            let use_fhwa_method = upstream_conduits.len() >= 1; // Use FHWA for all junctions

            let junction_head_loss = if use_fhwa_method {
                // FHWA Access Hole Method (Equations 9.11-9.31)
                self.calculate_access_hole_loss(
                    node,
                    &outlet_conduit,
                    &upstream_conduits,
                    &flows,
                    &conduit_velocities,
                    &conduit_areas,
                    &node_egls,
                    network,
                )
            } else {
                // Fallback to simple Equation 9.9 for very simple junctions
                self.calculate_simple_junction_loss(
                    &upstream_conduits,
                    &downstream_conduits,
                    &flows,
                    &conduit_velocities,
                    &conduit_areas,
                )
            };

            // Store junction loss for this node
            node_junction_losses.insert(node.id.clone(), junction_head_loss);

            // Apply junction loss to upstream EGL
            // The EGL at the upstream end of inlet pipes must be higher to overcome junction loss
            for inlet in &upstream_conduits {
                if let Some(upstream_egl) = node_egls.get_mut(&inlet.from_node) {
                    *upstream_egl += junction_head_loss;
                }
            }
        }

        // Step 4: Create node results
        let mut node_results = Vec::new();
        for node in &network.nodes {
            if let Some(&hgl) = node_hgls.get(&node.id) {
                let egl = node_egls.get(&node.id).cloned().unwrap_or(hgl);
                let velocity = node_velocities.get(&node.id).cloned().unwrap_or(0.0);
                let depth = node_depths.get(&node.id).cloned().unwrap_or(0.0);

                // Check for flooding
                let flooding = if let Some(rim) = node.rim_elevation {
                    hgl > rim
                } else {
                    false
                };

                node_results.push(NodeResult {
                    node_id: node.id.clone(),
                    hgl: Some(hgl),
                    egl: Some(egl),
                    depth: Some(depth),
                    velocity: Some(velocity),
                    flooding: Some(flooding),
                    pressure_head: Some(hgl - node.invert_elevation),
                    junction_loss: node_junction_losses.get(&node.id).copied(),
                });

                // Check for HGL violations
                if let Some(rim) = node.rim_elevation {
                    if hgl > rim {
                        let violation = Violation::hgl_violation(
                            node.id.clone(),
                            hgl,
                            rim,
                            Severity::Error,
                        );
                        analysis.add_violation(violation);
                    }
                }
            }
        }

        analysis.node_results = Some(node_results);

        Ok(analysis)
    }

    /// Get tailwater elevation at outfall
    fn get_tailwater_elevation(&self, outfall: &Node) -> Result<f64, String> {
        let outfall_props = outfall
            .outfall
            .as_ref()
            .ok_or_else(|| "Node is not an outfall".to_string())?;

        match outfall_props.boundary_condition {
            BoundaryCondition::Free => {
                // Free outfall: use critical depth at invert
                Ok(outfall.invert_elevation)
            }
            BoundaryCondition::FixedStage => {
                // Fixed stage: use specified tailwater
                outfall_props
                    .tailwater_elevation
                    .ok_or_else(|| "Fixed stage outfall missing tailwater elevation".to_string())
            }
            BoundaryCondition::NormalDepth => {
                // Normal depth: use specified tailwater or invert
                Ok(outfall_props
                    .tailwater_elevation
                    .unwrap_or(outfall.invert_elevation))
            }
            BoundaryCondition::Tidal => {
                // Tidal: for steady-state analysis, use mean tide level
                outfall_props
                    .tailwater_elevation
                    .ok_or_else(|| "Tidal outfall missing tailwater elevation".to_string())
            }
        }
    }

    /// Calculate access hole loss using FHWA Access Hole Method (Equations 9.11-9.31)
    ///
    /// This is the comprehensive method recommended by HEC-22 for analyzing energy
    /// losses at manholes and access holes. It accounts for:
    /// - Control conditions (outlet vs inlet control)
    /// - Benching configuration
    /// - Angled inflows
    /// - Plunging flows
    fn calculate_access_hole_loss(
        &self,
        node: &Node,
        outlet_conduit: &Conduit,
        upstream_conduits: &[&Conduit],
        flows: &HashMap<String, f64>,
        velocities: &HashMap<String, f64>,
        areas: &HashMap<String, f64>,
        node_egls: &HashMap<String, f64>,
        network: &Network,
    ) -> f64 {
        // Get outlet pipe properties
        let q_outlet = flows.get(&outlet_conduit.id).cloned().unwrap_or(0.0);
        let v_outlet = velocities.get(&outlet_conduit.id).cloned().unwrap_or(0.0);
        let a_outlet = areas.get(&outlet_conduit.id).cloned().unwrap_or(1.0);

        // Get outlet pipe diameter (assuming circular pipe)
        let d_outlet = if let Some(pipe_props) = &outlet_conduit.pipe {
            pipe_props.diameter.unwrap_or(24.0) / 12.0 // Convert inches to feet
        } else {
            2.0 // Default 24" diameter
        };

        // Get outflow EGL at the junction
        let outflow_egl = node_egls.get(&node.id).cloned().unwrap_or(node.invert_elevation);
        let outflow_invert = node.invert_elevation;

        // Build inflow pipe configurations
        let mut inflow_pipes = Vec::new();
        for (idx, conduit) in upstream_conduits.iter().enumerate() {
            let flow = flows.get(&conduit.id).cloned().unwrap_or(0.0);
            let velocity = velocities.get(&conduit.id).cloned().unwrap_or(0.0);
            let area = areas.get(&conduit.id).cloned().unwrap_or(1.0);

            // Get diameter (assuming circular pipe)
            let diameter = if let Some(pipe_props) = &conduit.pipe {
                pipe_props.diameter.unwrap_or(24.0) / 12.0 // inches to feet
            } else {
                2.0 // Default 24"
            };

            // Determine angle: first pipe is straight through (180°), others at 90°
            let angle = if idx == 0 { 180.0 } else { 90.0 };

            // Calculate invert offset (elevation difference from access hole invert)
            let from_node = upstream_conduits.iter()
                .find(|c| c.id == conduit.id)
                .and_then(|c| {
                    network.nodes.iter().find(|n| n.id == c.from_node)
                });

            let invert_offset = if let Some(from) = from_node {
                (from.invert_elevation - node.invert_elevation).max(0.0)
            } else {
                0.0
            };

            inflow_pipes.push(InflowPipe {
                flow,
                velocity,
                diameter,
                area,
                angle,
                invert_offset,
            });
        }

        // Determine benching type (default to Flat, could be configured per node)
        // In future, this could be a property of the junction node
        let benching = BenchingType::Flat;

        // Perform FHWA access hole analysis
        let result = self.fhwa_access_hole.analyze_access_hole(
            outflow_egl,
            outflow_invert,
            v_outlet,
            q_outlet,
            d_outlet,
            a_outlet,
            &inflow_pipes,
            benching,
            node.invert_elevation, // Access hole invert
        );

        // Return the energy loss computed by FHWA method
        // This is the difference between the access hole EGL and the outflow EGL
        result.final_energy_level - (outflow_egl - outflow_invert)
    }

    /// Calculate simple junction loss using Equation 9.9 (fallback method)
    ///
    /// This is the simpler momentum-based method. Only used as fallback
    /// for very simple junctions. For manholes and access holes, the FHWA
    /// method is preferred.
    fn calculate_simple_junction_loss(
        &self,
        upstream_conduits: &[&Conduit],
        downstream_conduits: &[&Conduit],
        flows: &HashMap<String, f64>,
        velocities: &HashMap<String, f64>,
        areas: &HashMap<String, f64>,
    ) -> f64 {
        if upstream_conduits.len() < 2 || downstream_conduits.is_empty() {
            return 0.0;
        }

        // Get outlet conduit
        let outlet_conduit = &downstream_conduits[0];
        let q_outlet = flows.get(&outlet_conduit.id).cloned().unwrap_or(0.0);
        let v_outlet = velocities.get(&outlet_conduit.id).cloned().unwrap_or(0.0);
        let a_outlet = areas.get(&outlet_conduit.id).cloned().unwrap_or(1.0);

        if q_outlet <= 0.0 {
            return 0.0;
        }

        // Find main inlet and lateral
        let mut inlet_conduits: Vec<_> = upstream_conduits.iter().collect();
        inlet_conduits.sort_by(|a, b| {
            let flow_a = flows.get(&a.id).cloned().unwrap_or(0.0);
            let flow_b = flows.get(&b.id).cloned().unwrap_or(0.0);
            flow_b.partial_cmp(&flow_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Main inlet (highest flow)
        let inlet_conduit = inlet_conduits[0];
        let q_inlet = flows.get(&inlet_conduit.id).cloned().unwrap_or(0.0);
        let v_inlet = velocities.get(&inlet_conduit.id).cloned().unwrap_or(0.0);
        let a_inlet = areas.get(&inlet_conduit.id).cloned().unwrap_or(1.0);

        // Lateral inlet (if exists)
        let (q_lateral, v_lateral) = if inlet_conduits.len() > 1 {
            let lateral = inlet_conduits[1];
            let q = flows.get(&lateral.id).cloned().unwrap_or(0.0);
            let v = velocities.get(&lateral.id).cloned().unwrap_or(0.0);
            (q, v)
        } else {
            (0.0, 0.0)
        };

        // Junction angle (default to 90 degrees)
        let theta_j = 90.0;

        // Calculate junction loss using HEC-22 Equation 9.9
        self.energy_loss.junction_loss(
            q_outlet,
            q_inlet,
            q_lateral,
            v_outlet,
            v_inlet,
            v_lateral,
            a_outlet,
            a_inlet,
            theta_j,
        )
    }

    /// Solve for HGL/EGL through a single conduit
    fn solve_conduit(
        &self,
        conduit: &Conduit,
        flow: f64,
        downstream_hgl: f64,
        network: &Network,
    ) -> Result<(f64, f64, ConduitResult), String> {
        match conduit.conduit_type {
            ConduitType::Pipe => self.solve_pipe(conduit, flow, downstream_hgl, network),
            ConduitType::Gutter => {
                // For now, simplified gutter solution
                Ok((downstream_hgl, downstream_hgl, self.default_conduit_result(conduit, flow)))
            }
            ConduitType::Channel => {
                // For now, simplified channel solution
                Ok((downstream_hgl, downstream_hgl, self.default_conduit_result(conduit, flow)))
            }
        }
    }

    /// Solve for HGL/EGL through a pipe
    fn solve_pipe(
        &self,
        conduit: &Conduit,
        flow: f64,
        downstream_hgl: f64,
        network: &Network,
    ) -> Result<(f64, f64, ConduitResult), String> {
        let pipe_props = conduit
            .pipe
            .as_ref()
            .ok_or_else(|| "Conduit is not a pipe".to_string())?;

        let diameter = pipe_props
            .diameter
            .ok_or_else(|| "Pipe diameter not specified".to_string())?
            / 12.0; // Convert inches to feet

        let slope = conduit
            .effective_slope()
            .ok_or_else(|| "Pipe slope cannot be determined".to_string())?;

        let downstream_node = network
            .find_node(&conduit.to_node)
            .ok_or_else(|| format!("Downstream node {} not found", conduit.to_node))?;

        let downstream_invert = conduit
            .downstream_invert
            .unwrap_or(downstream_node.invert_elevation);

        let upstream_invert = conduit
            .upstream_invert
            .unwrap_or(downstream_invert + slope * conduit.length);

        // Calculate flow properties
        let flow_result = if flow > 0.0 {
            let q_full = self.mannings.full_pipe_capacity(diameter, slope, pipe_props.manning_n);

            if flow >= q_full {
                // Pressurized flow
                self.mannings.partial_pipe_flow(
                    diameter,
                    diameter,
                    slope,
                    pipe_props.manning_n,
                    self.config.gravity,
                )
            } else {
                // Calculate normal depth
                let yn = self.mannings.normal_depth(
                    flow,
                    diameter,
                    slope,
                    pipe_props.manning_n,
                    self.config.gravity,
                );

                if let Some(depth) = yn {
                    self.mannings.partial_pipe_flow(
                        diameter,
                        depth,
                        slope,
                        pipe_props.manning_n,
                        self.config.gravity,
                    )
                } else {
                    return Err("Could not calculate normal depth".to_string());
                }
            }
        } else {
            return Ok((
                downstream_hgl,
                downstream_hgl,
                self.default_conduit_result(conduit, flow),
            ));
        };

        // Calculate energy losses
        let friction_loss = self.energy_loss.friction_loss(
            flow,
            conduit.length,
            flow_result.area,
            flow_result.hydraulic_radius,
            pipe_props.manning_n,
            self.config.manning_k,
        );

        let entrance_loss = self.energy_loss.entrance_loss(
            flow_result.velocity,
            pipe_props.entrance_loss.unwrap_or(0.5),
        );

        let exit_loss = self.energy_loss.exit_loss(
            flow_result.velocity,
            0.0, // Assume zero downstream velocity for now
            pipe_props.exit_loss.unwrap_or(1.0),
        );

        let bend_loss = if let Some(k_bend) = pipe_props.bend_loss {
            k_bend * flow_result.velocity_head
        } else {
            0.0
        };

        let total_loss = friction_loss + entrance_loss + exit_loss + bend_loss;

        // Calculate upstream HGL/EGL
        let downstream_egl = downstream_hgl + flow_result.velocity_head;
        let upstream_egl = downstream_egl + total_loss;
        let upstream_hgl = upstream_egl - flow_result.velocity_head;

        // Build conduit result
        let conduit_result = ConduitResult {
            conduit_id: conduit.id.clone(),
            flow: Some(flow),
            velocity: Some(flow_result.velocity),
            depth: Some(flow_result.depth),
            capacity_used: Some(flow / self.mannings.full_pipe_capacity(diameter, slope, pipe_props.manning_n)),
            froude_number: None, // Calculate if needed
            flow_regime: Some(crate::analysis::FlowRegime::Subcritical), // Simplified
            headloss: Some(HeadLoss {
                friction: Some(friction_loss),
                entrance: Some(entrance_loss),
                exit: Some(exit_loss),
                bend: Some(bend_loss),
                total: Some(total_loss),
            }),
        };

        Ok((upstream_hgl, upstream_egl, conduit_result))
    }

    /// Create default conduit result for non-pipe conduits
    fn default_conduit_result(&self, conduit: &Conduit, flow: f64) -> ConduitResult {
        ConduitResult {
            conduit_id: conduit.id.clone(),
            flow: Some(flow),
            velocity: None,
            depth: None,
            capacity_used: None,
            froude_number: None,
            flow_regime: None,
            headloss: None,
        }
    }

    /// Calculate flow area for a circular pipe at a given depth
    ///
    /// Uses the circular segment formula:
    /// A = (D²/4) × (θ - sin(θ))
    /// where θ is the central angle in radians
    fn circular_pipe_area(&self, diameter: f64, depth: f64) -> f64 {
        if depth <= 0.0 {
            return 0.0;
        }
        if depth >= diameter {
            // Full pipe
            return std::f64::consts::PI * diameter * diameter / 4.0;
        }

        // Partial flow - calculate area of circular segment
        let r = diameter / 2.0;
        let h = depth;

        // Central angle θ = 2 × arccos((r - h) / r)
        let cos_half_theta = (r - h) / r;
        let theta = 2.0 * cos_half_theta.acos();

        // Area = (r² / 2) × (θ - sin(θ))
        let area = (r * r / 2.0) * (theta - theta.sin());

        area
    }

    /// Perform topological sort to get conduit processing order
    ///
    /// Returns conduit IDs in order from downstream to upstream
    fn topological_sort(&self, network: &Network) -> Result<Vec<String>, String> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut visiting = std::collections::HashSet::new();

        // Start from each outfall
        for outfall in network.outfalls() {
            self.visit_node(
                &outfall.id,
                network,
                &mut visited,
                &mut visiting,
                &mut result,
            )?;
        }

        // Reverse to get downstream-to-upstream order
        result.reverse();

        Ok(result)
    }

    /// Recursive DFS for topological sort
    fn visit_node(
        &self,
        node_id: &str,
        network: &Network,
        visited: &mut std::collections::HashSet<String>,
        visiting: &mut std::collections::HashSet<String>,
        result: &mut Vec<String>,
    ) -> Result<(), String> {
        if visited.contains(node_id) {
            return Ok(());
        }

        if visiting.contains(node_id) {
            return Err(format!("Circular dependency detected at node {}", node_id));
        }

        visiting.insert(node_id.to_string());

        // Visit upstream conduits
        let upstream_conduits = network.upstream_conduits(node_id);
        for conduit in upstream_conduits {
            // First visit the upstream node recursively
            self.visit_node(
                &conduit.from_node,
                network,
                visited,
                visiting,
                result,
            )?;
            // Then add this conduit (after upstream node is processed)
            result.push(conduit.id.clone());
        }

        visiting.remove(node_id);
        visited.insert(node_id.to_string());

        Ok(())
    }
}

/// Helper function to compute flows from drainage areas
///
/// Uses rational method: Q = C × i × A
/// Returns node inflows (flow entering at each node)
pub fn compute_rational_flows(
    drainage_areas: &[DrainageArea],
    intensity: f64,
) -> HashMap<String, f64> {
    let mut flows = HashMap::new();

    for area in drainage_areas {
        if let Some(flow) = area.rational_method_runoff(intensity) {
            // Add flow to outlet node
            let node_flow = flows.entry(area.outlet.clone()).or_insert(0.0);
            *node_flow += flow;
        }
    }

    flows
}

/// Route node inflows through network to get conduit flows
///
/// Performs a topological traversal from outfalls upstream,
/// accumulating flows at each junction.
///
/// # Arguments
/// * `network` - The drainage network
/// * `node_inflows` - Direct inflows at each node (from drainage areas)
///
/// # Returns
/// Map of conduit ID to flow rate
pub fn route_flows(
    network: &Network,
    node_inflows: &HashMap<String, f64>,
) -> Result<HashMap<String, f64>, String> {
    let mut conduit_flows = HashMap::new();
    let mut node_total_flows: HashMap<String, f64> = HashMap::new();

    // Initialize with direct inflows
    for (node_id, &flow) in node_inflows {
        node_total_flows.insert(node_id.clone(), flow);
    }

    // Get traversal order
    let sorted_nodes = topological_sort_upstream_to_downstream(network)?;

    // Process nodes in topological order
    for node_id in sorted_nodes {
        // Get total flow at this node (direct inflow + upstream contributions)
        let node_flow = node_total_flows.get(&node_id).cloned().unwrap_or(0.0);

        // Route flow to downstream conduits
        let downstream_conduits = network.downstream_conduits(&node_id);
        if !downstream_conduits.is_empty() {
            let flow_per_conduit = node_flow / downstream_conduits.len() as f64;

            for conduit in downstream_conduits {
                conduit_flows.insert(conduit.id.clone(), flow_per_conduit);

                // Add this flow to the total for the downstream node
                let downstream_flow = node_total_flows
                    .entry(conduit.to_node.clone())
                    .or_insert(0.0);
                *downstream_flow += flow_per_conduit;
            }
        }
    }

    Ok(conduit_flows)
}

/// Inlet interception tracking for flow routing
#[derive(Debug, Clone)]
pub struct InletInterception {
    /// Node ID (inlet)
    pub node_id: String,
    /// Approach flow to inlet (cfs)
    pub approach_flow: f64,
    /// Intercepted flow entering system (cfs)
    pub intercepted_flow: f64,
    /// Bypass flow continuing downstream (cfs)
    pub bypass_flow: f64,
    /// Interception efficiency (0.0 to 1.0)
    pub efficiency: f64,
    /// Gutter spread at inlet (ft)
    pub spread: f64,
}

/// Route flows through network accounting for inlet interception
///
/// This enhanced routing function:
/// 1. Routes flows from upstream to downstream
/// 2. At each inlet node, calculates inlet interception efficiency
/// 3. Tracks bypass flows that continue in gutters to downstream inlets
/// 4. Sag inlets capture 100% of flow
///
/// # Arguments
/// * `network` - The drainage network
/// * `node_inflows` - Direct inflows at each node (from drainage areas)
/// * `unit_system` - Unit system for gutter calculations
///
/// # Returns
/// Tuple of (conduit flows, inlet interception results)
pub fn route_flows_with_inlets(
    network: &Network,
    node_inflows: &HashMap<String, f64>,
    unit_system: UnitSystem,
) -> Result<(HashMap<String, f64>, Vec<InletInterception>), String> {
    let mut conduit_flows = HashMap::new();
    let mut node_total_flows: HashMap<String, f64> = HashMap::new();
    let mut bypass_flows: HashMap<String, f64> = HashMap::new();
    let mut inlet_results = Vec::new();

    let k = match unit_system {
        UnitSystem::US => GUTTER_K_US,
        UnitSystem::SI => GUTTER_K_SI,
    };

    // Initialize with direct inflows
    for (node_id, &flow) in node_inflows {
        node_total_flows.insert(node_id.clone(), flow);
    }

    // Get traversal order
    let sorted_nodes = topological_sort_upstream_to_downstream(network)?;

    // Process nodes in topological order
    for node_id in sorted_nodes {

        // Find the node
        let node = network
            .nodes
            .iter()
            .find(|n| n.id == node_id)
            .ok_or_else(|| format!("Node {} not found", node_id))?;

        // Get total flow approaching this node
        let direct_inflow = node_total_flows.get(&node_id).cloned().unwrap_or(0.0);
        let bypass_inflow = bypass_flows.get(&node_id).cloned().unwrap_or(0.0);
        let approach_flow = direct_inflow + bypass_inflow;

        // Determine intercepted vs bypass flow
        let (intercepted_flow, bypass_flow, interception_result) =
            if let Some(ref inlet_props) = node.inlet {
                // This is an inlet - calculate interception
                calculate_inlet_interception(node, inlet_props, approach_flow, k)?
            } else {
                // Not an inlet - all flow enters system
                (approach_flow, 0.0, None)
            };

        // Store inlet interception result
        if let Some(result) = interception_result {
            inlet_results.push(result);
        }

        // Route intercepted flow to downstream conduits
        let downstream_conduits = network.downstream_conduits(&node_id);
        if !downstream_conduits.is_empty() {
            let flow_per_conduit = intercepted_flow / downstream_conduits.len() as f64;

            for conduit in downstream_conduits {
                // Add intercepted flow to the underground system
                conduit_flows.insert(conduit.id.clone(), flow_per_conduit);
                let downstream_total = node_total_flows
                    .entry(conduit.to_node.clone())
                    .or_insert(0.0);
                *downstream_total += flow_per_conduit;

                // Add bypass flow to the downstream gutter
                if bypass_flow > 0.0 {
                    let downstream_bypass = bypass_flows
                        .entry(conduit.to_node.clone())
                        .or_insert(0.0);
                    *downstream_bypass += bypass_flow;
                }
            }
        }
    }

    Ok((conduit_flows, inlet_results))
}

/// Calculate inlet interception for a given inlet node
///
/// Returns (intercepted_flow, bypass_flow, inlet_result)
fn calculate_inlet_interception(
    node: &Node,
    inlet_props: &crate::node::InletProperties,
    approach_flow: f64,
    k: f64,
) -> Result<(f64, f64, Option<InletInterception>), String> {
    if approach_flow <= 0.0 {
        return Ok((0.0, 0.0, None));
    }

    // Check if this is a sag inlet (100% capture)
    if inlet_props.location == InletLocation::Sag {
        let result = InletInterception {
            node_id: node.id.clone(),
            approach_flow,
            intercepted_flow: approach_flow,
            bypass_flow: 0.0,
            efficiency: 1.0,
            spread: 0.0, // Ponded at sag
        };
        return Ok((approach_flow, 0.0, Some(result)));
    }

    // On-grade inlet - need to calculate interception

    // Get gutter properties from upstream conduit (if it's a gutter)
    // For now, use default gutter assumptions
    let manning_n = 0.016; // Asphalt
    let cross_slope = 0.02; // 2%
    let longitudinal_slope = 0.01; // 1% (default)

    let gutter = UniformGutter::new(manning_n, cross_slope, longitudinal_slope, None);
    let gutter_result = gutter.result_for_flow(approach_flow, k);

    // Determine inlet type and calculate interception
    let local_depression = inlet_props.local_depression.unwrap_or(0.0);
    let clogging_factor = inlet_props.clogging_factor.unwrap_or(0.15);

    let interception: InletInterceptionResult = match inlet_props.inlet_type {
        crate::node::InletType::Grate => {
            if let Some(ref grate_props) = inlet_props.grate {
                let length = grate_props.length.unwrap_or(3.0);
                let width = grate_props.width.unwrap_or(2.0);
                let bar_config = match grate_props.bar_configuration {
                    Some(crate::node::BarConfiguration::Parallel) => InletBarConfig::Parallel,
                    _ => InletBarConfig::Perpendicular,
                };

                let inlet = GrateInletOnGrade::new(
                    length,
                    width,
                    bar_config,
                    clogging_factor,
                    local_depression,
                );

                inlet.interception(approach_flow, &gutter_result)
            } else {
                // No grate properties - assume default
                let inlet =
                    GrateInletOnGrade::new(3.0, 2.0, InletBarConfig::Perpendicular, 0.15, 2.0);
                inlet.interception(approach_flow, &gutter_result)
            }
        }

        crate::node::InletType::CurbOpening => {
            if let Some(ref curb_props) = inlet_props.curb_opening {
                let length = curb_props.length.unwrap_or(5.0);
                let height = curb_props.height.unwrap_or(0.5);
                let throat_type = match curb_props.throat_type {
                    Some(crate::node::ThroatType::Inclined) => InletThroatType::Inclined,
                    Some(crate::node::ThroatType::Vertical) => InletThroatType::Vertical,
                    _ => InletThroatType::Horizontal,
                };

                let inlet = CurbOpeningInletOnGrade::new(length, height, throat_type, clogging_factor);
                inlet.interception(approach_flow, &gutter_result)
            } else {
                // Default curb opening
                let inlet = CurbOpeningInletOnGrade::new(5.0, 0.5, InletThroatType::Horizontal, 0.10);
                inlet.interception(approach_flow, &gutter_result)
            }
        }

        crate::node::InletType::Combination => {
            // Combination inlet with both grate and curb opening
            let grate_length = inlet_props.grate.as_ref()
                .and_then(|g| g.length).unwrap_or(3.0);
            let grate_width = inlet_props.grate.as_ref()
                .and_then(|g| g.width).unwrap_or(2.0);
            let bar_config = inlet_props.grate.as_ref()
                .and_then(|g| g.bar_configuration)
                .map(|bc| match bc {
                    crate::node::BarConfiguration::Parallel => InletBarConfig::Parallel,
                    _ => InletBarConfig::Perpendicular,
                })
                .unwrap_or(InletBarConfig::Perpendicular);

            let curb_length = inlet_props.curb_opening.as_ref()
                .and_then(|c| c.length).unwrap_or(5.0);
            let curb_height = inlet_props.curb_opening.as_ref()
                .and_then(|c| c.height).unwrap_or(0.5);
            let curb_throat = inlet_props.curb_opening.as_ref()
                .and_then(|c| c.throat_type)
                .map(|tt| match tt {
                    crate::node::ThroatType::Inclined => InletThroatType::Inclined,
                    crate::node::ThroatType::Vertical => InletThroatType::Vertical,
                    _ => InletThroatType::Horizontal,
                })
                .unwrap_or(InletThroatType::Horizontal);

            let grate = GrateInletOnGrade::new(
                grate_length,
                grate_width,
                bar_config,
                clogging_factor,
                local_depression,
            );

            let curb = CurbOpeningInletOnGrade::new(curb_length, curb_height, curb_throat, clogging_factor);

            let combo = CombinationInletOnGrade::new(grate, curb);
            combo.interception(approach_flow, &gutter_result)
        }

        crate::node::InletType::Slotted => {
            // Slotted drains typically have high efficiency
            // For now, assume 80% efficiency
            InletInterceptionResult {
                approach_flow,
                intercepted_flow: approach_flow * 0.80,
                bypass_flow: approach_flow * 0.20,
                efficiency: 0.80,
                spread: gutter_result.spread,
                velocity: gutter_result.velocity,
            }
        }
    };

    let result = InletInterception {
        node_id: node.id.clone(),
        approach_flow: interception.approach_flow,
        intercepted_flow: interception.intercepted_flow,
        bypass_flow: interception.bypass_flow,
        efficiency: interception.efficiency,
        spread: interception.spread,
    };

    Ok((interception.intercepted_flow, interception.bypass_flow, Some(result)))
}

/// Perform an upstream-to-downstream topological sort of the network nodes.
///
/// This implementation uses Kahn's algorithm. It's used for flow routing
/// to ensure that a node is processed only after all its upstream contributors
/// have been accounted for.
///
/// # Arguments
/// * `network` - The drainage network.
///
/// # Returns
/// A `Vec<String>` containing the node IDs in topologically sorted order,
/// or an error if a cycle is detected.
fn topological_sort_upstream_to_downstream(
    network: &Network,
) -> Result<Vec<String>, String> {
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut queue: Vec<String> = Vec::new();
    let mut sorted_nodes: Vec<String> = Vec::new();

    // Initialize in-degree for all nodes
    for node in &network.nodes {
        in_degree.insert(node.id.clone(), network.upstream_conduits(&node.id).len());
        if *in_degree.get(&node.id).unwrap() == 0 {
            queue.push(node.id.clone());
        }
    }

    // Process nodes with an in-degree of 0
    while let Some(node_id) = queue.pop() {
        sorted_nodes.push(node_id.clone());

        // For each downstream node, decrement its in-degree
        for conduit in network.downstream_conduits(&node_id) {
            if let Some(degree) = in_degree.get_mut(&conduit.to_node) {
                *degree -= 1;
                if *degree == 0 {
                    queue.push(conduit.to_node.clone());
                }
            }
        }
    }

    // Check for cycles
    if sorted_nodes.len() != network.nodes.len() {
        Err("A cycle was detected in the network graph.".to_string())
    } else {
        Ok(sorted_nodes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::conduit::{PipeMaterial, PipeProperties, PipeShape};
    use crate::node::OutfallProperties;

    #[test]
    fn test_solver_config() {
        let config = SolverConfig::us_customary();
        assert_eq!(config.unit_system, UnitSystem::US);
        assert_eq!(config.gravity, 32.17);
        assert_eq!(config.manning_k, 1.486);
    }

    #[test]
    fn test_compute_rational_flows() {
        let areas = vec![
            DrainageArea {
                id: "DA-001".to_string(),
                name: None,
                area: 1.0,
                outlet: "IN-001".to_string(),
                land_use: None,
                runoff_coefficient: Some(0.8),
                time_of_concentration: Some(10.0),
                tc_calculation: None,
                curve_number: None,
                geometry: None,
            },
        ];

        let flows = compute_rational_flows(&areas, 4.0);

        assert_eq!(flows.get("IN-001"), Some(&3.2)); // 0.8 × 4.0 × 1.0
    }
}
