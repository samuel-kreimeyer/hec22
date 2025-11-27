//! HTML wrapper for interactive visualization
//!
//! Generates standalone HTML files that embed SVG visualizations
//! with interactive features like pan, zoom, and tooltips.

use crate::analysis::Analysis;
use crate::network::Network;
use crate::visualization::{NetworkPlanView, ProfileView};

/// HTML viewer generator
pub struct HtmlViewer<'a> {
    network: &'a Network,
    title: String,
}

impl<'a> HtmlViewer<'a> {
    /// Create a new HTML viewer
    pub fn new(network: &'a Network) -> Self {
        Self {
            network,
            title: "HEC-22 Drainage Network Visualization".to_string(),
        }
    }

    /// Set custom title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Generate HTML page with network plan view
    pub fn generate_plan_view(&self) -> String {
        let plan_view = NetworkPlanView::new(self.network);
        let svg_content = plan_view.to_svg();

        self.create_html_template(&svg_content, "Network Plan View")
    }

    /// Generate HTML page with profile view
    pub fn generate_profile_view(&self, node_path: &[&str]) -> String {
        let profile_view = ProfileView::new(self.network, node_path);
        let svg_content = profile_view.to_svg();

        self.create_html_template(&svg_content, "Profile View")
    }

    /// Generate HTML page with profile view including HGL/EGL from analysis
    pub fn generate_profile_view_with_analysis(
        &self,
        node_path: &[&str],
        analysis: &Analysis,
    ) -> String {
        let profile_view = ProfileView::with_analysis(self.network, node_path, analysis);
        let svg_content = profile_view.to_svg();

        self.create_html_template(&svg_content, "Profile View (HGL/EGL)")
    }

    /// Generate HTML page with both views
    pub fn generate_combined_view(&self, node_path: &[&str]) -> String {
        let plan_view = NetworkPlanView::new(self.network);
        let plan_svg = plan_view.to_svg();

        let profile_view = ProfileView::new(self.network, node_path);
        let profile_svg = profile_view.to_svg();

        self.create_combined_html(&plan_svg, &profile_svg)
    }

    /// Generate HTML page with both views including HGL/EGL from analysis
    pub fn generate_combined_view_with_analysis(
        &self,
        node_path: &[&str],
        analysis: &Analysis,
    ) -> String {
        let plan_view = NetworkPlanView::new(self.network);
        let plan_svg = plan_view.to_svg();

        let profile_view = ProfileView::with_analysis(self.network, node_path, analysis);
        let profile_svg = profile_view.to_svg();

        self.create_combined_html(&plan_svg, &profile_svg)
    }

