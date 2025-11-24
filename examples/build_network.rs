//! Example: Building a simple drainage network programmatically
//!
//! This example demonstrates how to create a drainage network using
//! the Rust type definitions, then serialize it to JSON.

use hec22::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create project metadata
    let project = project::Project {
        name: "Example Drainage System".to_string(),
        description: Some("Simple 3-node network for demonstration".to_string()),
        location: Some(project::Location {
            latitude: 38.8977,
            longitude: -77.0365,
            datum: Some("NAVD88".to_string()),
        }),
        units: project::Units::us_customary(),
        author: Some("Jane Engineer".to_string()),
        created: Some(chrono::Utc::now().to_rfc3339()),
        modified: None,
    };

    // 2. Create nodes
    let inlet = node::Node::new_inlet(
        "IN-001".to_string(),
        124.5,
        128.0,
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

    let junction = node::Node::new_junction(
        "MH-001".to_string(),
        118.5,
        125.0,
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
        115.0,
        node::OutfallProperties {
            boundary_condition: node::BoundaryCondition::NormalDepth,
            tailwater_elevation: Some(116.5),
            tidal_curve: None,
        },
    );

    // 3. Create conduits
    let pipe1 = conduit::Conduit::new_pipe(
        "P-101".to_string(),
        "IN-001".to_string(),
        "MH-001".to_string(),
        150.0,
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

    let mut pipe2 = conduit::Conduit::new_pipe(
        "P-102".to_string(),
        "MH-001".to_string(),
        "OUT-001".to_string(),
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

    // Set inverts and calculate slope
    pipe2.upstream_invert = Some(118.25);
    pipe2.downstream_invert = Some(115.15);
    let slope = pipe2.calculate_slope().unwrap();
    println!("Pipe P-102 slope: {:.4} ft/ft", slope);

    // 4. Build network
    let mut network = network::Network::new();
    network.add_node(inlet);
    network.add_node(junction);
    network.add_node(outfall);
    network.add_conduit(pipe1);
    network.add_conduit(pipe2);

    // Validate connectivity
    network.validate_connectivity()?;
    println!(
        "Network validated: {} nodes, {} conduits",
        network.node_count(),
        network.conduit_count()
    );

    // 5. Create drainage area
    let drainage_area = drainage::DrainageArea {
        id: "DA-001".to_string(),
        name: Some("Commercial Block A".to_string()),
        area: 1.25,
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

    // 6. Create design storm
    let storm = rainfall::DesignStorm::uniform(
        "storm-10yr".to_string(),
        "10-Year Design Storm".to_string(),
        10.0,
        3.8,
    );

    let rainfall_data = rainfall::Rainfall {
        design_storms: Some(vec![storm]),
        idf_curves: None,
    };

    // 7. Set design criteria
    let design_criteria = analysis::DesignCriteria {
        gutter_spread: Some(analysis::GutterSpreadCriteria {
            max_spread: Some(10.0),
            max_spread_local_street: None,
            max_spread_collector_street: None,
            max_spread_arterial_street: None,
        }),
        hgl_criteria: Some(analysis::HglCriteria {
            max_hgl_below_rim: Some(1.0),
            allow_surcharge: Some(false),
        }),
        velocity: Some(analysis::VelocityCriteria {
            min_velocity: Some(2.5),
            max_velocity: Some(15.0),
        }),
        cover: None,
        capacity: Some(analysis::CapacityCriteria {
            min_capacity_ratio: Some(1.0),
        }),
    };

    // 8. Create complete drainage network
    let mut drainage_network = DrainageNetwork::new(project, network);
    drainage_network.rainfall = Some(rainfall_data);
    drainage_network.drainage_areas = Some(vec![drainage_area]);
    drainage_network.design_criteria = Some(design_criteria);

    // 9. Serialize to JSON
    let json = drainage_network.to_json()?;
    println!("\nGenerated JSON network:");
    println!("{}", json);

    // 10. Demonstrate querying the network
    println!("\n--- Network Query Examples ---");

    // Find all inlets
    let inlets = drainage_network.nodes_by_type(node::NodeType::Inlet);
    println!("Found {} inlet(s)", inlets.len());

    // Find upstream conduits for junction
    let upstream = drainage_network.upstream_conduits("MH-001");
    println!("Junction MH-001 has {} upstream conduit(s)", upstream.len());

    // Calculate rational method runoff
    if let Some(areas) = &drainage_network.drainage_areas {
        for area in areas {
            if let Some(flow) = area.rational_method_runoff(3.8) {
                println!(
                    "Drainage area {} generates {:.2} cfs at i = 3.8 in/hr",
                    area.id, flow
                );
            }
        }
    }

    Ok(())
}
