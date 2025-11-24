//! Network topology module
//!
//! Defines the drainage network structure consisting of nodes and conduits.

use crate::{conduit::Conduit, node::Node};
use serde::{Deserialize, Serialize};

/// Drainage network topology
///
/// The network is a directed graph where:
/// - Nodes represent physical structures (junctions, inlets, outfalls)
/// - Conduits represent flow paths (pipes, gutters, channels)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Network {
    /// Collection of nodes in the network
    pub nodes: Vec<Node>,

    /// Collection of conduits in the network
    pub conduits: Vec<Conduit>,
}

impl Network {
    /// Create a new empty network
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            conduits: Vec::new(),
        }
    }

    /// Add a node to the network
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    /// Add a conduit to the network
    pub fn add_conduit(&mut self, conduit: Conduit) {
        self.conduits.push(conduit);
    }

    /// Find a node by ID
    pub fn find_node(&self, id: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Find a conduit by ID
    pub fn find_conduit(&self, id: &str) -> Option<&Conduit> {
        self.conduits.iter().find(|c| c.id == id)
    }

    /// Get all upstream conduits for a node
    pub fn upstream_conduits(&self, node_id: &str) -> Vec<&Conduit> {
        self.conduits
            .iter()
            .filter(|c| c.to_node == node_id)
            .collect()
    }

    /// Get all downstream conduits for a node
    pub fn downstream_conduits(&self, node_id: &str) -> Vec<&Conduit> {
        self.conduits
            .iter()
            .filter(|c| c.from_node == node_id)
            .collect()
    }

    /// Validate network connectivity
    ///
    /// Checks that all conduit endpoints reference valid nodes
    pub fn validate_connectivity(&self) -> Result<(), String> {
        for conduit in &self.conduits {
            if !self.nodes.iter().any(|n| n.id == conduit.from_node) {
                return Err(format!(
                    "Conduit {} references non-existent from_node: {}",
                    conduit.id, conduit.from_node
                ));
            }
            if !self.nodes.iter().any(|n| n.id == conduit.to_node) {
                return Err(format!(
                    "Conduit {} references non-existent to_node: {}",
                    conduit.id, conduit.to_node
                ));
            }
        }
        Ok(())
    }

    /// Get all outfall nodes
    pub fn outfalls(&self) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.is_outfall()).collect()
    }

    /// Get all inlet nodes
    pub fn inlets(&self) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.is_inlet()).collect()
    }

    /// Get all junction nodes
    pub fn junctions(&self) -> Vec<&Node> {
        self.nodes.iter().filter(|n| n.is_junction()).collect()
    }

    /// Count total number of nodes
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Count total number of conduits
    pub fn conduit_count(&self) -> usize {
        self.conduits.len()
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conduit::{ConduitType, PipeProperties, PipeShape};
    use crate::node::{BoundaryCondition, NodeType, OutfallProperties};

    #[test]
    fn test_empty_network() {
        let network = Network::new();
        assert_eq!(network.node_count(), 0);
        assert_eq!(network.conduit_count(), 0);
    }

    #[test]
    fn test_add_nodes() {
        let mut network = Network::new();

        let outfall = Node::new_outfall(
            "OUT-1".to_string(),
            100.0,
            OutfallProperties {
                boundary_condition: BoundaryCondition::Free,
                tailwater_elevation: None,
                tidal_curve: None,
            },
        );

        network.add_node(outfall);
        assert_eq!(network.node_count(), 1);
        assert_eq!(network.outfalls().len(), 1);
    }

    #[test]
    fn test_validate_connectivity() {
        let mut network = Network::new();

        // Add nodes
        network.add_node(Node {
            id: "N1".to_string(),
            node_type: NodeType::Inlet,
            name: None,
            invert_elevation: 120.0,
            rim_elevation: Some(125.0),
            coordinates: None,
            junction: None,
            inlet: None,
            outfall: None,
        });

        network.add_node(Node {
            id: "N2".to_string(),
            node_type: NodeType::Outfall,
            name: None,
            invert_elevation: 115.0,
            rim_elevation: None,
            coordinates: None,
            junction: None,
            inlet: None,
            outfall: None,
        });

        // Add valid conduit
        network.add_conduit(Conduit {
            id: "C1".to_string(),
            conduit_type: ConduitType::Pipe,
            name: None,
            from_node: "N1".to_string(),
            to_node: "N2".to_string(),
            length: 100.0,
            upstream_invert: None,
            downstream_invert: None,
            slope: None,
            pipe: Some(PipeProperties {
                shape: PipeShape::Circular,
                diameter: Some(18.0),
                width: None,
                height: None,
                material: None,
                manning_n: 0.013,
                entrance_loss: None,
                exit_loss: None,
                bend_loss: None,
            }),
            gutter: None,
            channel: None,
        });

        // Should validate successfully
        assert!(network.validate_connectivity().is_ok());

        // Add invalid conduit (references non-existent node)
        network.add_conduit(Conduit {
            id: "C2".to_string(),
            conduit_type: ConduitType::Pipe,
            name: None,
            from_node: "N1".to_string(),
            to_node: "N3".to_string(), // Doesn't exist
            length: 100.0,
            upstream_invert: None,
            downstream_invert: None,
            slope: None,
            pipe: None,
            gutter: None,
            channel: None,
        });

        // Should fail validation
        assert!(network.validate_connectivity().is_err());
    }
}
