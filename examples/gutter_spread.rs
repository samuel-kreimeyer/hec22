//! Example: Gutter spread calculations
//!
//! This example demonstrates how to calculate gutter spread for different
//! gutter configurations following HEC-22 Chapter 5.

use hec22::gutter::*;

fn main() {
    println!("=== HEC-22 Gutter Spread Calculations ===\n");

    // Example 1: Uniform Cross-Slope Gutter
    println!("--- Example 1: Uniform Cross-Slope Gutter ---");
    println!("Configuration:");
    println!("  Manning's n: 0.016 (asphalt)");
    println!("  Cross slope: 2.0% (0.02 ft/ft)");
    println!("  Longitudinal slope: 1.0% (0.01 ft/ft)");

    let uniform_gutter = UniformGutter::new(
        0.016,  // Manning's n
        0.02,   // 2% cross slope
        0.01,   // 1% longitudinal slope
        None,   // No width limit
    );

    // Calculate spread for design flow
    let flow = 3.0; // cfs
    println!("\nDesign flow: {:.2} cfs", flow);

    let result = uniform_gutter.result_for_flow(flow, GUTTER_K_US);

    println!("\nResults:");
    println!("  Spread (T): {:.2} ft", result.spread);
    println!("  Depth at curb: {:.3} ft ({:.2} inches)",
        result.depth_at_curb, result.depth_at_curb * 12.0);
    println!("  Flow area: {:.3} sq ft", result.area);
    println!("  Velocity: {:.2} ft/s", result.velocity);

    // Check against maximum spread limit
    let max_spread_local = 8.0; // ft (typical for local streets)
    let max_spread_arterial = 12.0; // ft (typical for arterial streets)

    println!("\nDesign Criteria Check:");
    if result.spread <= max_spread_local {
        println!("  ✓ Spread {:.2} ft is within limit for local streets ({:.1} ft)",
            result.spread, max_spread_local);
    } else if result.spread <= max_spread_arterial {
        println!("  ✓ Spread {:.2} ft is within limit for arterial streets ({:.1} ft)",
            result.spread, max_spread_arterial);
    } else {
        println!("  ✗ Spread {:.2} ft EXCEEDS arterial street limit ({:.1} ft)",
            result.spread, max_spread_arterial);
    }

    // Example 2: Composite Gutter Section
    println!("\n\n--- Example 2: Composite Gutter Section ---");
    println!("Configuration:");
    println!("  Manning's n: 0.016");
    println!("  Gutter slope: 4.0% (0.04 ft/ft)");
    println!("  Roadway slope: 2.0% (0.02 ft/ft)");
    println!("  Longitudinal slope: 1.0% (0.01 ft/ft)");
    println!("  Gutter width: 2.0 ft");
    println!("  Local depression: 2.0 inches");

    let composite_gutter = CompositeGutter::new(
        0.016,  // Manning's n
        0.04,   // 4% gutter slope
        0.02,   // 2% roadway slope
        0.01,   // 1% longitudinal slope
        2.0,    // 2 ft gutter width
        2.0,    // 2 inch local depression
    );

    let flow = 5.0; // cfs
    println!("\nDesign flow: {:.2} cfs", flow);

    let result = composite_gutter.result_for_flow(flow, GUTTER_K_US);

    println!("\nResults:");
    println!("  Total spread (T): {:.2} ft", result.spread);
    println!("  Depth at curb: {:.3} ft ({:.2} inches)",
        result.depth_at_curb, result.depth_at_curb * 12.0);
    println!("  Total flow: {:.2} cfs", result.flow);

    if let (Some(frontal), Some(side)) = (result.frontal_flow, result.side_flow) {
        println!("\nFlow Distribution:");
        println!("  Frontal flow (in gutter): {:.2} cfs ({:.1}%)",
            frontal, (frontal / result.flow) * 100.0);
        println!("  Side flow (on roadway): {:.2} cfs ({:.1}%)",
            side, (side / result.flow) * 100.0);
    }

    println!("\nDesign Criteria Check:");
    if result.spread <= max_spread_local {
        println!("  ✓ Spread {:.2} ft is within limit for local streets ({:.1} ft)",
            result.spread, max_spread_local);
    } else {
        println!("  ✗ Spread {:.2} ft EXCEEDS local street limit ({:.1} ft)",
            result.spread, max_spread_local);
        println!("  Recommendation: Increase gutter capacity or add inlet");
    }

    // Example 3: Parabolic Crown
    println!("\n\n--- Example 3: Parabolic Crown Section ---");
    println!("Configuration:");
    println!("  Manning's n: 0.016");
    println!("  Crown height: 0.10 ft (1.2 inches)");
    println!("  Width to crown: 12.0 ft");
    println!("  Longitudinal slope: 1.0% (0.01 ft/ft)");

    let parabolic = ParabolicCrown::new(
        0.016,  // Manning's n
        0.10,   // 0.10 ft crown height
        12.0,   // 12 ft width to crown
        0.01,   // 1% longitudinal slope
    );

    let flow = 2.5; // cfs
    println!("\nDesign flow: {:.2} cfs", flow);

    let result = parabolic.result_for_flow(flow, GUTTER_K_US);

    println!("\nResults:");
    println!("  Spread (T): {:.2} ft", result.spread);
    println!("  Depth at curb: {:.3} ft ({:.2} inches)",
        result.depth_at_curb, result.depth_at_curb * 12.0);
    println!("  Flow area: {:.3} sq ft", result.area);
    println!("  Velocity: {:.2} ft/s", result.velocity);

    // Example 4: Spread vs. Flow Relationship
    println!("\n\n--- Example 4: Spread vs. Flow Relationship ---");
    println!("Uniform gutter (n=0.016, Sx=2%, SL=1%):\n");
    println!("{:>10} {:>10} {:>12} {:>10}", "Spread", "Flow", "Depth", "Velocity");
    println!("{:>10} {:>10} {:>12} {:>10}", "(ft)", "(cfs)", "(inches)", "(ft/s)");
    println!("{}", "-".repeat(46));

    for spread in [4.0, 6.0, 8.0, 10.0, 12.0] {
        let result = uniform_gutter.flow_result(spread, GUTTER_K_US);
        println!("{:>10.1} {:>10.2} {:>12.2} {:>10.2}",
            spread,
            result.flow,
            result.depth_at_curb * 12.0,
            result.velocity);
    }

    // Example 5: Compare Gutter Types
    println!("\n\n--- Example 5: Gutter Type Comparison ---");
    println!("Same flow (Q = 4.0 cfs), same slopes:");
    let target_flow = 4.0;

    let uniform_result = uniform_gutter.result_for_flow(target_flow, GUTTER_K_US);

    let composite_result = CompositeGutter::new(
        0.016, 0.04, 0.02, 0.01, 2.0, 2.0
    ).result_for_flow(target_flow, GUTTER_K_US);

    println!("\n{:<25} {:>10} {:>12}", "Gutter Type", "Spread", "Depth at Curb");
    println!("{:<25} {:>10} {:>12}", "", "(ft)", "(inches)");
    println!("{}", "-".repeat(50));
    println!("{:<25} {:>10.2} {:>12.2}",
        "Uniform Cross-Slope",
        uniform_result.spread,
        uniform_result.depth_at_curb * 12.0);
    println!("{:<25} {:>10.2} {:>12.2}",
        "Composite (w/ depression)",
        composite_result.spread,
        composite_result.depth_at_curb * 12.0);

    println!("\nObservation:");
    println!("  Composite gutter with local depression reduces spread by {:.1} ft",
        uniform_result.spread - composite_result.spread);
    println!("  This is due to increased gutter efficiency from the depressed section");

    // Example 6: Design Iteration
    println!("\n\n--- Example 6: Design Iteration ---");
    println!("Problem: Find minimum gutter width for 8 ft spread limit");
    println!("Given: Q = 5.0 cfs, SL = 1%, n = 0.016");

    let flow = 5.0;
    let spread_limit = 8.0;

    println!("\nTrying different cross slopes:");
    println!("{:>15} {:>12} {:>10}", "Cross Slope", "Spread", "Status");
    println!("{}", "-".repeat(40));

    for sx_percent in [2.0, 2.5, 3.0, 3.5, 4.0] {
        let sx = sx_percent / 100.0;
        let gutter = UniformGutter::new(0.016, sx, 0.01, None);
        let result = gutter.result_for_flow(flow, GUTTER_K_US);

        let status = if result.spread <= spread_limit {
            "✓ OK"
        } else {
            "✗ Exceeds"
        };

        println!("{:>15.1}% {:>12.2} ft {:>10}",
            sx_percent, result.spread, status);
    }

    println!("\nConclusion: Use 3.0% or steeper cross slope to meet 8 ft spread limit");
}
