//! Test for HEC-22 Chapter 7, Example 7.2
//!
//! This test validates the curb opening inlet interception calculations
//! against the worked example from FHWA HEC-22 Chapter 7, Example 7.2.
//!
//! Example 7.2 Conditions:
//! - Longitudinal slope (SL) = 0.01 ft/ft (1%)
//! - Cross slope (Sx) = 0.02 ft/ft (2%)
//! - Manning's n = 0.016
//! - Flow (Q) = 1.77 ft³/s
//! - Curb opening length (L) = 9.84 ft
//! - Gutter width (W) = 2 ft
//! - Local depression (a) = 1 inch = 0.083 ft
//!
//! Expected Results from HEC-22:
//!
//! Without depression:
//! - LT = 23.9 ft (required length for 100% interception)
//! - E = 61% (efficiency)
//! - Qi = 1.08 ft³/s (intercepted flow)
//!
//! With depression:
//! - Se = 0.047 (effective cross slope)
//! - LT = 14.3 ft (required length with depression)
//! - E = 88% (efficiency with depression)
//! - Qi = 1.56 ft³/s (intercepted flow with depression)

use hec22::*;

#[test]
fn test_hec22_example_7_2_without_depression() {
    println!("\n=== HEC-22 Example 7.2: Without Depression ===\n");

    // Example 7.2 parameters
    let flow = 1.77; // ft³/s
    let longitudinal_slope = 0.01; // 1%
    let cross_slope = 0.02; // 2%
    let manning_n = 0.016;
    let curb_length = 9.84; // ft

    println!("Given conditions:");
    println!("  Flow (Q): {:.2} ft³/s", flow);
    println!(
        "  Longitudinal slope (SL): {:.2} ft/ft ({}%)",
        longitudinal_slope,
        longitudinal_slope * 100.0
    );
    println!(
        "  Cross slope (Sx): {:.2} ft/ft ({}%)",
        cross_slope,
        cross_slope * 100.0
    );
    println!("  Manning's n: {:.3}", manning_n);
    println!("  Curb opening length (L): {:.2} ft", curb_length);
    println!();

    // Create inlet without depression
    let inlet = inlet::CurbOpeningInletOnGrade::new(
        curb_length,
        4.0 / 12.0, // 4 inch curb height (typical)
        inlet::ThroatType::Horizontal,
        0.0, // no clogging factor for this example
    );

    // Calculate required length for total interception (LT) using HEC-22 Equation 7.10
    let lt = inlet::CurbOpeningInletOnGrade::length_for_total_interception(
        flow,
        manning_n,
        cross_slope,
        longitudinal_slope,
    );

    println!("Calculated values:");
    println!("  Required length for 100% interception (LT): {:.1} ft", lt);

    // Expected value from HEC-22 Example 7.2
    let expected_lt = 23.9; // ft

    // Allow 5% tolerance for minor rounding differences
    let lt_error = (lt - expected_lt).abs() / expected_lt;
    println!("  Expected LT from HEC-22: {:.1} ft", expected_lt);
    println!("  Error: {:.1}%", lt_error * 100.0);
    println!();

    assert!(
        lt_error < 0.05,
        "LT should match HEC-22 within 5%: expected {:.1}, got {:.1} ({:.1}% error)",
        expected_lt,
        lt,
        lt_error * 100.0
    );

    // Create gutter for flow analysis
    let gutter = gutter::UniformGutter::new(
        manning_n,
        cross_slope,
        longitudinal_slope,
        None, // no specific width for uniform gutter
    );

    let k = 0.56; // US customary units
    let gutter_result = gutter.result_for_flow(flow, k);

    println!("Gutter flow analysis:");
    println!("  Spread: {:.2} ft", gutter_result.spread);
    println!("  Depth at curb: {:.4} ft", gutter_result.depth_at_curb);
    println!("  Velocity: {:.2} ft/s", gutter_result.velocity);
    println!();

    // Calculate interception
    let interception = inlet.interception(
        flow,
        &gutter_result,
        manning_n,
        cross_slope,
        longitudinal_slope,
    );

    println!("Inlet performance:");
    println!("  L/LT ratio: {:.2}", curb_length / lt);
    println!("  Efficiency (E): {:.1}%", interception.efficiency * 100.0);
    println!(
        "  Intercepted flow (Qi): {:.2} ft³/s",
        interception.intercepted_flow
    );
    println!("  Bypass flow: {:.2} ft³/s", interception.bypass_flow);
    println!();

    // Expected values from HEC-22 Example 7.2
    let expected_efficiency = 0.61; // 61%
    let expected_intercepted = 1.08; // ft³/s

    println!("Expected from HEC-22:");
    println!("  Efficiency: {:.1}%", expected_efficiency * 100.0);
    println!("  Intercepted flow: {:.2} ft³/s", expected_intercepted);
    println!();

    // Verify efficiency
    let efficiency_error =
        (interception.efficiency - expected_efficiency).abs() / expected_efficiency;
    println!("Verification:");
    println!("  Efficiency error: {:.1}%", efficiency_error * 100.0);

    assert!(
        efficiency_error < 0.05,
        "Efficiency should match HEC-22 within 5%: expected {:.1}%, got {:.1}% ({:.1}% error)",
        expected_efficiency * 100.0,
        interception.efficiency * 100.0,
        efficiency_error * 100.0
    );

    // Verify intercepted flow
    let flow_error =
        (interception.intercepted_flow - expected_intercepted).abs() / expected_intercepted;
    println!("  Intercepted flow error: {:.1}%", flow_error * 100.0);

    assert!(
        flow_error < 0.05,
        "Intercepted flow should match HEC-22 within 5%: expected {:.2} ft³/s, got {:.2} ft³/s ({:.1}% error)",
        expected_intercepted, interception.intercepted_flow, flow_error * 100.0
    );

    println!();
    println!("✓ TEST PASSED: Results match HEC-22 Example 7.2 (without depression)");
}

