//! CSV input/output for drainage networks
//!
//! This module provides parsers for reading drainage network data from CSV files,
//! enabling non-programmers to use spreadsheets for data input.
//!
//! # CSV Formats
//!
//! ## Nodes CSV
//! Columns: `id`, `type`, `invert_elev`, `rim_elev`, `x`, `y`
//!
//! ## Conduits CSV
//! Columns: `id`, `from_node`, `to_node`, `diameter`, `length`, `slope`, `manning_n`
//!
//! ## Drainage Areas CSV
//! Columns: `id`, `area`, `runoff_coef`, `time_of_conc`, `outlet_node`
//!
//! ## IDF Curves CSV
//! Columns: `return_period`, `duration`, `intensity`
//!
//! ## Gutter Parameters CSV
//! Columns: `node_id`, `cross_slope`, `long_slope`, `curb_height`, `gutter_width`

use crate::conduit::{Conduit, ConduitType, GutterProperties, PipeMaterial, PipeProperties, PipeShape};
use crate::drainage::{DrainageArea, LandUse, LandUseType};
use crate::node::{BoundaryCondition, Coordinates, InletLocation, InletProperties, InletType, JunctionProperties, Node, NodeType, OutfallProperties};
use csv::{Reader, ReaderBuilder};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

// ============================================================================
// Node CSV Parser
// ============================================================================

/// CSV record for a node (inlet, junction, or outfall)
#[derive(Debug, Deserialize)]
pub struct NodeCsvRecord {
    /// Node ID
    pub id: String,
    /// Node type: "inlet", "junction", or "outfall"
    #[serde(rename = "type")]
    pub node_type: String,
    /// Invert elevation (ft)
    pub invert_elev: f64,
    /// Rim elevation (ft) - optional for outfalls
    pub rim_elev: Option<f64>,
    /// X coordinate (ft) - optional
    pub x: Option<f64>,
    /// Y coordinate (ft) - optional
    pub y: Option<f64>,
    /// Junction diameter (ft) - optional, for junctions
    pub diameter: Option<f64>,
    /// Inlet type - optional, for inlets: "grate", "curb", "combination", "slotted"
    pub inlet_type: Option<String>,
    /// Boundary condition - optional, for outfalls: "free", "normal", "fixed"
    pub boundary_condition: Option<String>,
}

impl NodeCsvRecord {
    /// Convert CSV record to Node
    pub fn to_node(&self) -> Result<Node, Box<dyn Error>> {
        let node_type_lower = self.node_type.to_lowercase();

        // Create coordinates if both x and y are provided
        let coordinates = match (self.x, self.y) {
            (Some(x), Some(y)) => Some(Coordinates {
                x: Some(x),
                y: Some(y),
                latitude: None,
                longitude: None,
            }),
            _ => None,
        };

        match node_type_lower.as_str() {
            "inlet" => {
                let rim_elev = self.rim_elev.ok_or("rim_elev required for inlets")?;
                let inlet_type = match self.inlet_type.as_deref() {
                    Some("grate") => InletType::Grate,
                    Some("curb") => InletType::CurbOpening,
                    Some("combination") => InletType::Combination,
                    Some("slotted") => InletType::Slotted,
                    None => InletType::Combination, // default
                    Some(t) => return Err(format!("Unknown inlet type: {}", t).into()),
                };

                let mut node = Node::new_inlet(
                    self.id.clone(),
                    self.invert_elev,
                    rim_elev,
                    InletProperties {
                        inlet_type,
                        location: InletLocation::OnGrade, // default
                        grate: None,
                        curb_opening: None,
                        local_depression: None,
                        clogging_factor: None,
                    },
                );
                node.coordinates = coordinates;
                Ok(node)
            }
            "junction" | "manhole" => {
                let rim_elev = self.rim_elev.ok_or("rim_elev required for junctions")?;
                let mut node = Node::new_junction(
                    self.id.clone(),
                    self.invert_elev,
                    rim_elev,
                    JunctionProperties {
                        diameter: self.diameter,
                        sump_depth: None,
                        loss_coefficient: Some(0.15), // default
                        benching: None,
                        drop_structure: None,
                    },
                );
                node.coordinates = coordinates;
                Ok(node)
            }
            "outfall" => {
                let boundary_condition = match self.boundary_condition.as_deref() {
                    Some("free") => BoundaryCondition::Free,
                    Some("normal") => BoundaryCondition::NormalDepth,
                    Some("fixed") => BoundaryCondition::FixedStage,
                    None => BoundaryCondition::Free, // default
                    Some(bc) => return Err(format!("Unknown boundary condition: {}", bc).into()),
                };

                let mut node = Node::new_outfall(
                    self.id.clone(),
                    self.invert_elev,
                    OutfallProperties {
                        boundary_condition,
                        tailwater_elevation: None,
                        tidal_curve: None,
                    },
                );
                node.coordinates = coordinates;
                Ok(node)
            }
            _ => Err(format!("Unknown node type: {}", self.node_type).into()),
        }
    }
}

