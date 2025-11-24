//! Node types for drainage networks
//!
//! Nodes represent physical structures in the drainage network:
//! - Junctions/Manholes: Connection points between pipes
//! - Inlets: Surface drainage collection points
//! - Outfalls: Discharge points to receiving waters

use serde::{Deserialize, Serialize};

/// A node in the drainage network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Node {
    /// Unique node identifier
    pub id: String,

    /// Node type
    #[serde(rename = "type")]
    pub node_type: NodeType,

    /// Descriptive name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Invert elevation of the lowest pipe (ft or m)
    #[serde(rename = "invertElevation")]
    pub invert_elevation: f64,

    /// Ground/rim elevation for flooding checks (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rimElevation")]
    pub rim_elevation: Option<f64>,

    /// Spatial location (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Coordinates>,

    /// Junction/manhole-specific properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub junction: Option<JunctionProperties>,

    /// Inlet-specific properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inlet: Option<InletProperties>,

    /// Outfall-specific properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outfall: Option<OutfallProperties>,
}

/// Node type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    /// Junction or manhole (pipe connection point)
    Junction,
    /// Inlet (surface drainage collection)
    Inlet,
    /// Outfall (discharge point)
    Outfall,
}

/// Spatial coordinates
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Coordinates {
    /// X coordinate (state plane, project coordinate system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,

    /// Y coordinate (state plane, project coordinate system)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,

    /// Latitude (decimal degrees)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    /// Longitude (decimal degrees)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
}

/// Junction/manhole properties
///
/// Junctions are connection points between pipes, typically manholes or
/// access structures. They account for energy losses due to flow changes
/// and provide access for maintenance.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JunctionProperties {
    /// Manhole diameter (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diameter: Option<f64>,

    /// Sump depth below lowest invert (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sumpDepth")]
    pub sump_depth: Option<f64>,

    /// Energy loss coefficient K (for HL = K × V²/(2g))
    /// Typical values: 0.05-0.25 for straight runs, 0.25-1.50 for angle changes
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "lossCoefficient")]
    pub loss_coefficient: Option<f64>,

    /// Whether benching is present (affects flow characteristics)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benching: Option<bool>,

    /// Whether this is a drop structure
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dropStructure")]
    pub drop_structure: Option<bool>,
}

/// Inlet properties
///
/// Inlets are surface drainage collection points that capture gutter flow.
/// Design is governed by HEC-22 Chapter 7 inlet design procedures.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InletProperties {
    /// Type of inlet
    #[serde(rename = "inletType")]
    pub inlet_type: InletType,

    /// Location type (on-grade or sag)
    pub location: InletLocation,

    /// Grate inlet properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grate: Option<GrateProperties>,

    /// Curb opening inlet properties
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "curbOpening")]
    pub curb_opening: Option<CurbOpeningProperties>,

    /// Local depression depth (in or mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "localDepression")]
    pub local_depression: Option<f64>,

    /// Clogging factor (fraction of area assumed clogged, 0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cloggingFactor")]
    pub clogging_factor: Option<f64>,
}

/// Inlet type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InletType {
    /// Grate inlet only
    Grate,
    /// Curb opening inlet only
    CurbOpening,
    /// Combination grate and curb opening
    Combination,
    /// Slotted drain inlet
    Slotted,
}

/// Inlet location type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InletLocation {
    /// On-grade (continuous grade, bypass flow possible)
    OnGrade,
    /// Sag (low point, no bypass flow)
    Sag,
}

/// Grate inlet properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GrateProperties {
    /// Grate length (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,

    /// Grate width (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Bar configuration relative to flow direction
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "barConfiguration")]
    pub bar_configuration: Option<BarConfiguration>,
}

/// Bar configuration for grate inlets
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BarConfiguration {
    /// Bars parallel to flow (better capacity)
    Parallel,
    /// Bars perpendicular to flow (better debris handling)
    Perpendicular,
}

/// Curb opening inlet properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CurbOpeningProperties {
    /// Opening length (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,

    /// Opening height (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    /// Throat type (horizontal, inclined, or vertical)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "throatType")]
    pub throat_type: Option<ThroatType>,
}

/// Curb opening throat type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ThroatType {
    /// Horizontal throat (standard)
    Horizontal,
    /// Inclined throat (improved capacity)
    Inclined,
    /// Vertical throat (maximum capacity)
    Vertical,
}

