//! # HEC-22 Drainage Network Analysis
//!
//! This library provides data structures and analysis tools for urban drainage systems
//! following the FHWA HEC-22 (4th Edition, 2024) methodology.
//!
//! ## Overview
//!
//! The library is organized into several modules:
//!
//! - [`project`] - Project metadata and unit definitions
//! - [`network`] - Network topology (nodes and conduits)
//! - [`node`] - Node types (junctions, inlets, outfalls)
//! - [`conduit`] - Conduit types (pipes, gutters, channels)
//! - [`drainage`] - Drainage areas and subcatchments
//! - [`rainfall`] - Rainfall events and IDF curves
//! - [`storage`] - Detention/retention equations and storage metadata (Chapter 10)
//! - [`quality`] - Stormwater quality and BMP calculations (Chapter 11)
//! - [`pump`] - Pump station equations and wet well sizing (Chapter 12)
//! - [`analysis`] - Analysis results and violations
//! - [`hydraulics`] - Hydraulic calculations (Manning's equation, HGL/EGL)
//! - [`gutter`] - Gutter spread calculations (Chapter 5)
//! - [`inlet`] - Inlet capacity calculations (Chapter 7)
//! - [`solver`] - HGL/EGL solver (9-step procedure from Chapter 9)
//! - [`csv`] - CSV input/output for tabular data
//! - [`visualization`] - SVG and HTML visualization tools (network plan and profile views)
//!
//! ## Example
//!
//! ```no_run
//! use hec22::DrainageNetwork;
//! use std::fs;
//!
//! // Load network from JSON
//! let json = fs::read_to_string("network.json").unwrap();
//! let network: DrainageNetwork = serde_json::from_str(&json).unwrap();
//!
//! // Access network components
//! for node in &network.network.nodes {
//!     println!("Node {} at elevation {}", node.id, node.invert_elevation);
//! }
//! ```

pub mod analysis;
pub mod conduit;
pub mod drainage;
pub mod gutter;
pub mod hydraulics;
pub mod inlet;
pub mod network;
pub mod node;
pub mod project;
pub mod pump;
pub mod quality;
pub mod rainfall;
pub mod solver;
pub mod storage;

#[cfg(feature = "csv")]
pub mod csv;

#[cfg(feature = "visualization")]
pub mod visualization;

use serde::{Deserialize, Serialize};

/// Root-level drainage network model
///
/// This is the top-level structure that contains all components of a drainage network
/// analysis, including project metadata, network topology, hydrologic data, and
/// optional analysis results.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrainageNetwork {
    /// Schema version (semantic versioning)
    pub version: String,

    /// Project metadata and settings
    pub project: project::Project,

    /// Network topology (nodes and conduits)
    pub network: network::Network,

    /// Rainfall events and IDF curves (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rainfall: Option<rainfall::Rainfall>,

    /// Drainage areas/subcatchments (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drainageAreas")]
    pub drainage_areas: Option<Vec<drainage::DrainageArea>>,

    /// Detention/retention facilities (optional, Chapter 10)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "storageFacilities")]
    pub storage_facilities: Option<Vec<storage::StorageFacility>>,

    /// Stormwater quality BMP facilities (optional, Chapter 11)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bmps: Option<Vec<quality::BmpFacility>>,

    /// Pump stations (optional, Chapter 12)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pumpStations")]
    pub pump_stations: Option<Vec<pump::PumpStation>>,

    /// Design criteria and constraints (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "designCriteria")]
    pub design_criteria: Option<analysis::DesignCriteria>,

    /// Analysis results (optional, populated by solver)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<analysis::Analysis>,
}

impl DrainageNetwork {
    /// Create a new drainage network with minimal required fields
    pub fn new(project: project::Project, network: network::Network) -> Self {
        Self {
            version: "1.0.0".to_string(),
            project,
            network,
            rainfall: None,
            drainage_areas: None,
            storage_facilities: None,
            bmps: None,
            pump_stations: None,
            design_criteria: None,
            analysis: None,
        }
    }

