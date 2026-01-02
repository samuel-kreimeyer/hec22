//! HEC-22 Urban Drainage Analysis CLI
//!
//! Command-line tool for hydraulic analysis of storm sewer networks using the
//! FHWA HEC-22 methodology.

use clap::{Parser, ValueEnum};
use hec22::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "hec22")]
#[command(version = "0.1.0")]
#[command(about = "HEC-22 Urban Drainage Analysis Tool", long_about = None)]
struct Cli {
    /// Path to nodes CSV file (required: id, type, invert_elev, rim_elev)
    #[arg(short, long, value_name = "FILE")]
    nodes: PathBuf,

    /// Path to conduits CSV file (required: id, from_node, to_node, type, diameter, length)
    #[arg(short, long, value_name = "FILE")]
    conduits: PathBuf,

    /// Path to drainage areas CSV file (required: id, area, runoff_coef, time_of_conc, outlet_node)
    #[arg(short = 'a', long, value_name = "FILE")]
    drainage_areas: Option<PathBuf>,

    /// Path to IDF curves CSV file (columns: return_period, duration, intensity)
    #[arg(long, value_name = "FILE")]
    idf_curves: Option<PathBuf>,

    /// Path to gutter parameters CSV file (columns: node_id, cross_slope, long_slope, ...)
    #[arg(long, value_name = "FILE")]
    gutter_params: Option<PathBuf>,

    /// Return period in years (used with IDF curves, default: 10)
    #[arg(short = 'r', long, default_value = "10")]
    return_period: f64,

    /// Rainfall intensity (in/hr for US units, mm/hr for SI units).
    /// If IDF curves are provided, this is used as fallback when time of concentration lookup fails.
    #[arg(short, long, default_value = "4.0")]
    intensity: f64,

    /// Maximum allowable gutter spread (ft for US units, m for SI units)
    #[arg(long)]
    max_spread: Option<f64>,

    /// Unit system to use for analysis
    #[arg(short, long, value_enum, default_value = "us")]
    units: UnitSystemArg,

    /// Output file path (default: stdout)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Output format
    #[arg(short = 'f', long, value_enum, default_value = "text")]
    format: OutputFormat,

    /// Export network plan view as SVG
    #[arg(long, value_name = "FILE")]
    export_network_plan: Option<PathBuf>,

    /// Export profile view as SVG
    #[arg(long, value_name = "FILE")]
    export_profile: Option<PathBuf>,

    /// Export interactive HTML viewer
    #[arg(long, value_name = "FILE")]
    export_html: Option<PathBuf>,

