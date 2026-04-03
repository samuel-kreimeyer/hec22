//! HEC-22 Urban Drainage Analysis CLI
//!
//! Command-line tool for hydraulic analysis of storm sewer networks using the
//! FHWA HEC-22 methodology.

use clap::{Parser, ValueEnum};
use hec22::*;
use serde::Serialize;
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

    /// Path to design criteria JSON file
    #[arg(long, value_name = "FILE")]
    design_criteria: Option<PathBuf>,

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

    /// Chapter 10: Time of concentration for detention screening (minutes)
    #[arg(long, value_name = "MIN")]
    detention_tc: Option<f64>,

    /// Chapter 10: Target controlled outflow for detention screening (cfs/cms)
    #[arg(long, value_name = "FLOW")]
    detention_target_outflow: Option<f64>,

    /// Chapter 10: Basin bottom length for depth screening (ft/m)
    #[arg(long, value_name = "LENGTH")]
    detention_basin_length: Option<f64>,

    /// Chapter 10: Basin bottom width for depth screening (ft/m)
    #[arg(long, value_name = "WIDTH")]
    detention_basin_width: Option<f64>,

    /// Chapter 10: Basin side slope Z (H:V) for depth screening
    #[arg(long, value_name = "SLOPE")]
    detention_basin_side_slope: Option<f64>,

    /// Chapter 11: Imperviousness for WQ screening (%)
    #[arg(long, value_name = "PERCENT")]
    wq_impervious_percent: Option<f64>,

    /// Chapter 11: Water quality design rainfall depth (in or mm)
    #[arg(long, value_name = "DEPTH")]
    wq_design_rainfall: Option<f64>,

    /// Chapter 11: Annual rainfall depth (in or mm)
    #[arg(long, value_name = "DEPTH")]
    wq_annual_rainfall: Option<f64>,

    /// Chapter 11: Runoff-producing event fraction Pj (default: 0.9)
    #[arg(long, default_value = "0.9")]
    wq_runoff_fraction: f64,

    /// Chapter 11: Pollutant concentration (mg/L for chemical, 1000/mL for bacteria)
    #[arg(long, value_name = "CONC")]
    wq_concentration: Option<f64>,

    /// Chapter 11: Pollutant type for annual load calculation
    #[arg(long, value_enum, default_value = "chemical")]
    wq_pollutant_type: PollutantTypeArg,

    /// Chapter 11: BMP removal efficiency fraction (0.0-1.0)
    #[arg(long, value_name = "EFF")]
    wq_removal_efficiency: Option<f64>,

    /// Chapter 11: Bioretention media depth (ft/m)
    #[arg(long, value_name = "DEPTH")]
    wq_media_depth: Option<f64>,

    /// Chapter 11: Bioretention media porosity (fraction)
    #[arg(long, value_name = "POROSITY")]
    wq_media_porosity: Option<f64>,

    /// Chapter 11: Bioretention infiltration rate (ft/hr or m/hr)
    #[arg(long, value_name = "RATE")]
    wq_infiltration_rate: Option<f64>,

    /// Chapter 11: Bioretention drawdown time (hours)
    #[arg(long, value_name = "HOURS")]
    wq_drawdown_hours: Option<f64>,

    /// Chapter 12: Pump design discharge override (cfs/cms). Defaults to peak conduit flow.
    #[arg(long, value_name = "FLOW")]
    pump_design_discharge: Option<f64>,

    /// Chapter 12: Pump static head Hs (ft/m)
    #[arg(long, value_name = "HEAD")]
    pump_static_head: Option<f64>,

    /// Chapter 12: Pump discharge pipe diameter (ft/m)
    #[arg(long, value_name = "DIAM")]
    pump_discharge_diameter: Option<f64>,

    /// Chapter 12: Pump discharge pipe length (ft/m)
    #[arg(long, value_name = "LENGTH")]
    pump_discharge_length: Option<f64>,

    /// Chapter 12: Pump discharge pipe Manning's n
    #[arg(long, value_name = "N")]
    pump_manning_n: Option<f64>,

    /// Chapter 12: Total minor loss coefficient sum K
    #[arg(long, value_name = "K")]
    pump_minor_k_total: Option<f64>,

    /// Chapter 12: Pump efficiency fraction (0.0-1.0)
    #[arg(long, value_name = "EFF")]
    pump_efficiency: Option<f64>,
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

