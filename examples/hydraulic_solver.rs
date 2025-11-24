//! Example: Hydraulic analysis using HGL/EGL solver
//!
//! This example demonstrates how to use the hydraulic solver to compute
//! water surface elevations (HGL) and energy grade lines (EGL) for a
//! simple drainage network.

use hec22::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== HEC-22 Hydraulic Analysis Example ===\n");

    // 1. Create a simple 3-node network
    //    IN-001 (inlet) → P-101 (pipe) → MH-001 (junction) → P-102 (pipe) → OUT-001 (outfall)

    let project = project::Project {
        name: "Hydraulic Analysis Example".to_string(),
        description: Some("Simple network demonstrating HGL/EGL solver".to_string()),
        location: None,
        units: project::Units::us_customary(),
        author: Some("HEC-22 Solver".to_string()),
        created: Some(chrono::Utc::now().to_rfc3339()),
        modified: None,
    };

    // Create nodes
    let inlet = node::Node::new_inlet(
        "IN-001".to_string(),
        124.5,  // invert elevation
        128.0,  // rim elevation
        node::InletProperties {
            inlet_type: node::InletType::Combination,
            location: node::InletLocation::OnGrade,
            grate: None,
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.15),
        },
    );

    let junction = node::Node::new_junction(
        "MH-001".to_string(),
        118.5,  // invert elevation
        125.0,  // rim elevation
        node::JunctionProperties {
            diameter: Some(4.0),
            sump_depth: Some(0.5),
            loss_coefficient: Some(0.15),
            benching: Some(true),
            drop_structure: Some(false),
        },
    );

    let outfall = node::Node::new_outfall(
        "OUT-001".to_string(),
        115.0,  // invert elevation
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::NormalDepth,
            tailwater_elevation: Some(116.5),
            tidal_curve: None,
        },
    );

    // Create pipes
    let mut pipe1 = conduit::Conduit::new_pipe(
        "P-101".to_string(),
        "IN-001".to_string(),
        "MH-001".to_string(),
        150.0,  // length
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(18.0),  // inches
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );

    // Set inverts
    pipe1.upstream_invert = Some(124.3);
    pipe1.downstream_invert = Some(118.65);

    let mut pipe2 = conduit::Conduit::new_pipe(
        "P-102".to_string(),
        "MH-001".to_string(),
        "OUT-001".to_string(),
        180.0,  // length
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(24.0),  // inches
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );

    pipe2.upstream_invert = Some(118.25);
    pipe2.downstream_invert = Some(115.15);

    // Build network
    let mut network = network::Network::new();
    network.add_node(inlet);
    network.add_node(junction);
    network.add_node(outfall);
    network.add_conduit(pipe1);
    network.add_conduit(pipe2);

    // Validate connectivity
    network.validate_connectivity()?;
    println!("✓ Network validated");
    println!("  Nodes: {}", network.node_count());
    println!("  Conduits: {}", network.conduit_count());

    // 2. Create drainage area
    let drainage_area = drainage::DrainageArea {
        id: "DA-001".to_string(),
        name: Some("Contributing Area".to_string()),
        area: 1.25,  // acres
        outlet: "IN-001".to_string(),
        land_use: Some(drainage::LandUse {
            primary: Some(drainage::LandUseType::Commercial),
            impervious_percent: Some(85.0),
            composition: None,
        }),
        runoff_coefficient: Some(0.82),
        time_of_concentration: Some(12.5),
        tc_calculation: None,
        curve_number: None,
        geometry: None,
    };

    // 3. Compute flows using Rational Method
    let intensity = 3.8; // in/hr (10-year storm)
    println!("\n--- Hydrologic Analysis ---");
    println!("Design intensity: {:.1} in/hr", intensity);

    let node_inflows = solver::compute_rational_flows(&[drainage_area.clone()], intensity);

    for (node_id, flow) in &node_inflows {
        println!("  Node {} inflow: {:.2} cfs", node_id, flow);
    }

    // Route flows through network
    let conduit_flows = solver::route_flows(&network, &node_inflows)?;

    println!("\nConduit flows:");
    for (conduit_id, flow) in &conduit_flows {
        println!("  Conduit {}: {:.2} cfs", conduit_id, flow);
    }

    // 4. Run hydraulic solver
    println!("\n--- Hydraulic Analysis (HGL/EGL Solver) ---");

    let config = solver::SolverConfig::us_customary();
    let hgl_solver = solver::HglSolver::new(config);

    let analysis_result = hgl_solver.solve(
        &network,
        &conduit_flows,
        "10-year-storm".to_string(),
    )?;

    // 5. Display results
    println!("\nNode Results:");
    println!("{:<10} {:>10} {:>10} {:>10} {:>10}", "Node", "HGL (ft)", "EGL (ft)", "Depth (ft)", "Flooding");
    println!("{}", "-".repeat(55));

    if let Some(ref node_results) = analysis_result.node_results {
        for result in node_results {
            let hgl = result.hgl.unwrap_or(0.0);
            let egl = result.egl.unwrap_or(0.0);
            let depth = result.depth.unwrap_or(0.0);
            let flooding = if result.flooding.unwrap_or(false) { "YES" } else { "No" };

            println!(
                "{:<10} {:>10.2} {:>10.2} {:>10.2} {:>10}",
                result.node_id, hgl, egl, depth, flooding
            );
        }
    }

    println!("\nConduit Results:");
    println!("{:<10} {:>10} {:>10} {:>10} {:>12}", "Conduit", "Flow (cfs)", "Vel (ft/s)", "Depth (ft)", "Capacity (%)");
    println!("{}", "-".repeat(60));

    if let Some(ref conduit_results) = analysis_result.conduit_results {
        for result in conduit_results {
            let flow = result.flow.unwrap_or(0.0);
            let velocity = result.velocity.unwrap_or(0.0);
            let depth = result.depth.unwrap_or(0.0);
            let capacity = result.capacity_used.unwrap_or(0.0) * 100.0;

            println!(
                "{:<10} {:>10.2} {:>10.2} {:>10.2} {:>12.1}",
                result.conduit_id, flow, velocity, depth, capacity
            );

            // Display energy losses
            if let Some(ref headloss) = result.headloss {
                println!("           Energy losses:");
                if let Some(friction) = headloss.friction {
                    println!("             Friction: {:.3} ft", friction);
                }
                if let Some(entrance) = headloss.entrance {
                    println!("             Entrance: {:.3} ft", entrance);
                }
                if let Some(exit) = headloss.exit {
                    println!("             Exit: {:.3} ft", exit);
                }
                if let Some(total) = headloss.total {
                    println!("             Total: {:.3} ft", total);
                }
            }
        }
    }

    // 6. Check for violations
    println!("\n--- Design Criteria Check ---");

    if let Some(ref violations) = analysis_result.violations {
        if violations.is_empty() {
            println!("✓ No violations found - design meets all criteria");
        } else {
            println!("⚠ Found {} violation(s):", violations.len());
            for violation in violations {
                println!(
                    "  [{:?}] {}: {}",
                    violation.severity,
                    violation.element_id,
                    violation.message
                );
            }
        }
    }

    // 7. Export to JSON
    let mut drainage_network = DrainageNetwork::new(project, network);
    drainage_network.drainage_areas = Some(vec![drainage_area]);
    drainage_network.analysis = Some(analysis_result);

    let json = drainage_network.to_json()?;
    std::fs::write("hydraulic_analysis_result.json", json)?;
    println!("\n✓ Results exported to: hydraulic_analysis_result.json");

    Ok(())
}
