//! Profile view visualization
//!
//! Generates elevation profile views showing:
//! - Hydraulic Grade Line (HGL)
//! - Energy Grade Line (EGL)
//! - Ground/rim elevations
//! - Pipe inverts and slopes
//! - Node locations

use crate::network::Network;
use crate::node::Node;
use crate::visualization::svg::SvgBuilder;
use std::collections::HashMap;

/// Configuration for profile visualization
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    /// Canvas width in pixels
    pub width: f64,
    /// Canvas height in pixels
    pub height: f64,
    /// Show HGL line
    pub show_hgl: bool,
    /// Show EGL line
    pub show_egl: bool,
    /// Show ground line
    pub show_ground: bool,
    /// Show pipe invert
    pub show_pipe: bool,
    /// Show node labels
    pub show_labels: bool,
    /// Vertical exaggeration factor
    pub vertical_exaggeration: f64,
    /// Margin in pixels
    pub margin: f64,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            width: 1400.0,
            height: 600.0,
            show_hgl: true,
            show_egl: true,
            show_ground: true,
            show_pipe: true,
            show_labels: true,
            vertical_exaggeration: 1.0,
            margin: 60.0,
        }
    }
}

/// Profile data point
#[derive(Debug, Clone)]
struct ProfilePoint {
    station: f64,
    node_id: String,
    invert_elev: f64,
    rim_elev: Option<f64>,
    // HGL and EGL will be added from analysis results in future enhancement
}

/// Profile view generator
pub struct ProfileView<'a> {
    network: &'a Network,
    config: ProfileConfig,
    node_path: Vec<String>,
    profile_points: Vec<ProfilePoint>,
}

impl<'a> ProfileView<'a> {
    /// Create a new profile view with default configuration
    ///
    /// # Arguments
    /// * `network` - The drainage network
    /// * `node_path` - Ordered list of node IDs to include in the profile
    pub fn new(network: &'a Network, node_path: &[&str]) -> Self {
        Self::with_config(network, node_path, ProfileConfig::default())
    }

    /// Create a new profile view with custom configuration
    pub fn with_config(network: &'a Network, node_path: &[&str], config: ProfileConfig) -> Self {
        let node_path: Vec<String> = node_path.iter().map(|s| s.to_string()).collect();
        let profile_points = Self::build_profile_points(network, &node_path);

        Self {
            network,
            config,
            node_path,
            profile_points,
        }
    }

    /// Build profile points from network and node path
    fn build_profile_points(network: &Network, node_path: &[String]) -> Vec<ProfilePoint> {
        let mut points = Vec::new();
        let mut cumulative_station = 0.0;

        // Create node lookup
        let node_map: HashMap<&str, &Node> = network.nodes
            .iter()
            .map(|n| (n.id.as_str(), n))
            .collect();

        for (i, node_id) in node_path.iter().enumerate() {
            if let Some(node) = node_map.get(node_id.as_str()) {
                // Calculate station (cumulative distance)
                if i > 0 {
                    // Find conduit connecting previous node to this node
                    let prev_node_id = &node_path[i - 1];
                    if let Some(conduit) = network.conduits.iter().find(|c|
                        (&c.from_node == prev_node_id && &c.to_node == node_id) ||
                        (&c.from_node == node_id && &c.to_node == prev_node_id)
                    ) {
                        cumulative_station += conduit.length;
                    }
                }

                points.push(ProfilePoint {
                    station: cumulative_station,
                    node_id: node.id.clone(),
                    invert_elev: node.invert_elevation,
                    rim_elev: node.rim_elevation,
                });
            }
        }

        points
    }

    /// Generate SVG representation
    pub fn to_svg(&self) -> String {
        let mut svg = SvgBuilder::new(self.config.width, self.config.height);

        if self.profile_points.is_empty() {
            return svg.build();
        }

        // Calculate elevation range
        let (min_elev, max_elev) = self.calculate_elevation_range();

        // Draw title
        self.draw_title(&mut svg);

        // Draw axes and grid
        self.draw_axes(&mut svg, min_elev, max_elev);

        // Draw profile elements
        if self.config.show_pipe {
            self.draw_pipe_profile(&mut svg, min_elev, max_elev);
        }
        if self.config.show_ground {
            self.draw_ground_line(&mut svg, min_elev, max_elev);
        }
        if self.config.show_hgl {
            self.draw_hgl(&mut svg, min_elev, max_elev);
        }
        if self.config.show_egl {
            self.draw_egl(&mut svg, min_elev, max_elev);
        }

        // Draw node markers
        self.draw_node_markers(&mut svg, min_elev, max_elev);

        // Draw legend
        self.draw_legend(&mut svg);

        svg.build()
    }

