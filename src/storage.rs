//! Chapter 10 detention and retention utilities.
//!
//! This module provides core equations for storage sizing, outlet hydraulics,
//! stage-storage/stage-discharge relationships, and Modified Puls routing.

use serde::{Deserialize, Serialize};

/// US unit conversion factor from acre-inches to cubic feet.
pub const ACRE_INCH_TO_CUBIC_FEET: f64 = 3630.0;

/// Representation of a detention or retention facility.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageFacility {
    /// Unique facility identifier.
    pub id: String,
    /// Descriptive name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Facility type.
    #[serde(rename = "type")]
    pub facility_type: StorageType,
    /// Upstream drainage area IDs routed to this facility.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drainageAreaIds")]
    pub drainage_area_ids: Option<Vec<String>>,
    /// Required storage volume (ft^3 or m^3).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "requiredVolume")]
    pub required_volume: Option<f64>,
    /// Maximum design stage above basin bottom (ft or m).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxStage")]
    pub max_stage: Option<f64>,
    /// Basin geometry metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<StorageGeometry>,
    /// Optional outflow structure metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlet: Option<OutletStructure>,
}

/// Storage facility type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum StorageType {
    /// Temporary storage with controlled release.
    Detention,
    /// Permanent storage volume.
    Retention,
}

/// Simplified basin geometry metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StorageGeometry {
    /// Geometry family for quick sizing equations.
    #[serde(rename = "shape")]
    pub shape: StorageGeometryType,
    /// Bottom length (ft or m).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bottomLength")]
    pub bottom_length: Option<f64>,
    /// Bottom width (ft or m).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bottomWidth")]
    pub bottom_width: Option<f64>,
    /// Side slope as H:V.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sideSlope")]
    pub side_slope: Option<f64>,
}

/// Geometry family for storage sizing equations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum StorageGeometryType {
    /// Horizontal-bottom rectangle.
    Rectangular,
    /// Trapezoidal side slopes.
    Trapezoidal,
    /// Non-analytical area-elevation table.
    Irregular,
}

/// Outlet metadata for storage discharge control.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutletStructure {
    /// Primary outlet element type.
    #[serde(rename = "type")]
    pub outlet_type: OutletType,
    /// Discharge coefficient.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dischargeCoefficient")]
    pub discharge_coefficient: Option<f64>,
    /// Opening area for orifices (ft^2 or m^2).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "openingArea")]
    pub opening_area: Option<f64>,
    /// Crest or throat length for weirs (ft or m).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
}

/// Outlet structure type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OutletType {
    /// Orifice-controlled outflow.
    Orifice,
    /// Weir-controlled outflow.
    Weir,
}

/// Stage-area point for irregular basin calculations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct StageAreaPoint {
    /// Water surface elevation or depth.
    pub stage: f64,
    /// Surface area at this stage.
    pub area: f64,
}

/// Stage-storage point for routing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct StageStoragePoint {
    /// Water surface elevation or depth.
    pub stage: f64,
    /// Cumulative storage volume at this stage.
    pub storage: f64,
}

/// Stage-discharge point for routing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct StageDischargePoint {
    /// Water surface elevation or depth.
    pub stage: f64,
    /// Outflow at this stage.
    pub discharge: f64,
}

/// Inflow hydrograph point.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct HydrographPoint {
    /// Time in seconds.
    #[serde(rename = "timeSeconds")]
    pub time_seconds: f64,
    /// Inflow rate at this time.
    pub inflow: f64,
}

/// Routed hydrograph point from Modified Puls method.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct RoutedHydrographPoint {
    /// Time in seconds.
    #[serde(rename = "timeSeconds")]
    pub time_seconds: f64,
    /// Inflow rate at this time.
    pub inflow: f64,
    /// Outflow rate at this time.
    pub outflow: f64,
    /// Storage volume at this time.
    pub storage: f64,
    /// Stage at this time.
    pub stage: f64,
}

/// Stage-storage and stage-discharge data for routing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StageStorageDischargeTable {
    /// Stage-storage curve.
    #[serde(rename = "stageStorage")]
    pub stage_storage: Vec<StageStoragePoint>,
    /// Stage-discharge curve.
    #[serde(rename = "stageDischarge")]
    pub stage_discharge: Vec<StageDischargePoint>,
}

