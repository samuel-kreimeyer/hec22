//! Integration tests for JSON schema loading
//!
//! These tests verify that the example JSON files conform to the Rust type definitions
//! and can be successfully loaded and validated.

use hec22::*;

#[test]
fn test_load_simple_network() {
    let json = std::fs::read_to_string("schema/examples/simple-network.json")
        .expect("Failed to read simple-network.json");

    let network: DrainageNetwork =
        serde_json::from_str(&json).expect("Failed to parse JSON");

    // Verify basic structure
    assert_eq!(network.version, "1.0.0");
    assert_eq!(
        network.project.name,
        "Main Street Drainage - Station 10+00 to 13+50"
    );
    assert_eq!(network.project.units.system, project::UnitSystem::US);

    // Verify network topology
    assert_eq!(network.network.nodes.len(), 4);
    assert_eq!(network.network.conduits.len(), 4);

    // Verify node types
    let inlets = network.network.inlets();
    let junctions = network.network.junctions();
    let outfalls = network.network.outfalls();

    assert_eq!(inlets.len(), 2, "Should have 2 inlets");
    assert_eq!(junctions.len(), 1, "Should have 1 junction");
    assert_eq!(outfalls.len(), 1, "Should have 1 outfall");

    // Verify drainage areas
    assert!(network.drainage_areas.is_some());
    let areas = network.drainage_areas.as_ref().unwrap();
    assert_eq!(areas.len(), 2);

    // Verify rainfall data
    assert!(network.rainfall.is_some());
    let rainfall = network.rainfall.as_ref().unwrap();
    assert!(rainfall.design_storms.is_some());
    assert_eq!(rainfall.design_storms.as_ref().unwrap().len(), 1);

    // Verify analysis results
    assert!(network.analysis.is_some());
    let analysis = network.analysis.as_ref().unwrap();
    assert_eq!(
        analysis.method,
        Some(analysis::AnalysisMethod::Rational)
    );

    // Verify no violations in this example
    let violations = analysis.violations.as_ref().unwrap();
    assert_eq!(violations.len(), 0, "Simple network should have no violations");

    // Validate connectivity
    network
        .network
        .validate_connectivity()
        .expect("Network should be valid");
}

#[test]
fn test_load_network_with_violations() {
    let json = std::fs::read_to_string("schema/examples/network-with-violations.json")
        .expect("Failed to read network-with-violations.json");

    let network: DrainageNetwork =
        serde_json::from_str(&json).expect("Failed to parse JSON");

    // Verify basic structure
    assert_eq!(network.version, "1.0.0");
    assert!(network.project.name.contains("Deficiency"));

    // Verify analysis exists
    assert!(network.analysis.is_some());
    let analysis = network.analysis.as_ref().unwrap();

    // Verify violations are present
    assert!(analysis.violations.is_some());
    let violations = analysis.violations.as_ref().unwrap();
    assert!(violations.len() > 0, "Should have violations");

    // Check for specific violation types
    let hgl_violations: Vec<_> = violations
        .iter()
        .filter(|v| v.violation_type == analysis::ViolationType::Hgl)
        .collect();

    let spread_violations: Vec<_> = violations
        .iter()
        .filter(|v| v.violation_type == analysis::ViolationType::Spread)
        .collect();

    let flooding_violations: Vec<_> = violations
        .iter()
        .filter(|v| v.violation_type == analysis::ViolationType::Flooding)
        .collect();

    assert!(hgl_violations.len() > 0, "Should have HGL violations");
    assert!(spread_violations.len() > 0, "Should have spread violations");
    assert!(flooding_violations.len() > 0, "Should have flooding violations");

    // Verify severity levels
    let errors: Vec<_> = violations
        .iter()
        .filter(|v| v.severity == analysis::Severity::Error)
        .collect();

    assert!(errors.len() > 0, "Should have error-level violations");
}