    /// Create HTML template with single SVG
    fn create_html_template(&self, svg_content: &str, view_title: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - {}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }}

        .container {{
            max-width: 1600px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
            overflow: hidden;
        }}

        header {{
            background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
            color: white;
            padding: 30px 40px;
        }}

        h1 {{
            font-size: 28px;
            margin-bottom: 8px;
        }}

        .subtitle {{
            font-size: 14px;
            opacity: 0.9;
            font-weight: 300;
        }}

        .content {{
            padding: 40px;
            background: #f8f9fa;
        }}

        .svg-container {{
            background: white;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
            overflow: auto;
        }}

        svg {{
            display: block;
            margin: 0 auto;
            max-width: 100%;
            height: auto;
        }}

        .info-panel {{
            margin-top: 30px;
            padding: 20px;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }}

        .info-panel h2 {{
            font-size: 18px;
            margin-bottom: 15px;
            color: #2c3e50;
        }}

        .info-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
        }}

        .info-item {{
            padding: 12px;
            background: #f8f9fa;
            border-radius: 6px;
            border-left: 4px solid #667eea;
        }}

        .info-label {{
            font-size: 12px;
            color: #6c757d;
            text-transform: uppercase;
            letter-spacing: 0.5px;
            margin-bottom: 4px;
        }}

        .info-value {{
            font-size: 18px;
            font-weight: 600;
            color: #2c3e50;
        }}

        footer {{
            padding: 20px 40px;
            background: #f8f9fa;
            border-top: 1px solid #dee2e6;
            text-align: center;
            color: #6c757d;
            font-size: 13px;
        }}

        .controls {{
            margin-bottom: 20px;
            padding: 15px;
            background: white;
            border-radius: 8px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }}

        .btn {{
            padding: 8px 16px;
            margin-right: 10px;
            border: none;
            border-radius: 4px;
            background: #667eea;
            color: white;
            cursor: pointer;
            font-size: 14px;
            transition: background 0.3s;
        }}

        .btn:hover {{
            background: #5568d3;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>{}</h1>
            <p class="subtitle">{}</p>
        </header>

        <div class="content">
            <div class="controls">
                <button class="btn" onclick="zoomIn()">Zoom In</button>
                <button class="btn" onclick="zoomOut()">Zoom Out</button>
                <button class="btn" onclick="resetZoom()">Reset</button>
                <button class="btn" onclick="downloadSvg()">Download SVG</button>
            </div>

            <div class="svg-container" id="svg-container">
                {}
            </div>

            <div class="info-panel">
                <h2>Network Information</h2>
                <div class="info-grid">
                    <div class="info-item">
                        <div class="info-label">Total Nodes</div>
                        <div class="info-value">{}</div>
                    </div>
                    <div class="info-item">
                        <div class="info-label">Total Conduits</div>
                        <div class="info-value">{}</div>
                    </div>
                    <div class="info-item">
                        <div class="info-label">View Type</div>
                        <div class="info-value">{}</div>
                    </div>
                </div>
            </div>
        </div>

        <footer>
            Generated by HEC-22 Urban Drainage Analysis System | Based on FHWA HEC-22 (4th Edition, 2024)
        </footer>
    </div>

    <script>
        let zoomLevel = 1.0;
        const zoomStep = 0.2;

        function zoomIn() {{
            zoomLevel += zoomStep;
            applyZoom();
        }}

        function zoomOut() {{
            zoomLevel = Math.max(0.2, zoomLevel - zoomStep);
            applyZoom();
        }}

        function resetZoom() {{
            zoomLevel = 1.0;
            applyZoom();
        }}

        function applyZoom() {{
            const svg = document.querySelector('#svg-container svg');
            if (svg) {{
                svg.style.transform = `scale(${{zoomLevel}})`;
                svg.style.transformOrigin = 'top left';
            }}
        }}

        function downloadSvg() {{
            const svg = document.querySelector('#svg-container svg');
            if (svg) {{
                const svgData = new XMLSerializer().serializeToString(svg);
                const blob = new Blob([svgData], {{ type: 'image/svg+xml' }});
                const url = URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = 'network_visualization.svg';
                document.body.appendChild(a);
                a.click();
                document.body.removeChild(a);
                URL.revokeObjectURL(url);
            }}
        }}

        // Enable pan with mouse drag
        let isPanning = false;
        let startX, startY, scrollLeft, scrollTop;

        const container = document.querySelector('#svg-container');

        container.addEventListener('mousedown', (e) => {{
            isPanning = true;
            startX = e.pageX - container.offsetLeft;
            startY = e.pageY - container.offsetTop;
            scrollLeft = container.scrollLeft;
            scrollTop = container.scrollTop;
            container.style.cursor = 'grabbing';
        }});

        container.addEventListener('mouseleave', () => {{
            isPanning = false;
            container.style.cursor = 'default';
        }});

        container.addEventListener('mouseup', () => {{
            isPanning = false;
            container.style.cursor = 'default';
        }});

        container.addEventListener('mousemove', (e) => {{
            if (!isPanning) return;
            e.preventDefault();
            const x = e.pageX - container.offsetLeft;
            const y = e.pageY - container.offsetTop;
            const walkX = (x - startX) * 1.5;
            const walkY = (y - startY) * 1.5;
            container.scrollLeft = scrollLeft - walkX;
            container.scrollTop = scrollTop - walkY;
        }});
    </script>
</body>
</html>"#,
            self.title,
            view_title,
            self.title,
            view_title,
            svg_content,
            self.network.nodes.len(),
            self.network.conduits.len(),
            view_title
        )
    }

    /// Create HTML with both plan and profile views
    fn create_combined_html(&self, plan_svg: &str, profile_svg: &str) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }}

        .container {{
            max-width: 1800px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
            overflow: hidden;
        }}

        header {{
            background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
            color: white;
            padding: 30px 40px;
        }}

        h1 {{
            font-size: 28px;
            margin-bottom: 8px;
        }}

        .subtitle {{
            font-size: 14px;
            opacity: 0.9;
            font-weight: 300;
        }}

        .content {{
            padding: 40px;
            background: #f8f9fa;
        }}

        .view-section {{
            margin-bottom: 30px;
        }}

        .section-title {{
            font-size: 20px;
            margin-bottom: 15px;
            color: #2c3e50;
            font-weight: 600;
        }}

        .svg-container {{
            background: white;
            border-radius: 8px;
            padding: 20px;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
            overflow: auto;
        }}

        svg {{
            display: block;
            margin: 0 auto;
            max-width: 100%;
            height: auto;
        }}

        footer {{
            padding: 20px 40px;
            background: #f8f9fa;
            border-top: 1px solid #dee2e6;
            text-align: center;
            color: #6c757d;
            font-size: 13px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>{}</h1>
            <p class="subtitle">Network Plan and Profile Views</p>
        </header>

        <div class="content">
            <div class="view-section">
                <h2 class="section-title">Network Plan View</h2>
                <div class="svg-container">
                    {}
                </div>
            </div>

            <div class="view-section">
                <h2 class="section-title">Profile View</h2>
                <div class="svg-container">
                    {}
                </div>
            </div>
        </div>

        <footer>
            Generated by HEC-22 Urban Drainage Analysis System | Based on FHWA HEC-22 (4th Edition, 2024)
        </footer>
    </div>
</body>
</html>"#,
            self.title, self.title, plan_svg, profile_svg
        )
    }

    /// Save HTML to file
    pub fn save_to_file(&self, path: &str, content: &str) -> std::io::Result<()> {
        std::fs::write(path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::{Node, NodeType};

    #[test]
    fn test_html_viewer_basic() {
        let mut network = Network::new();

        let mut node1 = Node::new("IN-001".to_string(), NodeType::Inlet);
        node1.x = Some(0.0);
        node1.y = Some(0.0);
        network.add_node(node1);

        let viewer = HtmlViewer::new(&network);
        let html = viewer.generate_plan_view();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("HEC-22"));
        assert!(html.contains("<svg"));
    }
}
