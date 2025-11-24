//! Example: Complete workflow with inlet bypass flow tracking
//!
//! This example demonstrates the complete drainage analysis workflow including:
//! 1. Network setup with series of inlets
//! 2. Drainage area calculations
//! 3. Inlet interception with bypass flow tracking
//! 4. Hydraulic analysis with HGL/EGL solver
//!
//! This shows how bypass flows from on-grade inlets affect downstream inlets.

use hec22::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Complete Drainage Analysis with Inlet Bypass Tracking ===\n");

    // 1. Create project
    let project = project::Project {
        name: "Street Drainage with Inlet Bypass".to_string(),
        description: Some("Series of inlets demonstrating bypass flow routing".to_string()),
        location: None,
        units: project::Units::us_customary(),
        author: Some("HEC-22 Solver".to_string()),
        created: Some(chrono::Utc::now().to_rfc3339()),
        modified: None,
    };

    println!("Project: {}\n", project.name);

    // 2. Create nodes - series of 3 inlets draining to outfall
    let inlet1 = node::Node::new_inlet(
        "IN-001".to_string(),
        100.0,  // invert elevation
        104.0,  // rim elevation
        node::InletProperties {
            inlet_type: node::InletType::Grate,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(3.0),
                width: Some(2.0),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.15),
        },
    );

    let inlet2 = node::Node::new_inlet(
        "IN-002".to_string(),
        98.0,
        102.0,
        node::InletProperties {
            inlet_type: node::InletType::Combination,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(3.0),
                width: Some(2.0),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: Some(node::CurbOpeningProperties {
                length: Some(5.0),
                height: Some(0.5),
                throat_type: Some(node::ThroatType::Horizontal),
            }),
            local_depression: Some(2.0),
            clogging_factor: Some(0.15),
        },
    );

    let inlet3 = node::Node::new_inlet(
        "IN-003".to_string(),
        95.0,
        99.0,
        node::InletProperties {
            inlet_type: node::InletType::Grate,
            location: node::InletLocation::Sag,  // Sag inlet captures 100%
            grate: Some(node::GrateProperties {
                length: Some(4.0),
                width: Some(3.0),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: None,
            clogging_factor: Some(0.50),
        },
    );

    let junction = node::Node::new_junction(
        "MH-001".to_string(),
        92.0,
        98.0,
        node::JunctionProperties {
            diameter: Some(5.0),
            sump_depth: Some(0.5),
            loss_coefficient: Some(0.15),
            benching: Some(true),
            drop_structure: Some(false),
        },
    );

    let outfall = node::Node::new_outfall(
        "OUT-001".to_string(),
        90.0,
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::NormalDepth,
            tailwater_elevation: Some(91.0),
            tidal_curve: None,
        },
    );

    // 3. Create conduits connecting inlets to junction
    let mut pipe1 = conduit::Conduit::new_pipe(
        "P-101".to_string(),
        "IN-001".to_string(),
        "MH-001".to_string(),
        250.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(18.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe1.upstream_invert = Some(99.8);
    pipe1.downstream_invert = Some(92.5);

    let mut pipe2 = conduit::Conduit::new_pipe(
        "P-102".to_string(),
        "IN-002".to_string(),
        "MH-001".to_string(),
        200.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(24.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe2.upstream_invert = Some(97.8);
    pipe2.downstream_invert = Some(92.5);

    let mut pipe3 = conduit::Conduit::new_pipe(
        "P-103".to_string(),
        "IN-003".to_string(),
        "MH-001".to_string(),
        150.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(30.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe3.upstream_invert = Some(94.8);
    pipe3.downstream_invert = Some(92.5);

    let mut pipe4 = conduit::Conduit::new_pipe(
        "P-104".to_string(),
        "MH-001".to_string(),
        "OUT-001".to_string(),
        180.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(36.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe4.upstream_invert = Some(92.0);
    pipe4.downstream_invert = Some(90.2);

    // 4. Build network
    let mut network = network::Network::new();
    network.add_node(inlet1);
    network.add_node(inlet2);
    network.add_node(inlet3);
    network.add_node(junction);
    network.add_node(outfall);
    network.add_conduit(pipe1);
    network.add_conduit(pipe2);
    network.add_conduit(pipe3);
    network.add_conduit(pipe4);

    network.validate_connectivity()?;
    println!("✓ Network validated");
    println!("  Nodes: {}", network.node_count());
    println!("  Conduits: {}\n", network.conduit_count());

    // 5. Create drainage areas
    let drainage_areas = vec![
        drainage::DrainageArea {
            id: "DA-001".to_string(),
            name: Some("Inlet 1 catchment".to_string()),
            area: 0.8,  // acres
            outlet: "IN-001".to_string(),
            land_use: Some(drainage::LandUse {
                primary: Some(drainage::LandUseType::Residential),
                impervious_percent: Some(35.0),
                composition: None,
            }),
            runoff_coefficient: Some(0.50),
            time_of_concentration: Some(10.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        },
        drainage::DrainageArea {
            id: "DA-002".to_string(),
            name: Some("Inlet 2 catchment".to_string()),
            area: 1.2,
            outlet: "IN-002".to_string(),
            land_use: Some(drainage::LandUse {
                primary: Some(drainage::LandUseType::Residential),
                impervious_percent: Some(40.0),
                composition: None,
            }),
            runoff_coefficient: Some(0.55),
            time_of_concentration: Some(12.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        },
        drainage::DrainageArea {
            id: "DA-003".to_string(),
            name: Some("Inlet 3 catchment".to_string()),
            area: 1.5,
            outlet: "IN-003".to_string(),
            land_use: Some(drainage::LandUse {
                primary: Some(drainage::LandUseType::Residential),
                impervious_percent: Some(45.0),
                composition: None,
            }),
            runoff_coefficient: Some(0.60),
            time_of_concentration: Some(15.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        },
    ];

    // 6. Compute runoff using Rational Method
    let intensity = 4.2; // in/hr (10-year storm)
    println!("--- Hydrologic Analysis ---");
    println!("Design storm: 10-year");
    println!("Intensity: {:.1} in/hr\n", intensity);

    let node_inflows = solver::compute_rational_flows(&drainage_areas, intensity);

    println!("Direct inflows (from drainage areas):");
    for (node_id, flow) in &node_inflows {
        println!("  {}: {:.2} cfs", node_id, flow);
    }
    println!();

    // 7. Route flows WITH inlet interception tracking
    println!("--- Flow Routing with Inlet Interception ---\n");

    let (conduit_flows, inlet_results) = solver::route_flows_with_inlets(
        &network,
        &node_inflows,
        project::UnitSystem::US,
    )?;

    // Display inlet interception results
    println!("Inlet Performance:");
    println!("{:<10} {:>10} {:>10} {:>10} {:>10} {:>10}",
             "Inlet", "Approach", "Captured", "Bypass", "Efficiency", "Spread");
    println!("{}", "-".repeat(70));

    for result in &inlet_results {
        println!("{:<10} {:>10.2} {:>10.2} {:>10.2} {:>9.1}% {:>10.2}",
                 result.node_id,
                 result.approach_flow,
                 result.intercepted_flow,
                 result.bypass_flow,
                 result.efficiency * 100.0,
                 result.spread);
    }
    println!();

    // Show how bypass affects next inlet
    println!("Bypass Flow Analysis:");
    if inlet_results.len() >= 2 {
        println!("• Inlet 1 bypassed {:.2} cfs", inlet_results[0].bypass_flow);
        println!("• This bypass would normally flow to the next downstream inlet");
        println!("  (In this example, inlets drain to separate pipes, so no accumulation)");
    }
    println!();

    println!("Underground System Flows (Pipe Network):");
    println!("{:<10} {:>10}", "Conduit", "Flow (cfs)");
    println!("{}", "-".repeat(22));
    for (conduit_id, flow) in &conduit_flows {
        println!("{:<10} {:>10.2}", conduit_id, flow);
    }
    println!();

    // 8. Run hydraulic analysis
    println!("--- Hydraulic Analysis (HGL/EGL) ---\n");

    let config = solver::SolverConfig::us_customary();
    let hgl_solver = solver::HglSolver::new(config);

    let analysis = hgl_solver.solve(
        &network,
        &conduit_flows,
        "10-year-storm".to_string(),
    )?;

    // Display results
    println!("Node Results:");
    println!("{:<10} {:>10} {:>10} {:>10} {:>10}",
             "Node", "HGL (ft)", "EGL (ft)", "Depth (ft)", "Flooding");
    println!("{}", "-".repeat(55));

    if let Some(ref node_results) = analysis.node_results {
        for result in node_results {
            let hgl = result.hgl.unwrap_or(0.0);
            let egl = result.egl.unwrap_or(0.0);
            let depth = result.depth.unwrap_or(0.0);
            let flooding = if result.flooding.unwrap_or(false) { "YES" } else { "No" };

            println!("{:<10} {:>10.2} {:>10.2} {:>10.2} {:>10}",
                     result.node_id, hgl, egl, depth, flooding);
        }
    }
    println!();

    println!("Conduit Results:");
    println!("{:<10} {:>10} {:>10} {:>10} {:>12}",
             "Conduit", "Flow", "Velocity", "Depth", "Capacity");
    println!("{}", "-".repeat(60));

    if let Some(ref conduit_results) = analysis.conduit_results {
        for result in conduit_results {
            let flow = result.flow.unwrap_or(0.0);
            let velocity = result.velocity.unwrap_or(0.0);
            let depth = result.depth.unwrap_or(0.0);
            let capacity = result.capacity_used.unwrap_or(0.0) * 100.0;

            println!("{:<10} {:>10.2} {:>10.2} {:>10.2} {:>11.1}%",
                     result.conduit_id, flow, velocity, depth, capacity);
        }
    }
    println!();

    // 9. Check for violations
    println!("--- Design Criteria Check ---");

    if let Some(ref violations) = analysis.violations {
        if violations.is_empty() {
            println!("✓ No violations - design meets all criteria");
        } else {
            println!("⚠ Found {} violation(s):", violations.len());
            for violation in violations {
                println!("  [{:?}] {}: {}",
                         violation.severity,
                         violation.element_id,
                         violation.message);
            }
        }
    }
    println!();

    // 10. Summary
    println!("--- Summary ---");
    let total_drainage_flow: f64 = node_inflows.values().sum();
    let total_intercepted: f64 = inlet_results.iter()
        .map(|r| r.intercepted_flow)
        .sum();
    let total_bypass: f64 = inlet_results.iter()
        .map(|r| r.bypass_flow)
        .sum();

    println!("Total drainage area runoff: {:.2} cfs", total_drainage_flow);
    println!("Total intercepted by inlets: {:.2} cfs", total_intercepted);
    println!("Total bypass flow: {:.2} cfs", total_bypass);
    println!("System capture efficiency: {:.1}%",
             (total_intercepted / total_drainage_flow) * 100.0);

    let outlet_flow = conduit_flows.get("P-104").unwrap_or(&0.0);
    println!("\nFlow at outfall: {:.2} cfs", outlet_flow);

    // 11. Export to JSON
    let mut drainage_network = DrainageNetwork::new(project, network);
    drainage_network.drainage_areas = Some(drainage_areas);
    drainage_network.analysis = Some(analysis);

    let json = drainage_network.to_json()?;
    std::fs::write("inlet_bypass_analysis.json", json)?;
    println!("\n✓ Results exported to: inlet_bypass_analysis.json");

    Ok(())
}