#[test]
fn test_network_roundtrip() {
    // Load JSON
    let original_json = std::fs::read_to_string("schema/examples/simple-network.json")
        .expect("Failed to read file");

    let network: DrainageNetwork =
        serde_json::from_str(&original_json).expect("Failed to parse JSON");

    // Serialize back to JSON
    let serialized_json = network.to_json().expect("Failed to serialize");

    // Parse again
    let reparsed: DrainageNetwork =
        serde_json::from_str(&serialized_json).expect("Failed to reparse");

    // Verify key fields match
    assert_eq!(network.version, reparsed.version);
    assert_eq!(network.project.name, reparsed.project.name);
    assert_eq!(network.network.nodes.len(), reparsed.network.nodes.len());
    assert_eq!(
        network.network.conduits.len(),
        reparsed.network.conduits.len()
    );
}

#[test]
fn test_node_properties() {
    let json = std::fs::read_to_string("schema/examples/simple-network.json")
        .expect("Failed to read file");

    let network: DrainageNetwork = serde_json::from_str(&json).expect("Failed to parse");

    // Find a specific inlet
    let inlet = network
        .find_node("IN-101")
        .expect("Should find inlet IN-101");

    assert_eq!(inlet.node_type, node::NodeType::Inlet);
    assert!(inlet.inlet.is_some());

    let inlet_props = inlet.inlet.as_ref().unwrap();
    assert_eq!(inlet_props.inlet_type, node::InletType::Combination);
    assert_eq!(inlet_props.location, node::InletLocation::OnGrade);

    // Find a junction
    let junction = network
        .find_node("MH-201")
        .expect("Should find junction MH-201");

    assert_eq!(junction.node_type, node::NodeType::Junction);
    assert!(junction.junction.is_some());

    let junction_props = junction.junction.as_ref().unwrap();
    assert_eq!(junction_props.diameter, Some(4.0));
    assert_eq!(junction_props.benching, Some(true));

    // Find outfall
    let outfall = network
        .find_node("OUT-001")
        .expect("Should find outfall OUT-001");

    assert_eq!(outfall.node_type, node::NodeType::Outfall);
    assert!(outfall.outfall.is_some());

    let outfall_props = outfall.outfall.as_ref().unwrap();
    assert_eq!(
        outfall_props.boundary_condition,
        node::BoundaryCondition::NormalDepth
    );
}

#[test]
fn test_conduit_properties() {
    let json = std::fs::read_to_string("schema/examples/simple-network.json")
        .expect("Failed to read file");

    let network: DrainageNetwork = serde_json::from_str(&json).expect("Failed to parse");

    // Find a pipe
    let pipe = network
        .find_conduit("P-101")
        .expect("Should find pipe P-101");

    assert_eq!(pipe.conduit_type, conduit::ConduitType::Pipe);
    assert!(pipe.pipe.is_some());

    let pipe_props = pipe.pipe.as_ref().unwrap();
    assert_eq!(pipe_props.shape, conduit::PipeShape::Circular);
    assert_eq!(pipe_props.diameter, Some(18.0));
    assert_eq!(pipe_props.material, Some(conduit::PipeMaterial::RCP));
    assert_eq!(pipe_props.manning_n, 0.013);

    // Calculate slope
    let slope = pipe.calculate_slope();
    assert!(slope.is_some());
    let slope_value = slope.unwrap();
    assert!(slope_value > 0.0, "Slope should be positive");

    // Find a gutter
    let gutter = network
        .find_conduit("G-101")
        .expect("Should find gutter G-101");

    assert_eq!(gutter.conduit_type, conduit::ConduitType::Gutter);
    assert!(gutter.gutter.is_some());

    let gutter_props = gutter.gutter.as_ref().unwrap();
    assert_eq!(gutter_props.cross_slope, 0.02);
    assert_eq!(gutter_props.longitudinal_slope, 0.015);
}

