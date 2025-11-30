//! Test for multi-level tributary flow isolation in complex networks
//!
//! This test verifies that tributary flows are correctly isolated even in networks
//! with multiple levels of branching and convergence.

use hec22::*;

/// Test multi-level branching with nested tributaries
///
/// Network topology:
///                    Inlet-A1 (2 cfs) ↘
///                                       Conduit-A1 → Junction-A (4 cfs) ↘
///                    Inlet-A2 (2 cfs) ↗                                   ↘
///                                                                           Trunk → Outfall (9 cfs)
///                    Inlet-B1 (3 cfs) ↘                                   ↗
///                                       Conduit-B1 → Junction-B (5 cfs) ↗
///                    Inlet-B2 (2 cfs) ↗
///
/// Expected behavior:
/// - Conduit-A1 should carry 4 cfs (2+2 from sub-branch A)
/// - Conduit-B1 should carry 5 cfs (3+2 from sub-branch B)
/// - Trunk should carry 9 cfs (4+5 from both main branches)
#[test]
fn test_multi_level_tributary_flow_isolation() {
    // Create project
    let project = project::Project {
        name: "Multi-Level Tributary Flow Test".to_string(),
        description: Some("Verify tributary flow isolation in complex multi-level networks".to_string()),
        location: None,
        units: project::Units::us_customary(),
        author: Some("Test Suite".to_string()),
        created: Some(chrono::Utc::now().to_rfc3339()),
        modified: None,
    };

    // Create nodes for Branch A (upper branch)
    let inlet_a1 = node::Node::new_inlet(
        "Inlet-A1".to_string(),
        145.0,
        149.0,
        node::InletProperties {
            inlet_type: node::InletType::Grate,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.10),
        },
    );

    let inlet_a2 = node::Node::new_inlet(
        "Inlet-A2".to_string(),
        143.0,
        147.0,
        node::InletProperties {
            inlet_type: node::InletType::Grate,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.10),
        },
    );

    let junction_a = node::Node::new_junction(
        "Junction-A".to_string(),
        138.0,
        145.0,
        node::JunctionProperties {
            diameter: Some(4.0),
            sump_depth: Some(1.0),
            loss_coefficient: Some(0.2),
            benching: Some(true),
            drop_structure: Some(false),
        },
    );

    // Create nodes for Branch B (lower branch)
    let inlet_b1 = node::Node::new_inlet(
        "Inlet-B1".to_string(),
        142.0,
        146.0,
        node::InletProperties {
            inlet_type: node::InletType::Grate,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.10),
        },
    );

    let inlet_b2 = node::Node::new_inlet(
        "Inlet-B2".to_string(),
        140.0,
        144.0,
        node::InletProperties {
            inlet_type: node::InletType::Grate,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.10),
        },
    );

    let junction_b = node::Node::new_junction(
        "Junction-B".to_string(),
        135.0,
        142.0,
        node::JunctionProperties {
            diameter: Some(4.0),
            sump_depth: Some(1.0),
            loss_coefficient: Some(0.2),
            benching: Some(true),
            drop_structure: Some(false),
        },
    );

    // Main junction where both branches meet
    let main_junction = node::Node::new_junction(
        "Main-Junction".to_string(),
        128.0,
        135.0,
        node::JunctionProperties {
            diameter: Some(5.0),
            sump_depth: Some(1.5),
            loss_coefficient: Some(0.2),
            benching: Some(true),
            drop_structure: Some(false),
        },
    );

    // Outfall
    let outfall = node::Node::new_outfall(
        "Outfall".to_string(),
        122.0,
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::NormalDepth,
            tailwater_elevation: Some(123.5),
            tidal_curve: None,
        },
    );

    // Create conduits
    // Branch A sub-conduits
    let mut pipe_a1a = conduit::Conduit::new_pipe(
        "Pipe-A1a".to_string(),
        "Inlet-A1".to_string(),
        "Junction-A".to_string(),
        150.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(15.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe_a1a.upstream_invert = Some(145.0);
    pipe_a1a.downstream_invert = Some(138.5);

    let mut pipe_a2a = conduit::Conduit::new_pipe(
        "Pipe-A2a".to_string(),
        "Inlet-A2".to_string(),
        "Junction-A".to_string(),
        160.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(15.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe_a2a.upstream_invert = Some(143.0);
    pipe_a2a.downstream_invert = Some(138.7);

    // Branch A main conduit
    let mut conduit_a = conduit::Conduit::new_pipe(
        "Conduit-A".to_string(),
        "Junction-A".to_string(),
        "Main-Junction".to_string(),
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
    conduit_a.upstream_invert = Some(138.0);
    conduit_a.downstream_invert = Some(128.5);

    // Branch B sub-conduits
    let mut pipe_b1b = conduit::Conduit::new_pipe(
        "Pipe-B1b".to_string(),
        "Inlet-B1".to_string(),
        "Junction-B".to_string(),
        170.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(15.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe_b1b.upstream_invert = Some(142.0);
    pipe_b1b.downstream_invert = Some(135.5);

    let mut pipe_b2b = conduit::Conduit::new_pipe(
        "Pipe-B2b".to_string(),
        "Inlet-B2".to_string(),
        "Junction-B".to_string(),
        180.0,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(15.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe_b2b.upstream_invert = Some(140.0);
    pipe_b2b.downstream_invert = Some(135.7);

    // Branch B main conduit
    let mut conduit_b = conduit::Conduit::new_pipe(
        "Conduit-B".to_string(),
        "Junction-B".to_string(),
        "Main-Junction".to_string(),
        260.0,
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
    conduit_b.upstream_invert = Some(135.0);
    conduit_b.downstream_invert = Some(128.7);

    // Trunk conduit
    let mut trunk = conduit::Conduit::new_pipe(
        "Trunk".to_string(),
        "Main-Junction".to_string(),
        "Outfall".to_string(),
        300.0,
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
    trunk.upstream_invert = Some(128.0);
    trunk.downstream_invert = Some(122.3);

    // Build network
    let mut network = network::Network::new();

    // Add all nodes
    network.add_node(inlet_a1);
    network.add_node(inlet_a2);
    network.add_node(junction_a);
    network.add_node(inlet_b1);
    network.add_node(inlet_b2);
    network.add_node(junction_b);
    network.add_node(main_junction);
    network.add_node(outfall);

    // Add all conduits
    network.add_conduit(pipe_a1a);
    network.add_conduit(pipe_a2a);
    network.add_conduit(conduit_a);
    network.add_conduit(pipe_b1b);
    network.add_conduit(pipe_b2b);
    network.add_conduit(conduit_b);
    network.add_conduit(trunk);

    // Validate network
    network
        .validate_connectivity()
        .expect("Network should be valid");

    // Create drainage areas
    // Using Q = C × i × A to get desired flows
    // i = 4.0 in/hr for all areas
    // Branch A1: 2 cfs → C=0.8, A=0.625 → Q = 0.8 × 4.0 × 0.625 = 2.0 cfs
    // Branch A2: 2 cfs → C=0.8, A=0.625 → Q = 0.8 × 4.0 × 0.625 = 2.0 cfs
    // Branch B1: 3 cfs → C=0.75, A=1.0 → Q = 0.75 × 4.0 × 1.0 = 3.0 cfs
    // Branch B2: 2 cfs → C=0.8, A=0.625 → Q = 0.8 × 4.0 × 0.625 = 2.0 cfs

    let drainage_areas = vec![
        drainage::DrainageArea {
            id: "DA-A1".to_string(),
            name: Some("Branch A1 Area".to_string()),
            area: 0.625,
            outlet: "Inlet-A1".to_string(),
            land_use: Some(drainage::LandUse {
                primary: Some(drainage::LandUseType::Commercial),
                impervious_percent: Some(80.0),
                composition: None,
            }),
            runoff_coefficient: Some(0.80),
            time_of_concentration: Some(15.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        },
        drainage::DrainageArea {
            id: "DA-A2".to_string(),
            name: Some("Branch A2 Area".to_string()),
            area: 0.625,
            outlet: "Inlet-A2".to_string(),
            land_use: Some(drainage::LandUse {
                primary: Some(drainage::LandUseType::Commercial),
                impervious_percent: Some(80.0),
                composition: None,
            }),
            runoff_coefficient: Some(0.80),
            time_of_concentration: Some(15.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        },
        drainage::DrainageArea {
            id: "DA-B1".to_string(),
            name: Some("Branch B1 Area".to_string()),
            area: 1.0,
            outlet: "Inlet-B1".to_string(),
            land_use: Some(drainage::LandUse {
                primary: Some(drainage::LandUseType::Commercial),
                impervious_percent: Some(75.0),
                composition: None,
            }),
            runoff_coefficient: Some(0.75),
            time_of_concentration: Some(15.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        },
        drainage::DrainageArea {
            id: "DA-B2".to_string(),
            name: Some("Branch B2 Area".to_string()),
            area: 0.625,
            outlet: "Inlet-B2".to_string(),
            land_use: Some(drainage::LandUse {
                primary: Some(drainage::LandUseType::Commercial),
                impervious_percent: Some(80.0),
                composition: None,
            }),
            runoff_coefficient: Some(0.80),
            time_of_concentration: Some(15.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        },
    ];

    // Compute flows
    let intensity = 4.0; // in/hr
    let node_inflows = solver::compute_rational_flows(&drainage_areas, intensity);

    // Expected flows
    let flow_a1 = 2.0; // cfs
    let flow_a2 = 2.0; // cfs
    let flow_b1 = 3.0; // cfs
    let flow_b2 = 2.0; // cfs
    let flow_branch_a = flow_a1 + flow_a2; // 4.0 cfs
    let flow_branch_b = flow_b1 + flow_b2; // 5.0 cfs
    let flow_total = flow_branch_a + flow_branch_b; // 9.0 cfs

    println!("\n=== Node Inflows ===");
    for (node_id, flow) in &node_inflows {
        println!("  {}: {:.2} cfs", node_id, flow);
    }

    // Route flows
    let conduit_flows = solver::route_flows(&network, &node_inflows)
        .expect("Flow routing should succeed");

    println!("\n=== Conduit Flows ===");
    for (conduit_id, flow) in &conduit_flows {
        println!("  {}: {:.2} cfs", conduit_id, flow);
    }

    // *** CRITICAL ASSERTIONS: Verify multi-level tributary flow isolation ***

    // Sub-branch pipes should carry only their individual inlet flows
    assert!(
        (conduit_flows["Pipe-A1a"] - flow_a1).abs() < 0.001,
        "Pipe-A1a should carry only {:.2} cfs, got {:.2} cfs",
        flow_a1,
        conduit_flows["Pipe-A1a"]
    );

    assert!(
        (conduit_flows["Pipe-A2a"] - flow_a2).abs() < 0.001,
        "Pipe-A2a should carry only {:.2} cfs, got {:.2} cfs",
        flow_a2,
        conduit_flows["Pipe-A2a"]
    );

    assert!(
        (conduit_flows["Pipe-B1b"] - flow_b1).abs() < 0.001,
        "Pipe-B1b should carry only {:.2} cfs, got {:.2} cfs",
        flow_b1,
        conduit_flows["Pipe-B1b"]
    );

    assert!(
        (conduit_flows["Pipe-B2b"] - flow_b2).abs() < 0.001,
        "Pipe-B2b should carry only {:.2} cfs, got {:.2} cfs",
        flow_b2,
        conduit_flows["Pipe-B2b"]
    );

    // Main branch conduits should carry combined flows from their sub-branches
    assert!(
        (conduit_flows["Conduit-A"] - flow_branch_a).abs() < 0.001,
        "Conduit-A should carry {:.2} cfs (combined from sub-branch A), got {:.2} cfs",
        flow_branch_a,
        conduit_flows["Conduit-A"]
    );

    assert!(
        (conduit_flows["Conduit-B"] - flow_branch_b).abs() < 0.001,
        "Conduit-B should carry {:.2} cfs (combined from sub-branch B), got {:.2} cfs",
        flow_branch_b,
        conduit_flows["Conduit-B"]
    );

    // Trunk should carry total combined flow from all branches
    assert!(
        (conduit_flows["Trunk"] - flow_total).abs() < 0.001,
        "Trunk should carry {:.2} cfs (total combined flow), got {:.2} cfs",
        flow_total,
        conduit_flows["Trunk"]
    );

    // Run hydraulic analysis
    let config = solver::SolverConfig::us_customary();
    let hgl_solver = solver::HglSolver::new(config);

    let analysis = hgl_solver
        .solve(&network, &conduit_flows, "multi-level-test".to_string())
        .expect("HGL solver should succeed");

    assert!(analysis.node_results.is_some(), "Should have node results");
    assert!(analysis.conduit_results.is_some(), "Should have conduit results");

    println!("\n=== TEST RESULTS ===");
    println!("✓ Multi-level tributary flow isolation verified:");
    println!("  Sub-branch A:");
    println!("    - Pipe-A1a: {:.2} cfs (Inlet-A1 tributary)", conduit_flows["Pipe-A1a"]);
    println!("    - Pipe-A2a: {:.2} cfs (Inlet-A2 tributary)", conduit_flows["Pipe-A2a"]);
    println!("    - Conduit-A: {:.2} cfs (combined sub-branch A)", conduit_flows["Conduit-A"]);
    println!("  Sub-branch B:");
    println!("    - Pipe-B1b: {:.2} cfs (Inlet-B1 tributary)", conduit_flows["Pipe-B1b"]);
    println!("    - Pipe-B2b: {:.2} cfs (Inlet-B2 tributary)", conduit_flows["Pipe-B2b"]);
    println!("    - Conduit-B: {:.2} cfs (combined sub-branch B)", conduit_flows["Conduit-B"]);
    println!("  Main trunk:");
    println!("    - Trunk: {:.2} cfs (total combined flow)", conduit_flows["Trunk"]);
    println!("\n✓ Flow accumulation verified at all levels:");
    println!("    {:.2} + {:.2} = {:.2} cfs (Branch A)", flow_a1, flow_a2, flow_branch_a);
    println!("    {:.2} + {:.2} = {:.2} cfs (Branch B)", flow_b1, flow_b2, flow_branch_b);
    println!("    {:.2} + {:.2} = {:.2} cfs (Total)", flow_branch_a, flow_branch_b, flow_total);
    println!("\n✓ TEST PASSED: Multi-level tributary flows correctly isolated and accumulated");
}
