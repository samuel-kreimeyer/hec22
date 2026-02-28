//! Chapter 11 stormwater quality and BMP utilities.
//!
//! This module captures common screening-level calculations for pollutant
//! loading and basic treatment sizing.

use crate::storage::ACRE_INCH_TO_CUBIC_FEET;
use serde::{Deserialize, Serialize};

/// Stormwater BMP metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BmpFacility {
    /// Unique BMP identifier.
    pub id: String,
    /// Descriptive BMP name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// BMP technology.
    #[serde(rename = "type")]
    pub bmp_type: BmpType,
    /// Contributing drainage area IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drainageAreaIds")]
    pub drainage_area_ids: Option<Vec<String>>,
    /// Water quality volume target (ft^3 or m^3).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "waterQualityVolume")]
    pub water_quality_volume: Option<f64>,
    /// Detention/retention time (hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detentionTimeHours")]
    pub detention_time_hours: Option<f64>,
    /// Pollutant removal assumptions by pollutant name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "removalEfficiencies")]
    pub removal_efficiencies: Option<Vec<PollutantRemoval>>,
}

/// BMP classification.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BmpType {
    /// Extended detention basin.
    ExtendedDetention,
    /// Wet pond / retention basin.
    WetPond,
    /// Infiltration trench or gallery.
    Infiltration,
    /// Bioretention / rain garden.
    Bioretention,
    /// Vegetated swale.
    Swale,
    /// Filter strip.
    FilterStrip,
}

/// Pollutant-specific removal metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PollutantRemoval {
    /// Pollutant identifier (e.g., TSS, TP, TN, Zn).
    pub pollutant: String,
    /// Removal fraction from 0.0 to 1.0.
    pub efficiency: f64,
}

/// Pollutant loading mode for Simple Method conversion factor.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PollutantType {
    /// Chemical pollutants with concentration in mg/L.
    Chemical,
    /// Bacteria with concentration in 1000/mL.
    Bacteria,
}

impl PollutantType {
    fn conversion_factor(self) -> f64 {
        match self {
            Self::Chemical => 0.226,
            Self::Bacteria => 1000.0,
        }
    }
}

/// Volumetric runoff coefficient from impervious percent.
pub fn volumetric_runoff_coefficient(impervious_percent: f64) -> f64 {
    let impervious = impervious_percent.clamp(0.0, 100.0);
    0.05 + 0.009 * impervious
}

/// Annual runoff depth estimate for Simple Method.
pub fn annual_runoff_depth(
    annual_precipitation_inches: f64,
    runoff_event_fraction: f64,
    impervious_percent: f64,
) -> f64 {
    if annual_precipitation_inches <= 0.0 || runoff_event_fraction <= 0.0 {
        return 0.0;
    }
    annual_precipitation_inches
        * runoff_event_fraction
        * volumetric_runoff_coefficient(impervious_percent)
}

/// Average annual pollutant loading via the Simple Method.
pub fn simple_method_annual_load(
    pollutant_type: PollutantType,
    annual_runoff_depth_inches: f64,
    concentration: f64,
    area_acres: f64,
) -> f64 {
    if annual_runoff_depth_inches <= 0.0 || concentration <= 0.0 || area_acres <= 0.0 {
        return 0.0;
    }
    pollutant_type.conversion_factor() * annual_runoff_depth_inches * concentration * area_acres
}

/// Water quality volume in cubic feet.
pub fn water_quality_volume(
    design_rainfall_inches: f64,
    area_acres: f64,
    impervious_percent: f64,
) -> f64 {
    if design_rainfall_inches <= 0.0 || area_acres <= 0.0 {
        return 0.0;
    }
    volumetric_runoff_coefficient(impervious_percent)
        * design_rainfall_inches
        * area_acres
        * ACRE_INCH_TO_CUBIC_FEET
}

/// Extended detention basin volume accounting for active storage porosity.
pub fn extended_detention_volume(wqv: f64, porosity: f64) -> Option<f64> {
    if wqv <= 0.0 || !(0.0..1.0).contains(&porosity) {
        return None;
    }
    Some(wqv / (1.0 - porosity))
}

