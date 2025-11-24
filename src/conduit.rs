//! Conduit types for drainage networks
//!
//! Conduits represent flow paths between nodes:
//! - Pipes: Closed conduits (circular, rectangular, etc.)
//! - Gutters: Surface flow along roadways
//! - Channels: Open channels (trapezoidal, natural)

use serde::{Deserialize, Serialize};

/// A conduit in the drainage network
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conduit {
    /// Unique conduit identifier
    pub id: String,

    /// Conduit type
    #[serde(rename = "type")]
    pub conduit_type: ConduitType,

    /// Descriptive name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Upstream node ID
    #[serde(rename = "fromNode")]
    pub from_node: String,

    /// Downstream node ID
    #[serde(rename = "toNode")]
    pub to_node: String,

    /// Conduit length (ft or m)
    pub length: f64,

    /// Upstream invert elevation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "upstreamInvert")]
    pub upstream_invert: Option<f64>,

    /// Downstream invert elevation (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "downstreamInvert")]
    pub downstream_invert: Option<f64>,

    /// Conduit slope (ft/ft or m/m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slope: Option<f64>,

    /// Pipe-specific properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipe: Option<PipeProperties>,

    /// Gutter-specific properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gutter: Option<GutterProperties>,

    /// Open channel-specific properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelProperties>,
}

/// Conduit type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ConduitType {
    /// Closed pipe conduit
    Pipe,
    /// Surface gutter flow
    Gutter,
    /// Open channel
    Channel,
}

/// Pipe properties
///
/// Pipes are closed conduits analyzed using Manning's equation.
/// Flow can be full-flow (pressurized) or partial-flow (gravity).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipeProperties {
    /// Pipe cross-sectional shape
    pub shape: PipeShape,

    /// Pipe diameter for circular pipes (in or mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diameter: Option<f64>,

    /// Width for rectangular/elliptical pipes (in or mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Height for rectangular/elliptical pipes (in or mm)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,

    /// Pipe material
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<PipeMaterial>,

    /// Manning's roughness coefficient n
    #[serde(rename = "manningN")]
    pub manning_n: f64,

    /// Entrance loss coefficient Ke
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entranceLoss")]
    pub entrance_loss: Option<f64>,

    /// Exit loss coefficient
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "exitLoss")]
    pub exit_loss: Option<f64>,

    /// Additional loss for bends/curves
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bendLoss")]
    pub bend_loss: Option<f64>,
}

/// Pipe cross-sectional shape
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PipeShape {
    /// Circular pipe (most common)
    Circular,
    /// Rectangular box culvert
    Rectangular,
    /// Elliptical pipe
    Elliptical,
    /// Arch pipe
    Arch,
}

/// Pipe material types
///
/// Each material has a typical Manning's n value:
/// - RCP (Reinforced Concrete Pipe): n = 0.013
/// - CMP (Corrugated Metal Pipe): n = 0.024
/// - PVC: n = 0.011
/// - HDPE: n = 0.011
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PipeMaterial {
    /// Reinforced Concrete Pipe (n ≈ 0.013)
    RCP,
    /// Corrugated Metal Pipe (n ≈ 0.024)
    CMP,
    /// Polyvinyl Chloride (n ≈ 0.011)
    PVC,
    /// High-Density Polyethylene (n ≈ 0.011)
    HDPE,
    /// Concrete pipe (n ≈ 0.013)
    Concrete,
    /// Steel pipe (n ≈ 0.012)
    Steel,
    /// Ductile iron pipe (n ≈ 0.013)
    #[serde(rename = "Ductile Iron")]
    DuctileIron,
}

impl PipeMaterial {
    /// Get typical Manning's n value for this material
    pub fn typical_manning_n(&self) -> f64 {
        match self {
            PipeMaterial::RCP => 0.013,
            PipeMaterial::CMP => 0.024,
            PipeMaterial::PVC => 0.011,
            PipeMaterial::HDPE => 0.011,
            PipeMaterial::Concrete => 0.013,
            PipeMaterial::Steel => 0.012,
            PipeMaterial::DuctileIron => 0.013,
        }
    }
}

/// Gutter properties
///
/// Gutters are surface flow paths along roadways, analyzed using
/// the triangular gutter section equations from HEC-22 Chapter 7.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GutterProperties {
    /// Cross slope Sx (ft/ft) - perpendicular to flow
    #[serde(rename = "crossSlope")]
    pub cross_slope: f64,

    /// Longitudinal slope SL (ft/ft) - in direction of flow
    #[serde(rename = "longitudinalSlope")]
    pub longitudinal_slope: f64,

    /// Gutter width (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Manning's roughness coefficient n (typical: 0.016 for asphalt)
    #[serde(rename = "manningN")]
    pub manning_n: f64,
}

/// Open channel properties
///
/// Open channels are natural or constructed watercourses analyzed
/// using Manning's equation with various cross-sectional shapes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelProperties {
    /// Channel cross-sectional shape
    pub shape: ChannelShape,

    /// Bottom width (ft or m) for trapezoidal/rectangular channels
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bottomWidth")]
    pub bottom_width: Option<f64>,

    /// Side slope (H:V) for trapezoidal/triangular channels
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sideSlope")]
    pub side_slope: Option<f64>,

    /// Manning's roughness coefficient n
    #[serde(rename = "manningN")]
    pub manning_n: f64,
}

