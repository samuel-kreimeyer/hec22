//! Network plan view visualization
//!
//! Generates a plan (top-down) view of the drainage network showing:
//! - Node locations (inlets, junctions, outfalls)
//! - Conduit connections
//! - Flow directions
//! - Optional labels and annotations

use crate::network::Network;
use crate::node::NodeType;
use crate::visualization::svg::{SvgBuilder, bounding_box, add_padding};
use std::collections::HashMap;

/// Configuration for network plan visualization
#[derive(Debug, Clone)]
pub struct NetworkPlanConfig {
    /// Canvas width in pixels
    pub width: f64,
    /// Canvas height in pixels
    pub height: f64,
    /// Show node labels
    pub show_labels: bool,
    /// Show conduit IDs
    pub show_conduit_labels: bool,
    /// Show flow directions
    pub show_flow_arrows: bool,
    /// Node radius in pixels
    pub node_radius: f64,
    /// Padding percentage around network
    pub padding: f64,
}

impl Default for NetworkPlanConfig {
    fn default() -> Self {
        Self {
            width: 1200.0,
            height: 800.0,
            show_labels: true,
            show_conduit_labels: false,
            show_flow_arrows: true,
            node_radius: 8.0,
            padding: 0.1,
        }
    }
}

/// Network plan view generator
pub struct NetworkPlanView<'a> {
    network: &'a Network,
    config: NetworkPlanConfig,
    node_positions: HashMap<String, (f64, f64)>,
}

impl<'a> NetworkPlanView<'a> {
    /// Create a new network plan view with default configuration
    pub fn new(network: &'a Network) -> Self {
        Self::with_config(network, NetworkPlanConfig::default())
    }

    /// Create a new network plan view with custom configuration
    pub fn with_config(network: &'a Network, config: NetworkPlanConfig) -> Self {
        let node_positions = Self::calculate_node_positions(network);
        Self {
            network,
            config,
            node_positions,
        }
    }

    /// Calculate node positions from network coordinates
    fn calculate_node_positions(network: &Network) -> HashMap<String, (f64, f64)> {
        let mut positions = HashMap::new();

        for node in &network.nodes {
            // Use node coordinates if available, otherwise use auto-layout
            let (x, y) = if let Some(ref coords) = node.coordinates {
                (coords.x.unwrap_or(0.0), coords.y.unwrap_or(0.0))
            } else {
                // Simple auto-layout based on node index
                // This is a placeholder - in a real implementation, you'd use a graph layout algorithm
                (0.0, 0.0)
            };
            positions.insert(node.id.clone(), (x, y));
        }

        positions
    }

    /// Transform coordinates from network space to SVG space
    fn transform_coordinates(&self, positions: &HashMap<String, (f64, f64)>) -> HashMap<String, (f64, f64)> {
        if positions.is_empty() {
            return HashMap::new();
        }

        // Collect all points
        let points: Vec<(f64, f64)> = positions.values().copied().collect();

        // Calculate bounding box
        let bbox = bounding_box(&points);
        let bbox_padded = add_padding(bbox, self.config.padding);

        let (min_x, min_y, max_x, max_y) = bbox_padded;
        let data_width = max_x - min_x;
        let data_height = max_y - min_y;

        // Avoid division by zero
        let data_width = if data_width == 0.0 { 1.0 } else { data_width };
        let data_height = if data_height == 0.0 { 1.0 } else { data_height };

        // Calculate scale to fit in canvas
        let scale_x = self.config.width / data_width;
        let scale_y = self.config.height / data_height;
        let scale = scale_x.min(scale_y);

        // Transform each position
        let mut transformed = HashMap::new();
        for (id, (x, y)) in positions {
            // Flip Y axis (SVG Y increases downward)
            let svg_x = (x - min_x) * scale;
            let svg_y = self.config.height - ((y - min_y) * scale);
            transformed.insert(id.clone(), (svg_x, svg_y));
        }

        transformed
    }

    /// Generate SVG representation
    pub fn to_svg(&self) -> String {
        let mut svg = SvgBuilder::new(self.config.width, self.config.height);

        // Transform coordinates
        let transformed_positions = self.transform_coordinates(&self.node_positions);

        // Draw conduits first (so they appear behind nodes)
        self.draw_conduits(&mut svg, &transformed_positions);

        // Draw nodes
        self.draw_nodes(&mut svg, &transformed_positions);

        svg.build()
    }

