//! SVG generation utilities for visualization

use std::fmt::Write;

/// SVG builder for creating structured SVG documents
pub struct SvgBuilder {
    width: f64,
    height: f64,
    viewbox: (f64, f64, f64, f64),
    elements: Vec<String>,
}

impl SvgBuilder {
    /// Create a new SVG builder with specified dimensions
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            width,
            height,
            viewbox: (0.0, 0.0, width, height),
            elements: Vec::new(),
        }
    }

    /// Set custom viewbox (min_x, min_y, width, height)
    pub fn viewbox(mut self, min_x: f64, min_y: f64, width: f64, height: f64) -> Self {
        self.viewbox = (min_x, min_y, width, height);
        self
    }

    /// Add a raw SVG element
    pub fn add_element(&mut self, element: String) {
        self.elements.push(element);
    }

    /// Add a circle
    pub fn circle(&mut self, cx: f64, cy: f64, r: f64, fill: &str, stroke: &str, stroke_width: f64) {
        let mut elem = String::new();
        write!(&mut elem,
            r#"<circle cx="{}" cy="{}" r="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            cx, cy, r, fill, stroke, stroke_width
        ).unwrap();
        self.elements.push(elem);
    }

    /// Add a rectangle
    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, fill: &str, stroke: &str, stroke_width: f64) {
        let mut elem = String::new();
        write!(&mut elem,
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            x, y, width, height, fill, stroke, stroke_width
        ).unwrap();
        self.elements.push(elem);
    }

    /// Add a line
    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, stroke_width: f64) {
        let mut elem = String::new();
        write!(&mut elem,
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="{}"/>"#,
            x1, y1, x2, y2, stroke, stroke_width
        ).unwrap();
        self.elements.push(elem);
    }

    /// Add a polyline
    pub fn polyline(&mut self, points: &[(f64, f64)], fill: &str, stroke: &str, stroke_width: f64) {
        let mut points_str = String::new();
        for (i, (x, y)) in points.iter().enumerate() {
            if i > 0 {
                points_str.push(' ');
            }
            write!(&mut points_str, "{},{}", x, y).unwrap();
        }

        let mut elem = String::new();
        write!(&mut elem,
            r#"<polyline points="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            points_str, fill, stroke, stroke_width
        ).unwrap();
        self.elements.push(elem);
    }

    /// Add a polyline with custom stroke-dasharray (for dashed/dotted lines)
    pub fn polyline_dashed(&mut self, points: &[(f64, f64)], fill: &str, stroke: &str, stroke_width: f64, dasharray: &str) {
        let mut points_str = String::new();
        for (i, (x, y)) in points.iter().enumerate() {
            if i > 0 {
                points_str.push(' ');
            }
            write!(&mut points_str, "{},{}", x, y).unwrap();
        }

        let mut elem = String::new();
        write!(&mut elem,
            r#"<polyline points="{}" fill="{}" stroke="{}" stroke-width="{}" stroke-dasharray="{}"/>"#,
            points_str, fill, stroke, stroke_width, dasharray
        ).unwrap();
        self.elements.push(elem);
    }

    /// Add text
    pub fn text(&mut self, x: f64, y: f64, content: &str, font_size: f64, anchor: &str, fill: &str) {
        let mut elem = String::new();
        write!(&mut elem,
            r#"<text x="{}" y="{}" font-size="{}" text-anchor="{}" fill="{}" font-family="Arial, sans-serif">{}</text>"#,
            x, y, font_size, anchor, fill, content
        ).unwrap();
        self.elements.push(elem);
    }

    /// Add a path
    pub fn path(&mut self, d: &str, fill: &str, stroke: &str, stroke_width: f64) {
        let mut elem = String::new();
        write!(&mut elem,
            r#"<path d="{}" fill="{}" stroke="{}" stroke-width="{}"/>"#,
            d, fill, stroke, stroke_width
        ).unwrap();
        self.elements.push(elem);
    }

    /// Add a group with transform
    pub fn group_start(&mut self, transform: Option<&str>, class: Option<&str>) {
        let mut elem = String::from("<g");
        if let Some(t) = transform {
            write!(&mut elem, r#" transform="{}""#, t).unwrap();
        }
        if let Some(c) = class {
            write!(&mut elem, r#" class="{}""#, c).unwrap();
        }
        elem.push('>');
        self.elements.push(elem);
    }

    /// Close a group
    pub fn group_end(&mut self) {
        self.elements.push("</g>".to_string());
    }

    /// Build the final SVG string
    pub fn build(self) -> String {
        let mut svg = String::new();

        writeln!(&mut svg, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
        writeln!(&mut svg,
            r#"<svg width="{}" height="{}" viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            self.width, self.height,
            self.viewbox.0, self.viewbox.1, self.viewbox.2, self.viewbox.3
        ).unwrap();

        // Add style section
        writeln!(&mut svg, r#"<style>"#).unwrap();
        writeln!(&mut svg, r#"  .node-label {{ font: 12px Arial, sans-serif; }}"#).unwrap();
        writeln!(&mut svg, r#"  .inlet {{ fill: #4CAF50; }}"#).unwrap();
        writeln!(&mut svg, r#"  .junction {{ fill: #2196F3; }}"#).unwrap();
        writeln!(&mut svg, r#"  .outfall {{ fill: #F44336; }}"#).unwrap();
        writeln!(&mut svg, r#"  .conduit {{ stroke: #666; fill: none; }}"#).unwrap();
        writeln!(&mut svg, r#"  .hgl-line {{ stroke: #2196F3; stroke-width: 2; fill: none; }}"#).unwrap();
        writeln!(&mut svg, r#"  .egl-line {{ stroke: #FF9800; stroke-width: 2; fill: none; }}"#).unwrap();
        writeln!(&mut svg, r#"  .ground-line {{ stroke: #8B4513; stroke-width: 2; fill: none; }}"#).unwrap();
        writeln!(&mut svg, r#"  .pipe-line {{ stroke: #000; stroke-width: 3; fill: none; }}"#).unwrap();
        writeln!(&mut svg, r#"</style>"#).unwrap();

        // Add all elements
        for element in self.elements {
            writeln!(&mut svg, "{}", element).unwrap();
        }

        writeln!(&mut svg, "</svg>").unwrap();
        svg
    }
}

/// Calculate bounding box for a set of points
pub fn bounding_box(points: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    if points.is_empty() {
        return (0.0, 0.0, 100.0, 100.0);
    }

    let mut min_x = points[0].0;
    let mut max_x = points[0].0;
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    for (x, y) in points.iter().skip(1) {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    (min_x, min_y, max_x, max_y)
}

/// Add padding to bounding box
pub fn add_padding(bbox: (f64, f64, f64, f64), padding_pct: f64) -> (f64, f64, f64, f64) {
    let (min_x, min_y, max_x, max_y) = bbox;
    let width = max_x - min_x;
    let height = max_y - min_y;
    let pad_x = width * padding_pct;
    let pad_y = height * padding_pct;

    (min_x - pad_x, min_y - pad_y, max_x + pad_x, max_y + pad_y)
}
