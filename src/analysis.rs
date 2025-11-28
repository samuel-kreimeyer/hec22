//! Analysis results and design criteria
//!
//! Defines design constraints, computed results, and violation reporting
//! for drainage network analysis.

use serde::{Deserialize, Serialize};

/// Design criteria and constraints
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesignCriteria {
    /// Gutter spread criteria (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gutterSpread")]
    pub gutter_spread: Option<GutterSpreadCriteria>,

    /// HGL (Hydraulic Grade Line) criteria (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hglCriteria")]
    pub hgl_criteria: Option<HglCriteria>,

    /// Velocity criteria (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<VelocityCriteria>,

    /// Cover depth criteria (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<CoverCriteria>,

    /// Capacity criteria (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<CapacityCriteria>,
}

/// Gutter spread design criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GutterSpreadCriteria {
    /// Maximum allowable gutter spread (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxSpread")]
    pub max_spread: Option<f64>,

    /// Maximum spread for local streets (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxSpreadLocalStreet")]
    pub max_spread_local_street: Option<f64>,

    /// Maximum spread for collector streets (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxSpreadCollectorStreet")]
    pub max_spread_collector_street: Option<f64>,

    /// Maximum spread for arterial streets (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxSpreadArterialStreet")]
    pub max_spread_arterial_street: Option<f64>,
}

/// HGL design criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HglCriteria {
    /// Minimum distance HGL must be below rim elevation (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxHglBelowRim")]
    pub max_hgl_below_rim: Option<f64>,

    /// Whether surcharge (HGL > top of pipe) is allowed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allowSurcharge")]
    pub allow_surcharge: Option<bool>,
}

/// Velocity design criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VelocityCriteria {
    /// Minimum velocity for self-cleansing (ft/s or m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minVelocity")]
    pub min_velocity: Option<f64>,

    /// Maximum velocity to prevent scour (ft/s or m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxVelocity")]
    pub max_velocity: Option<f64>,
}

/// Cover depth criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoverCriteria {
    /// Minimum cover over pipe (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minCover")]
    pub min_cover: Option<f64>,
}

/// Capacity criteria
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapacityCriteria {
    /// Minimum Q_available/Q_design ratio
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minCapacityRatio")]
    pub min_capacity_ratio: Option<f64>,
}

/// Analysis results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Analysis {
    /// Analysis method used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<AnalysisMethod>,

    /// ID of the design storm analyzed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "designStormId")]
    pub design_storm_id: Option<String>,

    /// When analysis was performed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    /// Solver information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solver: Option<SolverInfo>,

    /// Computed results at each node
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "nodeResults")]
    pub node_results: Option<Vec<NodeResult>>,

    /// Computed results for each conduit
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conduitResults")]
    pub conduit_results: Option<Vec<ConduitResult>>,

    /// Computed runoff from drainage areas
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "drainageAreaResults")]
    pub drainage_area_results: Option<Vec<DrainageAreaResult>>,

    /// Design criteria violations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violations: Option<Vec<Violation>>,
}

/// Analysis method
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AnalysisMethod {
    /// Rational method (Q = C × i × A)
    Rational,
    /// SCS/NRCS method
    Scs,
    /// Kinematic wave routing
    KinematicWave,
    /// Full dynamic wave routing
    DynamicWave,
}

/// Solver information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SolverInfo {
    /// Solver name
    pub name: String,

    /// Solver version
    pub version: String,
}

/// Computed results at a node
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NodeResult {
    /// Node ID
    #[serde(rename = "nodeId")]
    pub node_id: String,

    /// Hydraulic grade line elevation (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgl: Option<f64>,

    /// Energy grade line elevation (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egl: Option<f64>,

    /// Water depth at node (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,

    /// Velocity at node (ft/s or m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<f64>,

    /// Whether node is flooding (HGL > rim)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flooding: Option<bool>,

    /// Pressure head (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pressureHead")]
    pub pressure_head: Option<f64>,

    /// Junction loss at this node (ft or m) - only for junctions
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "junctionLoss")]
    pub junction_loss: Option<f64>,
}

/// Computed results for a conduit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConduitResult {
    /// Conduit ID
    #[serde(rename = "conduitId")]
    pub conduit_id: String,

    /// Flow rate through conduit (cfs or cms)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<f64>,

    /// Average velocity (ft/s or m/s)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub velocity: Option<f64>,

    /// Flow depth (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<f64>,

    /// Fraction of capacity used (Q/Q_full)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "capacityUsed")]
    pub capacity_used: Option<f64>,

    /// Froude number (flow regime indicator)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "froudeNumber")]
    pub froude_number: Option<f64>,

    /// Flow regime classification
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "flowRegime")]
    pub flow_regime: Option<FlowRegime>,

    /// Head loss breakdown
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headloss: Option<HeadLoss>,
}

/// Flow regime
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FlowRegime {
    /// Subcritical flow (Fr < 1)
    Subcritical,
    /// Critical flow (Fr ≈ 1)
    Critical,
    /// Supercritical flow (Fr > 1)
    Supercritical,
}