    /// Load a drainage network from a JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize the drainage network to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get all nodes of a specific type
    pub fn nodes_by_type(&self, node_type: node::NodeType) -> Vec<&node::Node> {
        self.network
            .nodes
            .iter()
            .filter(|n| n.node_type == node_type)
            .collect()
    }

    /// Find a node by ID
    pub fn find_node(&self, id: &str) -> Option<&node::Node> {
        self.network.nodes.iter().find(|n| n.id == id)
    }

    /// Find a conduit by ID
    pub fn find_conduit(&self, id: &str) -> Option<&conduit::Conduit> {
        self.network.conduits.iter().find(|c| c.id == id)
    }

    /// Get upstream conduits for a given node
    pub fn upstream_conduits(&self, node_id: &str) -> Vec<&conduit::Conduit> {
        self.network
            .conduits
            .iter()
            .filter(|c| c.to_node == node_id)
            .collect()
    }

    /// Get downstream conduits for a given node
    pub fn downstream_conduits(&self, node_id: &str) -> Vec<&conduit::Conduit> {
        self.network
            .conduits
            .iter()
            .filter(|c| c.from_node == node_id)
            .collect()
    }

    /// Run Chapter 10 Modified Puls routing for storage facilities that include
    /// stage-storage, stage-discharge, and inflow hydrograph data.
    pub fn run_storage_routing(&mut self) -> Result<(), String> {
        let facilities = match self.storage_facilities.as_ref() {
            Some(facilities) => facilities,
            None => return Ok(()),
        };

        let mut storage_results = Vec::new();
        for facility in facilities {
            let has_any_routing_input = facility.stage_storage.is_some()
                || facility.stage_discharge.is_some()
                || facility.inflow_hydrograph.is_some();

            if !has_any_routing_input {
                continue;
            }

            let routed = facility.route_modified_puls()?;
            let max_stage = routed
                .iter()
                .map(|p| p.stage)
                .reduce(f64::max)
                .unwrap_or(0.0);
            let max_storage = routed
                .iter()
                .map(|p| p.storage)
                .reduce(f64::max)
                .unwrap_or(0.0);
            let max_outflow = routed
                .iter()
                .map(|p| p.outflow)
                .reduce(f64::max)
                .unwrap_or(0.0);

            storage_results.push(analysis::StorageRoutingResult {
                facility_id: facility.id.clone(),
                max_stage,
                max_storage,
                max_outflow,
                routed_hydrograph: Some(routed),
            });
        }

        if storage_results.is_empty() {
            return Ok(());
        }

        if self.analysis.is_none() {
            self.analysis = Some(analysis::Analysis::new(
                analysis::AnalysisMethod::KinematicWave,
                "storage-routing".to_string(),
            ));
        }

        if let Some(analysis) = self.analysis.as_mut() {
            for result in storage_results {
                analysis.add_storage_result(result);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimal_network() {
        let project = project::Project {
            name: "Test Project".to_string(),
            description: None,
            location: None,
            units: project::Units {
                system: project::UnitSystem::US,
                length: Some(project::LengthUnit::Feet),
                elevation: Some(project::LengthUnit::Feet),
                flow: Some(project::FlowUnit::Cfs),
                area: Some(project::AreaUnit::Acres),
            },
            author: None,
            created: None,
            modified: None,
        };

        let network = network::Network {
            nodes: vec![],
            conduits: vec![],
        };

        let drainage_network = DrainageNetwork::new(project, network);

        assert_eq!(drainage_network.version, "1.0.0");
        assert_eq!(drainage_network.project.name, "Test Project");
    }

    #[test]
    fn test_chapter_10_12_fields_round_trip() {
        let project = project::Project {
            name: "Advanced Features Test".to_string(),
            description: None,
            location: None,
            units: project::Units::us_customary(),
            author: None,
            created: None,
            modified: None,
        };

        let mut drainage_network = DrainageNetwork::new(project, network::Network::new());

        drainage_network.storage_facilities = Some(vec![storage::StorageFacility {
            id: "DET-1".to_string(),
            name: Some("Detention Basin 1".to_string()),
            facility_type: storage::StorageType::Detention,
            drainage_area_ids: Some(vec!["DA-1".to_string()]),
            required_volume: Some(25000.0),
            max_stage: Some(8.0),
            geometry: None,
            outlet: None,
            stage_storage: None,
            stage_discharge: None,
            inflow_hydrograph: None,
            initial_stage: None,
        }]);

        drainage_network.bmps = Some(vec![quality::BmpFacility {
            id: "BMP-1".to_string(),
            name: Some("Bioretention A".to_string()),
            bmp_type: quality::BmpType::Bioretention,
            drainage_area_ids: Some(vec!["DA-1".to_string()]),
            water_quality_volume: Some(4500.0),
            detention_time_hours: Some(24.0),
            removal_efficiencies: None,
        }]);

        drainage_network.pump_stations = Some(vec![pump::PumpStation {
            id: "PS-1".to_string(),
            name: Some("Lift Station 1".to_string()),
            wet_well_node_id: Some("J-100".to_string()),
            pump_count: Some(2),
            rated_flow: Some(3.5),
            rated_head: Some(45.0),
            wet_well: None,
        }]);

        let json = drainage_network.to_json().unwrap();
        let parsed = DrainageNetwork::from_json(&json).unwrap();

        assert!(parsed.storage_facilities.is_some());
        assert!(parsed.bmps.is_some());
        assert!(parsed.pump_stations.is_some());
    }

    #[test]
    fn test_run_storage_routing_populates_analysis() {
        let project = project::Project {
            name: "Storage Routing Test".to_string(),
            description: None,
            location: None,
            units: project::Units::us_customary(),
            author: None,
            created: None,
            modified: None,
        };

        let mut drainage_network = DrainageNetwork::new(project, network::Network::new());
        drainage_network.storage_facilities = Some(vec![storage::StorageFacility {
            id: "DET-ROUTE-1".to_string(),
            name: None,
            facility_type: storage::StorageType::Detention,
            drainage_area_ids: None,
            required_volume: None,
            max_stage: None,
            geometry: None,
            outlet: None,
            stage_storage: Some(vec![
                storage::StageStoragePoint {
                    stage: 0.0,
                    storage: 0.0,
                },
                storage::StageStoragePoint {
                    stage: 1.0,
                    storage: 1000.0,
                },
                storage::StageStoragePoint {
                    stage: 2.0,
                    storage: 2000.0,
                },
            ]),
            stage_discharge: Some(vec![
                storage::StageDischargePoint {
                    stage: 0.0,
                    discharge: 0.0,
                },
                storage::StageDischargePoint {
                    stage: 1.0,
                    discharge: 10.0,
                },
                storage::StageDischargePoint {
                    stage: 2.0,
                    discharge: 20.0,
                },
            ]),
            inflow_hydrograph: Some(vec![
                storage::HydrographPoint {
                    time_seconds: 0.0,
                    inflow: 10.0,
                },
                storage::HydrographPoint {
                    time_seconds: 60.0,
                    inflow: 20.0,
                },
                storage::HydrographPoint {
                    time_seconds: 120.0,
                    inflow: 10.0,
                },
            ]),
            initial_stage: Some(1.0),
        }]);

        drainage_network.run_storage_routing().unwrap();

        let analysis = drainage_network.analysis.as_ref().unwrap();
        let storage_results = analysis.storage_results.as_ref().unwrap();
        assert_eq!(storage_results.len(), 1);
        assert_eq!(storage_results[0].facility_id, "DET-ROUTE-1");
        assert!(storage_results[0].max_storage > 0.0);
    }
}