#[derive(Debug, Clone, ValueEnum)]
enum PollutantTypeArg {
    Chemical,
    Bacteria,
}

#[derive(Debug, Clone, Serialize)]
struct ChapterSummaries {
    #[serde(skip_serializing_if = "Option::is_none")]
    chapter10: Option<Chapter10Summary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chapter11: Option<Chapter11Summary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chapter12: Option<Chapter12Summary>,
}

#[derive(Debug, Clone, Serialize)]
struct Chapter10Summary {
    #[serde(rename = "timeOfConcentrationMinutes")]
    tc_minutes: f64,
    #[serde(rename = "peakInflow")]
    peak_inflow: f64,
    #[serde(rename = "targetOutflow")]
    target_outflow: f64,
    #[serde(rename = "preliminaryStorageVolume")]
    preliminary_storage_volume: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "estimatedBasinDepth")]
    estimated_basin_depth: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct Chapter11Summary {
    #[serde(rename = "drainageArea")]
    drainage_area: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imperviousPercent")]
    impervious_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "volumetricRunoffCoefficient")]
    volumetric_runoff_coefficient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "waterQualityVolume")]
    water_quality_volume: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annualRunoffDepth")]
    annual_runoff_depth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annualPollutantLoad")]
    annual_pollutant_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "removedPollutantLoad")]
    removed_pollutant_load: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bioretentionSurfaceArea")]
    bioretention_surface_area: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