/// Parse nodes from CSV file
pub fn parse_nodes_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Node>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true) // Allow variable number of columns
        .from_reader(file);

    let mut nodes = Vec::new();

    for (line_num, result) in reader.deserialize().enumerate() {
        let record: NodeCsvRecord = result
            .map_err(|e| format!("Line {}: {}", line_num + 2, e))?; // +2 for header + 1-based
        let node = record.to_node()
            .map_err(|e| format!("Line {} (node {}): {}", line_num + 2, record.id, e))?;
        nodes.push(node);
    }

    Ok(nodes)
}

// ============================================================================
// Conduit CSV Parser
// ============================================================================

/// CSV record for a conduit (pipe or gutter)
#[derive(Debug, Deserialize)]
pub struct ConduitCsvRecord {
    /// Conduit ID
    pub id: String,
    /// From node ID
    pub from_node: String,
    /// To node ID
    pub to_node: String,
    /// Conduit type: "pipe" or "gutter"
    #[serde(rename = "type")]
    pub conduit_type: Option<String>,
    /// Pipe diameter (inches) - for pipes
    pub diameter: Option<f64>,
    /// Conduit length (ft)
    pub length: f64,
    /// Slope (ft/ft) - optional
    pub slope: Option<f64>,
    /// Manning's n - optional
    pub manning_n: Option<f64>,
    /// Pipe material - optional: "RCP", "CMP", "PVC", "HDPE"
    pub material: Option<String>,
    /// Cross slope (ft/ft) - for gutters
    pub cross_slope: Option<f64>,
    /// Longitudinal slope (ft/ft) - for gutters
    pub long_slope: Option<f64>,
}

impl ConduitCsvRecord {
    /// Convert CSV record to Conduit
    pub fn to_conduit(&self) -> Result<Conduit, Box<dyn Error>> {
        let conduit_type = self.conduit_type.as_deref().unwrap_or("pipe");

        match conduit_type.to_lowercase().as_str() {
            "pipe" => {
                let diameter = self.diameter.ok_or("diameter required for pipes")?;
                let material = match self.material.as_deref() {
                    Some("RCP") | Some("rcp") => Some(PipeMaterial::RCP),
                    Some("CMP") | Some("cmp") => Some(PipeMaterial::CMP),
                    Some("PVC") | Some("pvc") => Some(PipeMaterial::PVC),
                    Some("HDPE") | Some("hdpe") => Some(PipeMaterial::HDPE),
                    None => Some(PipeMaterial::RCP), // default
                    Some(m) => return Err(format!("Unknown material: {}", m).into()),
                };

                // Use material's typical n value if not specified
                let manning_n = self.manning_n.unwrap_or_else(|| {
                    material.as_ref().map(|m| m.typical_manning_n()).unwrap_or(0.013)
                });

                Ok(Conduit::new_pipe(
                    self.id.clone(),
                    self.from_node.clone(),
                    self.to_node.clone(),
                    self.length,
                    PipeProperties {
                        shape: PipeShape::Circular,
                        diameter: Some(diameter),
                        width: None,
                        height: None,
                        material,
                        manning_n,
                        entrance_loss: None,
                        exit_loss: None,
                        bend_loss: None,
                    },
                ))
            }
            "gutter" => {
                let cross_slope = self.cross_slope.ok_or("cross_slope required for gutters")?;
                let long_slope = self.long_slope.or(self.slope).ok_or("long_slope or slope required for gutters")?;
                let manning_n = self.manning_n.unwrap_or(0.016); // default for concrete gutter

                Ok(Conduit::new_gutter(
                    self.id.clone(),
                    self.from_node.clone(),
                    self.to_node.clone(),
                    self.length,
                    GutterProperties {
                        cross_slope,
                        longitudinal_slope: long_slope,
                        width: None,
                        manning_n,
                    },
                ))
            }
            _ => Err(format!("Unknown conduit type: {}", conduit_type).into()),
        }
    }
}

