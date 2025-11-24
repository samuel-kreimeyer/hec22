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
use crate::hydraulics::{EnergyLoss, FlowRegime, ManningsEquation, PipeFlowResult};
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
}

impl HglSolver {
    /// Create a new solver with the given configuration
    pub fn new(config: SolverConfig) -> Self {
        let mannings = ManningsEquation { k: config.manning_k };
        let energy_loss = EnergyLoss { gravity: config.gravity };

        Self {
            config,
            mannings,
            energy_loss,
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

            if let Some(ref mut results) = analysis.conduit_results {
                results.push(conduit_result);
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

    // Get all nodes in upstream-to-downstream order
    let mut visited = std::collections::HashSet::new();
    let mut stack = Vec::new();

    // Start from inlets (nodes with no upstream conduits)
    for node in &network.nodes {
        if network.upstream_conduits(&node.id).is_empty() && !node.is_outfall() {
            stack.push(node.id.clone());
        }
    }

    // Process nodes from upstream to downstream
    while let Some(node_id) = stack.pop() {
        if visited.contains(&node_id) {
            continue;
        }
        visited.insert(node_id.clone());

        // Get total flow at this node
        let node_flow = node_total_flows.get(&node_id).cloned().unwrap_or(0.0);

        // Route flow through downstream conduits
        let downstream_conduits = network.downstream_conduits(&node_id);

        if !downstream_conduits.is_empty() {
            // Distribute flow evenly if multiple outlets (shouldn't happen in typical networks)
            let flow_per_conduit = node_flow / downstream_conduits.len() as f64;

            for conduit in downstream_conduits {
                // Set conduit flow
                conduit_flows.insert(conduit.id.clone(), flow_per_conduit);

                // Add flow to downstream node
                let downstream_flow = node_total_flows
                    .entry(conduit.to_node.clone())
                    .or_insert(0.0);
                *downstream_flow += flow_per_conduit;

                // Add downstream node to processing queue
                if !visited.contains(&conduit.to_node) {
                    stack.push(conduit.to_node.clone());
                }
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

    // Get all nodes in upstream-to-downstream order
    let mut visited = std::collections::HashSet::new();
    let mut stack = Vec::new();

    // Start from inlets (nodes with no upstream conduits)
    for node in &network.nodes {
        if network.upstream_conduits(&node.id).is_empty() && !node.is_outfall() {
            stack.push(node.id.clone());
        }
    }

    // Process nodes from upstream to downstream
    while let Some(node_id) = stack.pop() {
        if visited.contains(&node_id) {
            continue;
        }
        visited.insert(node_id.clone());

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

        // Route intercepted flow through downstream conduits
        let downstream_conduits = network.downstream_conduits(&node_id);

        if !downstream_conduits.is_empty() {
            // For on-grade inlets, intercepted flow goes to underground system
            // Bypass flow continues in gutter to next inlet
            let flow_per_conduit = intercepted_flow / downstream_conduits.len() as f64;

            for conduit in downstream_conduits {
                // Set conduit flow (underground system)
                conduit_flows.insert(conduit.id.clone(), flow_per_conduit);

                // Add intercepted flow to downstream node
                let downstream_flow = node_total_flows
                    .entry(conduit.to_node.clone())
                    .or_insert(0.0);
                *downstream_flow += flow_per_conduit;

                // Add bypass flow to downstream node (gutter flow)
                if bypass_flow > 0.0 {
                    let downstream_bypass = bypass_flows
                        .entry(conduit.to_node.clone())
                        .or_insert(0.0);
                    *downstream_bypass += bypass_flow;
                }

                // Add downstream node to processing queue
                if !visited.contains(&conduit.to_node) {
                    stack.push(conduit.to_node.clone());
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