struct Chapter12Summary {
    #[serde(rename = "designDischarge")]
    design_discharge: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pipeArea")]
    pipe_area: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    velocity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "staticHead")]
    static_head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frictionHead")]
    friction_head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "velocityHead")]
    velocity_head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minorHead")]
    minor_head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalDynamicHead")]
    total_dynamic_head: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "waterHorsepower")]
    water_horsepower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "brakeHorsepower")]
    brake_horsepower: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "recommendedMotorHp")]
    recommended_motor_hp: Option<f64>,
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

    let mut design_criteria = if let Some(ref path) = cli.design_criteria {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read design criteria file: {}", e))?;
        let criteria: analysis::DesignCriteria = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse design criteria JSON: {}", e))?;
        println!("  Loaded design criteria");
        Some(criteria)
    } else {
        None
    };

    if let Some(max_spread) = cli.max_spread {
        let criteria = design_criteria
            .take()
            .unwrap_or_else(default_design_criteria_placeholder);
        design_criteria = Some(override_max_spread(criteria, max_spread));
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
    network
        .validate_connectivity()
        .map_err(|e| format!("Network validation failed: {}", e))?;

    println!(
        "  {} nodes, {} conduits",
        network.node_count(),
        network.conduit_count()
    );
    println!(
        "  {} inlets, {} junctions, {} outfalls",
        network.inlets().len(),
        network.junctions().len(),
        network.outfalls().len()
    );

    // Load IDF curves if provided
    let idf_curve = if let Some(ref idf_path) = cli.idf_curves {
        println!("\nLoading IDF curves...");
        let curves = csv::parse_idf_curves_csv(idf_path)
            .map_err(|e| format!("Failed to parse IDF curves file: {}", e))?;

        // Find curve for the requested return period
        let curve = curves
            .iter()
            .find(|c| (c.return_period - cli.return_period).abs() < 0.1)
            .ok_or_else(|| {
                format!(
                    "No IDF curve found for return period {} years",
                    cli.return_period
                )
            })?;

        println!(
            "  Using {}-year IDF curve with {} duration points",
            curve.return_period,
            curve.points.len()
        );
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
                            println!(
                                "  Area {}: Tc={:.1} min, i={:.2} in/hr (from IDF curve)",
                                area.id, tc, i
                            );
                            i
                        }
                        None => {
                            println!("  Area {}: Warning - could not interpolate intensity for Tc={:.1} min, using fallback",
                                     area.id, tc);
                            cli.intensity
                        }
                    }
                } else {
                    println!(
                        "  Area {}: Warning - no time of concentration, using fallback intensity",
                        area.id
                    );
                    cli.intensity
                }
            } else {
                // No IDF curve provided, use fixed intensity
                cli.intensity
            };

            // Compute rational method flow: Q = C * i * A
            let c = area.runoff_coefficient.unwrap_or(0.5);
            let flow = c * intensity * area.area;

            println!(
                "  Node {}: Q = {:.2} × {:.2} × {:.2} = {:.2} cfs",
                area.outlet, c, intensity, area.area, flow
            );

            *flows.entry(area.outlet.clone()).or_insert(0.0) += flow;
        }

        if idf_curve.is_none() {
            println!(
                "  (Using fixed intensity: {} {})",
                cli.intensity,
                if matches!(cli.units, UnitSystemArg::Us) {
                    "in/hr"
                } else {
                    "mm/hr"
                }
            );
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
    let mut analysis = hgl_solver
        .solve(
            &network,
            &conduit_flows,
            &inlet_results,
            "Design Storm".to_string(),
        )
        .map_err(|e| format!("HGL solver failed: {}", e))?;
    if let Some(criteria) = design_criteria.as_ref() {
        solver::apply_design_criteria(&mut analysis, &network, criteria, unit_system);
    }

    let chapter_summaries = build_chapter_summaries(&cli, drainage_areas.as_deref(), &analysis);

    // Generate output
    println!("\n{}", "=".repeat(80));
    println!("HYDRAULIC ANALYSIS RESULTS");
    println!("{}\n", "=".repeat(80));

    match cli.format {
        OutputFormat::Text => {
            let report = format_text_report(
                &network,
                &analysis,
                &cli.units,
                &inlet_results,
                chapter_summaries.as_ref(),
            );
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

    if let Some(summaries) = &chapter_summaries {
        print_chapter_summaries(summaries);
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
    chapter_summaries: Option<&ChapterSummaries>,
) -> String {
    let mut report = String::new();
    let unit_suffix = if matches!(units, UnitSystemArg::Us) {
        "ft"
    } else {
        "m"
    };

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
                if let Some(v) = result.velocity {
                    format!("{:.2}", v)
                } else {
                    "-".to_string()
                },
                spread_str,
                if result.flooding.unwrap_or(false) {
                    "YES"
                } else {
                    "No"
                }
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

    if let Some(summaries) = chapter_summaries {
        report.push('\n');
        report.push_str("CHAPTER 10-12 SCREENING SUMMARY\n");
        report.push_str(&format!("{:-<100}\n", ""));

        if let Some(ch10) = &summaries.chapter10 {
            report.push_str("Chapter 10 (Detention/Retention)\n");
            report.push_str(&format!(
                "  Tc: {:.2} min, Peak inflow: {:.2}, Target outflow: {:.2}\n",
                ch10.tc_minutes, ch10.peak_inflow, ch10.target_outflow
            ));
            report.push_str(&format!(
                "  Preliminary storage volume: {:.2}\n",
                ch10.preliminary_storage_volume
            ));
            if let Some(depth) = ch10.estimated_basin_depth {
                report.push_str(&format!(
                    "  Estimated trapezoidal basin depth: {:.2}\n",
                    depth
                ));
            }
        }

        if let Some(ch11) = &summaries.chapter11 {
            report.push_str("Chapter 11 (Stormwater Quality)\n");
            report.push_str(&format!("  Drainage area: {:.2}\n", ch11.drainage_area));
            if let Some(rv) = ch11.volumetric_runoff_coefficient {
                report.push_str(&format!(
                    "  Volumetric runoff coefficient (Rv): {:.3}\n",
                    rv
                ));
            }
            if let Some(wqv) = ch11.water_quality_volume {
                report.push_str(&format!("  Water quality volume (WQv): {:.2}\n", wqv));
            }
            if let Some(load) = ch11.annual_pollutant_load {
                report.push_str(&format!("  Annual pollutant load: {:.2}\n", load));
            }
            if let Some(removed) = ch11.removed_pollutant_load {
                report.push_str(&format!("  Removed pollutant load: {:.2}\n", removed));
            }
            if let Some(area) = ch11.bioretention_surface_area {
                report.push_str(&format!("  Bioretention surface area: {:.2}\n", area));
            }
        }

        if let Some(ch12) = &summaries.chapter12 {
            report.push_str("Chapter 12 (Pump Stations)\n");
            report.push_str(&format!(
                "  Design discharge: {:.2}\n",
                ch12.design_discharge
            ));
            if let Some(v) = ch12.velocity {
                report.push_str(&format!("  Pipe velocity: {:.2}\n", v));
            }
            if let Some(tdh) = ch12.total_dynamic_head {
                report.push_str(&format!("  Total dynamic head (TDH): {:.2}\n", tdh));
            }
            if let Some(whp) = ch12.water_horsepower {
                report.push_str(&format!("  Water horsepower: {:.2}\n", whp));
            }
            if let Some(bhp) = ch12.brake_horsepower {
                report.push_str(&format!("  Brake horsepower: {:.2}\n", bhp));
            }
            if let Some(motor_hp) = ch12.recommended_motor_hp {
                report.push_str(&format!("  Recommended motor size: {:.1} hp\n", motor_hp));
            }
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

fn build_chapter_summaries(
    cli: &Cli,
    drainage_areas: Option<&[drainage::DrainageArea]>,
    analysis: &analysis::Analysis,
) -> Option<ChapterSummaries> {
    let peak_conduit_flow = analysis
        .conduit_results
        .as_ref()
        .and_then(|results| results.iter().filter_map(|r| r.flow).reduce(f64::max));
    let total_drainage_area = drainage_areas.map(|areas| areas.iter().map(|a| a.area).sum::<f64>());
    let max_tc_minutes = drainage_areas.and_then(|areas| {
        areas
            .iter()
            .filter_map(|a| a.time_of_concentration)
            .reduce(f64::max)
    });

    let chapter10 = {
        let tc_minutes = cli.detention_tc.or(max_tc_minutes);
        let peak_inflow = peak_conduit_flow;
        let target_outflow = cli.detention_target_outflow;
        match (tc_minutes, peak_inflow, target_outflow) {
            (Some(tc), Some(peak), Some(target)) if tc > 0.0 && peak > target && target >= 0.0 => {
                let preliminary_storage_volume =
                    storage::triangular_hydrograph_storage_volume(tc, peak, target);
                let estimated_basin_depth = match (
                    cli.detention_basin_length,
                    cli.detention_basin_width,
                    cli.detention_basin_side_slope,
                ) {
                    (Some(l), Some(w), Some(z)) => storage::solve_trapezoidal_depth_for_volume(
                        preliminary_storage_volume,
                        l,
                        w,
                        z,
                    ),
                    _ => None,
                };
                Some(Chapter10Summary {
                    tc_minutes: tc,
                    peak_inflow: peak,
                    target_outflow: target,
                    preliminary_storage_volume,
                    estimated_basin_depth,
                })
            }
            _ => None,
        }
    };

    let chapter11 = {
        if let Some(area) = total_drainage_area {
            let impervious = cli.wq_impervious_percent;
            let rv = impervious.map(quality::volumetric_runoff_coefficient);
            let wqv = match (cli.wq_design_rainfall, impervious) {
                (Some(rainfall), Some(i)) => Some(quality::water_quality_volume(rainfall, area, i)),
                _ => None,
            };
            let annual_runoff = match (cli.wq_annual_rainfall, impervious) {
                (Some(annual_rainfall), Some(i)) => Some(quality::annual_runoff_depth(
                    annual_rainfall,
                    cli.wq_runoff_fraction,
                    i,
                )),
                _ => None,
            };
            let pollutant_type = match cli.wq_pollutant_type {
                PollutantTypeArg::Chemical => quality::PollutantType::Chemical,
                PollutantTypeArg::Bacteria => quality::PollutantType::Bacteria,
            };
            let annual_pollutant_load = match (annual_runoff, cli.wq_concentration) {
                (Some(runoff), Some(concentration)) => Some(quality::simple_method_annual_load(
                    pollutant_type,
                    runoff,
                    concentration,
                    area,
                )),
                _ => None,
            };
            let removed_pollutant_load = match (annual_pollutant_load, cli.wq_removal_efficiency) {
                (Some(load), Some(eff)) => Some(quality::removed_pollutant_load(load, eff)),
                _ => None,
            };
            let bioretention_surface_area = match (
                wqv,
                cli.wq_media_porosity,
                cli.wq_media_depth,
                cli.wq_infiltration_rate,
                cli.wq_drawdown_hours,
            ) {
                (Some(volume), Some(n), Some(depth), Some(k), Some(t)) => {
                    quality::bioretention_surface_area(volume, n, depth, k, t)
                }
                _ => None,
            };

            if rv.is_some()
                || wqv.is_some()
                || annual_runoff.is_some()
                || annual_pollutant_load.is_some()
                || removed_pollutant_load.is_some()
                || bioretention_surface_area.is_some()
            {
                Some(Chapter11Summary {
                    drainage_area: area,
                    impervious_percent: impervious,
                    volumetric_runoff_coefficient: rv,
                    water_quality_volume: wqv,
                    annual_runoff_depth: annual_runoff,
                    annual_pollutant_load,
                    removed_pollutant_load,
                    bioretention_surface_area,
                })
            } else {
                None
            }
        } else {
            None
        }
    };

    let chapter12 = {
        let design_discharge = cli.pump_design_discharge.or(peak_conduit_flow);
        if let Some(q_design) = design_discharge {
            let static_head = cli.pump_static_head;
            let pipe_area = cli.pump_discharge_diameter.map(pump::circular_pipe_area);
            let velocity = match (pipe_area, q_design > 0.0) {
                (Some(a), true) => Some(pump::flow_velocity(q_design, a)),
                _ => None,
            };
            let friction_head = match (
                cli.pump_manning_n,
                cli.pump_discharge_length,
                cli.pump_discharge_diameter,
                velocity,
            ) {
                (Some(n), Some(length), Some(d), Some(v)) => {
                    let hydraulic_radius = d / 4.0;
                    Some(pump::manning_friction_loss(n, length, v, hydraulic_radius))
                }
                _ => None,
            };
            let velocity_head = velocity.map(|v| pump::velocity_head(v, pump::GRAVITY_FTPS2));
            let minor_head = match (cli.pump_minor_k_total, velocity) {
                (Some(k_total), Some(v)) => {
                    Some(pump::minor_losses_head(&[k_total], v, pump::GRAVITY_FTPS2))
                }
                _ => None,
            };
            let total_dynamic_head = match (static_head, friction_head, velocity_head, minor_head) {
                (Some(hs), Some(hf), Some(hv), Some(hl)) => {
                    Some(pump::total_dynamic_head(pump::HeadComponents {
                        static_head: hs,
                        friction_head: hf,
                        velocity_head: hv,
                        minor_head: hl,
                    }))
                }
                _ => None,
            };
            let water_horsepower =
                total_dynamic_head.map(|tdh| pump::water_horsepower(q_design, tdh));
            let brake_horsepower = match (water_horsepower, cli.pump_efficiency) {
                (Some(whp), Some(eff)) => pump::brake_horsepower(whp, eff),
                _ => None,
            };
            let recommended_motor_hp = brake_horsepower.and_then(pump::next_standard_motor_hp);

            if static_head.is_some()
                || pipe_area.is_some()
                || velocity.is_some()
                || friction_head.is_some()
                || velocity_head.is_some()
                || minor_head.is_some()
                || total_dynamic_head.is_some()
                || water_horsepower.is_some()
                || brake_horsepower.is_some()
                || recommended_motor_hp.is_some()
            {
                Some(Chapter12Summary {
                    design_discharge: q_design,
                    pipe_area,
                    velocity,
                    static_head,
                    friction_head,
                    velocity_head,
                    minor_head,
                    total_dynamic_head,
                    water_horsepower,
                    brake_horsepower,
                    recommended_motor_hp,
                })
            } else {
                None
            }
        } else {
            None
        }
    };

    if chapter10.is_none() && chapter11.is_none() && chapter12.is_none() {
        None
    } else {
        Some(ChapterSummaries {
            chapter10,
            chapter11,
            chapter12,
        })
    }
}

fn print_chapter_summaries(summaries: &ChapterSummaries) {
    println!("\n{}", "=".repeat(80));
    println!("CHAPTER 10-12 SCREENING SUMMARY");
    println!("{}\n", "=".repeat(80));

    if let Some(ch10) = &summaries.chapter10 {
        println!("Chapter 10 (Detention/Retention)");
        println!(
            "  Tc: {:.2} min, Peak inflow: {:.2}, Target outflow: {:.2}",
            ch10.tc_minutes, ch10.peak_inflow, ch10.target_outflow
        );
        println!(
            "  Preliminary storage volume: {:.2}",
            ch10.preliminary_storage_volume
        );
        if let Some(depth) = ch10.estimated_basin_depth {
            println!("  Estimated trapezoidal basin depth: {:.2}", depth);
        }
    }

    if let Some(ch11) = &summaries.chapter11 {
        println!("Chapter 11 (Stormwater Quality)");
        println!("  Drainage area: {:.2}", ch11.drainage_area);
        if let Some(rv) = ch11.volumetric_runoff_coefficient {
            println!("  Volumetric runoff coefficient (Rv): {:.3}", rv);
        }
        if let Some(wqv) = ch11.water_quality_volume {
            println!("  Water quality volume (WQv): {:.2}", wqv);
        }
        if let Some(runoff) = ch11.annual_runoff_depth {
            println!("  Annual runoff depth: {:.2}", runoff);
        }
        if let Some(load) = ch11.annual_pollutant_load {
            println!("  Annual pollutant load: {:.2}", load);
        }
        if let Some(removed) = ch11.removed_pollutant_load {
            println!("  Removed pollutant load: {:.2}", removed);
        }
        if let Some(area) = ch11.bioretention_surface_area {
            println!("  Bioretention surface area: {:.2}", area);
        }
    }

    if let Some(ch12) = &summaries.chapter12 {
        println!("Chapter 12 (Pump Stations)");
        println!("  Design discharge: {:.2}", ch12.design_discharge);
        if let Some(area) = ch12.pipe_area {
            println!("  Pipe area: {:.3}", area);
        }
        if let Some(v) = ch12.velocity {
            println!("  Pipe velocity: {:.2}", v);
        }
        if let Some(hf) = ch12.friction_head {
            println!("  Friction head: {:.2}", hf);
        }
        if let Some(hv) = ch12.velocity_head {
            println!("  Velocity head: {:.2}", hv);
        }
        if let Some(hl) = ch12.minor_head {
            println!("  Minor head: {:.2}", hl);
        }
        if let Some(tdh) = ch12.total_dynamic_head {
            println!("  Total dynamic head (TDH): {:.2}", tdh);
        }
        if let Some(whp) = ch12.water_horsepower {
            println!("  Water horsepower: {:.2}", whp);
        }
        if let Some(bhp) = ch12.brake_horsepower {
            println!("  Brake horsepower: {:.2}", bhp);
        }
        if let Some(motor_hp) = ch12.recommended_motor_hp {
            println!("  Recommended motor size: {:.1} hp", motor_hp);
        }
    }
}

fn default_design_criteria_placeholder() -> analysis::DesignCriteria {
    analysis::DesignCriteria {
        gutter_spread: None,
        hgl_criteria: None,
        velocity: None,
        cover: None,
        capacity: None,
    }
}

fn override_max_spread(
    mut criteria: analysis::DesignCriteria,
    max_spread: f64,
) -> analysis::DesignCriteria {
    let gutter = criteria
        .gutter_spread
        .get_or_insert(analysis::GutterSpreadCriteria {
            max_spread: None,
            max_spread_local_street: None,
            max_spread_collector_street: None,
            max_spread_arterial_street: None,
        });
    gutter.max_spread = Some(max_spread);
    criteria
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
    writeln!(
        conduit_file,
        "conduit_id,flow,velocity,depth,capacity_used,froude_number"
    )?;

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
    use visualization::{HtmlViewer, NetworkPlanView, ProfileView};

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
        println!(
            "  Interactive viewer with HGL/EGL saved to: {}",
            path.display()
        );
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
    for _ in 0..100 {
        // Prevent infinite loops
        // Find conduit flowing into current node
        let upstream_conduit = network.conduits.iter().find(|c| c.to_node == current_id);

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
    fn test_apply_design_criteria_adds_spread_violation() {
        let mut analysis =
            analysis::Analysis::new(analysis::AnalysisMethod::Rational, "storm-10yr".to_string());

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

        let criteria = override_max_spread(default_design_criteria_placeholder(), 10.0);
        solver::apply_design_criteria(
            &mut analysis,
            &network::Network::new(),
            &criteria,
            project::UnitSystem::US,
        );

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
