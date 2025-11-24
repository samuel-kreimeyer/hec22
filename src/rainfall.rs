//! Rainfall and hydrologic event definitions
//!
//! Defines design storms, IDF curves, and rainfall distributions
//! for hydrologic analysis.

use serde::{Deserialize, Serialize};

/// Rainfall data including design storms and IDF curves
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rainfall {
    /// Design storm events
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "designStorms")]
    pub design_storms: Option<Vec<DesignStorm>>,

    /// Intensity-Duration-Frequency curves
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "idfCurves")]
    pub idf_curves: Option<Vec<IdfCurve>>,
}

/// Design storm event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DesignStorm {
    /// Unique storm identifier
    pub id: String,

    /// Storm name (e.g., "10-Year, 24-Hour")
    pub name: String,

    /// Return period in years
    #[serde(rename = "returnPeriod")]
    pub return_period: f64,

    /// Storm duration in minutes (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// Total rainfall depth (inches or mm) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "totalDepth")]
    pub total_depth: Option<f64>,

    /// Temporal distribution type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<DistributionType>,

    /// Peak rainfall intensity (in/hr or mm/hr) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "peakIntensity")]
    pub peak_intensity: Option<f64>,

    /// Time-series rainfall data for custom distributions (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyetograph: Option<Vec<HyetographPoint>>,
}

/// Temporal rainfall distribution type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DistributionType {
    /// SCS Type I (Pacific maritime climate)
    #[serde(rename = "SCS Type I")]
    ScsTypeI,
    /// SCS Type IA (Pacific coast, Intermountain)
    #[serde(rename = "SCS Type IA")]
    ScsTypeIA,
    /// SCS Type II (Most of US, moderate climates)
    #[serde(rename = "SCS Type II")]
    ScsTypeII,
    /// SCS Type III (Gulf of Mexico, Atlantic coastal areas)
    #[serde(rename = "SCS Type III")]
    ScsTypeIII,
    /// Uniform distribution (constant intensity)
    Uniform,
    /// Custom distribution (use hyetograph)
    Custom,
}

/// Hyetograph data point (time-series rainfall)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HyetographPoint {
    /// Time from storm start (minutes)
    pub time: f64,

    /// Rainfall intensity at this time step (in/hr or mm/hr)
    pub intensity: f64,
}

/// Intensity-Duration-Frequency curve
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdfCurve {
    /// Return period in years
    #[serde(rename = "returnPeriod")]
    pub return_period: f64,

    /// IDF equation parameters (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub equation: Option<IdfEquation>,

    /// Tabular IDF data points
    pub points: Vec<IdfPoint>,
}

/// IDF equation definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdfEquation {
    /// Equation type
    #[serde(rename = "type")]
    pub equation_type: IdfEquationType,

    /// Equation coefficients (a, b, c, etc.)
    pub coefficients: std::collections::HashMap<String, f64>,
}

/// IDF equation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdfEquationType {
    /// Sherman equation: i = a / (t + b)
    Sherman,
    /// Talbot equation: i = a / (t + b)
    Talbot,
    /// Modified Talbot: i = a / (t + b)^c
    #[serde(rename = "Modified Talbot")]
    ModifiedTalbot,
    /// NOAA Atlas 14 equations
    #[serde(rename = "NOAA Atlas 14")]
    NoaaAtlas14,
}

/// IDF curve data point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdfPoint {
    /// Duration in minutes
    pub duration: f64,

    /// Rainfall intensity (in/hr or mm/hr)
    pub intensity: f64,
}

