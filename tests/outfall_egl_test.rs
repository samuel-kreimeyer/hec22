//! Test for outfall EGL calculation
//!
//! This test verifies that the Energy Grade Line (EGL) at the outfall is correctly
//! calculated as EGL = HGL + velocity_head, where velocity_head = V²/(2g) and V is
//! the velocity in the discharge conduit.

use hec22::*;

/// Test that outfall EGL includes velocity head from discharge conduit
///
/// Network topology:
///   Inlet → Conduit → Outfall
///
/// Expected behavior:
/// - HGL at outfall = tailwater elevation
/// - EGL at outfall = HGL + V²/(2g)
/// - V = velocity in discharge conduit
#[test]
fn test_outfall_egl_includes_velocity_head() {
    // Create a simple network: Inlet → Pipe → Outfall
    let project = project::Project {
        name: "Outfall EGL Test".to_string(),
        description: Some("Verify outfall EGL = HGL + velocity_head".to_string()),
        location: None,
        units: project::Units::us_customary(),
        author: Some("Test Suite".to_string()),
        created: Some(chrono::Utc::now().to_rfc3339()),
        modified: None,
    };

    // Create inlet
    let inlet = node::Node::new_inlet(
        "Inlet".to_string(),
        130.0, // invert elevation
        134.0, // rim elevation
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

    // Create outfall with fixed tailwater elevation
    let outfall = node::Node::new_outfall(
        "Outfall".to_string(),
        120.0, // invert elevation
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::FixedStage,
            tailwater_elevation: Some(123.0), // Fixed HGL = 123.0 ft
            tidal_curve: None,
        },
    );

    // Create discharge pipe
    let mut pipe = conduit::Conduit::new_pipe(
        "Discharge-Pipe".to_string(),
        "Inlet".to_string(),
        "Outfall".to_string(),
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
    pipe.upstream_invert = Some(130.0);
    pipe.downstream_invert = Some(120.0);

    // Build network
    let mut network = network::Network::new();
    network.add_node(inlet);
    network.add_node(outfall);
    network.add_conduit(pipe);

    network
        .validate_connectivity()
        .expect("Network should be valid");

    // Create drainage area with known flow
    // Q = 5.0 cfs (designed to produce measurable velocity)
    let drainage_area = drainage::DrainageArea {
        id: "DA-1".to_string(),
        name: Some("Test Area".to_string()),
        area: 1.25, // acres
        outlet: "Inlet".to_string(),
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

    // Compute flows: Q = C × i × A = 0.8 × 5.0 × 1.25 = 5.0 cfs
    let intensity = 5.0; // in/hr
    let node_inflows = solver::compute_rational_flows(&vec![drainage_area], intensity);
    let conduit_flows = solver::route_flows(&network, &node_inflows)
        .expect("Flow routing should succeed");

    let flow = conduit_flows.get("Discharge-Pipe").unwrap();
    println!("\n=== Flow Information ===");
    println!("  Discharge pipe flow: {:.2} cfs", flow);

    // Run hydraulic solver
    let config = solver::SolverConfig::us_customary();
    let hgl_solver = solver::HglSolver::new(config);

    let analysis = hgl_solver
        .solve(&network, &conduit_flows, "outfall-egl-test".to_string())
        .expect("HGL solver should succeed");

    // Get results
    let node_results = analysis.node_results.as_ref().expect("Should have node results");
    let conduit_results = analysis.conduit_results.as_ref().expect("Should have conduit results");

    // Find outfall results
    let outfall_result = node_results
        .iter()
        .find(|r| r.node_id == "Outfall")
        .expect("Should have outfall result");

    // Find discharge pipe results
    let pipe_result = conduit_results
        .iter()
        .find(|r| r.conduit_id == "Discharge-Pipe")
        .expect("Should have pipe result");

    println!("\n=== Outfall Results ===");
    println!("  HGL: {:.4} ft", outfall_result.hgl.unwrap());
    println!("  EGL: {:.4} ft", outfall_result.egl.unwrap());

    println!("\n=== Discharge Pipe Results ===");
    println!("  Flow: {:.2} cfs", pipe_result.flow.unwrap());
    println!("  Velocity: {:.2} ft/s", pipe_result.velocity.unwrap());
    println!("  Depth: {:.2} ft", pipe_result.depth.unwrap());

    // Calculate expected values
    let hgl_outfall = outfall_result.hgl.unwrap();
    let egl_outfall = outfall_result.egl.unwrap();
    let velocity = pipe_result.velocity.unwrap();

    // Verify HGL matches tailwater
    assert!(
        (hgl_outfall - 123.0).abs() < 0.01,
        "Outfall HGL should match tailwater elevation (123.0 ft), got {:.2} ft",
        hgl_outfall
    );

    // Calculate velocity head: V²/(2g)
    let gravity = 32.2; // ft/s²
    let velocity_head = (velocity * velocity) / (2.0 * gravity);

    println!("\n=== Velocity Head Calculation ===");
    println!("  Velocity: {:.4} ft/s", velocity);
    println!("  Velocity² / (2g): {:.4} ft", velocity_head);
    println!("  Expected EGL: HGL + V²/(2g) = {:.4} + {:.4} = {:.4} ft",
             hgl_outfall, velocity_head, hgl_outfall + velocity_head);
    println!("  Actual EGL: {:.4} ft", egl_outfall);
    println!("  Difference: {:.4} ft", (egl_outfall - (hgl_outfall + velocity_head)).abs());

    // *** CRITICAL ASSERTION ***
    // EGL at outfall should equal HGL + velocity_head
    let expected_egl = hgl_outfall + velocity_head;

    assert!(
        (egl_outfall - expected_egl).abs() < 0.01,
        "FAILED: Outfall EGL should be HGL + V²/(2g)\n\
         Expected: {:.4} ft (HGL {:.4} + velocity head {:.4})\n\
         Got: {:.4} ft\n\
         Difference: {:.4} ft\n\
         \n\
         The EGL at the outfall must include the velocity head from the discharge conduit.\n\
         Velocity in discharge pipe: {:.2} ft/s\n\
         Velocity head (V²/2g): {:.4} ft",
        expected_egl,
        hgl_outfall,
        velocity_head,
        egl_outfall,
        (egl_outfall - expected_egl).abs(),
        velocity,
        velocity_head
    );

    println!("\n✓ TEST PASSED: Outfall EGL = HGL + V²/(2g)");
    println!("  EGL correctly includes velocity head from discharge conduit");
}

/// Test outfall EGL with different flow rates
///
/// This test verifies that the velocity head changes appropriately with flow rate
#[test]
fn test_outfall_egl_with_varying_flows() {
    // Test with multiple flow rates to verify velocity head calculation
    let test_flows = vec![
        (2.0, "Low flow"),
        (5.0, "Medium flow"),
        (10.0, "High flow"),
    ];

    for (target_flow, description) in test_flows {
        println!("\n=== Testing {} ({:.1} cfs) ===", description, target_flow);

        // Create simple network
        let inlet = node::Node::new_inlet(
            "Inlet".to_string(),
            130.0,
            134.0,
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

        let outfall = node::Node::new_outfall(
            "Outfall".to_string(),
            120.0,
            node::OutfallProperties {
                boundary_condition: node::BoundaryCondition::FixedStage,
                tailwater_elevation: Some(123.0),
                tidal_curve: None,
            },
        );

        let mut pipe = conduit::Conduit::new_pipe(
            "Pipe".to_string(),
            "Inlet".to_string(),
            "Outfall".to_string(),
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
        pipe.upstream_invert = Some(130.0);
        pipe.downstream_invert = Some(120.0);

        let mut network = network::Network::new();
        network.add_node(inlet);
        network.add_node(outfall);
        network.add_conduit(pipe);

        // Create flows
        let mut flows = std::collections::HashMap::new();
        flows.insert("Pipe".to_string(), target_flow);

        // Solve
        let config = solver::SolverConfig::us_customary();
        let hgl_solver = solver::HglSolver::new(config);
        let analysis = hgl_solver
            .solve(&network, &flows, format!("flow-test-{}", target_flow))
            .expect("Solver should succeed");

        // Get results
        let node_results = analysis.node_results.as_ref().unwrap();
        let conduit_results = analysis.conduit_results.as_ref().unwrap();

        let outfall_result = node_results.iter().find(|r| r.node_id == "Outfall").unwrap();
        let pipe_result = conduit_results.iter().find(|r| r.conduit_id == "Pipe").unwrap();

        let hgl = outfall_result.hgl.unwrap();
        let egl = outfall_result.egl.unwrap();
        let velocity = pipe_result.velocity.unwrap();
        let velocity_head = (velocity * velocity) / (2.0 * 32.2);
        let expected_egl = hgl + velocity_head;

        println!("  Flow: {:.2} cfs", target_flow);
        println!("  Velocity: {:.2} ft/s", velocity);
        println!("  Velocity head: {:.4} ft", velocity_head);
        println!("  HGL: {:.2} ft", hgl);
        println!("  Expected EGL: {:.4} ft", expected_egl);
        println!("  Actual EGL: {:.4} ft", egl);

        assert!(
            (egl - expected_egl).abs() < 0.01,
            "{}: EGL should be HGL + V²/(2g). Expected {:.4}, got {:.4}",
            description,
            expected_egl,
            egl
        );
    }

    println!("\n✓ All flow rates verified: EGL = HGL + V²/(2g)");
}
