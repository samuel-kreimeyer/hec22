//! Example: Inlet capacity and interception analysis
//!
//! This example demonstrates inlet capacity calculations following HEC-22 Chapter 7,
//! including on-grade and sag inlet configurations with bypass flow tracking.

use hec22::gutter::{GutterFlowResult, UniformGutter, GUTTER_K_US};
use hec22::inlet::*;

fn main() {
    println!("=== HEC-22 Inlet Capacity Analysis Example ===\n");

    // Example 1: Grate inlet on grade
    example_1_grate_on_grade();

    // Example 2: Curb opening on grade
    example_2_curb_opening_on_grade();

    // Example 3: Combination inlet on grade
    example_3_combination_inlet();

    // Example 4: Series of inlets with bypass flow
    example_4_series_with_bypass();

    // Example 5: Sag inlet sizing
    example_5_sag_inlet();

    // Example 6: 100% interception length
    example_6_total_interception();
}

fn example_1_grate_on_grade() {
    println!("--- Example 1: Grate Inlet On Grade ---");
    println!("Standard P-50×100 grate with perpendicular bars\n");

    // Gutter characteristics
    let gutter = UniformGutter::new(
        0.016, // Manning's n (asphalt)
        0.02,  // Cross slope (2%)
        0.01,  // Longitudinal slope (1%)
        None,
    );

    // Design flow approaching inlet
    let approach_flow = 4.0; // cfs
    let gutter_result = gutter.result_for_flow(approach_flow, GUTTER_K_US);

    println!("Approach conditions:");
    println!("  Flow: {:.2} cfs", approach_flow);
    println!("  Spread: {:.2} ft", gutter_result.spread);
    println!("  Velocity: {:.2} ft/s", gutter_result.velocity);
    println!("  Depth at curb: {:.3} ft", gutter_result.depth_at_curb);

    // Create grate inlet (2 ft wide × 3 ft long)
    let inlet = GrateInletOnGrade::new(
        3.0,                             // 3 ft length
        2.0,                             // 2 ft width
        BarConfiguration::Perpendicular, // Bars perpendicular to flow
        0.15,                            // 15% clogging factor
        2.0,                             // 2 inch local depression
    );

    let result = inlet.interception(approach_flow, &gutter_result);

    println!("\nInlet performance:");
    println!("  Intercepted flow: {:.2} cfs", result.intercepted_flow);
    println!("  Bypass flow: {:.2} cfs", result.bypass_flow);
    println!("  Efficiency: {:.1}%", result.efficiency * 100.0);
    println!();
}

fn example_2_curb_opening_on_grade() {
    println!("--- Example 2: Curb Opening Inlet On Grade ---");
    println!("5-ft horizontal throat curb opening\n");

    let gutter = UniformGutter::new(0.016, 0.02, 0.008, None);
    let approach_flow = 3.5;
    let gutter_result = gutter.result_for_flow(approach_flow, GUTTER_K_US);

    println!("Approach conditions:");
    println!("  Flow: {:.2} cfs", approach_flow);
    println!("  Spread: {:.2} ft", gutter_result.spread);
    println!("  Velocity: {:.2} ft/s", gutter_result.velocity);

    let inlet = CurbOpeningInletOnGrade::new(
        5.0,                  // 5 ft length
        0.5,                  // 6 inch height
        ThroatType::Horizontal,
        0.10,                 // 10% clogging
    );

    let result = inlet.interception(approach_flow, &gutter_result);

    println!("\nInlet performance:");
    println!("  Intercepted flow: {:.2} cfs", result.intercepted_flow);
    println!("  Bypass flow: {:.2} cfs", result.bypass_flow);
    println!("  Efficiency: {:.1}%", result.efficiency * 100.0);

    // Calculate required length for 100% interception
    let lt = CurbOpeningInletOnGrade::length_for_total_interception(
        approach_flow,
        gutter_result.velocity,
    );
    println!("\nRequired length for 100% interception: {:.1} ft", lt);
    println!();
}

