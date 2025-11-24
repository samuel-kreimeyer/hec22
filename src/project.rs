//! Project metadata and unit definitions

use serde::{Deserialize, Serialize};

/// Project metadata and settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    /// Project name
    pub name: String,

    /// Project description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Geographic location (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// Unit system for the project
    pub units: Units,

    /// Project author (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,

    /// Creation timestamp (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,

    /// Last modified timestamp (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified: Option<String>,
}

/// Geographic location information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Location {
    /// Latitude in decimal degrees (-90 to 90)
    pub latitude: f64,

    /// Longitude in decimal degrees (-180 to 180)
    pub longitude: f64,

    /// Vertical datum (e.g., "NAVD88", "NGVD29")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datum: Option<String>,
}

/// Unit system definitions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Units {
    /// Overall unit system (US customary or SI metric)
    pub system: UnitSystem,

    /// Length units (optional, inferred from system if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<LengthUnit>,

    /// Elevation units (optional, inferred from system if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<LengthUnit>,

    /// Flow rate units (optional, inferred from system if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<FlowUnit>,

    /// Area units (optional, inferred from system if not specified)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<AreaUnit>,
}

/// Unit system (US customary or SI metric)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum UnitSystem {
    /// US customary units (feet, inches, cfs, acres)
    US,
    /// SI metric units (meters, millimeters, cms, hectares)
    SI,
}

/// Length and elevation units
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LengthUnit {
    /// Feet
    #[serde(rename = "ft")]
    Feet,
    /// Meters
    #[serde(rename = "m")]
    Meters,
    /// Inches
    #[serde(rename = "in")]
    Inches,
    /// Millimeters
    #[serde(rename = "mm")]
    Millimeters,
}

/// Flow rate units
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FlowUnit {
    /// Cubic feet per second
    #[serde(rename = "cfs")]
    Cfs,
    /// Cubic meters per second
    #[serde(rename = "cms")]
    Cms,
    /// Gallons per minute
    #[serde(rename = "gpm")]
    Gpm,
    /// Liters per second
    #[serde(rename = "lps")]
    Lps,
}

/// Area units
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AreaUnit {
    /// Acres
    Acres,
    /// Hectares
    #[serde(rename = "ha")]
    Hectares,
    /// Square feet
    #[serde(rename = "sqft")]
    SquareFeet,
    /// Square meters
    #[serde(rename = "sqm")]
    SquareMeters,
}

impl Units {
    /// Create a US customary unit system with standard units
    pub fn us_customary() -> Self {
        Self {
            system: UnitSystem::US,
            length: Some(LengthUnit::Feet),
            elevation: Some(LengthUnit::Feet),
            flow: Some(FlowUnit::Cfs),
            area: Some(AreaUnit::Acres),
        }
    }

    /// Create an SI metric unit system with standard units
    pub fn si_metric() -> Self {
        Self {
            system: UnitSystem::SI,
            length: Some(LengthUnit::Meters),
            elevation: Some(LengthUnit::Meters),
            flow: Some(FlowUnit::Cms),
            area: Some(AreaUnit::Hectares),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_us_customary_units() {
        let units = Units::us_customary();
        assert_eq!(units.system, UnitSystem::US);
        assert_eq!(units.length, Some(LengthUnit::Feet));
        assert_eq!(units.flow, Some(FlowUnit::Cfs));
    }

    #[test]
    fn test_si_metric_units() {
        let units = Units::si_metric();
        assert_eq!(units.system, UnitSystem::SI);
        assert_eq!(units.length, Some(LengthUnit::Meters));
        assert_eq!(units.flow, Some(FlowUnit::Cms));
    }
}
