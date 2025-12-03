//! Unit tests based on actual HEC-22 Chapter 5 worked examples
//!
//! These tests verify that our implementation matches the exact values
//! shown in the HEC-22 worked examples from Chapter 5.

use hec22::gutter::{CompositeGutter, UniformGutter, GUTTER_K_US};

const TOLERANCE_SPREAD: f64 = 0.1; // 0.1 ft tolerance for spread
const TOLERANCE_FLOW: f64 = 0.1;    // 0.1 cfs tolerance for flow

/// HEC-22 Chapter 5, Example 5.1: Computation of triangular gutter flow
///
/// **Part A:** Given Q = 1.8 cfs, find spread T
/// - Given: SL = 0.010 ft/ft, Sx = 0.020 ft/ft, n = 0.016
/// - Expected: T = 9.0 ft
///
/// **Part B:** Given T = 8.2 ft, find flow Q
/// - Given: Same parameters
/// - Expected: Q = 1.4 cfs
#[test]
fn test_example_5_1_triangular_gutter() {
    println!("\n=== HEC-22 Example 5.1: Triangular Gutter Flow ===\n");

    let gutter = UniformGutter::new(
        0.016, // n
        0.020, // Sx (2%)
        0.010, // SL (1%)
        None,
    );

    // Part A: Find spread for Q = 1.8 cfs
    println!("Part A: Find spread for Q = 1.8 cfs");
    let q_partA = 1.8;
    let expected_spread_a = 9.0;
    let computed_spread_a = gutter.spread_for_flow(q_partA, GUTTER_K_US);

    println!("  Given: Q = {:.1} cfs", q_partA);
    println!("  Computed spread: {:.2} ft", computed_spread_a);
    println!("  Expected spread: {:.1} ft", expected_spread_a);
    println!("  Error: {:.2} ft\n", (computed_spread_a - expected_spread_a).abs());

    assert!(
        (computed_spread_a - expected_spread_a).abs() < TOLERANCE_SPREAD,
        "Example 5.1 Part A: Expected T={}, got T={}",
        expected_spread_a,
        computed_spread_a
    );

    // Part B: Find flow for T = 8.2 ft
    println!("Part B: Find flow for T = 8.2 ft");
    let t_partB = 8.2;
    let expected_flow_b = 1.4;
    let computed_flow_b = gutter.flow_capacity(t_partB, GUTTER_K_US);

    println!("  Given: T = {:.1} ft", t_partB);
    println!("  Computed flow: {:.2} cfs", computed_flow_b);
    println!("  Expected flow: {:.1} cfs", expected_flow_b);
    println!("  Error: {:.2} cfs", (computed_flow_b - expected_flow_b).abs());

    assert!(
        (computed_flow_b - expected_flow_b).abs() < TOLERANCE_FLOW,
        "Example 5.1 Part B: Expected Q={}, got Q={}",
        expected_flow_b,
        computed_flow_b
    );
}

/// HEC-22 Chapter 5, Example 5.2: Composite gutter flow
///
/// **Part A:** Given T = 8.2 ft, find flow Q
/// - Given: W = 2 ft, SL = 0.01, Sx = 0.02, n = 0.016, a = 2 inches
/// - Expected: Q = 2.3 cfs
///
/// **Part B:** Given Q = 4.2 cfs, find spread T (requires iteration)
/// - Given: Same parameters
/// - Expected: T = 11.1 ft
#[test]
fn test_example_5_2_composite_gutter() {
    println!("\n=== HEC-22 Example 5.2: Composite Gutter Flow ===\n");

    let gutter = CompositeGutter::new(
        0.016, // n
        0.02,  // Sx (pavement cross slope) - 2%
        0.02,  // Sw (gutter slope) - same as pavement initially
        0.01,  // SL (longitudinal slope) - 1%
        2.0,   // W (gutter width) - 2 ft
        2.0,   // a (depression) - 2 inches
    );

    // Part A: Find flow for T = 8.2 ft
    println!("Part A: Find flow for T = 8.2 ft");
    let t_partA = 8.2;
    let expected_flow_a = 2.3;
    let computed_flow_a = gutter.flow_capacity(t_partA, GUTTER_K_US);

    println!("  Gutter width W = {:.1} ft", gutter.gutter_width);
    println!("  Depression a = {:.1} inches", gutter.local_depression);
    println!("  Given: T = {:.1} ft", t_partA);
    println!("  Computed flow: {:.2} cfs", computed_flow_a);
    println!("  Expected flow: {:.1} cfs", expected_flow_a);
    println!("  Error: {:.2} cfs\n", (computed_flow_a - expected_flow_a).abs());

    assert!(
        (computed_flow_a - expected_flow_a).abs() < TOLERANCE_FLOW,
        "Example 5.2 Part A: Expected Q={}, got Q={}",
        expected_flow_a,
        computed_flow_a
    );

    // Part B: Find spread for Q = 4.2 cfs (iterative)
    println!("Part B: Find spread for Q = 4.2 cfs (requires iteration)");
    let q_partB = 4.2;
    let expected_spread_b = 11.1;
    let computed_spread_b = gutter.spread_for_flow(q_partB, GUTTER_K_US);

    println!("  Given: Q = {:.1} cfs", q_partB);
    println!("  Computed spread: {:.2} ft", computed_spread_b);
    println!("  Expected spread: {:.1} ft", expected_spread_b);
    println!("  Error: {:.2} ft", (computed_spread_b - expected_spread_b).abs());

    // Verification: check that computed spread gives approximately the input flow
    let verification_flow = gutter.flow_capacity(computed_spread_b, GUTTER_K_US);
    println!("  Verification: Q at T={:.2} is {:.2} cfs", computed_spread_b, verification_flow);

    assert!(
        (computed_spread_b - expected_spread_b).abs() < TOLERANCE_SPREAD * 2.0, // Allow slightly larger tolerance for iterative
        "Example 5.2 Part B: Expected T={}, got T={}",
        expected_spread_b,
        computed_spread_b
    );
}

