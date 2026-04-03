//! Regression tests for HEC-22 Chapter 9 Tables 9.6 and 9.7 behavior.

use hec22::*;
use std::collections::HashMap;

fn build_simple_network(
    upstream_invert: f64,
    downstream_invert: f64,
    boundary_condition: node::BoundaryCondition,
    tailwater: f64,
    length: f64,
    entrance_loss: f64,
    exit_loss: f64,
) -> network::Network {
    let upstream = node::Node::new_junction(
        "Upstream".to_string(),
        upstream_invert,
        upstream_invert + 3.0,
        node::JunctionProperties {
            diameter: Some(4.0),
            sump_depth: None,
            loss_coefficient: None,
            benching: None,
            drop_structure: None,
        },
    );

    let outfall = node::Node::new_outfall(
        "Outfall".to_string(),
        downstream_invert,
        node::OutfallProperties {
            boundary_condition,
            tailwater_elevation: Some(tailwater),
            tidal_curve: None,
        },
    );

    let mut pipe = conduit::Conduit::new_pipe(
        "Pipe-1".to_string(),
        "Upstream".to_string(),
        "Outfall".to_string(),
        length,
        conduit::PipeProperties {
            shape: conduit::PipeShape::Circular,
            diameter: Some(18.0),
            width: None,
            height: None,
            material: Some(conduit::PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(entrance_loss),
            exit_loss: Some(exit_loss),
            bend_loss: Some(0.0),
        },
    );
    pipe.upstream_invert = Some(upstream_invert);
    pipe.downstream_invert = Some(downstream_invert);

    let mut network = network::Network::new();
    network.add_node(upstream);
    network.add_node(outfall);
    network.add_conduit(pipe);
    network.validate_connectivity().unwrap();

    network
}

#[test]
fn supercritical_condition_d_sets_hgl_to_normal_depth() {
    // Steep pipe with small flow to force supercritical shallow flow (Condition D).
    let slope = 0.05;
    let flow = 0.30; // cfs
    let upstream_invert = 5.0;
    let downstream_invert = 0.0;
    let diameter_ft = 18.0 / 12.0;
    let manning_n = 0.013;
    let gravity = 32.17;

    let network = build_simple_network(
        upstream_invert,
        downstream_invert,
        node::BoundaryCondition::FixedStage,
        0.0,
        100.0,
        0.0,
        0.0,
    );

    let mut flows = HashMap::new();
    flows.insert("Pipe-1".to_string(), flow);

    let config = solver::SolverConfig::us_customary();
    let hgl_solver = solver::HglSolver::new(config);
    let analysis = hgl_solver
        .solve(&network, &flows, &[], "table-9-cond-d".to_string())
        .expect("HGL solver should succeed");

    // Expected HGL at upstream end = invert + normal depth
    let mannings = hydraulics::ManningsEquation { k: 1.486 };
    let yn = mannings
        .normal_depth(flow, diameter_ft, slope, manning_n, gravity)
        .expect("normal depth should compute");

    let expected_hgl = upstream_invert + yn;

    let node_results = analysis.node_results.as_ref().unwrap();
    let upstream_result = node_results
        .iter()
        .find(|r| r.node_id == "Upstream")
        .expect("Upstream node result should exist");

    let hgl = upstream_result.hgl.unwrap();
    assert!(
        (hgl - expected_hgl).abs() < 1e-3,
        "Condition D should set HGL to invert + normal depth. Expected {:.4}, got {:.4}",
        expected_hgl,
        hgl
    );
}

#[test]
fn plunging_outlet_uses_case_e_downstream_seed() {
    // Tailwater below invert to trigger Table 9.6 Case E (plunging), then apply Table 9.7.
    let slope = 0.01;
    let flow = 1.0; // cfs
    let upstream_invert = 1.0;
    let downstream_invert = 0.0;
    let diameter_ft = 18.0 / 12.0;
    let manning_n = 0.013;
    let gravity = 32.17;

    let network = build_simple_network(
        upstream_invert,
        downstream_invert,
        node::BoundaryCondition::FixedStage,
        -1.0, // Stage below invert to force EGLa < BOCo
        100.0,
        0.0,
        0.0,
    );

    let mut flows = HashMap::new();
    flows.insert("Pipe-1".to_string(), flow);

    let config = solver::SolverConfig::us_customary();
    let manning_k = config.manning_k;
    let hgl_solver = solver::HglSolver::new(config);
    let analysis = hgl_solver
        .solve(&network, &flows, &[], "table-9-case-e".to_string())
        .expect("HGL solver should succeed");

    // Replicate Table 9.6/9.7 calculations for expected upstream HGL
    let mannings = hydraulics::ManningsEquation { k: 1.486 };
    let yn = mannings
        .normal_depth(flow, diameter_ft, slope, manning_n, gravity)
        .expect("normal depth should compute");
    let yc = mannings
        .critical_depth(flow, diameter_ft, gravity)
        .expect("critical depth should compute");
    let flow_result = mannings.partial_pipe_flow(diameter_ft, yn, slope, manning_n, gravity);

    let boc_o = downstream_invert;
    let downstream_hgl = -1.0; // fixed stage
    let downstream_egl = downstream_hgl + flow_result.velocity_head; // EGLa

    // Table 9.6 Case E expected EGL at downstream end inside pipe
    assert!(
        downstream_egl < boc_o,
        "EGLa should be below invert for case E"
    );
    let egl_o = boc_o + yn + flow_result.velocity_head;

    // Losses
    let energy_loss = hydraulics::EnergyLoss { gravity };

    let friction_loss = energy_loss.friction_loss(
        flow,
        100.0,
        flow_result.area,
        flow_result.hydraulic_radius,
        manning_n,
        manning_k,
    );
    let entrance_loss = energy_loss.entrance_loss(flow_result.velocity, 0.0);
    let exit_loss = energy_loss.exit_loss(flow_result.velocity, 0.0, 0.0);
    let total_loss = friction_loss + entrance_loss + exit_loss;

    // Provisional upstream HGL from energy equation
    let provisional_upstream_egl = egl_o + total_loss;
    let provisional_upstream_hgl = provisional_upstream_egl - flow_result.velocity_head;

    // Determine condition: expect downstream-controlled partial (Condition B/C) or surcharge if high
    let boc_i = upstream_invert;
    let expected_hgl = if provisional_upstream_hgl > boc_i + yc {
        provisional_upstream_hgl
    } else {
        boc_i + yn
    };

    let node_results = analysis.node_results.as_ref().unwrap();
    let upstream_result = node_results
        .iter()
        .find(|r| r.node_id == "Upstream")
        .expect("Upstream node result should exist");

    let hgl = upstream_result.hgl.unwrap();
    assert!(
        (hgl - expected_hgl).abs() < 1e-3,
        "Plunging outlet seed should follow Table 9.6 Case E. Expected HGL {:.4}, got {:.4}",
        expected_hgl,
        hgl
    );
}
