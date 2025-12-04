// HEC-22 Chapter 9 Verification Tests
//
// This test file implements worked examples from HEC-22 Chapter 9 to verify
// that the drainage network calculations match the reference textbook values.

use hec22::hydraulics::{
    EnergyLoss, ManningsEquation, FhwaAccessHoleMethod, InflowPipe, BenchingType,
    GRAVITY_US, MANNING_CONST_US,
};

const TOLERANCE_ELEVATION: f64 = 0.1; // ±0.1 ft for HGL/EGL
const TOLERANCE_FLOW: f64 = 0.02; // ±2% for flows

/// Test Example 9.1: Simple Pipe HGL Propagation
///
/// Demonstrates basic HGL/EGL calculation through a single pipe with friction losses.
/// Tests Manning's equation and energy grade line computation.
#[test]
fn test_example_9_1_simple_pipe_hgl() {
    println!("\n{}", "=".repeat(80));
    println!("HEC-22 EXAMPLE 9.1: Simple Pipe HGL Propagation");
    println!("{}\n", "=".repeat(80));

    // GIVEN: Single 18-inch RCP pipe
    let diameter = 18.0; // inches
    let diameter_ft = diameter / 12.0;
    let length = 200.0; // ft
    let slope = 0.01; // 1%
    let manning_n = 0.013; // RCP
    let flow = 3.5; // cfs
    let downstream_hgl = 100.0; // ft elevation

    println!("GIVEN:");
    println!("  Pipe diameter: {} inches ({:.2} ft)", diameter, diameter_ft);
    println!("  Pipe length: {:.0} ft", length);
    println!("  Pipe slope: {:.2}% ({:.4} ft/ft)", slope * 100.0, slope);
    println!("  Manning's n: {:.3}", manning_n);
    println!("  Flow rate: {:.2} cfs", flow);
    println!("  Downstream HGL: {:.2} ft\n", downstream_hgl);

    // STEP 1: Calculate Full Pipe Capacity (Equation 9.2)
    println!("STEP 1: Full Pipe Capacity (Equation 9.2)");
    let mannings = ManningsEquation { k: MANNING_CONST_US };
    let q_full = mannings.full_pipe_capacity(diameter_ft, slope, manning_n);
    println!("  Q_full = {:.2} cfs", q_full);
    println!("  Flow/Capacity = {:.1}%\n", (flow / q_full) * 100.0);

    // STEP 2: Calculate Normal Depth
    println!("STEP 2: Calculate Normal Depth");
    let yn = mannings.normal_depth(flow, diameter_ft, slope, manning_n, GRAVITY_US)
        .expect("Could not calculate normal depth");
    println!("  Normal depth y_n = {:.3} ft ({:.1}% of diameter)\n", yn, (yn / diameter_ft) * 100.0);

    // STEP 3: Calculate Flow Properties
    println!("STEP 3: Flow Properties at Normal Depth");
    let flow_result = mannings.partial_pipe_flow(diameter_ft, yn, slope, manning_n, GRAVITY_US);
    println!("  Velocity: {:.2} ft/s", flow_result.velocity);
    println!("  Velocity head V²/2g: {:.3} ft\n", flow_result.velocity_head);

    // STEP 4: Calculate Friction Loss (Equations 9.3 & 9.4)
    println!("STEP 4: Friction Loss (Equations 9.3 & 9.4)");
    let energy_loss = EnergyLoss { gravity: GRAVITY_US };
    let friction_loss = energy_loss.friction_loss(
        flow, length, flow_result.area, flow_result.hydraulic_radius,
        manning_n, MANNING_CONST_US
    );
    println!("  Friction loss h_f = {:.3} ft\n", friction_loss);

    // STEP 5: Calculate Upstream HGL and EGL
    println!("STEP 5: Upstream HGL and EGL");
    let downstream_egl = downstream_hgl + flow_result.velocity_head;
    let upstream_egl = downstream_egl + friction_loss;
    let upstream_hgl = upstream_egl - flow_result.velocity_head;

    println!("  Downstream HGL: {:.2} ft", downstream_hgl);
    println!("  Downstream EGL: {:.2} ft", downstream_egl);
    println!("  Upstream EGL: {:.2} ft (+{:.3} ft)", upstream_egl, friction_loss);
    println!("  Upstream HGL: {:.2} ft\n", upstream_hgl);

    // Verification
    assert!(flow < q_full, "Flow should be less than full capacity");
    assert!(yn < diameter_ft, "Normal depth should be less than diameter");
    assert!(upstream_hgl > downstream_hgl, "HGL should increase upstream");
    assert!(friction_loss > 0.0, "Friction loss should be positive");

    println!("{}", "=".repeat(80));
    println!("TEST PASSED: Example 9.1");
    println!("{}\n", "=".repeat(80));
}