/// Parse conduits from CSV file
pub fn parse_conduits_csv<P: AsRef<Path>>(path: P) -> Result<Vec<Conduit>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(file);

    let mut conduits = Vec::new();

    for (line_num, result) in reader.deserialize().enumerate() {
        let record: ConduitCsvRecord = result
            .map_err(|e| format!("Line {}: {}", line_num + 2, e))?;
        let conduit = record.to_conduit()
            .map_err(|e| format!("Line {} (conduit {}): {}", line_num + 2, record.id, e))?;
        conduits.push(conduit);
    }

    Ok(conduits)
}

// ============================================================================
// Drainage Area CSV Parser
// ============================================================================

/// CSV record for a drainage area
#[derive(Debug, Deserialize)]
pub struct DrainageAreaCsvRecord {
    /// Drainage area ID
    pub id: String,
    /// Area (acres)
    pub area: f64,
    /// Runoff coefficient (0-1)
    pub runoff_coef: f64,
    /// Time of concentration (minutes)
    pub time_of_conc: f64,
    /// Outlet node ID
    pub outlet_node: String,
    /// Land use description - optional
    pub land_use: Option<String>,
}

impl DrainageAreaCsvRecord {
    /// Convert CSV record to DrainageArea
    pub fn to_drainage_area(&self) -> DrainageArea {
        // Convert land use string to LandUseType
        let land_use = self.land_use.as_ref().and_then(|lu_str| {
            let land_use_type = match lu_str.to_lowercase().as_str() {
                "commercial" => Some(LandUseType::Commercial),
                "industrial" => Some(LandUseType::Industrial),
                "residential" => Some(LandUseType::Residential),
                "open space" | "openspace" => Some(LandUseType::OpenSpace),
                "transportation" => Some(LandUseType::Transportation),
                "agricultural" => Some(LandUseType::Agricultural),
                "mixed" => Some(LandUseType::Mixed),
                _ => None,
            };

            land_use_type.map(|primary| LandUse {
                primary: Some(primary),
                impervious_percent: None,
                composition: None,
            })
        });

        DrainageArea {
            id: self.id.clone(),
            name: None,
            area: self.area,
            outlet: self.outlet_node.clone(),
            land_use,
            runoff_coefficient: Some(self.runoff_coef),
            time_of_concentration: Some(self.time_of_conc),
            tc_calculation: None,
            curve_number: None,
            geometry: None,
        }
    }
}

/// Parse drainage areas from CSV file
pub fn parse_drainage_areas_csv<P: AsRef<Path>>(path: P) -> Result<Vec<DrainageArea>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(file);

    let mut areas = Vec::new();

    for (line_num, result) in reader.deserialize().enumerate() {
        let record: DrainageAreaCsvRecord = result
            .map_err(|e| format!("Line {}: {}", line_num + 2, e))?;
        areas.push(record.to_drainage_area());
    }

    Ok(areas)
}

// ============================================================================
// IDF Curves CSV Parser
// ============================================================================

use crate::rainfall::{IdfCurve, IdfPoint};

/// CSV record for IDF curve data point
#[derive(Debug, Deserialize)]
pub struct IdfCurveCsvRecord {
    /// Return period in years
    pub return_period: f64,
    /// Duration in minutes
    pub duration: f64,
    /// Rainfall intensity (in/hr or mm/hr)
    pub intensity: f64,
}