/// Outfall properties
///
/// Outfalls represent discharge points to receiving waters or downstream systems.
/// They define the downstream boundary condition for hydraulic analysis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutfallProperties {
    /// Downstream boundary condition type
    #[serde(rename = "boundaryCondition")]
    pub boundary_condition: BoundaryCondition,

    /// Fixed tailwater elevation (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tailwaterElevation")]
    pub tailwater_elevation: Option<f64>,

    /// Tidal stage variation (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tidalCurve")]
    pub tidal_curve: Option<Vec<TidalPoint>>,
}

/// Downstream boundary condition type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BoundaryCondition {
    /// Free outfall (critical depth)
    Free,
    /// Normal depth based on conduit slope
    NormalDepth,
    /// Fixed stage elevation
    FixedStage,
    /// Tidal boundary (time-varying)
    Tidal,
}

/// Tidal stage data point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TidalPoint {
    /// Time from start (minutes or hours)
    pub time: f64,
    /// Water surface elevation at this time
    pub elevation: f64,
}

impl Node {
    /// Create a new junction node
    pub fn new_junction(
        id: String,
        invert_elevation: f64,
        rim_elevation: f64,
        properties: JunctionProperties,
    ) -> Self {
        Self {
            id,
            node_type: NodeType::Junction,
            name: None,
            invert_elevation,
            rim_elevation: Some(rim_elevation),
            coordinates: None,
            junction: Some(properties),
            inlet: None,
            outfall: None,
        }
    }

    /// Create a new inlet node
    pub fn new_inlet(
        id: String,
        invert_elevation: f64,
        rim_elevation: f64,
        properties: InletProperties,
    ) -> Self {
        Self {
            id,
            node_type: NodeType::Inlet,
            name: None,
            invert_elevation,
            rim_elevation: Some(rim_elevation),
            coordinates: None,
            junction: None,
            inlet: Some(properties),
            outfall: None,
        }
    }

    /// Create a new outfall node
    pub fn new_outfall(
        id: String,
        invert_elevation: f64,
        properties: OutfallProperties,
    ) -> Self {
        Self {
            id,
            node_type: NodeType::Outfall,
            name: None,
            invert_elevation,
            rim_elevation: None,
            coordinates: None,
            junction: None,
            inlet: None,
            outfall: Some(properties),
        }
    }

    /// Check if the node is an inlet
    pub fn is_inlet(&self) -> bool {
        self.node_type == NodeType::Inlet
    }

    /// Check if the node is a junction
    pub fn is_junction(&self) -> bool {
        self.node_type == NodeType::Junction
    }

    /// Check if the node is an outfall
    pub fn is_outfall(&self) -> bool {
        self.node_type == NodeType::Outfall
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_junction() {
        let props = JunctionProperties {
            diameter: Some(4.0),
            sump_depth: Some(0.5),
            loss_coefficient: Some(0.15),
            benching: Some(true),
            drop_structure: Some(false),
        };

        let node = Node::new_junction("MH-001".to_string(), 120.0, 125.0, props);

        assert_eq!(node.id, "MH-001");
        assert_eq!(node.node_type, NodeType::Junction);
        assert!(node.is_junction());
        assert!(!node.is_inlet());
    }

    #[test]
    fn test_create_inlet() {
        let props = InletProperties {
            inlet_type: InletType::Combination,
            location: InletLocation::OnGrade,
            grate: Some(GrateProperties {
                length: Some(2.0),
                width: Some(1.5),
                bar_configuration: Some(BarConfiguration::Perpendicular),
            }),
            curb_opening: None,
            local_depression: Some(2.0),
            clogging_factor: Some(0.15),
        };

        let node = Node::new_inlet("IN-001".to_string(), 124.5, 128.0, props);

        assert_eq!(node.id, "IN-001");
        assert!(node.is_inlet());
        assert_eq!(node.inlet.as_ref().unwrap().inlet_type, InletType::Combination);
    }

    #[test]
    fn test_create_outfall() {
        let props = OutfallProperties {
            boundary_condition: BoundaryCondition::NormalDepth,
            tailwater_elevation: None,
            tidal_curve: None,
        };

        let node = Node::new_outfall("OUT-001".to_string(), 115.0, props);

        assert_eq!(node.id, "OUT-001");
        assert!(node.is_outfall());
    }
}