/// Channel cross-sectional shape
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChannelShape {
    /// Trapezoidal channel (most common)
    Trapezoidal,
    /// Rectangular channel
    Rectangular,
    /// Triangular (V-shaped) channel
    Triangular,
    /// Natural/irregular channel
    Natural,
}

impl Conduit {
    /// Create a new pipe conduit
    pub fn new_pipe(
        id: String,
        from_node: String,
        to_node: String,
        length: f64,
        properties: PipeProperties,
    ) -> Self {
        Self {
            id,
            conduit_type: ConduitType::Pipe,
            name: None,
            from_node,
            to_node,
            length,
            upstream_invert: None,
            downstream_invert: None,
            slope: None,
            pipe: Some(properties),
            gutter: None,
            channel: None,
        }
    }

    /// Create a new gutter conduit
    pub fn new_gutter(
        id: String,
        from_node: String,
        to_node: String,
        length: f64,
        properties: GutterProperties,
    ) -> Self {
        Self {
            id,
            conduit_type: ConduitType::Gutter,
            name: None,
            from_node,
            to_node,
            length,
            upstream_invert: None,
            downstream_invert: None,
            slope: Some(properties.longitudinal_slope),
            pipe: None,
            gutter: Some(properties),
            channel: None,
        }
    }

    /// Create a new channel conduit
    pub fn new_channel(
        id: String,
        from_node: String,
        to_node: String,
        length: f64,
        properties: ChannelProperties,
    ) -> Self {
        Self {
            id,
            conduit_type: ConduitType::Channel,
            name: None,
            from_node,
            to_node,
            length,
            upstream_invert: None,
            downstream_invert: None,
            slope: None,
            pipe: None,
            gutter: None,
            channel: Some(properties),
        }
    }

    /// Calculate slope from invert elevations
    pub fn calculate_slope(&self) -> Option<f64> {
        match (self.upstream_invert, self.downstream_invert) {
            (Some(up), Some(down)) if self.length > 0.0 => {
                Some((up - down) / self.length)
            }
            _ => None,
        }
    }

    /// Get the effective slope (either specified or calculated)
    pub fn effective_slope(&self) -> Option<f64> {
        self.slope.or_else(|| self.calculate_slope())
    }

    /// Check if this is a pipe
    pub fn is_pipe(&self) -> bool {
        self.conduit_type == ConduitType::Pipe
    }

    /// Check if this is a gutter
    pub fn is_gutter(&self) -> bool {
        self.conduit_type == ConduitType::Gutter
    }

    /// Check if this is a channel
    pub fn is_channel(&self) -> bool {
        self.conduit_type == ConduitType::Channel
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pipe() {
        let props = PipeProperties {
            shape: PipeShape::Circular,
            diameter: Some(18.0),
            width: None,
            height: None,
            material: Some(PipeMaterial::RCP),
            manning_n: 0.013,
            entrance_loss: Some(0.5),
            exit_loss: Some(1.0),
            bend_loss: Some(0.0),
        };

        let conduit = Conduit::new_pipe(
            "P-101".to_string(),
            "IN-001".to_string(),
            "MH-001".to_string(),
            150.0,
            props,
        );

        assert_eq!(conduit.id, "P-101");
        assert!(conduit.is_pipe());
        assert_eq!(conduit.pipe.as_ref().unwrap().diameter, Some(18.0));
    }

    #[test]
    fn test_pipe_material_manning_n() {
        assert_eq!(PipeMaterial::RCP.typical_manning_n(), 0.013);
        assert_eq!(PipeMaterial::CMP.typical_manning_n(), 0.024);
        assert_eq!(PipeMaterial::PVC.typical_manning_n(), 0.011);
    }

    #[test]
    fn test_calculate_slope() {
        let mut conduit = Conduit::new_pipe(
            "P-101".to_string(),
            "N1".to_string(),
            "N2".to_string(),
            100.0,
            PipeProperties {
                shape: PipeShape::Circular,
                diameter: Some(18.0),
                width: None,
                height: None,
                material: Some(PipeMaterial::RCP),
                manning_n: 0.013,
                entrance_loss: None,
                exit_loss: None,
                bend_loss: None,
            },
        );

        conduit.upstream_invert = Some(125.0);
        conduit.downstream_invert = Some(123.0);

        let slope = conduit.calculate_slope().unwrap();
        assert!((slope - 0.02).abs() < 1e-6);
    }

    #[test]
    fn test_create_gutter() {
        let props = GutterProperties {
            cross_slope: 0.02,
            longitudinal_slope: 0.015,
            width: Some(12.0),
            manning_n: 0.016,
        };

        let conduit = Conduit::new_gutter(
            "G-101".to_string(),
            "IN-001".to_string(),
            "IN-001".to_string(),
            400.0,
            props,
        );

        assert!(conduit.is_gutter());
        assert_eq!(conduit.gutter.as_ref().unwrap().cross_slope, 0.02);
    }
}