/// Parse IDF curves from CSV file and organize by return period
pub fn parse_idf_curves_csv<P: AsRef<Path>>(path: P) -> Result<Vec<IdfCurve>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(file);

    let mut records = Vec::new();

    for (line_num, result) in reader.deserialize().enumerate() {
        let record: IdfCurveCsvRecord = result
            .map_err(|e| format!("Line {}: {}", line_num + 2, e))?;
        records.push(record);
    }

    // Group by return period
    use std::collections::HashMap;
    let mut curves_map: HashMap<i32, Vec<IdfPoint>> = HashMap::new();

    for record in records {
        let rp_key = record.return_period as i32;
        curves_map.entry(rp_key).or_insert_with(Vec::new).push(IdfPoint {
            duration: record.duration,
            intensity: record.intensity,
        });
    }

    // Convert to IdfCurve structs
    let mut curves: Vec<IdfCurve> = curves_map
        .into_iter()
        .map(|(rp, mut points)| {
            // Sort points by duration
            points.sort_by(|a, b| a.duration.partial_cmp(&b.duration).unwrap());
            IdfCurve {
                return_period: rp as f64,
                equation: None,
                points,
            }
        })
        .collect();

    // Sort curves by return period
    curves.sort_by(|a, b| a.return_period.partial_cmp(&b.return_period).unwrap());

    Ok(curves)
}

// ============================================================================
// Gutter Parameters CSV Parser
// ============================================================================

/// CSV record for gutter/curb parameters at an inlet
#[derive(Debug, Deserialize)]
pub struct GutterParametersCsvRecord {
    /// Node ID (must be an inlet)
    pub node_id: String,
    /// Cross slope (ft/ft)
    pub cross_slope: f64,
    /// Longitudinal slope (ft/ft)
    pub long_slope: f64,
    /// Curb height (inches) - optional
    pub curb_height: Option<f64>,
    /// Gutter width (ft) - optional
    pub gutter_width: Option<f64>,
    /// Manning's n - optional
    pub manning_n: Option<f64>,
    /// Local depression depth (inches) - optional
    pub depression: Option<f64>,
    /// Depression width (ft) - optional
    pub depression_width: Option<f64>,
}

/// Parse gutter parameters from CSV file
pub fn parse_gutter_parameters_csv<P: AsRef<Path>>(path: P) -> Result<Vec<GutterParametersCsvRecord>, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new()
        .flexible(true)
        .from_reader(file);

    let mut params = Vec::new();

    for result in reader.deserialize() {
        let record: GutterParametersCsvRecord = result?;
        params.push(record);
    }

    Ok(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_csv_record_to_inlet() {
        let record = NodeCsvRecord {
            id: "IN-001".to_string(),
            node_type: "inlet".to_string(),
            invert_elev: 100.0,
            rim_elev: Some(105.0),
            x: Some(0.0),
            y: Some(0.0),
            diameter: None,
            inlet_type: Some("grate".to_string()),
            boundary_condition: None,
        };

        let node = record.to_node().unwrap();
        assert_eq!(node.id, "IN-001");
        assert_eq!(node.node_type, NodeType::Inlet);
    }

    #[test]
    fn test_node_csv_record_to_junction() {
        let record = NodeCsvRecord {
            id: "MH-001".to_string(),
            node_type: "junction".to_string(),
            invert_elev: 95.0,
            rim_elev: Some(100.0),
            x: Some(100.0),
            y: Some(50.0),
            diameter: Some(4.0),
            inlet_type: None,
            boundary_condition: None,
        };

        let node = record.to_node().unwrap();
        assert_eq!(node.id, "MH-001");
        assert_eq!(node.node_type, NodeType::Junction);
    }

    #[test]
    fn test_conduit_csv_record_to_pipe() {
        let record = ConduitCsvRecord {
            id: "P-001".to_string(),
            from_node: "MH-001".to_string(),
            to_node: "MH-002".to_string(),
            conduit_type: Some("pipe".to_string()),
            diameter: Some(18.0),
            length: 120.0,
            slope: Some(0.005),
            manning_n: Some(0.013),
            material: Some("RCP".to_string()),
            cross_slope: None,
            long_slope: None,
        };

        let conduit = record.to_conduit().unwrap();
        assert_eq!(conduit.id, "P-001");
        assert_eq!(conduit.conduit_type, ConduitType::Pipe);
    }

    #[test]
    fn test_drainage_area_csv_record() {
        let record = DrainageAreaCsvRecord {
            id: "DA-001".to_string(),
            area: 2.5,
            runoff_coef: 0.75,
            time_of_conc: 15.0,
            outlet_node: "IN-001".to_string(),
            land_use: Some("Commercial".to_string()),
        };

        let area = record.to_drainage_area();
        assert_eq!(area.id, "DA-001");
        assert_eq!(area.area, 2.5);
        assert_eq!(area.runoff_coefficient, Some(0.75));
    }
}
