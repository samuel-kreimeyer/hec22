//! Chapter 10-12 reference tests from docs/reference/TEST_CASE_REFERENCE.md

use hec22::{pump, quality, storage};

#[test]
fn test_case_8_1_preliminary_detention_basin_sizing() {
    // Test Case 8.1 inputs
    let tc_minutes = 20.0;
    let post_development_peak = 131.0;
    let target_release = 50.0;

    // Step 1 expected from reference: 97,200 ft^3
    let preliminary_storage = storage::triangular_hydrograph_storage_volume(
        tc_minutes,
        post_development_peak,
        target_release,
    );
    assert!((preliminary_storage - 97_200.0).abs() < 1e-6);

    // Solve trapezoidal volume equation using stated geometry.
    // With the stated equation inputs, computed depth is about 8.41 ft.
    let depth =
        storage::solve_trapezoidal_depth_for_volume(preliminary_storage, 100.0, 50.0, 4.0).unwrap();
    assert!((depth - 8.4112).abs() < 0.01);
}

#[test]
fn test_case_9_1_bioretention_basin_design() {
    // Test Case 9.1 inputs
    let area_acres = 10.0;
    let impervious_percent = 75.0;
    let design_rainfall_inches = 1.0;
    let annual_rainfall_inches = 40.0;
    let pj = 0.9;
    let tss_concentration = 100.0;
    let removal_efficiency = 0.85;

    // Expected: Rv = 0.725
    let rv = quality::volumetric_runoff_coefficient(impervious_percent);
    assert!((rv - 0.725).abs() < 1e-6);

    // Expected: WQv = 26,318 ft^3
    let wqv = quality::water_quality_volume(design_rainfall_inches, area_acres, impervious_percent);
    assert!((wqv - 26_318.0).abs() < 1.0);

    // Expected: annual runoff depth = 26.1 inches
    let annual_runoff =
        quality::annual_runoff_depth(annual_rainfall_inches, pj, impervious_percent);
    assert!((annual_runoff - 26.1).abs() < 1e-6);

    // Expected: annual load = 5,899 lb/yr
    let annual_load = quality::simple_method_annual_load(
        quality::PollutantType::Chemical,
        annual_runoff,
        tss_concentration,
        area_acres,
    );
    assert!((annual_load - 5_899.0).abs() < 1.0);

    // Expected: bioretention area = 9,570 ft^2
    let bio_area = quality::bioretention_surface_area(wqv, 0.25, 3.0, 2.0, 24.0).unwrap();
    assert!((bio_area - 9_570.0).abs() < 2.0);

    // Expected: removed load = 5,014 lb/yr
    let removed = quality::removed_pollutant_load(annual_load, removal_efficiency);
    assert!((removed - 5_014.0).abs() < 2.0);
}

#[test]
fn test_case_10_1_pump_station_design() {
    // Test Case 10.1 inputs
    let q_cfs = 10.0;
    let static_head = 15.0;
    let diameter_ft = 1.0; // 12 in
    let length_ft = 200.0;
    let manning_n = 0.011;
    let k_total = 5.5;
    let pump_efficiency = 0.75;

    // Expected: area = 0.785 ft^2, velocity = 12.7 ft/s
    let area = pump::circular_pipe_area(diameter_ft);
    assert!((area - 0.785).abs() < 0.001);
    let velocity = pump::flow_velocity(q_cfs, area);
    assert!((velocity - 12.7).abs() < 0.1);

    // Expected: Hf = 4.96 ft
    let hydraulic_radius = diameter_ft / 4.0;
    let hf = pump::manning_friction_loss(manning_n, length_ft, velocity, hydraulic_radius);
    assert!((hf - 4.96).abs() < 0.1);

    // Expected: Hv = 2.5 ft, Hl = 13.8 ft
    let hv = pump::velocity_head(velocity, pump::GRAVITY_FTPS2);
    assert!((hv - 2.5).abs() < 0.1);
    let hl = pump::minor_losses_head(&[k_total], velocity, pump::GRAVITY_FTPS2);
    assert!((hl - 13.8).abs() < 0.2);

    // Expected: TDH = 36.3 ft
    let tdh = pump::total_dynamic_head(pump::HeadComponents {
        static_head,
        friction_head: hf,
        velocity_head: hv,
        minor_head: hl,
    });
    assert!((tdh - 36.3).abs() < 0.2);

    // Expected: WHP = 41.3 hp, BHP = 55 hp, motor = 60 hp
    let whp = pump::water_horsepower(q_cfs, tdh);
    assert!((whp - 41.3).abs() < 0.2);
    let bhp = pump::brake_horsepower(whp, pump_efficiency).unwrap();
    assert!((bhp - 55.0).abs() < 0.3);
    let motor_hp = pump::next_standard_motor_hp(bhp).unwrap();
    assert_eq!(motor_hp, 60.0);
}