    /// Node path for profile view (comma-separated node IDs)
    /// Example: "IN-001,MH-001,OUT-001"
    #[arg(long, value_name = "PATH")]
    profile_path: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
enum UnitSystemArg {
    /// US Customary units (ft, cfs, in/hr)
    Us,
    /// SI Metric units (m, m³/s, mm/hr)
    Si,
}

#[derive(Debug, Clone, ValueEnum)]
enum OutputFormat {
    /// Human-readable text report
    Text,
    /// JSON output
    Json,
    /// CSV tables
    Csv,
}

fn main() {
    let cli = Cli::parse();

    // Run the analysis and handle errors
    if let Err(e) = run_analysis(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run_analysis(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    // Parse input files
    println!("Loading network data...");

    let nodes = csv::parse_nodes_csv(&cli.nodes)
        .map_err(|e| format!("Failed to parse nodes file: {}", e))?;
    println!("  Loaded {} nodes", nodes.len());

    let conduits = csv::parse_conduits_csv(&cli.conduits)
        .map_err(|e| format!("Failed to parse conduits file: {}", e))?;
    println!("  Loaded {} conduits", conduits.len());

    let drainage_areas = if let Some(ref path) = cli.drainage_areas {
        let areas = csv::parse_drainage_areas_csv(path)
            .map_err(|e| format!("Failed to parse drainage areas file: {}", e))?;
        println!("  Loaded {} drainage areas", areas.len());
        Some(areas)
    } else {
        println!("  No drainage areas provided");
        None
    };

    let mut gutter_params = HashMap::new();
    if let Some(ref path) = cli.gutter_params {
        let params = csv::parse_gutter_parameters_csv(path)
            .map_err(|e| format!("Failed to parse gutter parameters file: {}", e))?;
        println!("  Loaded {} gutter parameter rows", params.len());
        for record in params {
            gutter_params.insert(
                record.node_id.clone(),
                solver::InletGutterParameters {
                    cross_slope: Some(record.cross_slope),
                    longitudinal_slope: Some(record.long_slope),
                    manning_n: record.manning_n,
                    gutter_width: record.gutter_width,
                    depression: record.depression,
                    depression_width: record.depression_width,
                },
            );
        }
    }

    // Build network
    println!("\nBuilding network...");
    let mut network = network::Network::new();

    // Add nodes first
    for node in nodes {
        network.add_node(node);
    }

    // Add conduits and set invert elevations from nodes
    for mut conduit in conduits {
        // Set upstream and downstream inverts from node elevations if not already set
        if conduit.upstream_invert.is_none() {
            if let Some(from_node) = network.find_node(&conduit.from_node) {
                conduit.upstream_invert = Some(from_node.invert_elevation);
            }
        }
        if conduit.downstream_invert.is_none() {
            if let Some(to_node) = network.find_node(&conduit.to_node) {
                conduit.downstream_invert = Some(to_node.invert_elevation);
            }
        }
        network.add_conduit(conduit);
    }

    // Validate network
    network.validate_connectivity()
        .map_err(|e| format!("Network validation failed: {}", e))?;

    println!("  {} nodes, {} conduits", network.node_count(), network.conduit_count());
    println!("  {} inlets, {} junctions, {} outfalls",
             network.inlets().len(),
             network.junctions().len(),
             network.outfalls().len());

    // Load IDF curves if provided
    let idf_curve = if let Some(ref idf_path) = cli.idf_curves {
        println!("\nLoading IDF curves...");
        let curves = csv::parse_idf_curves_csv(idf_path)
            .map_err(|e| format!("Failed to parse IDF curves file: {}", e))?;

        // Find curve for the requested return period
        let curve = curves.iter()
            .find(|c| (c.return_period - cli.return_period).abs() < 0.1)
            .ok_or_else(|| format!("No IDF curve found for return period {} years", cli.return_period))?;

        println!("  Using {}-year IDF curve with {} duration points",
                 curve.return_period, curve.points.len());
        Some(curve.clone())
    } else {
        None
    };

    // Compute flows from drainage areas
    let node_inflows = if let Some(ref areas) = drainage_areas {
        println!("\nComputing rational method flows...");

        let mut flows = HashMap::new();

        for area in areas {
            // Determine intensity for this drainage area
            let intensity = if let Some(ref curve) = idf_curve {
                // Use time of concentration to look up intensity from IDF curve
                if let Some(tc) = area.time_of_concentration {
                    match curve.get_intensity(tc) {
                        Some(i) => {
                            println!("  Area {}: Tc={:.1} min, i={:.2} in/hr (from IDF curve)",
                                     area.id, tc, i);
                            i
                        }
                        None => {
                            println!("  Area {}: Warning - could not interpolate intensity for Tc={:.1} min, using fallback",
                                     area.id, tc);
                            cli.intensity
                        }
                    }
                } else {
                    println!("  Area {}: Warning - no time of concentration, using fallback intensity",
                             area.id);
                    cli.intensity
                }
            } else {
                // No IDF curve provided, use fixed intensity
                cli.intensity
            };

            // Compute rational method flow: Q = C * i * A
            let c = area.runoff_coefficient.unwrap_or(0.5);
            let flow = c * intensity * area.area;

            println!("  Node {}: Q = {:.2} × {:.2} × {:.2} = {:.2} cfs",
                     area.outlet, c, intensity, area.area, flow);

            *flows.entry(area.outlet.clone()).or_insert(0.0) += flow;
        }

        if idf_curve.is_none() {
            println!("  (Using fixed intensity: {} {})",
                     cli.intensity,
                     if matches!(cli.units, UnitSystemArg::Us) { "in/hr" } else { "mm/hr" });
        }

        flows
    } else {
        // No drainage areas provided, create zero flows for all nodes
        println!("\nNo drainage areas provided, using zero inflows");
        HashMap::new()
    };

    // Route flows through network
    println!("\nRouting flows through network...");
    let unit_system = match cli.units {
        UnitSystemArg::Us => project::UnitSystem::US,
        UnitSystemArg::Si => project::UnitSystem::SI,
    };
    let (conduit_flows, inlet_results) =
        solver::route_flows_with_inlets(&network, &node_inflows, unit_system, &gutter_params)
        .map_err(|e| format!("Flow routing failed: {}", e))?;

    for (conduit_id, flow) in &conduit_flows {
        println!("  Conduit {}: {:.2} cfs", conduit_id, flow);
    }

    // Run HGL/EGL solver
    println!("\nSolving for hydraulic grade line...");
    let config = match cli.units {
        UnitSystemArg::Us => solver::SolverConfig::us_customary(),
        UnitSystemArg::Si => solver::SolverConfig::si_metric(),
    };

    let hgl_solver = solver::HglSolver::new(config);
    let mut analysis =
        hgl_solver.solve(&network, &conduit_flows, &inlet_results, "Design Storm".to_string())
        .map_err(|e| format!("HGL solver failed: {}", e))?;
    apply_spread_criteria(&mut analysis, cli.max_spread);

    // Generate output
    println!("\n{}", "=".repeat(80));
    println!("HYDRAULIC ANALYSIS RESULTS");
    println!("{}\n", "=".repeat(80));

    match cli.format {
        OutputFormat::Text => {
            let report = format_text_report(&network, &analysis, &cli.units, &inlet_results);
            if let Some(ref output_path) = cli.output {
                std::fs::write(output_path, &report)?;
                println!("Results written to file");
            } else {
                println!("{}", report);
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&analysis)?;
            if let Some(ref output_path) = cli.output {
                std::fs::write(output_path, &json)?;
                println!("Results written to file");
            } else {
                println!("{}", json);
            }
        }
        OutputFormat::Csv => {
            if let Some(ref output_path) = cli.output {
                write_csv_output(&analysis, output_path)?;
                println!("Results written to CSV files");
            } else {
                println!("CSV output requires --output flag to specify base filename");
            }
        }
    }

    // Print summary
    if let Some(violations) = &analysis.violations {
        if !violations.is_empty() {
            println!("\n{}", "=".repeat(80));
            println!("DESIGN VIOLATIONS");
            println!("{}\n", "=".repeat(80));
            for violation in violations {
                println!("{}", format_violation(violation));
            }
        } else {
            println!("\n✓ No design violations found");
        }
    }

    // Export visualizations if requested
    export_visualizations(&cli, &network, &analysis)?;

    Ok(())
}

fn format_text_report(
    network: &network::Network,
    analysis: &analysis::Analysis,
    units: &UnitSystemArg,
    inlet_results: &[solver::InletInterception],
) -> String {
    let mut report = String::new();
    let unit_suffix = if matches!(units, UnitSystemArg::Us) { "ft" } else { "m" };

    // Node Results
    report.push_str("NODE RESULTS\n");
    report.push_str(&format!("{:-<100}\n", ""));
    report.push_str(&format!(
        "{:<16} {:<10} {:<10} {:<10} {:<10} {:<12} {:<10}\n",
        "Node ID",
        format!("HGL ({})", unit_suffix),
        format!("EGL ({})", unit_suffix),
        format!("Depth ({})", unit_suffix),
        "Velocity",
        format!("Spread ({})", unit_suffix),
        "Flooding"
    ));
    report.push_str(&format!("{:-<100}\n", ""));

    if let Some(ref node_results) = analysis.node_results {
        for result in node_results {
            let spread_str = if let Some(spread) = result.gutter_spread {
                format!("{:>10.2}", spread)
            } else {
                format!("{:>10}", "-")
            };

            report.push_str(&format!(
                "{:<16} {:>10.2} {:>10.2} {:>10.2} {:>10} {:>12} {:>10}\n",
                result.node_id,
                result.hgl.unwrap_or(0.0),
                result.egl.unwrap_or(0.0),
                result.depth.unwrap_or(0.0),
                if let Some(v) = result.velocity { format!("{:.2}", v) } else { "-".to_string() },
                spread_str,
                if result.flooding.unwrap_or(false) { "YES" } else { "No" }
            ));
        }
    }

    report.push('\n');

    // Inlet Analysis
    if !inlet_results.is_empty() {
        report.push_str("INLET ANALYSIS\n");
        report.push_str(&format!("{:-<140}\n", ""));
        report.push_str(&format!(
            "{:<16} {:<10} {:<20} {:<12} {:<12} {:<10} {:<12} {:<16} {:<16}\n",
            "Inlet ID",
            format!("Area (ac)"),
            "Inlet Dimensions",
            "Approach (cfs)",
            "Captured (cfs)",
            "Efficiency",
            "Bypass (cfs)",
            "Spread (ft)",
            "Bypass To"
        ));
        report.push_str(&format!("{:-<140}\n", ""));

        for inlet in inlet_results {
            // Get the node to extract inlet dimensions
            let node = network.nodes.iter().find(|n| n.id == inlet.node_id);

            // Format inlet dimensions
            let dimensions = if let Some(n) = node {
                if let Some(ref inlet_props) = n.inlet {
                    let mut dim_parts = Vec::new();

                    // Check for grate
                    if let Some(ref grate) = inlet_props.grate {
                        if let (Some(l), Some(w)) = (grate.length, grate.width) {
                            dim_parts.push(format!("Grate {}×{}'", l, w));
                        }
                    }

                    // Check for curb opening
                    if let Some(ref curb) = inlet_props.curb_opening {
                        if let Some(l) = curb.length {
                            dim_parts.push(format!("Curb {}'", l));
                        }
                    }

                    if dim_parts.is_empty() {
                        "-".to_string()
                    } else {
                        dim_parts.join(", ")
                    }
                } else {
                    "-".to_string()
                }
            } else {
                "-".to_string()
            };

            report.push_str(&format!(
                "{:<16} {:>10.2} {:<20} {:>12.2} {:>12.2} {:>9.1}% {:>12.2} {:>16.2} {:<16}\n",
                inlet.node_id,
                inlet.drainage_area.unwrap_or(0.0),
                dimensions,
                inlet.approach_flow,
                inlet.intercepted_flow,
                inlet.efficiency * 100.0,
                inlet.bypass_flow,
                inlet.spread,
                inlet.bypass_to_node.as_deref().unwrap_or("-")
            ));
        }
        report.push('\n');
    }

    // Conduit Results
    report.push_str("CONDUIT RESULTS\n");
    report.push_str(&format!("{:-<100}\n", ""));
    report.push_str(&format!(
        "{:<12} {:<10} {:<10} {:<10} {:<12} {:<10} {:<10}\n",
        "Conduit ID",
        "Flow (cfs)",
        "Velocity",
        format!("Depth ({})", unit_suffix),
        "Capacity %",
        "Froude #",
        "Regime"
    ));
    report.push_str(&format!("{:-<100}\n", ""));

    if let Some(ref conduit_results) = analysis.conduit_results {
        for result in conduit_results {
            let regime = if let Some(froude) = result.froude_number {
                if froude < 1.0 {
                    "Subcritical"
                } else if froude > 1.0 {
                    "Supercritical"
                } else {
                    "Critical"
                }
            } else {
                "N/A"
            };

            report.push_str(&format!(
                "{:<12} {:>10.2} {:>10.2} {:>10.2} {:>11.1}% {:>10.2} {:<10}\n",
                result.conduit_id,
                result.flow.unwrap_or(0.0),
                result.velocity.unwrap_or(0.0),
                result.depth.unwrap_or(0.0),
                result.capacity_used.unwrap_or(0.0) * 100.0,
                result.froude_number.unwrap_or(0.0),
                regime
            ));
        }
    }

    report
}

fn format_violation(violation: &analysis::Violation) -> String {
    format!(
        "[{}] {} at {}: {}",
        match violation.severity {
            analysis::Severity::Warning => "WARNING",
            analysis::Severity::Error => "ERROR",
            analysis::Severity::Info => "INFO",
        },
        match violation.violation_type {
            analysis::ViolationType::Spread => "Spread violation",
            analysis::ViolationType::Hgl => "HGL violation",
            analysis::ViolationType::Velocity => "Velocity violation",
            analysis::ViolationType::Cover => "Cover violation",
            analysis::ViolationType::Capacity => "Capacity violation",
            analysis::ViolationType::Flooding => "Flooding",
        },
        violation.element_id,
        violation.message
    )
}

fn apply_spread_criteria(analysis: &mut analysis::Analysis, max_spread: Option<f64>) {
    let max_spread = match max_spread {
        Some(value) if value > 0.0 => value,
        _ => return,
    };

    let node_results = match analysis.node_results.as_ref() {
        Some(results) => results,
        None => return,
    };

    let mut violations = Vec::new();
    for result in node_results {
        let spread = match result.gutter_spread {
            Some(value) => value,
            None => continue,
        };
        if spread > max_spread {
            violations.push(analysis::Violation::spread_violation(
                result.node_id.clone(),
                spread,
                max_spread,
                analysis::Severity::Warning,
            ));
        }
    }

    for violation in violations {
        analysis.add_violation(violation);
    }
}

fn write_csv_output(
    analysis: &analysis::Analysis,
    base_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;

    // Write node results
    let node_path = base_path.with_extension("nodes.csv");
    let mut node_file = File::create(node_path)?;
    writeln!(
        node_file,
        "node_id,hgl,egl,depth,velocity,gutter_spread,flooding"
    )?;

    if let Some(ref node_results) = analysis.node_results {
        for result in node_results {
            let spread = result
                .gutter_spread
                .map(|value| format!("{:.2}", value))
                .unwrap_or_else(|| "".to_string());
            writeln!(
                node_file,
                "{},{:.2},{:.2},{:.2},{:.2},{},{}",
                result.node_id,
                result.hgl.unwrap_or(0.0),
                result.egl.unwrap_or(0.0),
                result.depth.unwrap_or(0.0),
                result.velocity.unwrap_or(0.0),
                spread,
                result.flooding.unwrap_or(false)
            )?;
        }
    }

    // Write conduit results
    let conduit_path = base_path.with_extension("conduits.csv");
    let mut conduit_file = File::create(conduit_path)?;
    writeln!(conduit_file, "conduit_id,flow,velocity,depth,capacity_used,froude_number")?;

    if let Some(ref conduit_results) = analysis.conduit_results {
        for result in conduit_results {
            writeln!(
                conduit_file,
                "{},{:.2},{:.2},{:.2},{:.3},{:.2}",
                result.conduit_id,
                result.flow.unwrap_or(0.0),
                result.velocity.unwrap_or(0.0),
                result.depth.unwrap_or(0.0),
                result.capacity_used.unwrap_or(0.0),
                result.froude_number.unwrap_or(0.0)
            )?;
        }
    }

    Ok(())
}

fn export_visualizations(
    cli: &Cli,
    network: &network::Network,
    analysis: &analysis::Analysis,
) -> Result<(), Box<dyn std::error::Error>> {
    use visualization::{NetworkPlanView, ProfileView, HtmlViewer};

    // Export network plan view if requested
    if let Some(ref path) = cli.export_network_plan {
        println!("\nExporting network plan view...");
        let plan_view = NetworkPlanView::new(network);
        plan_view.save_to_file(path.to_str().unwrap())?;
        println!("  Network plan saved to: {}", path.display());
    }

    // Export profile view if requested
    if let Some(ref path) = cli.export_profile {
        println!("\nExporting profile view with HGL/EGL...");

        // Determine node path for profile
        let node_path: Vec<&str> = if let Some(ref path_str) = cli.profile_path {
            // User provided explicit path
            path_str.split(',').map(|s| s.trim()).collect()
        } else {
            // Auto-generate path from outfall to first inlet (simple linear path)
            find_profile_path(network)
        };

        if node_path.is_empty() {
            println!("  Warning: No valid profile path found. Skipping profile export.");
        } else {
            println!("  Profile path: {}", node_path.join(" → "));
            // Use analysis-aware profile view to include HGL/EGL
            let profile_view = ProfileView::with_analysis(network, &node_path, analysis);
            profile_view.save_to_file(path.to_str().unwrap())?;
            println!("  Profile view with HGL/EGL saved to: {}", path.display());
        }
    }

    // Export HTML viewer if requested
    if let Some(ref path) = cli.export_html {
        println!("\nExporting interactive HTML viewer with HGL/EGL...");

        // Determine node path for profile
        let node_path: Vec<&str> = if let Some(ref path_str) = cli.profile_path {
            path_str.split(',').map(|s| s.trim()).collect()
        } else {
            find_profile_path(network)
        };

        let viewer = HtmlViewer::new(network);

        let html_content = if node_path.is_empty() {
            // Only plan view if no profile path available
            viewer.generate_plan_view()
        } else {
            // Combined view with both plan and profile (with HGL/EGL)
            viewer.generate_combined_view_with_analysis(&node_path, analysis)
        };

        viewer.save_to_file(path.to_str().unwrap(), &html_content)?;
        println!("  Interactive viewer with HGL/EGL saved to: {}", path.display());
        println!("  Open in browser to view the network");
    }

    Ok(())
}

/// Find a simple profile path from outfall upstream
fn find_profile_path(network: &network::Network) -> Vec<&str> {
    // Find first outfall ID
    let outfall_id = match network.outfalls().first() {
        Some(node) => &node.id,
        None => return vec![],
    };

    let mut path = vec![];
    let mut current_id = outfall_id.as_str();

    // Trace upstream from outfall
    path.push(current_id);

    // Follow upstream conduits (simple linear path)
    for _ in 0..100 {  // Prevent infinite loops
        // Find conduit flowing into current node
        let upstream_conduit = network.conduits
            .iter()
            .find(|c| c.to_node == current_id);

        match upstream_conduit {
            Some(conduit) => {
                current_id = conduit.from_node.as_str();
                path.push(current_id);
            }
            None => break,
        }
    }

    // Reverse to go from upstream to downstream
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_spread_criteria_adds_violation() {
        let mut analysis = analysis::Analysis::new(
            analysis::AnalysisMethod::Rational,
            "storm-10yr".to_string(),
        );

        analysis.node_results = Some(vec![analysis::NodeResult {
            node_id: "IN-1".to_string(),
            hgl: Some(100.0),
            egl: Some(100.0),
            depth: Some(1.0),
            velocity: None,
            flooding: Some(false),
            pressure_head: None,
            junction_loss: None,
            gutter_spread: Some(12.0),
        }]);

        apply_spread_criteria(&mut analysis, Some(10.0));

        let violations = analysis.violations.as_ref().unwrap();
        assert_eq!(violations.len(), 1);
        let violation = &violations[0];
        assert_eq!(violation.violation_type, analysis::ViolationType::Spread);
        assert_eq!(violation.severity, analysis::Severity::Warning);
        assert_eq!(violation.element_id, "IN-1");
        assert_eq!(violation.value, Some(12.0));
        assert_eq!(violation.limit, Some(10.0));
    }
}
