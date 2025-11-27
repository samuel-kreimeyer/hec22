//! Visualization module for HEC-22 drainage networks
//!
//! This module provides SVG-based visualization capabilities for:
//! - Network plan views (nodes and conduits in plan)
//! - Profile views (HGL/EGL elevation profiles along pipe runs)
//! - Interactive HTML viewers
//!
//! # Examples
//!
//! ```no_run
//! use hec22::visualization::{NetworkPlanView, ProfileView};
//! use hec22::network::Network;
//!
//! let network = Network::new();
//! // ... build network ...
//!
//! // Generate network plan view
//! let plan_svg = NetworkPlanView::new(&network).to_svg();
//! std::fs::write("network_plan.svg", plan_svg)?;
//!
//! // Generate profile view
//! let profile_svg = ProfileView::new(&network, &["IN-001", "MH-001", "OUT-001"]).to_svg();
//! std::fs::write("profile.svg", profile_svg)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

pub mod network_plan;
pub mod profile;
pub mod svg;
pub mod html;

pub use network_plan::NetworkPlanView;
pub use profile::ProfileView;
pub use html::HtmlViewer;