    /// Draw conduits
    fn draw_conduits(&self, svg: &mut SvgBuilder, positions: &HashMap<String, (f64, f64)>) {
        for conduit in &self.network.conduits {
            if let (Some(&(x1, y1)), Some(&(x2, y2))) = (
                positions.get(&conduit.from_node),
                positions.get(&conduit.to_node),
            ) {
                // Draw conduit line
                svg.line(x1, y1, x2, y2, "#666", 2.0);

                // Draw flow arrow if enabled
                if self.config.show_flow_arrows {
                    self.draw_arrow(svg, x1, y1, x2, y2);
                }

                // Draw conduit label if enabled
                if self.config.show_conduit_labels {
                    let mid_x = (x1 + x2) / 2.0;
                    let mid_y = (y1 + y2) / 2.0;
                    svg.text(mid_x, mid_y - 5.0, &conduit.id, 10.0, "middle", "#333");
                }
            }
        }
    }

    /// Draw a simple arrow head
    fn draw_arrow(&self, svg: &mut SvgBuilder, x1: f64, y1: f64, x2: f64, y2: f64) {
        let arrow_size = 10.0;

        // Calculate direction
        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = (dx * dx + dy * dy).sqrt();

        if length == 0.0 {
            return;
        }

        // Normalized direction
        let ux = dx / length;
        let uy = dy / length;

        // Arrow position (2/3 along the line)
        let arrow_pos = 0.67;
        let ax = x1 + dx * arrow_pos;
        let ay = y1 + dy * arrow_pos;

        // Arrow head points
        let angle = 25.0_f64.to_radians();
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        // Rotate by +angle
        let p1x = ax - arrow_size * (ux * cos_a - uy * sin_a);
        let p1y = ay - arrow_size * (ux * sin_a + uy * cos_a);

        // Rotate by -angle
        let p2x = ax - arrow_size * (ux * cos_a + uy * sin_a);
        let p2y = ay - arrow_size * (ux * -sin_a + uy * cos_a);

        // Draw arrow head
        svg.polyline(&[(p1x, p1y), (ax, ay), (p2x, p2y)], "none", "#666", 2.0);
    }

    /// Draw nodes
    fn draw_nodes(&self, svg: &mut SvgBuilder, positions: &HashMap<String, (f64, f64)>) {
        for node in &self.network.nodes {
            if let Some(&(x, y)) = positions.get(&node.id) {
                // Determine color based on node type
                let (fill, stroke) = match node.node_type {
                    NodeType::Inlet => ("#4CAF50", "#2E7D32"),
                    NodeType::Junction => ("#2196F3", "#1565C0"),
                    NodeType::Outfall => ("#F44336", "#C62828"),
                };

                // Draw node circle
                svg.circle(x, y, self.config.node_radius, fill, stroke, 2.0);

                // Draw label if enabled
                if self.config.show_labels {
                    svg.text(x, y - self.config.node_radius - 5.0, &node.id, 11.0, "middle", "#000");
                }
            }
        }
    }

    /// Export to file
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let svg_content = self.to_svg();
        std::fs::write(path, svg_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node::{Coordinates, InletProperties, InletType, InletLocation, JunctionProperties, OutfallProperties, BoundaryCondition};

    #[test]
    fn test_network_plan_basic() {
        let mut network = Network::new();

        // Add nodes with coordinates
        let mut node1 = Node::new_inlet(
            "IN-001".to_string(),
            100.0,
            105.0,
            InletProperties {
                inlet_type: InletType::Combination,
                location: InletLocation::OnGrade,
                grate: None,
                curb_opening: None,
                local_depression: None,
                clogging_factor: None,
            },
        );
        node1.coordinates = Some(Coordinates {
            x: Some(0.0),
            y: Some(0.0),
            latitude: None,
            longitude: None,
        });

        let mut node2 = Node::new_junction(
            "MH-001".to_string(),
            99.0,
            104.0,
            JunctionProperties {
                diameter: Some(4.0),
                sump_depth: None,
                loss_coefficient: Some(0.15),
                benching: None,
                drop_structure: None,
            },
        );
        node2.coordinates = Some(Coordinates {
            x: Some(100.0),
            y: Some(0.0),
            latitude: None,
            longitude: None,
        });

        let mut node3 = Node::new_outfall(
            "OUT-001".to_string(),
            98.0,
            OutfallProperties {
                boundary_condition: BoundaryCondition::Free,
                tailwater_elevation: None,
                tidal_curve: None,
            },
        );
        node3.coordinates = Some(Coordinates {
            x: Some(200.0),
            y: Some(0.0),
            latitude: None,
            longitude: None,
        });

        network.add_node(node1);
        network.add_node(node2);
        network.add_node(node3);

        // Generate SVG
        let plan = NetworkPlanView::new(&network);
        let svg = plan.to_svg();

        // Basic checks
        assert!(svg.contains("<svg"));
        assert!(svg.contains("IN-001"));
        assert!(svg.contains("MH-001"));
        assert!(svg.contains("OUT-001"));
    }
}
