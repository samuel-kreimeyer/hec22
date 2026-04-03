use wasm_bindgen::prelude::*;

use hec22::analysis::{Analysis, DesignCriteria};
use hec22::drainage::DrainageArea;
use hec22::network::Network;
use hec22::project::UnitSystem;
use hec22::rainfall::Rainfall;
use hec22::solver::{self, HglSolver, InletInterception, SolverConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct SolverRequest {
    network: Network,
    #[serde(default)]
    drainage_areas: Option<Vec<DrainageArea>>,
    #[serde(default)]
    node_inflows: Option<HashMap<String, f64>>,
    #[serde(default)]
    conduit_flows: Option<HashMap<String, f64>>,
    #[serde(default)]
    rainfall: Option<Rainfall>,
    #[serde(default, alias = "returnPeriod")]
    return_period: Option<f64>,
    #[serde(default)]
    intensity: Option<f64>,
    #[serde(default)]
    unit_system: Option<UnitSystemArg>,
    #[serde(default)]
    use_inlet_interception: Option<bool>,
    #[serde(default)]
    design_storm_id: Option<String>,
    #[serde(default, rename = "designCriteria", alias = "design_criteria")]
    design_criteria: Option<DesignCriteria>,
}

#[derive(Debug, Serialize)]
struct SolverResponse {
    analysis: Analysis,
    conduit_flows: HashMap<String, f64>,
    node_inflows: Option<HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inlet_results: Option<Vec<InletInterception>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
enum UnitSystemArg {
    #[serde(alias = "US", alias = "us", alias = "Us")]
    Us,
    #[serde(alias = "SI", alias = "si", alias = "Si")]
    Si,
}

impl UnitSystemArg {
    fn to_unit_system(self) -> UnitSystem {
        match self {
            UnitSystemArg::Us => UnitSystem::US,
            UnitSystemArg::Si => UnitSystem::SI,
        }
    }
}

fn compute_rational_flows_with_idf(
    areas: &[DrainageArea],
    curve: &hec22::rainfall::IdfCurve,
    fallback_intensity: Option<f64>,
) -> Result<HashMap<String, f64>, String> {
    let mut flows = HashMap::new();
    for area in areas {
        let intensity = if let Some(tc) = area.time_of_concentration {
            curve.get_intensity(tc)
        } else {
            None
        };
        let intensity = match intensity {
            Some(value) => value,
            None => fallback_intensity.ok_or_else(|| {
                format!(
                    "Missing timeOfConcentration for area {} and no fallback intensity provided.",
                    area.id
                )
            })?,
        };

        let c = area.runoff_coefficient.unwrap_or(0.5);
        let flow = c * intensity * area.area;
        *flows.entry(area.outlet.clone()).or_insert(0.0) += flow;
    }
    Ok(flows)
}

fn js_error(message: &str) -> JsValue {
    JsValue::from_str(message)
}

#[wasm_bindgen]
pub fn solve_from_json(request_json: &str) -> Result<String, JsValue> {
    let request: SolverRequest = serde_json::from_str(request_json)
        .map_err(|err| js_error(&format!("Invalid JSON: {}", err)))?;

    request
        .network
        .validate_connectivity()
        .map_err(|err| js_error(&format!("Network invalid: {}", err)))?;

    let unit_system = request
        .unit_system
        .unwrap_or(UnitSystemArg::Us)
        .to_unit_system();

    let (conduit_flows, inlet_results, node_inflows) = if let Some(conduit_flows) = request.conduit_flows {
        (conduit_flows, Vec::new(), request.node_inflows)
    } else {
        let node_inflows = match (request.node_inflows, request.drainage_areas, request.intensity) {
            (Some(inflows), _, _) => inflows,
            (None, Some(areas), intensity) => {
                if let Some(ref rainfall) = request.rainfall {
                    if let Some(ref curves) = rainfall.idf_curves {
                        if let Some(return_period) = request.return_period {
                            let curve = curves
                                .iter()
                                .find(|curve| (curve.return_period - return_period).abs() < 0.1)
                                .ok_or_else(|| {
                                    js_error("No IDF curve found for requested return period.")
                                })?;
                            compute_rational_flows_with_idf(&areas, curve, intensity)
                                .map_err(|err| js_error(&err))?
                        } else {
                            return Err(js_error(
                                "return_period is required when rainfall.idfCurves are provided.",
                            ));
                        }
                    } else {
                        match intensity {
                            Some(value) => solver::compute_rational_flows(&areas, value),
                            None => {
                                return Err(js_error(
                                    "Provide intensity when drainage_areas are supplied without IDF curves.",
                                ));
                            }
                        }
                    }
                } else {
                    match intensity {
                        Some(value) => solver::compute_rational_flows(&areas, value),
                        None => {
                            return Err(js_error(
                                "Provide intensity when drainage_areas are supplied without IDF curves.",
                            ));
                        }
                    }
                }
            }
            _ => {
                return Err(js_error(
                    "Provide conduit_flows, node_inflows, or (drainage_areas + intensity or IDF curves).",
                ));
            }
        };

        let use_inlet_interception = request.use_inlet_interception.unwrap_or(true);
        if use_inlet_interception {
            let (conduit_flows, inlet_results) = solver::route_flows_with_inlets(
                &request.network,
                &node_inflows,
                unit_system,
                &std::collections::HashMap::new(),
            )
            .map_err(|err| js_error(&format!("Flow routing failed: {}", err)))?;
            (conduit_flows, inlet_results, Some(node_inflows))
        } else {
            let conduit_flows = solver::route_flows(&request.network, &node_inflows)
                .map_err(|err| js_error(&format!("Flow routing failed: {}", err)))?;
            (conduit_flows, Vec::new(), Some(node_inflows))
        }
    };

    let config = match unit_system {
        UnitSystem::US => SolverConfig::us_customary(),
        UnitSystem::SI => SolverConfig::si_metric(),
    };

    let solver = HglSolver::new(config);
    let design_storm_id = request.design_storm_id.unwrap_or_else(|| "wasm".to_string());

    let mut analysis = solver
        .solve(&request.network, &conduit_flows, &inlet_results, design_storm_id)
        .map_err(|err| js_error(&format!("Solver failed: {}", err)))?;

    if let Some(ref criteria) = request.design_criteria {
        solver::apply_design_criteria(&mut analysis, &request.network, criteria, unit_system);
    }

    let response = SolverResponse {
        analysis,
        conduit_flows,
        node_inflows,
        inlet_results: if inlet_results.is_empty() {
            None
        } else {
            Some(inlet_results)
        },
    };

    serde_json::to_string(&response)
        .map_err(|err| js_error(&format!("JSON encode failed: {}", err)))
}