/// Head loss components
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeadLoss {
    /// Friction loss (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friction: Option<f64>,

    /// Entrance loss (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrance: Option<f64>,

    /// Exit loss (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit: Option<f64>,

    /// Bend loss (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bend: Option<f64>,

    /// Total head loss (ft or m)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

/// Computed runoff from a drainage area
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DrainageAreaResult {
    /// Drainage area ID
    #[serde(rename = "drainageAreaId")]
    pub drainage_area_id: String,

    /// Peak runoff flow rate (cfs or cms)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "peakFlow")]
    pub peak_flow: Option<f64>,

    /// Time of peak flow (minutes from storm start)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timeOfPeak")]
    pub time_of_peak: Option<f64>,

    /// Total runoff volume
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalVolume")]
    pub total_volume: Option<f64>,

    /// Rainfall intensity used (in/hr or mm/hr)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intensity: Option<f64>,
}

/// Design criteria violation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Violation {
    /// Violation type
    #[serde(rename = "type")]
    pub violation_type: ViolationType,

    /// Severity level
    pub severity: Severity,

    /// ID of node or conduit with violation
    #[serde(rename = "elementId")]
    pub element_id: String,

    /// Human-readable violation description
    pub message: String,

    /// Actual computed value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,

    /// Design limit that was exceeded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
}

/// Violation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ViolationType {
    /// Gutter spread violation
    Spread,
    /// HGL elevation violation
    Hgl,
    /// Velocity violation
    Velocity,
    /// Cover depth violation
    Cover,
    /// Capacity violation
    Capacity,
    /// Flooding violation
    Flooding,
}

/// Severity level
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Error (must be fixed)
    Error,
    /// Warning (should be reviewed)
    Warning,
    /// Information (FYI)
    Info,
}

impl Analysis {
    /// Create a new empty analysis result
    pub fn new(method: AnalysisMethod, design_storm_id: String) -> Self {
        Self {
            method: Some(method),
            design_storm_id: Some(design_storm_id),
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
            solver: None,
            node_results: Some(Vec::new()),
            conduit_results: Some(Vec::new()),
            drainage_area_results: Some(Vec::new()),
            violations: Some(Vec::new()),
        }
    }

    /// Add a violation to the analysis
    pub fn add_violation(&mut self, violation: Violation) {
        if let Some(ref mut violations) = self.violations {
            violations.push(violation);
        } else {
            self.violations = Some(vec![violation]);
        }
    }

    /// Get all violations of a specific type
    pub fn get_violations_by_type(&self, violation_type: ViolationType) -> Vec<&Violation> {
        self.violations
            .as_ref()
            .map(|v| {
                v.iter()
                    .filter(|viol| viol.violation_type == violation_type)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get all errors (severity = Error)
    pub fn get_errors(&self) -> Vec<&Violation> {
        self.violations
            .as_ref()
            .map(|v| {
                v.iter()
                    .filter(|viol| viol.severity == Severity::Error)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check if analysis has any errors
    pub fn has_errors(&self) -> bool {
        self.violations
            .as_ref()
            .map(|v| v.iter().any(|viol| viol.severity == Severity::Error))
            .unwrap_or(false)
    }
}

impl Violation {
    /// Create a new HGL violation
    pub fn hgl_violation(
        element_id: String,
        hgl: f64,
        rim: f64,
        severity: Severity,
    ) -> Self {
        Self {
            violation_type: ViolationType::Hgl,
            severity,
            element_id: element_id.clone(),
            message: format!(
                "HGL at {:.2} ft is {:.2} ft above rim elevation of {:.2} ft",
                hgl,
                hgl - rim,
                rim
            ),
            value: Some(hgl),
            limit: Some(rim),
        }
    }

    /// Create a new spread violation
    pub fn spread_violation(
        element_id: String,
        spread: f64,
        max_spread: f64,
        severity: Severity,
    ) -> Self {
        Self {
            violation_type: ViolationType::Spread,
            severity,
            element_id,
            message: format!(
                "Gutter spread of {:.1} ft exceeds maximum allowable spread of {:.1} ft",
                spread, max_spread
            ),
            value: Some(spread),
            limit: Some(max_spread),
        }
    }

    /// Create a new capacity violation
    pub fn capacity_violation(
        element_id: String,
        capacity_used: f64,
        severity: Severity,
    ) -> Self {
        Self {
            violation_type: ViolationType::Capacity,
            severity,
            element_id: element_id.clone(),
            message: format!(
                "Pipe {} is operating at {:.0}% capacity (undersized)",
                element_id,
                capacity_used * 100.0
            ),
            value: Some(capacity_used),
            limit: Some(1.0),
        }
    }
}

// Note: Using chrono for timestamps. Add to Cargo.toml if not present:
// chrono = "0.4"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hgl_violation() {
        let violation = Violation::hgl_violation(
            "MH-001".to_string(),
            125.5,
            125.0,
            Severity::Error,
        );

        assert_eq!(violation.violation_type, ViolationType::Hgl);
        assert_eq!(violation.severity, Severity::Error);
        assert_eq!(violation.value, Some(125.5));
        assert_eq!(violation.limit, Some(125.0));
    }

    #[test]
    fn test_analysis_add_violation() {
        let mut analysis = Analysis::new(
            AnalysisMethod::Rational,
            "storm-10yr".to_string(),
        );

        let violation = Violation::spread_violation(
            "G-101".to_string(),
            12.5,
            10.0,
            Severity::Warning,
        );

        analysis.add_violation(violation);

        assert_eq!(analysis.violations.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_filter_violations() {
        let mut analysis = Analysis::new(
            AnalysisMethod::Rational,
            "storm-10yr".to_string(),
        );

        analysis.add_violation(Violation::hgl_violation(
            "MH-001".to_string(),
            125.5,
            125.0,
            Severity::Error,
        ));

        analysis.add_violation(Violation::spread_violation(
            "G-101".to_string(),
            12.5,
            10.0,
            Severity::Warning,
        ));

        let hgl_violations = analysis.get_violations_by_type(ViolationType::Hgl);
        assert_eq!(hgl_violations.len(), 1);

        let errors = analysis.get_errors();
        assert_eq!(errors.len(), 1);

        assert!(analysis.has_errors());
    }
}