impl StageStorageDischargeTable {
    /// Construct a routing table with basic validation.
    pub fn new(
        stage_storage: Vec<StageStoragePoint>,
        stage_discharge: Vec<StageDischargePoint>,
    ) -> Result<Self, String> {
        if stage_storage.len() < 2 {
            return Err("Stage-storage curve must have at least 2 points".to_string());
        }
        if stage_discharge.len() < 2 {
            return Err("Stage-discharge curve must have at least 2 points".to_string());
        }
        validate_monotonic_curve(
            &stage_storage
                .iter()
                .map(|p| (p.stage, p.storage))
                .collect::<Vec<_>>(),
            "stage-storage",
            true,
        )?;
        validate_monotonic_curve(
            &stage_discharge
                .iter()
                .map(|p| (p.stage, p.discharge))
                .collect::<Vec<_>>(),
            "stage-discharge",
            true,
        )?;

        Ok(Self {
            stage_storage,
            stage_discharge,
        })
    }

    /// Interpolate storage at a stage.
    pub fn storage_at_stage(&self, stage: f64) -> Option<f64> {
        interpolate_by_stage(
            &self
                .stage_storage
                .iter()
                .map(|p| (p.stage, p.storage))
                .collect::<Vec<_>>(),
            stage,
        )
    }

    /// Interpolate discharge at a stage.
    pub fn discharge_at_stage(&self, stage: f64) -> Option<f64> {
        interpolate_by_stage(
            &self
                .stage_discharge
                .iter()
                .map(|p| (p.stage, p.discharge))
                .collect::<Vec<_>>(),
            stage,
        )
    }

    /// Compute Modified Puls routing factor at a stage: 2S/dt + O.
    pub fn routing_factor_at_stage(&self, stage: f64, delta_t_seconds: f64) -> Option<f64> {
        if delta_t_seconds <= 0.0 {
            return None;
        }
        let storage = self.storage_at_stage(stage)?;
        let outflow = self.discharge_at_stage(stage)?;
        Some((2.0 * storage / delta_t_seconds) + outflow)
    }

    /// Find stage corresponding to a routing factor target.
    ///
    /// Uses interpolation in factor-space after evaluating all unique stage breakpoints.
    pub fn stage_from_routing_factor(&self, target: f64, delta_t_seconds: f64) -> Option<f64> {
        if delta_t_seconds <= 0.0 {
            return None;
        }

        let mut stages: Vec<f64> = self.stage_storage.iter().map(|p| p.stage).collect();
        stages.extend(self.stage_discharge.iter().map(|p| p.stage));
        stages.sort_by(|a, b| a.total_cmp(b));
        stages.dedup_by(|a, b| (*a - *b).abs() < 1e-12);

        if stages.len() < 2 {
            return None;
        }

        let mut factor_stage: Vec<(f64, f64)> = Vec::with_capacity(stages.len());
        for stage in stages {
            let factor = self.routing_factor_at_stage(stage, delta_t_seconds)?;
            factor_stage.push((factor, stage));
        }

        factor_stage.sort_by(|a, b| a.0.total_cmp(&b.0));
        interpolate_inverse(&factor_stage, target)
    }
}

/// Loss-of-natural-storage depth, clamped at zero.
pub fn required_storage_depth(post_development_depth: f64, pre_development_depth: f64) -> f64 {
    (post_development_depth - pre_development_depth).max(0.0)
}

/// Loss-of-natural-storage volume estimate.
pub fn loss_of_natural_storage_volume(
    area_acres: f64,
    post_development_depth: f64,
    pre_development_depth: f64,
) -> f64 {
    if area_acres <= 0.0 {
        return 0.0;
    }

    let storage_depth = required_storage_depth(post_development_depth, pre_development_depth);
    ACRE_INCH_TO_CUBIC_FEET * area_acres * storage_depth
}

/// Triangular hydrograph storage estimate (Equation 10.4 style).
pub fn triangular_hydrograph_storage_volume(
    time_of_concentration_minutes: f64,
    peak_inflow_cfs: f64,
    target_outflow_cfs: f64,
) -> f64 {
    if time_of_concentration_minutes <= 0.0 || peak_inflow_cfs <= target_outflow_cfs {
        return 0.0;
    }

    let inflow_duration_seconds = 2.0 * time_of_concentration_minutes * 60.0;
    0.5 * inflow_duration_seconds * (peak_inflow_cfs - target_outflow_cfs)
}

