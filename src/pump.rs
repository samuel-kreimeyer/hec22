//! Chapter 12 pump station utilities.
//!
//! This module provides helper equations for head calculations,
//! pump performance checks, and wet well sizing.

use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

/// Gravity constant in US customary units (ft/s^2).
pub const GRAVITY_FTPS2: f64 = 32.2;

/// Metadata for a pump station.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PumpStation {
    /// Unique station identifier.
    pub id: String,
    /// Descriptive name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Connected wet well node ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wetWellNodeId")]
    pub wet_well_node_id: Option<String>,
    /// Number of installed pumps.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pumpCount")]
    pub pump_count: Option<u32>,
    /// Rated discharge per pump (cfs or cms).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ratedFlow")]
    pub rated_flow: Option<f64>,
    /// Rated head (ft or m).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ratedHead")]
    pub rated_head: Option<f64>,
    /// Wet well geometry information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "wetWell")]
    pub wet_well: Option<WetWellGeometry>,
}

/// Wet well geometry metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WetWellGeometry {
    /// Geometry family.
    #[serde(rename = "shape")]
    pub shape: WetWellShape,
    /// Base area for rectangular custom wells (ft^2 or m^2).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseArea")]
    pub base_area: Option<f64>,
    /// Radius for cylindrical wells (ft or m).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

/// Wet well shape family.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WetWellShape {
    /// Cylindrical wet well.
    Cylindrical,
    /// User-provided base area.
    Rectangular,
}

/// Head components that make up total dynamic head.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct HeadComponents {
    /// Static lift between inlet and outlet water surfaces.
    #[serde(rename = "staticHead")]
    pub static_head: f64,
    /// Pipe friction losses.
    #[serde(rename = "frictionHead")]
    pub friction_head: f64,
    /// Velocity head in discharge line.
    #[serde(rename = "velocityHead")]
    pub velocity_head: f64,
    /// Minor losses (fittings, valves, appurtenances).
    #[serde(rename = "minorHead")]
    pub minor_head: f64,
}

/// Total dynamic head from component parts.
pub fn total_dynamic_head(components: HeadComponents) -> f64 {
    components.static_head
        + components.friction_head
        + components.velocity_head
        + components.minor_head
}

/// Static head from water-surface elevation difference.
pub fn static_head(
    discharge_water_surface_elevation: f64,
    suction_water_surface_elevation: f64,
) -> f64 {
    discharge_water_surface_elevation - suction_water_surface_elevation
}

/// Pipe cross-sectional area for a full circular pipe.
pub fn circular_pipe_area(diameter: f64) -> f64 {
    if diameter <= 0.0 {
        return 0.0;
    }
    PI * diameter.powi(2) / 4.0
}

/// Flow velocity from discharge and area.
pub fn flow_velocity(discharge: f64, area: f64) -> f64 {
    if discharge <= 0.0 || area <= 0.0 {
        return 0.0;
    }
    discharge / area
}

/// Darcy-Weisbach friction head loss.
pub fn darcy_weisbach_friction_loss(
    friction_factor: f64,
    pipe_length: f64,
    pipe_diameter: f64,
    velocity: f64,
    gravity: f64,
) -> f64 {
    if friction_factor <= 0.0 || pipe_length <= 0.0 || pipe_diameter <= 0.0 || gravity <= 0.0 {
        return 0.0;
    }

    friction_factor * (pipe_length / pipe_diameter) * (velocity.powi(2) / (2.0 * gravity))
}

/// Friction head loss using HEC-22 Manning form for pressure pipes.
pub fn manning_friction_loss(
    manning_n: f64,
    pipe_length: f64,
    velocity: f64,
    hydraulic_radius: f64,
) -> f64 {
    if manning_n <= 0.0 || pipe_length <= 0.0 || velocity <= 0.0 || hydraulic_radius <= 0.0 {
        return 0.0;
    }
    // Matches the worked-example arithmetic used in the project reference set.
    (manning_n.powi(2) * pipe_length * velocity.powi(2)) / (2.22 * hydraulic_radius.powf(0.75))
}

/// Velocity head in the discharge conduit.
pub fn velocity_head(velocity: f64, gravity: f64) -> f64 {
    if velocity <= 0.0 || gravity <= 0.0 {
        return 0.0;
    }
    velocity.powi(2) / (2.0 * gravity)
}

/// Total minor losses from K values.
pub fn minor_losses_head(loss_coefficients: &[f64], velocity: f64, gravity: f64) -> f64 {
    if loss_coefficients.is_empty() {
        return 0.0;
    }
    let k_sum: f64 = loss_coefficients.iter().copied().sum();
    k_sum * velocity_head(velocity, gravity)
}

/// Pump specific speed in US customary units (rpm, gpm, ft).
pub fn specific_speed(rpm: f64, flow_gpm: f64, head_ft: f64) -> Option<f64> {
    if rpm <= 0.0 || flow_gpm <= 0.0 || head_ft <= 0.0 {
        return None;
    }
    Some((rpm * flow_gpm.sqrt()) / head_ft.powf(0.75))
}

/// Water horsepower from cfs and TDH (ft), using HEC-22 simplified form.
pub fn water_horsepower(flow_cfs: f64, total_dynamic_head_ft: f64) -> f64 {
    if flow_cfs <= 0.0 || total_dynamic_head_ft <= 0.0 {
        return 0.0;
    }
    (flow_cfs * total_dynamic_head_ft) / 8.8
}

