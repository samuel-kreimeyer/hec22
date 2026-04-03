//! Test to verify that conduits only carry tributary flows from upstream branches
//!
//! This test ensures that when calculating network hydraulics, flows used for conduits
//! are only those flows from upstream branches. Parallel branches should maintain
//! separate flows until they converge at a junction.

use hec22::*;

/// Test parallel branches with different flows converging at a junction
///
/// Network topology:
///   Branch A: Inlet-A (5 cfs) → Conduit-A ↘
///                                            Junction → Trunk → Outfall
///   Branch B: Inlet-B (3 cfs) → Conduit-B ↗
///
/// Expected behavior:
/// - Conduit-A should carry ONLY 5 cfs (from Inlet-A)
/// - Conduit-B should carry ONLY 3 cfs (from Inlet-B)
/// - Trunk conduit should carry 8 cfs (5 + 3, combined at junction)
/// - At outfall, total flow is 8 cfs
#[test]
fn test_tributary_flow_isolation_in_parallel_branches() {
    // Create project metadata
    let _project = project::Project {
        name: "Tributary Flow Isolation Test".to_string(),
        description: Some(
            "Verify that parallel branches maintain separate tributary flows".to_string(),
        ),
        location: None,
        units: project::Units::us_customary(),
        author: Some("Test Suite".to_string()),
        created: Some(chrono::Utc::now().to_rfc3339()),
        modified: None,
    };

    // Branch A: Single inlet with 5 cfs tributary flow
    let inlet_a = node::Node::new_inlet(
        "Inlet-A".to_string(),
        135.0, // invert elevation
        139.0, // rim elevation
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
            bypass_to: None,
        },
    );

    // Branch B: Single inlet with 3 cfs tributary flow
    let inlet_b = node::Node::new_inlet(
        "Inlet-B".to_string(),
        133.0, // invert elevation (slightly lower)
        137.0, // rim elevation
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
            bypass_to: None,
        },
    );

    // Junction where both branches converge
    let junction = node::Node::new_junction(
        "Junction".to_string(),
        128.0, // invert elevation
        135.0, // rim elevation
        node::JunctionProperties {
            diameter: Some(4.0),
            sump_depth: Some(1.0),
            loss_coefficient: Some(0.2),
            benching: Some(true),
            drop_structure: Some(false),
        },
    );

    // Outfall
    let outfall = node::Node::new_outfall(
        "Outfall".to_string(),
        122.0, // invert elevation
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::NormalDepth,
            tailwater_elevation: Some(123.5),
            tidal_curve: None,
        },
    );

    // Conduit A: Inlet-A → Junction
    let mut conduit_a = conduit::Conduit::new_pipe(
        "Conduit-A".to_string(),
        "Inlet-A".to_string(),
        "Junction".to_string(),
        200.0, // length (ft)
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(18.0), // 18" pipe
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    conduit_a.upstream_invert = Some(135.0);
    conduit_a.downstream_invert = Some(128.5);

    // Conduit B: Inlet-B → Junction
    let mut conduit_b = conduit::Conduit::new_pipe(
        "Conduit-B".to_string(),
        "Inlet-B".to_string(),
        "Junction".to_string(),
        220.0, // length (ft)
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(18.0), // 18" pipe
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    conduit_b.upstream_invert = Some(133.0);
    conduit_b.downstream_invert = Some(128.7);

    // Trunk conduit: Junction → Outfall
    let mut trunk = conduit::Conduit::new_pipe(
        "Trunk".to_string(),
        "Junction".to_string(),
        "Outfall".to_string(),
        300.0, // length (ft)
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(24.0), // 24" pipe (larger to handle combined flow)
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
    network.add_node(inlet_a);
    network.add_node(inlet_b);
    network.add_node(junction);
    network.add_node(outfall);
    network.add_conduit(conduit_a);
    network.add_conduit(conduit_b);
    network.add_conduit(trunk);

    // Validate network connectivity
    network
        .validate_connectivity()
        .expect("Network should be valid");

    // Create drainage areas with specific flows to match the user's example:
    // Inlet-A should have 5 cfs
    // Inlet-B should have 3 cfs
    //
    // Using rational method: Q = C × i × A
    // For 5 cfs: C=0.8, i=5.0 in/hr, A=1.25 acres → Q = 0.8 × 5.0 × 1.25 = 5.0 cfs
    // For 3 cfs: C=0.75, i=5.0 in/hr, A=0.8 acres → Q = 0.75 × 5.0 × 0.8 = 3.0 cfs
    let drainage_area_a = drainage::DrainageArea {
        id: "DA-A".to_string(),
        name: Some("Branch A Contributing Area".to_string()),
        area: 1.25, // acres
        outlet: "Inlet-A".to_string(),
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
    };

    let drainage_area_b = drainage::DrainageArea {
        id: "DA-B".to_string(),
        name: Some("Branch B Contributing Area".to_string()),
        area: 0.8, // acres
        outlet: "Inlet-B".to_string(),
        land_use: Some(drainage::LandUse {
            primary: Some(drainage::LandUseType::Commercial),
            impervious_percent: Some(75.0),
            composition: None,
        }),
        runoff_coefficient: Some(0.75),
        time_of_concentration: Some(12.0),
        tc_calculation: None,
        curve_number: None,
        geometry: None,
    };

    let drainage_areas = vec![drainage_area_a, drainage_area_b];

    // Compute rational method flows
    let intensity = 5.0; // in/hr (design storm intensity)
    let node_inflows = solver::compute_rational_flows(&drainage_areas, intensity);

    // Verify node inflows
    println!("\n=== Node Inflows ===");
    for (node_id, flow) in &node_inflows {
        println!("  {}: {:.2} cfs", node_id, flow);
    }

    assert!(
        node_inflows.contains_key("Inlet-A"),
        "Should have inflow at Inlet-A"
    );
    assert!(
        node_inflows.contains_key("Inlet-B"),
        "Should have inflow at Inlet-B"
    );

    // Expected flows from rational method
    let expected_flow_a = 0.80 * 5.0 * 1.25; // 5.0 cfs
    let expected_flow_b = 0.75 * 5.0 * 0.8; // 3.0 cfs
    let expected_total_flow = expected_flow_a + expected_flow_b; // 8.0 cfs

    let actual_flow_a = node_inflows.get("Inlet-A").unwrap();
    let actual_flow_b = node_inflows.get("Inlet-B").unwrap();

    assert!(
        (actual_flow_a - expected_flow_a).abs() < 0.001,
        "Inlet-A should have {:.2} cfs, got {:.2} cfs",
        expected_flow_a,
        actual_flow_a
    );

    assert!(
        (actual_flow_b - expected_flow_b).abs() < 0.001,
        "Inlet-B should have {:.2} cfs, got {:.2} cfs",
        expected_flow_b,
        actual_flow_b
    );

    // Route flows through network
    let conduit_flows =
        solver::route_flows(&network, &node_inflows).expect("Flow routing should succeed");

    // Verify conduit flows
    println!("\n=== Conduit Flows ===");
    for (conduit_id, flow) in &conduit_flows {
        println!("  {}: {:.2} cfs", conduit_id, flow);
    }

    assert!(
        conduit_flows.contains_key("Conduit-A"),
        "Should have flow in Conduit-A"
    );
    assert!(
        conduit_flows.contains_key("Conduit-B"),
        "Should have flow in Conduit-B"
    );
    assert!(
        conduit_flows.contains_key("Trunk"),
        "Should have flow in Trunk"
    );

    let flow_a = conduit_flows.get("Conduit-A").unwrap();
    let flow_b = conduit_flows.get("Conduit-B").unwrap();
    let flow_trunk = conduit_flows.get("Trunk").unwrap();

    // *** CRITICAL ASSERTIONS: Verify tributary flow isolation ***

    // Conduit-A should carry ONLY its tributary flow (5 cfs), not combined flow
    assert!(
        (flow_a - expected_flow_a).abs() < 0.001,
        "FAILED: Conduit-A should carry ONLY its tributary flow of {:.2} cfs, but got {:.2} cfs",
        expected_flow_a,
        flow_a
    );

    // Conduit-B should carry ONLY its tributary flow (3 cfs), not combined flow
    assert!(
        (flow_b - expected_flow_b).abs() < 0.001,
        "FAILED: Conduit-B should carry ONLY its tributary flow of {:.2} cfs, but got {:.2} cfs",
        expected_flow_b,
        flow_b
    );

    // Trunk should carry the COMBINED flow (8 cfs) from both branches
    assert!(
        (flow_trunk - expected_total_flow).abs() < 0.001,
        "FAILED: Trunk should carry combined flow of {:.2} cfs ({}+{}), but got {:.2} cfs",
        expected_total_flow,
        expected_flow_a,
        expected_flow_b,
        flow_trunk
    );

    // Run HGL/EGL solver to verify hydraulic calculations
    let config = solver::SolverConfig::us_customary();
    let hgl_solver = solver::HglSolver::new(config);

    let analysis = hgl_solver
        .solve(
            &network,
            &conduit_flows,
            &[],
            "tributary-flow-test".to_string(),
        )
        .expect("HGL solver should succeed");

    // Verify analysis results
    assert!(analysis.node_results.is_some(), "Should have node results");
    assert!(
        analysis.conduit_results.is_some(),
        "Should have conduit results"
    );

    let node_results = analysis.node_results.as_ref().unwrap();
    let conduit_results = analysis.conduit_results.as_ref().unwrap();

    // Verify all nodes have HGL values
    for node_result in node_results {
        assert!(
            node_result.hgl.is_some(),
            "Node {} should have HGL",
            node_result.node_id
        );

        let hgl = node_result.hgl.unwrap();
        let node = network.find_node(&node_result.node_id).unwrap();

        assert!(
            hgl > node.invert_elevation,
            "Node {} HGL ({:.2}) should be above invert elevation ({:.2})",
            node_result.node_id,
            hgl,
            node.invert_elevation
        );
    }

    // Verify conduit hydraulics
    for conduit_result in conduit_results {
        if let Some(flow) = conduit_result.flow {
            assert!(
                flow > 0.0,
                "Conduit {} should have positive flow",
                conduit_result.conduit_id
            );
        }

        if let Some(velocity) = conduit_result.velocity {
            assert!(
                velocity > 0.0 && velocity < 30.0,
                "Conduit {} velocity ({:.2} ft/s) should be reasonable",
                conduit_result.conduit_id,
                velocity
            );
        }
    }

    println!("\n=== TEST RESULTS ===");
    println!("✓ Tributary flow isolation verified:");
    println!(
        "  - Conduit-A carries only {:.2} cfs (Branch A tributary)",
        flow_a
    );
    println!(
        "  - Conduit-B carries only {:.2} cfs (Branch B tributary)",
        flow_b
    );
    println!(
        "  - Trunk carries combined {:.2} cfs (total at junction)",
        flow_trunk
    );
    println!(
        "  - Flows combine correctly: {:.2} + {:.2} = {:.2} cfs",
        flow_a, flow_b, flow_trunk
    );
    println!("\n✓ Hydraulic calculations completed successfully");
    println!("  - All nodes have valid HGL values");
    println!("  - All conduits have reasonable velocities");
    println!("\n✓ TEST PASSED: Conduits only carry flows from their upstream branches");
}

#[test]
fn test_inlet_routing_preserves_upstream_pipe_flow() {
    let inlet_a = node::Node::new_inlet(
        "IN-A".to_string(),
        130.0,
        134.0,
        node::InletProperties {
            inlet_type: node::InletType::Grate,
            location: node::InletLocation::Sag,
            grate: Some(node::GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.10),
            bypass_to: None,
        },
    );

    let inlet_b = node::Node::new_inlet(
        "IN-B".to_string(),
        125.0,
        129.0,
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
            bypass_to: None,
        },
    );

    let outfall = node::Node::new_outfall(
        "OUT".to_string(),
        120.0,
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::NormalDepth,
            tailwater_elevation: Some(121.0),
            tidal_curve: None,
        },
    );

    let mut pipe_1 = conduit::Conduit::new_pipe(
        "P-1".to_string(),
        "IN-A".to_string(),
        "IN-B".to_string(),
        200.0,
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
    pipe_1.upstream_invert = Some(130.0);
    pipe_1.downstream_invert = Some(125.1);

    let mut pipe_2 = conduit::Conduit::new_pipe(
        "P-2".to_string(),
        "IN-B".to_string(),
        "OUT".to_string(),
        240.0,
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
    pipe_2.upstream_invert = Some(125.0);
    pipe_2.downstream_invert = Some(120.2);

    let mut network = network::Network::new();
    network.add_node(inlet_a);
    network.add_node(inlet_b);
    network.add_node(outfall);
    network.add_conduit(pipe_1);
    network.add_conduit(pipe_2);
    network
        .validate_connectivity()
        .expect("Network should be valid");

    let drainage_area = drainage::DrainageArea {
        id: "DA-A".to_string(),
        name: None,
        area: 1.0,
        outlet: "IN-A".to_string(),
        land_use: None,
        runoff_coefficient: Some(1.0),
        time_of_concentration: Some(10.0),
        tc_calculation: None,
        curve_number: None,
        geometry: None,
    };

    let node_inflows = solver::compute_rational_flows(&[drainage_area], 1.0);
    let (conduit_flows, _inlet_results) = solver::route_flows_with_inlets(
        &network,
        &node_inflows,
        project::UnitSystem::US,
        &std::collections::HashMap::new(),
    )
    .expect("Flow routing should succeed");

    let flow_p1 = conduit_flows.get("P-1").cloned().unwrap_or(0.0);
    let flow_p2 = conduit_flows.get("P-2").cloned().unwrap_or(0.0);

    assert!(
        (flow_p1 - 1.0).abs() < 0.001,
        "P-1 should carry the inlet A flow, expected 1.0 cfs, got {:.3}",
        flow_p1
    );
    assert!(
        (flow_p2 - flow_p1).abs() < 0.001,
        "Downstream inlet should not reduce upstream pipe flow, expected {:.3} cfs, got {:.3}",
        flow_p1,
        flow_p2
    );
}