/// Rectangular basin volume (horizontal bottom).
pub fn rectangular_basin_volume(length: f64, width: f64, depth: f64) -> f64 {
    if length <= 0.0 || width <= 0.0 || depth <= 0.0 {
        return 0.0;
    }
    length * width * depth
}

/// Trapezoidal basin volume with side slope `z` in H:V.
pub fn trapezoidal_basin_volume(
    bottom_length: f64,
    bottom_width: f64,
    depth: f64,
    side_slope: f64,
) -> f64 {
    if bottom_length <= 0.0 || bottom_width <= 0.0 || depth <= 0.0 || side_slope < 0.0 {
        return 0.0;
    }

    (bottom_length * bottom_width * depth)
        + ((bottom_length + bottom_width) * side_slope * depth.powi(2))
        + ((4.0 / 3.0) * side_slope.powi(2) * depth.powi(3))
}

/// Solve for trapezoidal basin depth for a target volume using bisection.
pub fn solve_trapezoidal_depth_for_volume(
    required_volume: f64,
    bottom_length: f64,
    bottom_width: f64,
    side_slope: f64,
) -> Option<f64> {
    if required_volume < 0.0 || bottom_length <= 0.0 || bottom_width <= 0.0 || side_slope < 0.0 {
        return None;
    }
    if required_volume == 0.0 {
        return Some(0.0);
    }

    let mut lo = 0.0;
    let mut hi = 1.0;
    while trapezoidal_basin_volume(bottom_length, bottom_width, hi, side_slope) < required_volume {
        hi *= 2.0;
        if hi > 1e5 {
            return None;
        }
    }

    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        let volume_mid = trapezoidal_basin_volume(bottom_length, bottom_width, mid, side_slope);
        if volume_mid < required_volume {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    Some(0.5 * (lo + hi))
}

/// Incremental storage from average-end area method.
pub fn average_end_area_increment(area_1: f64, area_2: f64, delta_h: f64) -> f64 {
    if area_1 < 0.0 || area_2 < 0.0 || delta_h <= 0.0 {
        return 0.0;
    }
    ((area_1 + area_2) / 2.0) * delta_h
}

/// Incremental storage from conic section method.
pub fn conic_section_increment(area_1: f64, area_2: f64, delta_h: f64) -> f64 {
    if area_1 < 0.0 || area_2 < 0.0 || delta_h <= 0.0 {
        return 0.0;
    }
    (delta_h / 3.0) * (area_1 + (area_1 * area_2).sqrt() + area_2)
}

/// Build a stage-storage curve from stage-area points using average-end-area.
pub fn stage_storage_from_stage_area_average_end_area(
    stage_area: &[StageAreaPoint],
) -> Result<Vec<StageStoragePoint>, String> {
    if stage_area.len() < 2 {
        return Err("Stage-area table must have at least 2 points".to_string());
    }

    let pairs: Vec<(f64, f64)> = stage_area.iter().map(|p| (p.stage, p.area)).collect();
    validate_monotonic_curve(&pairs, "stage-area", false)?;

    let mut storage_curve = Vec::with_capacity(stage_area.len());
    let mut cumulative_storage = 0.0;
    storage_curve.push(StageStoragePoint {
        stage: stage_area[0].stage,
        storage: 0.0,
    });

    for i in 1..stage_area.len() {
        let delta_h = stage_area[i].stage - stage_area[i - 1].stage;
        cumulative_storage +=
            average_end_area_increment(stage_area[i - 1].area, stage_area[i].area, delta_h);
        storage_curve.push(StageStoragePoint {
            stage: stage_area[i].stage,
            storage: cumulative_storage,
        });
    }

    Ok(storage_curve)
}

/// Build a stage-storage curve from stage-area points using conic section method.
pub fn stage_storage_from_stage_area_conic(
    stage_area: &[StageAreaPoint],
) -> Result<Vec<StageStoragePoint>, String> {
    if stage_area.len() < 2 {
        return Err("Stage-area table must have at least 2 points".to_string());
    }

    let pairs: Vec<(f64, f64)> = stage_area.iter().map(|p| (p.stage, p.area)).collect();
    validate_monotonic_curve(&pairs, "stage-area", false)?;

    let mut storage_curve = Vec::with_capacity(stage_area.len());
    let mut cumulative_storage = 0.0;
    storage_curve.push(StageStoragePoint {
        stage: stage_area[0].stage,
        storage: 0.0,
    });

    for i in 1..stage_area.len() {
        let delta_h = stage_area[i].stage - stage_area[i - 1].stage;
        cumulative_storage +=
            conic_section_increment(stage_area[i - 1].area, stage_area[i].area, delta_h);
        storage_curve.push(StageStoragePoint {
            stage: stage_area[i].stage,
            storage: cumulative_storage,
        });
    }

    Ok(storage_curve)
}

/// Orifice discharge.
pub fn orifice_discharge(
    discharge_coefficient: f64,
    opening_area: f64,
    head: f64,
    gravity: f64,
) -> f64 {
    if discharge_coefficient <= 0.0 || opening_area <= 0.0 || head <= 0.0 || gravity <= 0.0 {
        return 0.0;
    }
    discharge_coefficient * opening_area * (2.0 * gravity * head).sqrt()
}

/// Sharp-crested weir discharge without end contractions.
pub fn sharp_crested_weir_discharge(
    discharge_coefficient: f64,
    weir_length: f64,
    head: f64,
) -> f64 {
    if discharge_coefficient <= 0.0 || weir_length <= 0.0 || head <= 0.0 {
        return 0.0;
    }
    2.96 * discharge_coefficient * weir_length * head.powf(1.5)
}

/// Continuity-based storage change for one Modified Puls-style time step.
pub fn modified_puls_storage_delta(
    inflow_1: f64,
    inflow_2: f64,
    outflow_1: f64,
    outflow_2: f64,
    delta_t_seconds: f64,
) -> f64 {
    if delta_t_seconds <= 0.0 {
        return 0.0;
    }
    (((inflow_1 + inflow_2) / 2.0) - ((outflow_1 + outflow_2) / 2.0)) * delta_t_seconds
}

/// Route an inflow hydrograph through a storage facility using Modified Puls.
pub fn route_modified_puls(
    inflow_hydrograph: &[HydrographPoint],
    table: &StageStorageDischargeTable,
    initial_stage: f64,
) -> Result<Vec<RoutedHydrographPoint>, String> {
    if inflow_hydrograph.is_empty() {
        return Ok(Vec::new());
    }

    let mut routed: Vec<RoutedHydrographPoint> = Vec::with_capacity(inflow_hydrograph.len());
    let stage_1 = initial_stage;
    let mut storage_1 = table
        .storage_at_stage(stage_1)
        .ok_or_else(|| "Could not interpolate initial storage from stage".to_string())?;
    let mut outflow_1 = table
        .discharge_at_stage(stage_1)
        .ok_or_else(|| "Could not interpolate initial outflow from stage".to_string())?;

    routed.push(RoutedHydrographPoint {
        time_seconds: inflow_hydrograph[0].time_seconds,
        inflow: inflow_hydrograph[0].inflow,
        outflow: outflow_1,
        storage: storage_1,
        stage: stage_1,
    });

    for i in 1..inflow_hydrograph.len() {
        let p1 = inflow_hydrograph[i - 1];
        let p2 = inflow_hydrograph[i];
        let delta_t_seconds = p2.time_seconds - p1.time_seconds;
        if delta_t_seconds <= 0.0 {
            return Err("Hydrograph times must be strictly increasing".to_string());
        }

        let rhs = (p1.inflow + p2.inflow) + ((2.0 * storage_1 / delta_t_seconds) - outflow_1);
        let stage_2 = table
            .stage_from_routing_factor(rhs, delta_t_seconds)
            .ok_or_else(|| "Could not solve stage from routing factor".to_string())?;
        let storage_2 = table
            .storage_at_stage(stage_2)
            .ok_or_else(|| "Could not interpolate storage at solved stage".to_string())?;
        let outflow_2 = table
            .discharge_at_stage(stage_2)
            .ok_or_else(|| "Could not interpolate outflow at solved stage".to_string())?;

        routed.push(RoutedHydrographPoint {
            time_seconds: p2.time_seconds,
            inflow: p2.inflow,
            outflow: outflow_2,
            storage: storage_2,
            stage: stage_2,
        });

        storage_1 = storage_2;
        outflow_1 = outflow_2;
    }

    Ok(routed)
}

fn validate_monotonic_curve(
    pairs: &[(f64, f64)],
    name: &str,
    require_non_decreasing_value: bool,
) -> Result<(), String> {
    for i in 1..pairs.len() {
        if pairs[i].0 <= pairs[i - 1].0 {
            return Err(format!("{name} stages must be strictly increasing"));
        }
        if require_non_decreasing_value && pairs[i].1 < pairs[i - 1].1 {
            return Err(format!("{name} values must be non-decreasing"));
        }
    }
    Ok(())
}

fn interpolate_by_stage(points: &[(f64, f64)], stage: f64) -> Option<f64> {
    if points.len() < 2 {
        return None;
    }
    if stage <= points[0].0 {
        return Some(linear_interp(points[0], points[1], stage));
    }
    if stage >= points[points.len() - 1].0 {
        return Some(linear_interp(
            points[points.len() - 2],
            points[points.len() - 1],
            stage,
        ));
    }
    for w in points.windows(2) {
        let p1 = w[0];
        let p2 = w[1];
        if stage >= p1.0 && stage <= p2.0 {
            return Some(linear_interp(p1, p2, stage));
        }
    }
    None
}

fn interpolate_inverse(factor_stage: &[(f64, f64)], target: f64) -> Option<f64> {
    if factor_stage.len() < 2 {
        return None;
    }
    if target <= factor_stage[0].0 {
        return Some(linear_interp(factor_stage[0], factor_stage[1], target));
    }
    if target >= factor_stage[factor_stage.len() - 1].0 {
        return Some(linear_interp(
            factor_stage[factor_stage.len() - 2],
            factor_stage[factor_stage.len() - 1],
            target,
        ));
    }
    for w in factor_stage.windows(2) {
        let p1 = w[0];
        let p2 = w[1];
        if target >= p1.0 && target <= p2.0 {
            return Some(linear_interp(p1, p2, target));
        }
    }
    None
}

fn linear_interp(p1: (f64, f64), p2: (f64, f64), x: f64) -> f64 {
    if (p2.0 - p1.0).abs() < 1e-12 {
        return p1.1;
    }
    p1.1 + ((x - p1.0) * (p2.1 - p1.1) / (p2.0 - p1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_depth_is_clamped() {
        assert!((required_storage_depth(2.2, 1.8) - 0.4).abs() < 1e-12);
        assert_eq!(required_storage_depth(1.0, 1.4), 0.0);
    }

    #[test]
    fn test_loss_of_natural_storage_volume() {
        let volume = loss_of_natural_storage_volume(10.0, 2.0, 1.5);
        assert!((volume - 18150.0).abs() < 1e-6);
    }

    #[test]
    fn test_triangular_hydrograph_volume() {
        let volume = triangular_hydrograph_storage_volume(10.0, 50.0, 20.0);
        assert!((volume - 18000.0).abs() < 1e-6);
    }

    #[test]
    fn test_basin_geometry_volumes() {
        let rect = rectangular_basin_volume(100.0, 50.0, 8.0);
        assert!((rect - 40000.0).abs() < 1e-6);

        let trap = trapezoidal_basin_volume(100.0, 50.0, 8.0, 3.0);
        assert!(trap > rect);
    }

    #[test]
    fn test_solve_trapezoidal_depth() {
        let depth = solve_trapezoidal_depth_for_volume(97200.0, 100.0, 50.0, 4.0).unwrap();
        assert!((depth - 8.4112).abs() < 0.01);
    }

    #[test]
    fn test_increment_methods() {
        let avg = average_end_area_increment(1000.0, 1200.0, 1.0);
        let conic = conic_section_increment(1000.0, 1200.0, 1.0);
        assert!(conic > 0.0);
        assert!((avg - 1100.0).abs() < 1e-6);
    }

    #[test]
    fn test_stage_storage_from_stage_area() {
        let stage_area = vec![
            StageAreaPoint {
                stage: 0.0,
                area: 100.0,
            },
            StageAreaPoint {
                stage: 1.0,
                area: 200.0,
            },
            StageAreaPoint {
                stage: 2.0,
                area: 300.0,
            },
        ];

        let curve_avg = stage_storage_from_stage_area_average_end_area(&stage_area).unwrap();
        assert!((curve_avg[2].storage - 400.0).abs() < 1e-6);

        let curve_conic = stage_storage_from_stage_area_conic(&stage_area).unwrap();
        assert!(curve_conic[2].storage > 0.0);
    }

    #[test]
    fn test_stage_storage_discharge_table_interpolation() {
        let table = StageStorageDischargeTable::new(
            vec![
                StageStoragePoint {
                    stage: 0.0,
                    storage: 0.0,
                },
                StageStoragePoint {
                    stage: 1.0,
                    storage: 100.0,
                },
                StageStoragePoint {
                    stage: 2.0,
                    storage: 250.0,
                },
            ],
            vec![
                StageDischargePoint {
                    stage: 0.0,
                    discharge: 0.0,
                },
                StageDischargePoint {
                    stage: 1.0,
                    discharge: 10.0,
                },
                StageDischargePoint {
                    stage: 2.0,
                    discharge: 30.0,
                },
            ],
        )
        .unwrap();

        assert!((table.storage_at_stage(1.5).unwrap() - 175.0).abs() < 1e-6);
        assert!((table.discharge_at_stage(1.5).unwrap() - 20.0).abs() < 1e-6);
    }

    #[test]
    fn test_outlet_equations() {
        let q_orifice = orifice_discharge(0.6, 1.0, 4.0, 32.2);
        assert!(q_orifice > 0.0);

        let q_weir = sharp_crested_weir_discharge(0.37, 3.0, 1.0);
        assert!((q_weir - 3.2856).abs() < 1e-4);
    }

    #[test]
    fn test_modified_puls_step() {
        let delta_s = modified_puls_storage_delta(20.0, 30.0, 10.0, 15.0, 60.0);
        assert!((delta_s - 750.0).abs() < 1e-6);
    }

    #[test]
    fn test_route_modified_puls_continuity() {
        let table = StageStorageDischargeTable::new(
            vec![
                StageStoragePoint {
                    stage: 0.0,
                    storage: 0.0,
                },
                StageStoragePoint {
                    stage: 1.0,
                    storage: 1000.0,
                },
                StageStoragePoint {
                    stage: 2.0,
                    storage: 2000.0,
                },
                StageStoragePoint {
                    stage: 3.0,
                    storage: 3000.0,
                },
            ],
            vec![
                StageDischargePoint {
                    stage: 0.0,
                    discharge: 0.0,
                },
                StageDischargePoint {
                    stage: 1.0,
                    discharge: 10.0,
                },
                StageDischargePoint {
                    stage: 2.0,
                    discharge: 20.0,
                },
                StageDischargePoint {
                    stage: 3.0,
                    discharge: 30.0,
                },
            ],
        )
        .unwrap();

        let inflow = vec![
            HydrographPoint {
                time_seconds: 0.0,
                inflow: 10.0,
            },
            HydrographPoint {
                time_seconds: 60.0,
                inflow: 20.0,
            },
            HydrographPoint {
                time_seconds: 120.0,
                inflow: 35.0,
            },
            HydrographPoint {
                time_seconds: 180.0,
                inflow: 20.0,
            },
            HydrographPoint {
                time_seconds: 240.0,
                inflow: 5.0,
            },
        ];

        let routed = route_modified_puls(&inflow, &table, 1.0).unwrap();
        assert_eq!(routed.len(), inflow.len());
        assert!(routed.iter().all(|p| p.outflow >= 0.0));
        assert!(routed.iter().all(|p| p.storage >= 0.0));

        for i in 1..routed.len() {
            let dt = routed[i].time_seconds - routed[i - 1].time_seconds;
            let expected_delta_s = modified_puls_storage_delta(
                routed[i - 1].inflow,
                routed[i].inflow,
                routed[i - 1].outflow,
                routed[i].outflow,
                dt,
            );
            let actual_delta_s = routed[i].storage - routed[i - 1].storage;
            assert!((expected_delta_s - actual_delta_s).abs() < 1e-6);
        }
    }
}