    /// Calculate elevation range for scaling
    fn calculate_elevation_range(&self) -> (f64, f64) {
        let mut min_elev = f64::INFINITY;
        let mut max_elev = f64::NEG_INFINITY;

        for point in &self.profile_points {
            min_elev = min_elev.min(point.invert_elev);
            if let Some(elev) = point.rim_elev {
                max_elev = max_elev.max(elev);
            } else {
                // If no rim elevation, use invert + some height
                max_elev = max_elev.max(point.invert_elev + 5.0);
            }
        }

        // Handle edge case where all points are at same elevation
        if (max_elev - min_elev).abs() < 0.1 {
            max_elev = min_elev + 10.0;
        }

        // Add some padding
        let range = max_elev - min_elev;
        let padding = range * 0.1;

        (min_elev - padding, max_elev + padding)
    }

    /// Transform station and elevation to SVG coordinates
    fn transform(&self, station: f64, elevation: f64, min_elev: f64, max_elev: f64) -> (f64, f64) {
        let max_station = self.profile_points.last().map(|p| p.station).unwrap_or(100.0);

        let plot_width = self.config.width - 2.0 * self.config.margin;
        let plot_height = self.config.height - 2.0 * self.config.margin - 40.0; // Extra space for title

        let x = self.config.margin + (station / max_station) * plot_width;

        // Apply vertical exaggeration
        let elev_range = max_elev - min_elev;
        let normalized_elev = (elevation - min_elev) / elev_range;
        let y = self.config.height - self.config.margin - (normalized_elev * plot_height);

        (x, y)
    }

    /// Draw title
    fn draw_title(&self, svg: &mut SvgBuilder) {
        svg.text(
            self.config.width / 2.0,
            25.0,
            "Profile View - HGL/EGL Elevations",
            16.0,
            "middle",
            "#000"
        );
    }

    /// Draw axes and grid
    fn draw_axes(&self, svg: &mut SvgBuilder, min_elev: f64, max_elev: f64) {
        let plot_width = self.config.width - 2.0 * self.config.margin;
        let plot_height = self.config.height - 2.0 * self.config.margin - 40.0;

        let x_start = self.config.margin;
        let y_start = self.config.height - self.config.margin;
        let y_end = self.config.margin + 40.0;

        // Draw axes
        svg.line(x_start, y_start, x_start + plot_width, y_start, "#000", 2.0);
        svg.line(x_start, y_start, x_start, y_end, "#000", 2.0);

        // Draw elevation labels
        svg.text(15.0, (y_start + y_end) / 2.0, "Elevation (ft)", 12.0, "middle", "#000");

        // Draw station label
        svg.text(
            self.config.width / 2.0,
            self.config.height - 15.0,
            "Station (ft)",
            12.0,
            "middle",
            "#000"
        );

        // Draw grid lines and elevation ticks
        let num_ticks = 10;
        for i in 0..=num_ticks {
            let elev = min_elev + (max_elev - min_elev) * (i as f64 / num_ticks as f64);
            let (_, y) = self.transform(0.0, elev, min_elev, max_elev);

            // Grid line
            svg.line(x_start, y, x_start + plot_width, y, "#ddd", 1.0);

            // Elevation label
            svg.text(x_start - 10.0, y + 4.0, &format!("{:.1}", elev), 10.0, "end", "#000");
        }
    }

    /// Draw pipe profile (invert line)
    fn draw_pipe_profile(&self, svg: &mut SvgBuilder, min_elev: f64, max_elev: f64) {
        let mut points = Vec::new();

        for point in &self.profile_points {
            let (x, y) = self.transform(point.station, point.invert_elev, min_elev, max_elev);
            points.push((x, y));
        }

        if points.len() >= 2 {
            svg.polyline(&points, "none", "#000", 3.0);
        }
    }

    /// Draw ground line (rim elevations)
    fn draw_ground_line(&self, svg: &mut SvgBuilder, min_elev: f64, max_elev: f64) {
        let mut points = Vec::new();

        for point in &self.profile_points {
            if let Some(rim) = point.rim_elev {
                let (x, y) = self.transform(point.station, rim, min_elev, max_elev);
                points.push((x, y));
            }
        }

        if points.len() >= 2 {
            svg.polyline(&points, "none", "#8B4513", 2.0);
        }
    }