#[test]
fn test_hec22_example_7_2_with_depression() {
    println!("\n=== HEC-22 Example 7.2: With Depression ===\n");

    // Example 7.2 parameters
    let flow = 1.77; // ft³/s
    let longitudinal_slope = 0.01; // 1%
    let cross_slope = 0.02; // 2%
    let manning_n = 0.016;
    let curb_length = 9.84; // ft
    let gutter_width = 2.0; // ft
    let depression = 1.0 / 12.0; // 1 inch = 0.083 ft

    println!("Given conditions:");
    println!("  Flow (Q): {:.2} ft³/s", flow);
    println!(
        "  Longitudinal slope (SL): {:.2} ft/ft ({}%)",
        longitudinal_slope,
        longitudinal_slope * 100.0
    );
    println!(
        "  Cross slope (Sx): {:.2} ft/ft ({}%)",
        cross_slope,
        cross_slope * 100.0
    );
    println!("  Manning's n: {:.3}", manning_n);
    println!("  Curb opening length (L): {:.2} ft", curb_length);
    println!("  Gutter width (W): {:.2} ft", gutter_width);
    println!(
        "  Local depression (a): {:.3} ft ({} inch)",
        depression,
        depression * 12.0
    );
    println!();

    // Create inlet with depression
    let inlet = inlet::CurbOpeningInletOnGrade::new_with_depression(
        curb_length,
        4.0 / 12.0, // 4 inch curb height (typical)
        inlet::ThroatType::Horizontal,
        0.0, // no clogging factor for this example
        depression,
        gutter_width,
    );

    // Use CompositeGutter to properly calculate frontal flow for depression
    let gutter = gutter::CompositeGutter::new(
        manning_n,
        cross_slope,        // Gutter cross slope (Sw) - same as roadway for this example
        cross_slope,        // Roadway cross slope (Sx)
        longitudinal_slope, // Longitudinal slope (SL)
        gutter_width,       // Gutter width (W)
        depression,         // Depression depth (a)
    );

    let k = 0.56; // US customary units
    let gutter_result = gutter.result_for_flow(flow, k);

    println!("Gutter flow analysis:");
    println!("  Spread: {:.2} ft", gutter_result.spread);
    println!("  Depth at curb: {:.4} ft", gutter_result.depth_at_curb);
    println!("  Velocity: {:.2} ft/s", gutter_result.velocity);
    if let Some(frontal_flow) = gutter_result.frontal_flow {
        println!(
            "  Frontal flow (in depressed section): {:.3} ft³/s",
            frontal_flow
        );
        let eo = frontal_flow / flow;
        println!("  Frontal flow ratio (Eo): {:.3}", eo);
    }
    println!();

    // Calculate interception
    let interception = inlet.interception(
        flow,
        &gutter_result,
        manning_n,
        cross_slope,
        longitudinal_slope,
    );

    // Calculate effective cross slope and LT with depression
    // The inlet's interception method should handle this internally
    println!("Inlet performance with depression:");
    println!("  Efficiency (E): {:.1}%", interception.efficiency * 100.0);
    println!(
        "  Intercepted flow (Qi): {:.2} ft³/s",
        interception.intercepted_flow
    );
    println!("  Bypass flow: {:.2} ft³/s", interception.bypass_flow);
    println!();

    // Expected values from HEC-22 Example 7.2
    let expected_efficiency = 0.88; // 88%
    let expected_intercepted = 1.56; // ft³/s

    println!("Expected from HEC-22:");
    println!("  Efficiency: {:.1}%", expected_efficiency * 100.0);
    println!("  Intercepted flow: {:.2} ft³/s", expected_intercepted);
    println!();

    // Verify efficiency
    let efficiency_error =
        (interception.efficiency - expected_efficiency).abs() / expected_efficiency;
    println!("Verification:");
    println!("  Efficiency error: {:.1}%", efficiency_error * 100.0);

    assert!(
        efficiency_error < 0.10, // Allow 10% tolerance for depression case (more complex)
        "Efficiency should match HEC-22 within 10%: expected {:.1}%, got {:.1}% ({:.1}% error)",
        expected_efficiency * 100.0,
        interception.efficiency * 100.0,
        efficiency_error * 100.0
    );

    // Verify intercepted flow
    let flow_error =
        (interception.intercepted_flow - expected_intercepted).abs() / expected_intercepted;
    println!("  Intercepted flow error: {:.1}%", flow_error * 100.0);

    assert!(
        flow_error < 0.10,  // Allow 10% tolerance
        "Intercepted flow should match HEC-22 within 10%: expected {:.2} ft³/s, got {:.2} ft³/s ({:.1}% error)",
        expected_intercepted, interception.intercepted_flow, flow_error * 100.0
    );

    println!();
    println!("✓ TEST PASSED: Results match HEC-22 Example 7.2 (with depression)");
}