/// Wet pond permanent pool target.
pub fn wet_pond_permanent_pool_volume(wqv: f64) -> f64 {
    if wqv <= 0.0 {
        return 0.0;
    }
    2.0 * wqv
}

/// Infiltration trench storage requirement.
pub fn infiltration_trench_volume(wqv: f64, media_porosity: f64) -> Option<f64> {
    if wqv <= 0.0 || media_porosity <= 0.0 {
        return None;
    }
    Some(wqv / media_porosity)
}

/// Bioretention surface area estimate.
///
/// Uses `A_bio = WQv / [(n*d) + K*(t/24)]` with:
/// - `n` as media porosity,
/// - `d` as media depth (ft),
/// - `K` as infiltration rate (ft/hr),
/// - `t` as drawdown time (hours).
pub fn bioretention_surface_area(
    wqv: f64,
    media_porosity: f64,
    media_depth_ft: f64,
    infiltration_rate_ft_per_hr: f64,
    drawdown_time_hours: f64,
) -> Option<f64> {
    if wqv <= 0.0
        || media_porosity < 0.0
        || media_depth_ft <= 0.0
        || infiltration_rate_ft_per_hr < 0.0
        || drawdown_time_hours < 0.0
    {
        return None;
    }
    let denominator = (media_porosity * media_depth_ft)
        + (infiltration_rate_ft_per_hr * (drawdown_time_hours / 24.0));
    if denominator <= 0.0 {
        return None;
    }
    Some(wqv / denominator)
}

/// First-order outlet concentration estimate.
pub fn first_order_outlet_concentration(
    inlet_concentration: f64,
    removal_rate_ft_per_year: f64,
    detention_time_years: f64,
    average_depth_ft: f64,
) -> Option<f64> {
    if inlet_concentration < 0.0
        || removal_rate_ft_per_year < 0.0
        || detention_time_years < 0.0
        || average_depth_ft <= 0.0
    {
        return None;
    }
    Some(
        inlet_concentration
            * (-removal_rate_ft_per_year * detention_time_years / average_depth_ft).exp(),
    )
}

/// Pollutant mass removed.
pub fn removed_pollutant_load(inlet_load: f64, efficiency: f64) -> f64 {
    if inlet_load <= 0.0 {
        return 0.0;
    }
    inlet_load * efficiency.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volumetric_runoff_coefficient() {
        let rv = volumetric_runoff_coefficient(75.0);
        assert!((rv - 0.725).abs() < 1e-6);
    }

    #[test]
    fn test_annual_runoff_depth() {
        let runoff_depth = annual_runoff_depth(40.0, 0.9, 75.0);
        assert!((runoff_depth - 26.1).abs() < 1e-6);
    }

    #[test]
    fn test_simple_method_chemical_load() {
        let load = simple_method_annual_load(PollutantType::Chemical, 20.0, 78.0, 10.0);
        assert!((load - 3525.6).abs() < 1e-6);
    }

    #[test]
    fn test_water_quality_volume() {
        let wqv = water_quality_volume(1.0, 10.0, 75.0);
        assert!((wqv - 26317.5).abs() < 1e-3);
    }

    #[test]
    fn test_bmp_sizing_helpers() {
        let detention = extended_detention_volume(1000.0, 0.0).unwrap();
        assert!((detention - 1000.0).abs() < 1e-6);

        let wet_pool = wet_pond_permanent_pool_volume(1000.0);
        assert!((wet_pool - 2000.0).abs() < 1e-6);

        let trench = infiltration_trench_volume(1000.0, 0.4).unwrap();
        assert!((trench - 2500.0).abs() < 1e-6);
    }

    #[test]
    fn test_bioretention_surface_area() {
        let area = bioretention_surface_area(26318.0, 0.25, 3.0, 2.0, 24.0).unwrap();
        assert!((area - 9570.18).abs() < 1.0);
    }

    #[test]
    fn test_first_order_removal_and_mass_removed() {
        let c_out = first_order_outlet_concentration(100.0, 1.0, 0.5, 5.0).unwrap();
        assert!(c_out < 100.0);

        let removed = removed_pollutant_load(500.0, 0.65);
        assert!((removed - 325.0).abs() < 1e-6);
    }
}
