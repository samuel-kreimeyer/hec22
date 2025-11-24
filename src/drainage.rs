//! Drainage area and subcatchment definitions
//!
//! Drainage areas represent the contributing areas that generate runoff
//! to specific inlet points in the network.

use serde::{Deserialize, Serialize};

/// Drainage area (subcatchment) definition
///
/// Represents a contributing area that drains to a specific outlet node.
/// Used for hydrologic analysis to compute runoff flows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrainageArea {
    /// Unique drainage area identifier
    pub id: String,

    /// Descriptive name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Drainage area (acres, hectares, etc.)
    pub area: f64,

    /// ID of the outlet node (inlet or junction)
    pub outlet: String,

    /// Land use information (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "landUse")]
    pub land_use: Option<LandUse>,

    /// Rational method runoff coefficient C (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "runoffCoefficient")]
    pub runoff_coefficient: Option<f64>,

    /// Time of concentration (minutes)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeOfConcentration")]
    pub time_of_concentration: Option<f64>,

    /// Breakdown of Tc calculation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tcCalculation")]
    pub tc_calculation: Option<TcCalculation>,

    /// SCS Curve Number for TR-55 method (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "curveNumber")]
    pub curve_number: Option<f64>,

    /// Spatial geometry for GIS integration (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
}

/// Land use information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LandUse {
    /// Primary land use type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<LandUseType>,

    /// Percent impervious area (0-100)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "imperviousPercent")]
    pub impervious_percent: Option<f64>,

    /// Detailed land use breakdown (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition: Option<Vec<LandUseComponent>>,
}

/// Land use type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LandUseType {
    /// Commercial development
    Commercial,
    /// Industrial development
    Industrial,
    /// Residential development
    Residential,
    /// Open space/parks
    #[serde(rename = "Open Space")]
    OpenSpace,
    /// Transportation corridors
    Transportation,
    /// Agricultural land
    Agricultural,
    /// Mixed use
    Mixed,
}

/// Land use composition component
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LandUseComponent {
    /// Land use type
    #[serde(rename = "type")]
    pub land_use_type: String,

    /// Area of this land use type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<f64>,

    /// Percentage of total area (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<f64>,
}

/// Time of concentration calculation breakdown
///
/// Tc = sheet flow + shallow concentrated flow + channel flow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TcCalculation {
    /// Sheet flow component (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sheetFlow")]
    pub sheet_flow: Option<SheetFlow>,

    /// Shallow concentrated flow component (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "shallowConcentrated")]
    pub shallow_concentrated: Option<ShallowConcentratedFlow>,

    /// Channel flow component (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "channelFlow")]
    pub channel_flow: Option<ChannelFlow>,
}

/// Sheet flow component of Tc
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SheetFlow {
    /// Flow length (ft or m)
    pub length: f64,

    /// Average slope (ft/ft or m/m)
    pub slope: f64,

    /// Surface roughness coefficient (Manning's n)
    pub roughness: f64,

    /// Travel time (minutes)
    pub time: f64,
}

/// Shallow concentrated flow component of Tc
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShallowConcentratedFlow {
    /// Flow length (ft or m)
    pub length: f64,

    /// Average slope (ft/ft or m/m)
    pub slope: f64,

    /// Surface type
    #[serde(rename = "surfaceType")]
    pub surface_type: SurfaceType,

    /// Travel time (minutes)
    pub time: f64,
}

/// Surface type for shallow concentrated flow
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SurfaceType {
    /// Paved surface (higher velocity)
    Paved,
    /// Unpaved surface (lower velocity)
    Unpaved,
}

/// Channel flow component of Tc
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelFlow {
    /// Flow length (ft or m)
    pub length: f64,

    /// Average velocity (ft/s or m/s)
    pub velocity: f64,

    /// Travel time (minutes)
    pub time: f64,
}

/// Spatial geometry (GeoJSON-compatible)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Geometry {
    /// Geometry type (e.g., "Polygon")
    #[serde(rename = "type")]
    pub geometry_type: String,

    /// Coordinate array (GeoJSON format)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<serde_json::Value>,
}

impl DrainageArea {
    /// Calculate total time of concentration from components
    pub fn calculate_total_tc(&self) -> Option<f64> {
        self.tc_calculation.as_ref().map(|calc| {
            let sheet = calc.sheet_flow.as_ref().map(|s| s.time).unwrap_or(0.0);
            let shallow = calc
                .shallow_concentrated
                .as_ref()
                .map(|s| s.time)
                .unwrap_or(0.0);
            let channel = calc.channel_flow.as_ref().map(|c| c.time).unwrap_or(0.0);
            sheet + shallow + channel
        })
    }

    /// Calculate runoff using Rational Method: Q = C × i × A
    ///
    /// Returns flow in cfs (or cms if SI units)
    pub fn rational_method_runoff(&self, intensity: f64) -> Option<f64> {
        self.runoff_coefficient
            .map(|c| c * intensity * self.area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tc() {
        let drainage_area = DrainageArea {
            id: "DA-001".to_string(),
            name: Some("Test Area".to_string()),
            area: 1.5,
            outlet: "IN-001".to_string(),
            land_use: None,
            runoff_coefficient: Some(0.85),
            time_of_concentration: None,
            tc_calculation: Some(TcCalculation {
                sheet_flow: Some(SheetFlow {
                    length: 50.0,
                    slope: 0.02,
                    roughness: 0.011,
                    time: 3.0,
                }),
                shallow_concentrated: Some(ShallowConcentratedFlow {
                    length: 200.0,
                    slope: 0.015,
                    surface_type: SurfaceType::Paved,
                    time: 5.0,
                }),
                channel_flow: Some(ChannelFlow {
                    length: 150.0,
                    velocity: 3.5,
                    time: 2.0,
                }),
            }),
            curve_number: None,
            geometry: None,
        };

        let total_tc = drainage_area.calculate_total_tc().unwrap();
        assert_eq!(total_tc, 10.0);
    }

    #[test]
    fn test_rational_method() {
        let drainage_area = DrainageArea {
            id: "DA-001".to_string(),
            name: None,
            area: 2.0,
            outlet: "IN-001".to_string(),
            land_use: None,
            runoff_coefficient: Some(0.80),
            time_of_concentration: Some(10.0),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        };

        let intensity = 3.5; // in/hr
        let runoff = drainage_area.rational_method_runoff(intensity).unwrap();

        // Q = C × i × A = 0.80 × 3.5 × 2.0 = 5.6 cfs
        assert!((runoff - 5.6).abs() < 0.001);
    }
}
