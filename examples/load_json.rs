//! Example: Loading and analyzing a drainage network from JSON
//!
//! This example demonstrates how to load a JSON network file,
//! parse it, and perform analysis on the data.

use hec22::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the simple network example
    let json_path = "schema/examples/simple-network.json";

    println!("Loading drainage network from: {}", json_path);

    let json_content = fs::read_to_string(json_path)?;
    let network: DrainageNetwork = serde_json::from_str(&json_content)?;

    println!("✓ Successfully loaded network: {}", network.project.name);
    println!("  Schema version: {}", network.version);
    println!("  Unit system: {:?}", network.project.units.system);

    // Display network statistics
    println!("\n--- Network Statistics ---");
    println!("Nodes: {}", network.network.nodes.len());
    println!("  Inlets: {}", network.network.inlets().len());
    println!("  Junctions: {}", network.network.junctions().len());
    println!("  Outfalls: {}", network.network.outfalls().len());
    println!("Conduits: {}", network.network.conduits.len());

    if let Some(ref areas) = network.drainage_areas {
        println!("Drainage Areas: {}", areas.len());
        let total_area: f64 = areas.iter().map(|a| a.area).sum();
        println!("  Total contributing area: {:.2} acres", total_area);
    }

    // Display node details
    println!("\n--- Node Details ---");
    for node in &network.network.nodes {
        println!(
            "{} ({:?}): Invert = {:.2} ft",
            node.id, node.node_type, node.invert_elevation
        );

        if let Some(rim) = node.rim_elevation {
            println!("  Rim elevation: {:.2} ft", rim);
        }

        if let Some(ref inlet_props) = node.inlet {
            println!("  Inlet type: {:?}", inlet_props.inlet_type);
            println!("  Location: {:?}", inlet_props.location);
        }

        if let Some(ref junction_props) = node.junction {
            if let Some(diameter) = junction_props.diameter {
                println!("  Manhole diameter: {:.1} ft", diameter);
            }
            if let Some(k) = junction_props.loss_coefficient {
                println!("  Loss coefficient: {:.2}", k);
            }
        }

        if let Some(ref outfall_props) = node.outfall {
            println!("  Boundary condition: {:?}", outfall_props.boundary_condition);
        }
    }

    // Display conduit details
    println!("\n--- Conduit Details ---");
    for conduit in &network.network.conduits {
        println!(
            "{} ({:?}): {} → {}",
            conduit.id, conduit.conduit_type, conduit.from_node, conduit.to_node
        );
        println!("  Length: {:.1} ft", conduit.length);

        if let Some(slope) = conduit.effective_slope() {
            println!("  Slope: {:.4} ft/ft", slope);
        }

        if let Some(ref pipe_props) = conduit.pipe {
            if let Some(diameter) = pipe_props.diameter {
                println!("  Diameter: {:.0} inches", diameter);
            }
            if let Some(material) = pipe_props.material {
                println!("  Material: {:?} (n = {:.3})", material, pipe_props.manning_n);
            }
        }

        if let Some(ref gutter_props) = conduit.gutter {
            println!("  Cross slope: {:.3}", gutter_props.cross_slope);
            println!("  Longitudinal slope: {:.3}", gutter_props.longitudinal_slope);
        }
    }

    // Display drainage area results
    if let Some(ref analysis) = network.analysis {
        println!("\n--- Analysis Results ---");
        println!("Method: {:?}", analysis.method);

        if let Some(ref storm_id) = analysis.design_storm_id {
            println!("Design storm: {}", storm_id);
        }

        if let Some(ref node_results) = analysis.node_results {
            println!("\nNode Results:");
            for result in node_results {
                print!("  {}: ", result.node_id);

                if let Some(hgl) = result.hgl {
                    print!("HGL = {:.2} ft, ", hgl);
                }
                if let Some(egl) = result.egl {
                    print!("EGL = {:.2} ft, ", egl);
                }
                if let Some(flooding) = result.flooding {
                    if flooding {
                        print!("⚠ FLOODING");
                    } else {
                        print!("✓ No flooding");
                    }
                }
                println!();
            }
        }

        if let Some(ref conduit_results) = analysis.conduit_results {
            println!("\nConduit Results:");
            for result in conduit_results {
                print!("  {}: ", result.conduit_id);

                if let Some(flow) = result.flow {
                    print!("Q = {:.2} cfs, ", flow);
                }
                if let Some(velocity) = result.velocity {
                    print!("V = {:.1} ft/s, ", velocity);
                }
                if let Some(capacity) = result.capacity_used {
                    print!("Capacity = {:.0}%", capacity * 100.0);
                }
                println!();
            }
        }

        if let Some(ref violations) = analysis.violations {
            if violations.is_empty() {
                println!("\n✓ No design violations found!");
            } else {
                println!("\n⚠ Design Violations ({}):", violations.len());
                for violation in violations {
                    println!(
                        "  [{:?}] {} - {}",
                        violation.severity, violation.element_id, violation.message
                    );
                }
            }
        }
    }

    // Validate network connectivity
    println!("\n--- Validation ---");
    match network.network.validate_connectivity() {
        Ok(_) => println!("✓ Network connectivity is valid"),
        Err(e) => println!("✗ Connectivity error: {}", e),
    }

    // Display design criteria
    if let Some(ref criteria) = network.design_criteria {
        println!("\n--- Design Criteria ---");

        if let Some(ref spread_criteria) = criteria.gutter_spread {
            if let Some(max) = spread_criteria.max_spread {
                println!("Maximum gutter spread: {:.1} ft", max);
            }
        }

        if let Some(ref hgl_criteria) = criteria.hgl_criteria {
            if let Some(clearance) = hgl_criteria.max_hgl_below_rim {
                println!("Minimum HGL clearance below rim: {:.1} ft", clearance);
            }
        }

        if let Some(ref velocity_criteria) = criteria.velocity {
            if let (Some(min), Some(max)) =
                (velocity_criteria.min_velocity, velocity_criteria.max_velocity)
            {
                println!("Velocity range: {:.1} - {:.1} ft/s", min, max);
            }
        }
    }

    Ok(())
}