impl IdfCurve {
    /// Interpolate intensity for a given duration
    ///
    /// Uses linear interpolation between adjacent points.
    /// Extrapolates using nearest point if duration is outside range.
    pub fn get_intensity(&self, duration: f64) -> Option<f64> {
        if self.points.is_empty() {
            return None;
        }

        // Find bracketing points
        let mut lower = None;
        let mut upper = None;

        for point in &self.points {
            if point.duration <= duration {
                lower = Some(point);
            }
            if point.duration >= duration && upper.is_none() {
                upper = Some(point);
            }
        }

        match (lower, upper) {
            (Some(l), Some(u)) if l.duration == u.duration => {
                // Exact match
                Some(l.intensity)
            }
            (Some(l), Some(u)) => {
                // Linear interpolation
                let t = (duration - l.duration) / (u.duration - l.duration);
                Some(l.intensity + t * (u.intensity - l.intensity))
            }
            (Some(l), None) => {
                // Beyond upper bound, use last point
                Some(l.intensity)
            }
            (None, Some(u)) => {
                // Below lower bound, use first point
                Some(u.intensity)
            }
            _ => None,
        }
    }

    /// Create IDF curve from equation
    pub fn from_equation(
        return_period: f64,
        equation: IdfEquation,
        durations: &[f64],
    ) -> Self {
        let points = durations
            .iter()
            .filter_map(|&d| {
                equation.evaluate(d).map(|intensity| IdfPoint {
                    duration: d,
                    intensity,
                })
            })
            .collect();

        Self {
            return_period,
            equation: Some(equation),
            points,
        }
    }
}

impl IdfEquation {
    /// Evaluate the equation for a given duration
    pub fn evaluate(&self, duration: f64) -> Option<f64> {
        match self.equation_type {
            IdfEquationType::Sherman | IdfEquationType::Talbot => {
                // i = a / (t + b)
                let a = self.coefficients.get("a")?;
                let b = self.coefficients.get("b")?;
                Some(a / (duration + b))
            }
            IdfEquationType::ModifiedTalbot => {
                // i = a / (t + b)^c
                let a = self.coefficients.get("a")?;
                let b = self.coefficients.get("b")?;
                let c = self.coefficients.get("c")?;
                Some(a / (duration + b).powf(*c))
            }
            IdfEquationType::NoaaAtlas14 => {
                // More complex equation, simplified here
                // Actual implementation would depend on specific NOAA Atlas 14 format
                None
            }
        }
    }
}

impl DesignStorm {
    /// Create a simple uniform intensity storm
    pub fn uniform(id: String, name: String, return_period: f64, intensity: f64) -> Self {
        Self {
            id,
            name,
            return_period,
            duration: None,
            total_depth: None,
            distribution: Some(DistributionType::Uniform),
            peak_intensity: Some(intensity),
            hyetograph: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idf_interpolation() {
        let idf = IdfCurve {
            return_period: 10.0,
            equation: None,
            points: vec![
                IdfPoint {
                    duration: 5.0,
                    intensity: 6.5,
                },
                IdfPoint {
                    duration: 10.0,
                    intensity: 5.2,
                },
                IdfPoint {
                    duration: 30.0,
                    intensity: 3.8,
                },
            ],
        };

        // Exact match
        assert_eq!(idf.get_intensity(10.0), Some(5.2));

        // Interpolation between 10 and 30 minutes
        let intensity = idf.get_intensity(20.0).unwrap();
        assert!((intensity - 4.5).abs() < 0.001);
    }

    #[test]
    fn test_sherman_equation() {
        let mut coefficients = std::collections::HashMap::new();
        coefficients.insert("a".to_string(), 100.0);
        coefficients.insert("b".to_string(), 10.0);

        let equation = IdfEquation {
            equation_type: IdfEquationType::Sherman,
            coefficients,
        };

        // i = 100 / (15 + 10) = 4.0
        let intensity = equation.evaluate(15.0).unwrap();
        assert!((intensity - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_create_uniform_storm() {
        let storm = DesignStorm::uniform(
            "storm-1".to_string(),
            "10-Year".to_string(),
            10.0,
            3.8,
        );

        assert_eq!(storm.return_period, 10.0);
        assert_eq!(storm.peak_intensity, Some(3.8));
        assert_eq!(storm.distribution, Some(DistributionType::Uniform));
    }
}