/// Test that our Equation 5.2 implementation matches the HEC-22 formula
#[test]
fn test_equation_5_2_verification() {
    println!("\n=== Equation 5.2 Verification ===\n");

    let n = 0.016;
    let sx = 0.020;
    let sl = 0.010;
    let t = 9.0;

    let gutter = UniformGutter::new(n, sx, sl, None);
    let q = gutter.flow_capacity(t, GUTTER_K_US);

    // Manual calculation: Q = (Ku/n) × Sx^1.67 × SL^0.5 × T^2.67
    let ku = GUTTER_K_US;
    let q_manual = (ku / n) * sx.powf(1.67) * sl.powf(0.5) * t.powf(2.67);

    println!("Parameters:");
    println!("  n = {}", n);
    println!("  Sx = {} ({:.1}%)", sx, sx * 100.0);
    println!("  SL = {} ({:.1}%)", sl, sl * 100.0);
    println!("  T = {} ft", t);
    println!("\nEquation 5.2: Q = (Ku/n) × Sx^1.67 × SL^0.5 × T^2.67");
    println!("  Ku = {} (US customary)", ku);
    println!("  Computed Q: {:.4} cfs", q);
    println!("  Manual Q:   {:.4} cfs", q_manual);
    println!("  Difference: {:.6} cfs", (q - q_manual).abs());

    assert!(
        (q - q_manual).abs() < 1e-10,
        "Equation 5.2 implementation should match manual calculation"
    );
}

/// Test that our Equation 5.4 implementation matches the HEC-22 formula
#[test]
fn test_equation_5_4_verification() {
    println!("\n=== Equation 5.4 Verification ===\n");

    let n = 0.016;
    let sx = 0.020;
    let sl = 0.010;
    let q = 1.8;

    let gutter = UniformGutter::new(n, sx, sl, None);
    let t = gutter.spread_for_flow(q, GUTTER_K_US);

    // Manual calculation: T = [(Qn)/(Ku × Sx^1.67 × SL^0.5)]^0.375
    let ku = GUTTER_K_US;
    let t_manual = ((q * n) / (ku * sx.powf(1.67) * sl.powf(0.5))).powf(0.375);

    println!("Parameters:");
    println!("  n = {}", n);
    println!("  Sx = {} ({:.1}%)", sx, sx * 100.0);
    println!("  SL = {} ({:.1}%)", sl, sl * 100.0);
    println!("  Q = {} cfs", q);
    println!("\nEquation 5.4: T = [(Qn)/(Ku × Sx^1.67 × SL^0.5)]^0.375");
    println!("  Ku = {} (US customary)", ku);
    println!("  Computed T: {:.4} ft", t);
    println!("  Manual T:   {:.4} ft", t_manual);
    println!("  Difference: {:.6} ft", (t - t_manual).abs());

    assert!(
        (t - t_manual).abs() < 1e-10,
        "Equation 5.4 implementation should match manual calculation"
    );
}