/// Brake horsepower from water horsepower and efficiency.
pub fn brake_horsepower(water_horsepower: f64, efficiency: f64) -> Option<f64> {
    if water_horsepower < 0.0 || efficiency <= 0.0 || efficiency > 1.0 {
        return None;
    }
    Some(water_horsepower / efficiency)
}

/// NPSH available.
pub fn npsh_available(
    atmospheric_pressure_head: f64,
    static_suction_head: f64,
    suction_friction_loss: f64,
    vapor_pressure_head: f64,
) -> f64 {
    atmospheric_pressure_head + static_suction_head - suction_friction_loss - vapor_pressure_head
}

/// Minimum cycling storage volume for a single pump.
pub fn min_storage_single_pump(pump_discharge: f64, minimum_cycle_time_seconds: f64) -> f64 {
    if pump_discharge <= 0.0 || minimum_cycle_time_seconds <= 0.0 {
        return 0.0;
    }
    pump_discharge * minimum_cycle_time_seconds
}

/// Minimum cycling storage for two equal pumps with alternation.
pub fn min_storage_two_pump_alternating(
    pump_discharge: f64,
    minimum_cycle_time_seconds: f64,
) -> f64 {
    0.5 * min_storage_single_pump(pump_discharge, minimum_cycle_time_seconds)
}

/// Cylindrical wet well storage volume.
pub fn cylindrical_wet_well_volume(radius: f64, depth: f64) -> f64 {
    if radius <= 0.0 || depth <= 0.0 {
        return 0.0;
    }
    PI * radius.powi(2) * depth
}

/// Storage volume from base area and depth for non-cylindrical wells.
pub fn wet_well_volume_from_area(base_area: f64, depth: f64) -> f64 {
    if base_area <= 0.0 || depth <= 0.0 {
        return 0.0;
    }
    base_area * depth
}

/// Drawdown time in seconds.
pub fn drawdown_time_seconds(stored_volume: f64, pump_discharge: f64) -> Option<f64> {
    if stored_volume < 0.0 || pump_discharge <= 0.0 {
        return None;
    }
    Some(stored_volume / pump_discharge)
}

/// Select the next standard motor horsepower at or above required BHP.
pub fn next_standard_motor_hp(required_bhp: f64) -> Option<f64> {
    if required_bhp <= 0.0 {
        return None;
    }
    const STANDARD_HP: &[f64] = &[
        1.0, 1.5, 2.0, 3.0, 5.0, 7.5, 10.0, 15.0, 20.0, 25.0, 30.0, 40.0, 50.0, 60.0, 75.0, 100.0,
        125.0, 150.0, 200.0, 250.0, 300.0,
    ];
    for hp in STANDARD_HP {
        if *hp >= required_bhp {
            return Some(*hp);
        }
    }
    Some(*STANDARD_HP.last().unwrap_or(&required_bhp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_components_and_tdh() {
        let friction = darcy_weisbach_friction_loss(0.02, 500.0, 1.0, 8.0, GRAVITY_FTPS2);
        let velocity = velocity_head(8.0, GRAVITY_FTPS2);
        let minor = minor_losses_head(&[1.0, 0.9, 2.0], 8.0, GRAVITY_FTPS2);

        let tdh = total_dynamic_head(HeadComponents {
            static_head: 20.0,
            friction_head: friction,
            velocity_head: velocity,
            minor_head: minor,
        });

        assert!(tdh > 20.0);
    }

    #[test]
    fn test_pipe_area_velocity_and_manning_friction() {
        let area = circular_pipe_area(1.0);
        assert!((area - std::f64::consts::FRAC_PI_4).abs() < 1e-6);

        let velocity = flow_velocity(10.0, area);
        assert!((velocity - 12.7324).abs() < 1e-3);

        let hf = manning_friction_loss(0.011, 200.0, velocity, 0.25);
        assert!((hf - 4.96).abs() < 0.1);
    }

    #[test]
    fn test_pump_performance_metrics() {
        let ns = specific_speed(1760.0, 3000.0, 55.0).unwrap();
        assert!(ns > 0.0);

        let whp = water_horsepower(6.0, 50.0);
        assert!((whp - 34.090909).abs() < 1e-5);

        let bhp = brake_horsepower(whp, 0.75).unwrap();
        assert!(bhp > whp);
    }

    #[test]
    fn test_npsh_and_storage_helpers() {
        let npsh_a = npsh_available(34.0, 5.0, 1.0, 0.8);
        assert!((npsh_a - 37.2).abs() < 1e-6);

        let single = min_storage_single_pump(2.5, 600.0);
        let alternating = min_storage_two_pump_alternating(2.5, 600.0);
        assert!((single - 1500.0).abs() < 1e-6);
        assert!((alternating - 750.0).abs() < 1e-6);
    }

    #[test]
    fn test_wet_well_and_drawdown() {
        let volume = cylindrical_wet_well_volume(4.0, 10.0);
        assert!((volume - (160.0 * PI)).abs() < 1e-6);

        let drawdown = drawdown_time_seconds(1000.0, 2.0).unwrap();
        assert!((drawdown - 500.0).abs() < 1e-6);
    }

    #[test]
    fn test_next_standard_motor_hp() {
        assert_eq!(next_standard_motor_hp(55.0), Some(60.0));
        assert_eq!(next_standard_motor_hp(60.0), Some(60.0));
    }
}