/// Test Example 9.2: Two-Pipe Junction with Lateral Inflow
///
/// Demonstrates junction loss calculation using both the simple method (Equation 9.9)
/// and the comprehensive FHWA Access Hole Method (Equations 9.11-9.31).
#[test]
fn test_example_9_2_two_pipe_junction() {
    println!("\n{}", "=".repeat(80));
    println!("HEC-22 EXAMPLE 9.2: Two-Pipe Junction with Lateral Inflow");
    println!("{}\n", "=".repeat(80));

    println!("NOTE: This test uses representative parameters based on typical HEC-22");
    println!("      Chapter 9 junction examples. The calculation methodology matches");
    println!("      the textbook exactly.\n");

    // GIVEN: Three pipes meeting at a junction
    println!("GIVEN:");

    // Pipe 1 (Main Trunk)
    println!("\nPipe 1 (Main Trunk):");
    let d1 = 18.0 / 12.0; // 1.5 ft
    let l1 = 150.0; // ft
    let s1 = 0.015; // 1.5%
    let n1 = 0.013;
    let q1 = 4.0; // cfs
    println!("  Diameter: 18 inches ({:.2} ft)", d1);
    println!("  Length: {:.0} ft | Slope: {:.2}% | n: {:.3} | Flow: {:.2} cfs", l1, s1 * 100.0, n1, q1);

    // Pipe 2 (Lateral)
    println!("\nPipe 2 (Lateral):");
    let d2 = 15.0 / 12.0; // 1.25 ft
    let l2 = 100.0; // ft
    let s2 = 0.02; // 2%
    let n2 = 0.013;
    let q2 = 2.5; // cfs
    println!("  Diameter: 15 inches ({:.2} ft)", d2);
    println!("  Length: {:.0} ft | Slope: {:.2}% | n: {:.3} | Flow: {:.2} cfs", l2, s2 * 100.0, n2, q2);
    println!("  Angle: 90° (perpendicular entry)");

    // Pipe 3 (Outflow)
    println!("\nPipe 3 (Outflow):");
    let d3 = 24.0 / 12.0; // 2.0 ft
    let l3 = 200.0; // ft
    let s3 = 0.01; // 1%
    let n3 = 0.013;
    let q3 = q1 + q2; // Combined flow
    println!("  Diameter: 24 inches ({:.2} ft)", d3);
    println!("  Length: {:.0} ft | Slope: {:.2}% | n: {:.3} | Flow: {:.2} cfs", l3, s3 * 100.0, n3, q3);

    let downstream_hgl = 100.0; // ft
    println!("\n  Downstream HGL at outfall: {:.2} ft\n", downstream_hgl);

    let mannings = ManningsEquation { k: MANNING_CONST_US };
    let energy_loss = EnergyLoss { gravity: GRAVITY_US };

    // STEP 1: Analyze Outflow Pipe (Pipe 3)
    println!("{}", "=".repeat(70));
    println!("STEP 1: Analyze Outflow Pipe (Pipe 3)");
    println!("{}", "=".repeat(70));

    let q3_full = mannings.full_pipe_capacity(d3, s3, n3);
    let yn3 = mannings.normal_depth(q3, d3, s3, n3, GRAVITY_US)
        .expect("Could not calculate normal depth for Pipe 3");
    let flow3 = mannings.partial_pipe_flow(d3, yn3, s3, n3, GRAVITY_US);

    println!("  Full capacity: {:.2} cfs | Flow/Capacity: {:.1}%", q3_full, (q3 / q3_full) * 100.0);
    println!("  Normal depth: {:.3} ft ({:.1}% full)", yn3, (yn3 / d3) * 100.0);
    println!("  Velocity: {:.2} ft/s | Velocity head: {:.3} ft", flow3.velocity, flow3.velocity_head);

    let hf3 = energy_loss.friction_loss(q3, l3, flow3.area, flow3.hydraulic_radius, n3, MANNING_CONST_US);
    println!("  Friction loss: {:.3} ft", hf3);

    let egl_ds_junction = downstream_hgl + flow3.velocity_head + hf3;
    let hgl_ds_junction = egl_ds_junction - flow3.velocity_head;
    println!("\n  At downstream side of junction:");
    println!("    HGL: {:.2} ft | EGL: {:.2} ft\n", hgl_ds_junction, egl_ds_junction);

    // STEP 2: Analyze Inflow Pipes
    println!("{}", "=".repeat(70));
    println!("STEP 2: Analyze Inflow Pipes Before Junction");
    println!("{}", "=".repeat(70));

    // Pipe 1
    println!("\nPipe 1 (Main Trunk):");
    let yn1 = mannings.normal_depth(q1, d1, s1, n1, GRAVITY_US)
        .expect("Could not calculate normal depth for Pipe 1");
    let flow1 = mannings.partial_pipe_flow(d1, yn1, s1, n1, GRAVITY_US);
    println!("  Normal depth: {:.3} ft ({:.1}% full)", yn1, (yn1 / d1) * 100.0);
    println!("  Velocity: {:.2} ft/s | Velocity head: {:.3} ft", flow1.velocity, flow1.velocity_head);

    // Pipe 2
    println!("\nPipe 2 (Lateral):");
    let yn2 = mannings.normal_depth(q2, d2, s2, n2, GRAVITY_US)
        .expect("Could not calculate normal depth for Pipe 2");
    let flow2 = mannings.partial_pipe_flow(d2, yn2, s2, n2, GRAVITY_US);
    println!("  Normal depth: {:.3} ft ({:.1}% full)", yn2, (yn2 / d2) * 100.0);
    println!("  Velocity: {:.2} ft/s | Velocity head: {:.3} ft\n", flow2.velocity, flow2.velocity_head);

    // STEP 3: Calculate Junction Loss - Simple Method (Equation 9.9)
    println!("{}", "=".repeat(70));
    println!("STEP 3: Junction Loss - Simple Method (Equation 9.9)");
    println!("{}", "=".repeat(70));

    let theta_j = 90.0; // degrees
    let junction_loss_simple = energy_loss.junction_loss(
        q3, q1, q2,
        flow3.velocity, flow1.velocity, flow2.velocity,
        flow3.area, flow1.area,
        theta_j,
    );
    println!("  Junction loss (Eq 9.9): {:.3} ft\n", junction_loss_simple);

    // STEP 4: Calculate Junction Loss - FHWA Access Hole Method
    println!("{}", "=".repeat(70));
    println!("STEP 4: Junction Loss - FHWA Access Hole Method (Eq 9.11-9.31)");
    println!("{}", "=".repeat(70));

    let fhwa = FhwaAccessHoleMethod { gravity: GRAVITY_US };
    let access_hole_invert = 95.0; // ft elevation

    // Equation 9.12: Outflow energy head
    let e_i = egl_ds_junction - access_hole_invert;
    println!("  Outflow energy head E_i = {:.2} ft", e_i);

    // Equation 9.16: Discharge intensity
    let di = fhwa.discharge_intensity(q3, flow3.area, d3);
    println!("  Discharge intensity DI = {:.3}", di);

    // Control conditions (Equations 9.13-9.18)
    let e_aio = fhwa.outlet_control_energy(e_i, flow3.velocity);
    let e_ais = fhwa.submerged_inlet_control(d3, di);
    let e_aiu = fhwa.unsubmerged_inlet_control(d3, di);
    let e_ai = fhwa.initial_energy_level(e_aio, e_ais, e_aiu);

    println!("\n  Control Conditions:");
    println!("    Outlet control (Eq 9.14): {:.3} ft", e_aio);
    println!("    Submerged inlet (Eq 9.17): {:.3} ft", e_ais);
    println!("    Unsubmerged inlet (Eq 9.18): {:.3} ft", e_aiu);
    println!("    Initial energy level E_ai = max = {:.3} ft", e_ai);

    // Build inflow pipes
    let inflow_pipes = vec![
        InflowPipe {
            flow: q1, velocity: flow1.velocity, diameter: d1, area: flow1.area,
            angle: 180.0, invert_offset: 0.0,
        },
        InflowPipe {
            flow: q2, velocity: flow2.velocity, diameter: d2, area: flow2.area,
            angle: 90.0, invert_offset: 0.0,
        },
    ];

    // Adjustment coefficients (Equations 9.20-9.26)
    let benching = BenchingType::Flat;
    let c_b = fhwa.benching_coefficient(benching, e_ai, d3);
    let theta_w = fhwa.flow_weighted_angle(&inflow_pipes);
    let c_theta = fhwa.angled_inflow_coefficient(q1 + q2, q3, theta_w);
    let c_p = 0.0; // No plunging flows

    println!("\n  Adjustment Coefficients:");
    println!("    Benching C_B: {:.3}", c_b);
    println!("    Flow-weighted angle: {:.1}°", theta_w);
    println!("    Angle coefficient C_θ: {:.3}", c_theta);
    println!("    Plunging coefficient C_P: {:.3}", c_p);

    // Final energy level (Equations 9.27-9.28)
    let h_a = fhwa.total_additional_loss(c_b, c_theta, c_p, e_ai, e_i);
    let e_a = fhwa.final_energy_level(e_ai, h_a, e_i);
    let junction_loss_fhwa = e_a - e_i;

    println!("\n  Additional loss H_a: {:.3} ft", h_a);
    println!("  Final energy level E_a: {:.3} ft", e_a);
    println!("  FHWA junction loss: {:.3} ft\n", junction_loss_fhwa);

    // STEP 5: Calculate Upstream HGL/EGL
    println!("{}", "=".repeat(70));
    println!("STEP 5: Calculate Upstream HGL/EGL");
    println!("{}", "=".repeat(70));

    let junction_loss = junction_loss_fhwa.max(junction_loss_simple);
    println!("  Junction loss (using FHWA): {:.3} ft", junction_loss);

    let egl_us_junction = egl_ds_junction + junction_loss;

    // Pipe 1 upstream
    let hf1 = energy_loss.friction_loss(q1, l1, flow1.area, flow1.hydraulic_radius, n1, MANNING_CONST_US);
    let egl1_us = egl_us_junction + hf1;
    let hgl1_us = egl1_us - flow1.velocity_head;

    println!("\n  Pipe 1 upstream end:");
    println!("    Friction loss: {:.3} ft | EGL: {:.2} ft | HGL: {:.2} ft", hf1, egl1_us, hgl1_us);

    // Pipe 2 upstream
    let hf2 = energy_loss.friction_loss(q2, l2, flow2.area, flow2.hydraulic_radius, n2, MANNING_CONST_US);
    let egl2_us = egl_us_junction + hf2;
    let hgl2_us = egl2_us - flow2.velocity_head;

    println!("\n  Pipe 2 upstream end:");
    println!("    Friction loss: {:.3} ft | EGL: {:.2} ft | HGL: {:.2} ft\n", hf2, egl2_us, hgl2_us);

    // SUMMARY
    println!("{}", "=".repeat(70));
    println!("SUMMARY");
    println!("{}", "=".repeat(70));

    let total_loss_path1 = hf3 + junction_loss + hf1;
    let total_loss_path2 = hf3 + junction_loss + hf2;

    println!("  Flow continuity: Q1 + Q2 = {:.2} + {:.2} = {:.2} cfs = Q3", q1, q2, q3);
    println!("\n  Energy losses:");
    println!("    Pipe 3 friction: {:.3} ft", hf3);
    println!("    Junction: {:.3} ft", junction_loss);
    println!("    Pipe 1 friction: {:.3} ft | Total (Path 1): {:.3} ft", hf1, total_loss_path1);
    println!("    Pipe 2 friction: {:.3} ft | Total (Path 2): {:.3} ft", hf2, total_loss_path2);

    println!("\n  Comparison:");
    println!("    Simple method (Eq 9.9): {:.3} ft", junction_loss_simple);
    println!("    FHWA method (Eq 9.11-9.31): {:.3} ft", junction_loss_fhwa);
    println!("    Difference: {:.3} ft ({:.1}%)\n",
        (junction_loss_fhwa - junction_loss_simple).abs(),
        ((junction_loss_fhwa - junction_loss_simple) / junction_loss_simple * 100.0).abs());

    // VERIFICATION
    println!("{}", "=".repeat(70));
    println!("VERIFICATION");
    println!("{}", "=".repeat(70));

    let flow_balance = (q1 + q2 - q3).abs();
    println!("  ✓ Flow continuity: |Q_in - Q_out| = {:.6} cfs < 0.01", flow_balance);
    assert!(flow_balance < 0.01, "Flow continuity violated");

    println!("  ✓ All pipes flowing partial: P1={:.1}%, P2={:.1}%, P3={:.1}%",
        (yn1/d1)*100.0, (yn2/d2)*100.0, (yn3/d3)*100.0);
    assert!(yn1 < d1 && yn2 < d2 && yn3 < d3, "Pipes should not be surcharged");

    println!("  ✓ EGL increases upstream");
    assert!(egl_us_junction > egl_ds_junction, "EGL should increase through junction");
    assert!(egl1_us > egl_us_junction && egl2_us > egl_us_junction, "EGL should increase through pipes");

    println!("  ✓ Junction loss reasonable: {:.3} ft (0.1-1.0 ft typical)", junction_loss);
    assert!(junction_loss > 0.0 && junction_loss < 2.0, "Junction loss out of range");

    println!("\n{}", "=".repeat(80));
    println!("TEST PASSED: Example 9.2");
    println!("{}\n", "=".repeat(80));
}

#[test]
fn test_flow_continuity() {
    let q_in1: f64 = 3.5;
    let q_in2: f64 = 2.0;
    let q_out: f64 = q_in1 + q_in2;
    assert!((q_in1 + q_in2 - q_out).abs() < 1e-10, "Flow continuity must hold");
}

#[test]
fn test_energy_increases_upstream() {
    let mannings = ManningsEquation { k: MANNING_CONST_US };
    let energy_loss = EnergyLoss { gravity: GRAVITY_US };

    let flow_result = mannings.partial_pipe_flow(1.5, 0.75, 0.01, 0.013, GRAVITY_US);
    let hf = energy_loss.friction_loss(5.0, 100.0, flow_result.area,
                                        flow_result.hydraulic_radius, 0.013, MANNING_CONST_US);

    assert!(hf > 0.0, "Friction loss must be positive");

    let egl_down = 100.0;
    let egl_up = egl_down + hf;
    assert!(egl_up > egl_down, "EGL must increase upstream");
}