#[test]
fn test_drainage_area_calculations() {
    let json = std::fs::read_to_string("schema/examples/simple-network.json")
        .expect("Failed to read file");

    let network: DrainageNetwork = serde_json::from_str(&json).expect("Failed to parse");

    let areas = network.drainage_areas.as_ref().expect("Should have drainage areas");

    for area in areas {
        // Verify required fields
        assert!(area.area > 0.0);
        assert!(!area.outlet.is_empty());

        // Test rational method calculation if coefficient is provided
        if let Some(c) = area.runoff_coefficient {
            let intensity = 3.8; // in/hr
            let flow = area.rational_method_runoff(intensity);
            assert!(flow.is_some());

            let q = flow.unwrap();
            // Q = C × i × A
            let expected = c * intensity * area.area;
            assert!((q - expected).abs() < 0.001);
        }

        // Test Tc calculation if breakdown is provided
        if area.tc_calculation.is_some() {
            let tc = area.calculate_total_tc();
            assert!(tc.is_some());
            assert!(tc.unwrap() > 0.0);
        }
    }
}

#[test]
fn test_analysis_violation_filtering() {
    let json = std::fs::read_to_string("schema/examples/network-with-violations.json")
        .expect("Failed to read file");

    let network: DrainageNetwork = serde_json::from_str(&json).expect("Failed to parse");

    let analysis = network.analysis.as_ref().expect("Should have analysis");

    // Test violation filtering methods
    let hgl_violations = analysis.get_violations_by_type(analysis::ViolationType::Hgl);
    let errors = analysis.get_errors();

    assert!(hgl_violations.len() > 0);
    assert!(errors.len() > 0);
    assert!(analysis.has_errors());

    // Verify error messages are present
    for violation in errors {
        assert!(!violation.message.is_empty());
        assert!(!violation.element_id.is_empty());
    }
}

#[test]
fn test_idf_curve_interpolation() {
    let json = std::fs::read_to_string("schema/examples/simple-network.json")
        .expect("Failed to read file");

    let network: DrainageNetwork = serde_json::from_str(&json).expect("Failed to parse");

    if let Some(ref rainfall) = network.rainfall {
        if let Some(ref idf_curves) = rainfall.idf_curves {
            for curve in idf_curves {
                // Test exact match
                for point in &curve.points {
                    let intensity = curve.get_intensity(point.duration);
                    assert!(intensity.is_some());
                    assert!((intensity.unwrap() - point.intensity).abs() < 0.001);
                }

                // Test interpolation between points
                if curve.points.len() >= 2 {
                    let d1 = curve.points[0].duration;
                    let d2 = curve.points[1].duration;
                    let mid_duration = (d1 + d2) / 2.0;

                    let intensity = curve.get_intensity(mid_duration);
                    assert!(intensity.is_some());

                    // Should be between the two bracketing intensities
                    let i1 = curve.points[0].intensity;
                    let i2 = curve.points[1].intensity;
                    let interp = intensity.unwrap();

                    assert!(interp >= i2.min(i1) && interp <= i1.max(i2));
                }
            }
        }
    }
}

#[test]
fn test_upstream_downstream_queries() {
    let json = std::fs::read_to_string("schema/examples/simple-network.json")
        .expect("Failed to read file");

    let network: DrainageNetwork = serde_json::from_str(&json).expect("Failed to parse");

    // Test upstream conduits
    let upstream = network.upstream_conduits("MH-201");
    assert!(upstream.len() > 0, "MH-201 should have upstream conduits");

    // Test downstream conduits
    let downstream = network.downstream_conduits("IN-101");
    assert!(downstream.len() > 0, "IN-101 should have downstream conduits");

    // Outfall should have no downstream conduits
    let outfall_downstream = network.downstream_conduits("OUT-001");
    assert_eq!(outfall_downstream.len(), 0, "Outfall should have no downstream conduits");
}
