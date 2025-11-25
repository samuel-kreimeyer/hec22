//! Integration tests for end-to-end network analysis
//!
//! These tests verify that the complete workflow (network construction -> parser -> solver)
//! works correctly for common drainage network configurations.

use hec22::*;

/// Test a simple linear network: Inlet1 -> Inlet2 -> Outfall
///
/// This test verifies the complete workflow:
/// 1. Build a simple network programmatically
/// 2. Create drainage areas
/// 3. Compute rational method flows
/// 4. Route flows through the network
/// 5. Run the HGL/EGL solver
/// 6. Verify plausible outputs without errors or panics
#[test]
fn test_simple_linear_network() {
    // Create project metadata
    let project = project::Project {
        name: "Simple Linear Network Integration Test".to_string(),
        description: Some("Test network with 2 inlets and 1 outfall in series".to_string()),
        location: None,
        units: project::Units::us_customary(),
        author: Some("Integration Test".to_string()),
        created: Some(chrono::Utc::now().to_rfc3339()),
        modified: None,
    };

    // Create nodes: IN-001 (upstream) -> IN-002 (midpoint) -> OUT-001 (downstream)
    // Elevations decrease in flow direction
    let inlet1 = node::Node::new_inlet(
        "IN-001".to_string(),
        130.0, // invert elevation (highest upstream)
        134.0, // rim elevation
        node::InletProperties {
            inlet_type: node::InletType::Combination,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: Some(node::CurbOpeningProperties {
                length: Some(3.0),
                height: Some(0.5),
                throat_type: Some(node::ThroatType::Horizontal),
            }),
            local_depression: Some(2.0),
            clogging_factor: Some(0.15),
        },
    );

    let inlet2 = node::Node::new_inlet(
        "IN-002".to_string(),
        125.0, // invert elevation (middle)
        129.0, // rim elevation
        node::InletProperties {
            inlet_type: node::InletType::Combination,
            location: node::InletLocation::OnGrade,
            grate: Some(node::GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(node::BarConfiguration::Perpendicular),
            }),
            curb_opening: Some(node::CurbOpeningProperties {
                length: Some(3.0),
                height: Some(0.5),
                throat_type: Some(node::ThroatType::Horizontal),
            }),
            local_depression: Some(2.0),
            clogging_factor: Some(0.15),
        },
    );

    let outfall = node::Node::new_outfall(
        "OUT-001".to_string(),
        120.0, // invert elevation (lowest downstream)
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::NormalDepth,
            tailwater_elevation: Some(121.0),
            tidal_curve: None,
        },
    );

    // Create conduits connecting the nodes
    let mut pipe1 = conduit::Conduit::new_pipe(
        "P-001".to_string(),
        "IN-001".to_string(), // from upstream inlet
        "IN-002".to_string(), // to midpoint inlet
        200.0,                // length (ft)
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(18.0), // inches
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe1.upstream_invert = Some(130.0);
    pipe1.downstream_invert = Some(125.2);

    let mut pipe2 = conduit::Conduit::new_pipe(
        "P-002".to_string(),
        "IN-002".to_string(), // from midpoint inlet
        "OUT-001".to_string(), // to outfall
        250.0,                 // length (ft)
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(24.0), // inches (larger to accommodate combined flow)
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        },
    );
    pipe2.upstream_invert = Some(125.0);
    pipe2.downstream_invert = Some(120.3);

    // Build network
    let mut network = network::Network::new();
    network.add_node(inlet1);
    network.add_node(inlet2);
    network.add_node(outfall);
    network.add_conduit(pipe1);
    network.add_conduit(pipe2);

    // Validate network connectivity
    network
        .validate_connectivity()
        .expect("Network should be valid");

    // Verify network structure
    assert_eq!(network.node_count(), 3, "Should have 3 nodes");
    assert_eq!(network.conduit_count(), 2, "Should have 2 conduits");
    assert_eq!(network.inlets().len(), 2, "Should have 2 inlets");
    assert_eq!(network.outfalls().len(), 1, "Should have 1 outfall");

    // Create drainage areas for each inlet
    let drainage_area1 = drainage::DrainageArea {
        id: "DA-001".to_string(),
        name: Some("Upstream Contributing Area".to_string()),
        area: 1.5, // acres
        outlet: "IN-001".to_string(),
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

    let drainage_area2 = drainage::DrainageArea {
        id: "DA-002".to_string(),
        name: Some("Midpoint Contributing Area".to_string()),
        area: 1.0, // acres
        outlet: "IN-002".to_string(),
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

    let drainage_areas = vec![drainage_area1.clone(), drainage_area2.clone()];

    // Compute rational method flows
    let intensity = 4.0; // in/hr (design storm intensity)
    let node_inflows = solver::compute_rational_flows(&drainage_areas, intensity);

    // Verify we got inflows at both inlets
    assert!(
        node_inflows.contains_key("IN-001"),
        "Should have inflow at IN-001"
    );
    assert!(
        node_inflows.contains_key("IN-002"),
        "Should have inflow at IN-002"
    );

    // Verify rational method calculation: Q = C * i * A
    let q1 = node_inflows.get("IN-001").unwrap();
    let expected_q1 = 0.80 * 4.0 * 1.5; // C * i * A
    assert!(
        (q1 - expected_q1).abs() < 0.001,
        "IN-001 flow should match rational method: expected {}, got {}",
        expected_q1,
        q1
    );

    let q2 = node_inflows.get("IN-002").unwrap();
    let expected_q2 = 0.75 * 4.0 * 1.0;
    assert!(
        (q2 - expected_q2).abs() < 0.001,
        "IN-002 flow should match rational method: expected {}, got {}",
        expected_q2,
        q2
    );

    // Route flows through network
    let conduit_flows = solver::route_flows(&network, &node_inflows)
        .expect("Flow routing should succeed");

    // Verify conduit flows
    assert!(
        conduit_flows.contains_key("P-001"),
        "Should have flow in P-001"
    );
    assert!(
        conduit_flows.contains_key("P-002"),
        "Should have flow in P-002"
    );

    // P-001 should carry only flow from IN-001
    let p1_flow = conduit_flows.get("P-001").unwrap();
    assert!(
        (p1_flow - expected_q1).abs() < 0.001,
        "P-001 should carry flow from IN-001 only"
    );

    // P-002 should carry combined flow from both inlets
    let p2_flow = conduit_flows.get("P-002").unwrap();
    let expected_p2_flow = expected_q1 + expected_q2;
    assert!(
        (p2_flow - expected_p2_flow).abs() < 0.001,
        "P-002 should carry combined flow: expected {}, got {}",
        expected_p2_flow,
        p2_flow
    );

    // Run HGL/EGL solver
    let config = solver::SolverConfig::us_customary();
    let hgl_solver = solver::HglSolver::new(config);

    let analysis = hgl_solver
        .solve(&network, &conduit_flows, "integration-test".to_string())
        .expect("HGL solver should succeed");

    // Verify analysis results structure
    assert!(
        analysis.node_results.is_some(),
        "Should have node results"
    );
    assert!(
        analysis.conduit_results.is_some(),
        "Should have conduit results"
    );

    // Clone the results for verification before moving analysis
    let node_results = analysis.node_results.clone().unwrap();
    let conduit_results = analysis.conduit_results.clone().unwrap();

    assert_eq!(
        node_results.len(),
        3,
        "Should have results for all 3 nodes"
    );
    assert_eq!(
        conduit_results.len(),
        2,
        "Should have results for both conduits"
    );

    // Verify plausible node results
    for node_result in &node_results {
        // HGL should be defined
        assert!(
            node_result.hgl.is_some(),
            "Node {} should have HGL",
            node_result.node_id
        );
        let hgl = node_result.hgl.unwrap();

        // HGL should be greater than the corresponding invert elevation
        let node = network.find_node(&node_result.node_id).unwrap();
        assert!(
            hgl > node.invert_elevation,
            "Node {} HGL ({:.2}) should be above invert elevation ({:.2})",
            node_result.node_id,
            hgl,
            node.invert_elevation
        );

        // HGL should generally be below rim elevation (unless flooding)
        if let Some(flooding) = node_result.flooding {
            if !flooding {
                if let Some(rim) = node.rim_elevation {
                    assert!(
                        hgl <= rim,
                        "Node {} HGL ({:.2}) should be below rim ({:.2}) when not flooding",
                        node_result.node_id,
                        hgl,
                        rim
                    );
                }
            }
        }

        // Depth should be non-negative if present
        if let Some(depth) = node_result.depth {
            assert!(
                depth >= 0.0,
                "Node {} depth ({:.2}) should be non-negative",
                node_result.node_id,
                depth
            );
        }

        // EGL should be >= HGL (energy grade line includes velocity head)
        if let Some(egl) = node_result.egl {
            assert!(
                egl >= hgl,
                "Node {} EGL ({:.2}) should be >= HGL ({:.2})",
                node_result.node_id,
                egl,
                hgl
            );
        }
    }

    // Verify plausible conduit results
    for conduit_result in &conduit_results {
        // Flow should be positive
        if let Some(flow) = conduit_result.flow {
            assert!(
                flow > 0.0,
                "Conduit {} flow should be positive",
                conduit_result.conduit_id
            );
        }

        // Velocity should be positive and reasonable (typically 2-15 ft/s for storm drains)
        if let Some(velocity) = conduit_result.velocity {
            assert!(
                velocity > 0.0,
                "Conduit {} velocity should be positive",
                conduit_result.conduit_id
            );
            assert!(
                velocity < 30.0,
                "Conduit {} velocity ({:.2} ft/s) should be reasonable",
                conduit_result.conduit_id,
                velocity
            );
        }

        // Depth should be non-negative if present
        if let Some(depth) = conduit_result.depth {
            assert!(
                depth >= 0.0,
                "Conduit {} depth ({:.2}) should be non-negative",
                conduit_result.conduit_id,
                depth
            );
        }

        // Capacity used should be between 0 and 2 (100% = 1.0, allow for slight surcharge)
        if let Some(capacity) = conduit_result.capacity_used {
            assert!(
                capacity > 0.0 && capacity < 2.0,
                "Conduit {} capacity used ({:.2}) should be reasonable",
                conduit_result.conduit_id,
                capacity
            );
        }
    }

    // Verify HGL decreases in flow direction (upstream to downstream)
    let hgl_in001 = node_results
        .iter()
        .find(|r| r.node_id == "IN-001")
        .unwrap()
        .hgl
        .unwrap();
    let hgl_in002 = node_results
        .iter()
        .find(|r| r.node_id == "IN-002")
        .unwrap()
        .hgl
        .unwrap();
    let hgl_out001 = node_results
        .iter()
        .find(|r| r.node_id == "OUT-001")
        .unwrap()
        .hgl
        .unwrap();

    assert!(
        hgl_in001 > hgl_in002,
        "HGL should decrease from IN-001 ({:.2}) to IN-002 ({:.2})",
        hgl_in001,
        hgl_in002
    );
    assert!(
        hgl_in002 > hgl_out001,
        "HGL should decrease from IN-002 ({:.2}) to OUT-001 ({:.2})",
        hgl_in002,
        hgl_out001
    );

    // Build complete drainage network model
    let mut drainage_network = DrainageNetwork::new(project, network);
    drainage_network.drainage_areas = Some(drainage_areas);
    drainage_network.analysis = Some(analysis);

    // Verify JSON serialization works
    let json = drainage_network
        .to_json()
        .expect("Should serialize to JSON");
    assert!(json.len() > 0, "JSON should not be empty");

    // Verify JSON deserialization works (roundtrip)
    let reparsed: DrainageNetwork =
        serde_json::from_str(&json).expect("Should deserialize from JSON");
    assert_eq!(
        reparsed.network.nodes.len(),
        3,
        "Roundtrip should preserve node count"
    );
    assert_eq!(
        reparsed.network.conduits.len(),
        2,
        "Roundtrip should preserve conduit count"
    );

    println!("✓ Simple linear network integration test passed");
    println!("  Network: 2 inlets → 1 outfall");
    println!("  Node results: {} nodes analyzed", node_results.len());
    println!("  Conduit results: {} conduits analyzed", conduit_results.len());
    println!("  HGL gradient verified: {:.2} → {:.2} → {:.2} ft", hgl_in001, hgl_in002, hgl_out001);
}