fn example_3_combination_inlet() {
    println!("--- Example 3: Combination Inlet (Grate + Curb Opening) ---");
    println!("P-50×100 grate with 5-ft curb opening\n");

    let gutter = UniformGutter::new(0.016, 0.02, 0.01, None);
    let approach_flow = 6.0;
    let gutter_result = gutter.result_for_flow(approach_flow, GUTTER_K_US);

    println!("Approach conditions:");
    println!("  Flow: {:.2} cfs", approach_flow);
    println!("  Spread: {:.2} ft", gutter_result.spread);
    println!("  Velocity: {:.2} ft/s", gutter_result.velocity);

    // Grate component
    let grate = GrateInletOnGrade::new(
        3.0,
        2.0,
        BarConfiguration::Perpendicular,
        0.15,
        2.0,
    );

    // Curb opening component
    let curb = CurbOpeningInletOnGrade::new(5.0, 0.5, ThroatType::Horizontal, 0.10);

    // Test grate alone
    let grate_only = grate.interception(approach_flow, &gutter_result);
    println!("\nGrate alone:");
    println!("  Intercepted: {:.2} cfs", grate_only.intercepted_flow);
    println!("  Bypass: {:.2} cfs", grate_only.bypass_flow);
    println!("  Efficiency: {:.1}%", grate_only.efficiency * 100.0);

    // Test combination
    let combo = CombinationInletOnGrade::new(grate, curb);
    let combo_result = combo.interception(approach_flow, &gutter_result);

    println!("\nCombination inlet:");
    println!("  Intercepted: {:.2} cfs", combo_result.intercepted_flow);
    println!("  Bypass: {:.2} cfs", combo_result.bypass_flow);
    println!("  Efficiency: {:.1}%", combo_result.efficiency * 100.0);
    println!(
        "  Improvement: {:.1} percentage points",
        (combo_result.efficiency - grate_only.efficiency) * 100.0
    );
    println!();
}

fn example_4_series_with_bypass() {
    println!("--- Example 4: Series of Inlets with Bypass Flow ---");
    println!("Three inlets in series along a street\n");

    let gutter = UniformGutter::new(0.016, 0.02, 0.01, None);

    // Initial flow from drainage area
    let initial_flow = 8.0; // cfs

    println!("Gutter configuration:");
    println!("  Manning's n: {}", gutter.manning_n);
    println!("  Cross slope: {:.1}%", gutter.cross_slope * 100.0);
    println!("  Longitudinal slope: {:.1}%", gutter.longitudinal_slope * 100.0);
    println!();

    // Inlet 1: First inlet captures some flow
    println!("Inlet 1 (Station 0+00):");
    let inlet1 = GrateInletOnGrade::new(
        3.0,
        2.0,
        BarConfiguration::Perpendicular,
        0.15,
        2.0,
    );
    let gutter1 = gutter.result_for_flow(initial_flow, GUTTER_K_US);
    let result1 = inlet1.interception(initial_flow, &gutter1);

    println!("  Approach flow: {:.2} cfs", result1.approach_flow);
    println!("  Spread: {:.2} ft", result1.spread);
    println!("  Intercepted: {:.2} cfs", result1.intercepted_flow);
    println!("  Bypass: {:.2} cfs", result1.bypass_flow);
    println!("  Efficiency: {:.1}%", result1.efficiency * 100.0);
    println!();

    // Inlet 2: Receives bypass from inlet 1 plus additional drainage
    let additional_flow_2 = 2.0; // Additional drainage between inlets
    let approach_flow_2 = result1.bypass_flow + additional_flow_2;

    println!("Inlet 2 (Station 2+00):");
    println!("  Bypass from Inlet 1: {:.2} cfs", result1.bypass_flow);
    println!("  Additional drainage: {:.2} cfs", additional_flow_2);

    let inlet2 = GrateInletOnGrade::new(3.0, 2.0, BarConfiguration::Perpendicular, 0.15, 2.0);
    let gutter2 = gutter.result_for_flow(approach_flow_2, GUTTER_K_US);
    let result2 = inlet2.interception(approach_flow_2, &gutter2);

    println!("  Total approach flow: {:.2} cfs", result2.approach_flow);
    println!("  Spread: {:.2} ft", result2.spread);
    println!("  Intercepted: {:.2} cfs", result2.intercepted_flow);
    println!("  Bypass: {:.2} cfs", result2.bypass_flow);
    println!("  Efficiency: {:.1}%", result2.efficiency * 100.0);
    println!();

    // Inlet 3: Combination inlet at sag (100% capture)
    let additional_flow_3 = 1.5;
    let approach_flow_3 = result2.bypass_flow + additional_flow_3;

    println!("Inlet 3 (Station 4+00 - Sag location):");
    println!("  Bypass from Inlet 2: {:.2} cfs", result2.bypass_flow);
    println!("  Additional drainage: {:.2} cfs", additional_flow_3);
    println!("  Total approach flow: {:.2} cfs", approach_flow_3);

    let sag_inlet = GrateInletSag::new(
        4.0,  // 4 ft length
        3.0,  // 3 ft width
        2,    // 2 grates
        0.50, // 50% clogging
    );

    // Check if inlet can handle flow without flooding
    let rim_elevation = 100.0;
    let invert_elevation = 95.0;
    let (flooding, depth) = sag_inlet.check_flooding(approach_flow_3, rim_elevation, invert_elevation);

    println!("  Sag inlet capacity check:");
    println!("    Ponding depth: {:.2} ft", depth);
    println!("    Flooding: {}", if flooding { "YES - INCREASE SIZE" } else { "No" });
    println!("    Intercepted: {:.2} cfs (100%)", approach_flow_3);
    println!("    Bypass: 0.00 cfs");
    println!();

    // Summary
    println!("Summary:");
    let total_intercepted = result1.intercepted_flow + result2.intercepted_flow + approach_flow_3;
    let total_input = initial_flow + additional_flow_2 + additional_flow_3;
    println!("  Total drainage area flow: {:.2} cfs", total_input);
    println!("  Total intercepted: {:.2} cfs", total_intercepted);
    println!("  Overall system efficiency: {:.1}%", (total_intercepted / total_input) * 100.0);
    println!();
}