    /// Draw HGL line
    /// Note: HGL data will be integrated from analysis results in future enhancement
    fn draw_hgl(&self, _svg: &mut SvgBuilder, _min_elev: f64, _max_elev: f64) {
        // TODO: Get HGL from analysis results
        // For now, this is a placeholder
    }

    /// Draw EGL line
    /// Note: EGL data will be integrated from analysis results in future enhancement
    fn draw_egl(&self, _svg: &mut SvgBuilder, _min_elev: f64, _max_elev: f64) {
        // TODO: Get EGL from analysis results
        // For now, this is a placeholder
    }

    /// Draw node markers
    fn draw_node_markers(&self, svg: &mut SvgBuilder, min_elev: f64, max_elev: f64) {
        for point in &self.profile_points {
            let (x, y) = self.transform(point.station, point.invert_elev, min_elev, max_elev);

            // Draw node marker
            svg.circle(x, y, 4.0, "#000", "#000", 1.0);

            // Draw node label if enabled
            if self.config.show_labels {
                svg.text(x, y - 10.0, &point.node_id, 10.0, "middle", "#000");

                // Draw station label
                svg.text(
                    x,
                    self.config.height - self.config.margin + 20.0,
                    &format!("{:.0}", point.station),
                    9.0,
                    "middle",
                    "#666"
                );
            }
        }
    }

    /// Draw legend
    fn draw_legend(&self, svg: &mut SvgBuilder) {
        let legend_x = self.config.width - 150.0;
        let legend_y = 60.0;
        let line_height = 20.0;

        svg.text(legend_x, legend_y, "Legend:", 12.0, "start", "#000");

        let mut y_offset = legend_y + line_height;

        if self.config.show_egl {
            svg.line(legend_x, y_offset, legend_x + 30.0, y_offset, "#FF9800", 2.0);
            svg.text(legend_x + 40.0, y_offset + 4.0, "EGL", 11.0, "start", "#000");
            y_offset += line_height;
        }

        if self.config.show_hgl {
            svg.line(legend_x, y_offset, legend_x + 30.0, y_offset, "#2196F3", 2.0);
            svg.text(legend_x + 40.0, y_offset + 4.0, "HGL", 11.0, "start", "#000");
            y_offset += line_height;
        }

        if self.config.show_ground {
            svg.line(legend_x, y_offset, legend_x + 30.0, y_offset, "#8B4513", 2.0);
            svg.text(legend_x + 40.0, y_offset + 4.0, "Ground", 11.0, "start", "#000");
            y_offset += line_height;
        }

        if self.config.show_pipe {
            svg.line(legend_x, y_offset, legend_x + 30.0, y_offset, "#000", 3.0);
            svg.text(legend_x + 40.0, y_offset + 4.0, "Pipe Invert", 11.0, "start", "#000");
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
    use crate::conduit::{Conduit, ConduitType, PipeProperties, PipeShape, PipeMaterial};
    use crate::node::{InletProperties, InletType, InletLocation, JunctionProperties};

    #[test]
    fn test_profile_view_basic() {
        let mut network = Network::new();

        // Add nodes
        let node1 = Node::new_inlet(
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

        let node2 = Node::new_junction(
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

        network.add_node(node1);
        network.add_node(node2);

        // Add conduit
        let conduit = Conduit {
            id: "C-001".to_string(),
            from_node: "IN-001".to_string(),
            to_node: "MH-001".to_string(),
            conduit_type: ConduitType::Pipe,
            length: 100.0,
            slope: Some(0.01),
            manning_n: 0.013,
            upstream_invert: Some(100.0),
            downstream_invert: Some(99.0),
            pipe: Some(PipeProperties {
                shape: PipeShape::Circular,
                diameter: Some(1.5),
                width: None,
                height: None,
                material: PipeMaterial::Rcp,
                barrel_count: Some(1),
            }),
            gutter: None,
        };
        network.add_conduit(conduit);

        // Generate profile
        let profile = ProfileView::new(&network, &["IN-001", "MH-001"]);
        let svg = profile.to_svg();

        // Basic checks
        assert!(svg.contains("<svg"));
        assert!(svg.contains("Profile View"));
    }
}
