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
pub mod csv;
pub mod drainage;
pub mod gutter;
pub mod hydraulics;
pub mod inlet;
pub mod network;
pub mod node;
pub mod project;
pub mod rainfall;
pub mod solver;
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
}