/// Test Equation 5.7 (Eo calculation) for composite gutters
#[test]
fn test_equation_5_7_flow_ratio() {
    println!("\n=== Equation 5.7 Verification ===\n");

    // From Example 5.2, Step A3
    let sx = 0.02;
    let a = 2.0 / 12.0; // 2 inches to feet
    let w = 2.0;
    let t = 8.2;

    let sw = sx + a / w; // Equation 5.8
    let sw_over_sx = sw / sx;
    let t_over_w = t / w;

    println!("Parameters:");
    println!("  Sx = {:.3} ({:.1}%)", sx, sx * 100.0);
    println!("  a = {:.3} ft ({:.1} in)", a, a * 12.0);
    println!("  W = {:.1} ft", w);
    println!("  T = {:.1} ft", t);
    println!("\nComputed values:");
    println!("  Sw = Sx + a/W = {:.4}", sw);
    println!("  Sw/Sx = {:.2}", sw_over_sx);
    println!("  T/W = {:.2}", t_over_w);

    // Equation 5.7
    let denominator_term = (1.0_f64 + sw_over_sx / (t_over_w - 1.0_f64)).powf(2.67) - 1.0_f64;
    let eo = 1.0_f64 / (1.0_f64 + sw_over_sx / denominator_term);

    println!("\nEquation 5.7:");
    println!("  Denominator term: {:.4}", denominator_term);
    println!("  Eo = {:.2}", eo);

    // From Example 5.2, the expected Eo is 0.70
    let expected_eo = 0.70_f64;
    println!("  Expected (from Example 5.2): ~{:.2}", expected_eo);
    println!("  Error: {:.3}", (eo - expected_eo).abs());

    assert!(
        (eo - expected_eo).abs() < 0.05_f64,
        "Equation 5.7 should give Eo ≈ {}",
        expected_eo
    );
}

/// Test that composite gutter has higher capacity than uniform gutter
#[test]
fn test_composite_vs_uniform_capacity() {
    println!("\n=== Composite vs Uniform Capacity Comparison ===\n");

    let n = 0.016;
    let sx = 0.02;
    let sl = 0.01;
    let spread = 10.0;

    // Uniform gutter
    let uniform = UniformGutter::new(n, sx, sl, None);
    let q_uniform = uniform.flow_capacity(spread, GUTTER_K_US);

    // Composite gutter with depression
    let composite = CompositeGutter::new(
        n,
        sx,   // roadway slope
        sx,   // gutter slope (before depression)
        sl,
        2.0,  // 2 ft gutter width
        2.0,  // 2 inch depression
    );
    let q_composite = composite.flow_capacity(spread, GUTTER_K_US);

    println!("Spread T = {:.1} ft\n", spread);
    println!("Uniform gutter:");
    println!("  Q = {:.2} cfs", q_uniform);
    println!("\nComposite gutter (W=2ft, a=2in):");
    println!("  Q = {:.2} cfs", q_composite);
    println!("\nEnhancement:");
    println!("  Increase: {:.2} cfs ({:.1}%)",
        q_composite - q_uniform,
        ((q_composite / q_uniform) - 1.0) * 100.0
    );

    assert!(
        q_composite > q_uniform,
        "Composite gutter should have higher capacity than uniform gutter"
    );
}

/// Test roundtrip: spread -> flow -> spread
#[test]
fn test_roundtrip_uniform_gutter() {
    println!("\n=== Roundtrip Test: Uniform Gutter ===\n");

    let gutter = UniformGutter::new(0.016, 0.02, 0.01, None);

    let original_spread = 10.0;
    let flow = gutter.flow_capacity(original_spread, GUTTER_K_US);
    let computed_spread = gutter.spread_for_flow(flow, GUTTER_K_US);

    println!("Original spread: {:.2} ft", original_spread);
    println!("Flow: {:.2} cfs", flow);
    println!("Computed spread: {:.2} ft", computed_spread);
    println!("Error: {:.4} ft", (computed_spread - original_spread).abs());

    assert!(
        (computed_spread - original_spread).abs() < 0.001,
        "Roundtrip should recover original spread"
    );
}

/// Test roundtrip: flow -> spread -> flow for composite gutter
#[test]
fn test_roundtrip_composite_gutter() {
    println!("\n=== Roundtrip Test: Composite Gutter ===\n");

    let gutter = CompositeGutter::new(
        0.016,
        0.02,
        0.02,
        0.01,
        2.0,
        2.0,
    );

    let original_flow = 4.0;
    let spread = gutter.spread_for_flow(original_flow, GUTTER_K_US);
    let computed_flow = gutter.flow_capacity(spread, GUTTER_K_US);

    println!("Original flow: {:.2} cfs", original_flow);
    println!("Spread: {:.2} ft", spread);
    println!("Computed flow: {:.2} cfs", computed_flow);
    println!("Error: {:.4} cfs", (computed_flow - original_flow).abs());

    assert!(
        (computed_flow - original_flow).abs() < 0.1,
        "Roundtrip should recover original flow within tolerance"
    );
}