fn example_5_sag_inlet() {
    println!("--- Example 5: Sag Inlet Sizing ---");
    println!("Comparing grate and curb opening inlets at sag\n");

    let design_flow = 12.0; // cfs
    let rim_elevation = 100.0;
    let invert_elevation = 95.0;

    println!("Design flow: {:.2} cfs", design_flow);
    println!("Available depth: {:.1} ft\n", rim_elevation - invert_elevation);

    // Option 1: Single large grate
    println!("Option 1: Single 4×3 ft grate");
    let grate1 = GrateInletSag::new(4.0, 3.0, 1, 0.50);
    let (flood1, depth1) = grate1.check_flooding(design_flow, rim_elevation, invert_elevation);
    let capacity1 = grate1.capacity(depth1);

    println!("  Ponding depth: {:.2} ft", depth1);
    println!("  Capacity: {:.2} cfs", capacity1);
    println!("  Status: {}", if flood1 { "INADEQUATE" } else { "OK" });
    println!();

    // Option 2: Two smaller grates
    println!("Option 2: Two 3×2 ft grates");
    let grate2 = GrateInletSag::new(3.0, 2.0, 2, 0.50);
    let (flood2, depth2) = grate2.check_flooding(design_flow, rim_elevation, invert_elevation);
    let capacity2 = grate2.capacity(depth2);

    println!("  Ponding depth: {:.2} ft", depth2);
    println!("  Capacity: {:.2} cfs", capacity2);
    println!("  Status: {}", if flood2 { "INADEQUATE" } else { "OK" });
    println!();

    // Option 3: Curb opening
    println!("Option 3: 10-ft curb opening (6-inch height)");
    let curb = CurbOpeningInletSag::new(10.0, 0.5, ThroatType::Horizontal, 0.10);

    // Test at various depths
    for test_depth in [0.5, 1.0, 1.5, 2.0] {
        let capacity = curb.capacity(test_depth);
        println!("  At {:.1} ft depth: {:.2} cfs capacity", test_depth, capacity);
        if capacity >= design_flow {
            println!("    → Adequate capacity at {:.2} ft depth", test_depth);
            break;
        }
    }
    println!();
}

fn example_6_total_interception() {
    println!("--- Example 6: Length for 100% Interception ---");
    println!("Calculate required grate length to intercept all flow\n");

    let flows = [2.0, 4.0, 6.0, 8.0];
    let slopes = [0.005, 0.01, 0.02]; // 0.5%, 1%, 2%

    let n = 0.016;
    let sx = 0.02; // Cross slope 2%

    println!("Conditions: n = {}, Sx = {:.1}%\n", n, sx * 100.0);
    println!("{:<12} {:>10} {:>10} {:>10}", "Flow (cfs)", "0.5% slope", "1.0% slope", "2.0% slope");
    println!("{}", "-".repeat(45));

    for flow in flows {
        print!("{:<12.1}", flow);
        for slope in slopes {
            let lt = GrateInletOnGrade::length_for_total_interception(flow, n, sx, slope);
            print!(" {:>10.1}", lt);
        }
        println!();
    }

    println!("\nObservations:");
    println!("  • Steeper longitudinal slopes require longer grates");
    println!("  • Higher flows require significantly longer grates");
    println!("  • For practical grate lengths (3-5 ft), use multiple inlets or combinations");
    println!();
}
