const statusEl = document.getElementById("status");
const requestEl = document.getElementById("requestJson");
const outputEl = document.getElementById("output");
const runButton = document.getElementById("runSolver");
const copyButton = document.getElementById("copyResult");
const fileInput = document.getElementById("fileInput");
const loadSampleButton = document.getElementById("loadSample");
const intensityInput = document.getElementById("intensity");
const unitSelect = document.getElementById("unitSystem");
const interceptionToggle = document.getElementById("useInterception");
const nodesCsvInput = document.getElementById("nodesCsv");
const conduitsCsvInput = document.getElementById("conduitsCsv");
const areasCsvInput = document.getElementById("areasCsv");
const buildFromCsvButton = document.getElementById("buildFromCsv");
const downloadTemplatesButton = document.getElementById("downloadTemplates");
const csvStatus = document.getElementById("csvStatus");
const csvPreview = document.getElementById("csvPreview");
const downloadResultsJsonButton = document.getElementById("downloadResultsJson");
const downloadResultsCsvButton = document.getElementById("downloadResultsCsv");
const openReportButton = document.getElementById("openReport");
const exportStatus = document.getElementById("exportStatus");
const jsonStatus = document.getElementById("jsonStatus");
const jsonErrors = document.getElementById("jsonErrors");
const jsonPreview = document.getElementById("jsonPreview");
const nodeTableEl = document.getElementById("nodeTable");
const conduitTableEl = document.getElementById("conduitTable");
const nodeFilterInput = document.getElementById("nodeFilter");
const conduitFilterInput = document.getElementById("conduitFilter");
const nodeTableMeta = document.getElementById("nodeTableMeta");
const conduitTableMeta = document.getElementById("conduitTableMeta");
const profileStartSelect = document.getElementById("profileStart");
const profileEndSelect = document.getElementById("profileEnd");
const profileInvertToggle = document.getElementById("profileInvert");
const profileBuildButton = document.getElementById("profileBuild");
const profileStatus = document.getElementById("profileStatus");
const profileChart = document.getElementById("profileChart");
const profileWrap = profileChart ? profileChart.parentElement : null;
const planMetricSelect = document.getElementById("planMetric");
const networkPlan = document.getElementById("networkPlan");
const planLegend = document.getElementById("planLegend");
const planStatus = document.getElementById("planStatus");
const planWrap = networkPlan ? networkPlan.parentElement : null;
const planLabelsToggle = document.getElementById("planLabels");
const planZoomInButton = document.getElementById("planZoomIn");
const planZoomOutButton = document.getElementById("planZoomOut");
const planResetButton = document.getElementById("planReset");

const sampleRequest = {
  network: {
    nodes: [
      {
        id: "IN-001",
        type: "inlet",
        invertElevation: 130.0,
        rimElevation: 134.0,
        inlet: {
          inletType: "grate",
          location: "on-grade",
          grate: {
            length: 2.0,
            width: 1.5,
            barConfiguration: "perpendicular"
          },
          localDepression: 2.0,
          cloggingFactor: 0.1
        }
      },
      {
        id: "OUT-001",
        type: "outfall",
        invertElevation: 120.0,
        outfall: {
          boundaryCondition: "normal-depth",
          tailwaterElevation: 121.0
        }
      }
    ],
    conduits: [
      {
        id: "P-001",
        type: "pipe",
        fromNode: "IN-001",
        toNode: "OUT-001",
        length: 240.0,
        upstreamInvert: 130.0,
        downstreamInvert: 120.4,
        pipe: {
          shape: "circular",
          diameter: 18.0,
          manningN: 0.013,
          entranceLoss: 0.5,
          exitLoss: 1.0,
          bendLoss: 0.0
        }
      }
    ]
  },
  drainage_areas: [
    {
      id: "DA-001",
      area: 1.2,
      outlet: "IN-001",
      runoffCoefficient: 0.85,
      timeOfConcentration: 10.0
    }
  ],
  intensity: 4.0,
  unit_system: "US",
  use_inlet_interception: true,
  design_storm_id: "web-sample"
};

requestEl.value = JSON.stringify(sampleRequest, null, 2);

const worker = new Worker("worker.js", { type: "module" });
let requestId = 0;
const pending = new Map();
const csvState = {
  nodes: null,
  conduits: null,
  areas: null
};
let lastResult = null;
let lastResultRaw = "";
let validationTimer = null;
let lastRequest = null;
const PLAN_DEFAULT_VIEWBOX = { x: 0, y: 0, width: 800, height: 500 };
const PLAN_MIN_SCALE = 0.5;
const PLAN_MAX_SCALE = 4;
const planViewBox = { ...PLAN_DEFAULT_VIEWBOX };
let isPanning = false;
let panStart = { x: 0, y: 0 };
let panViewBoxStart = { x: 0, y: 0 };
const tableState = {
  nodes: {
    sortKey: "nodeId",
    sortDir: "asc",
    filter: ""
  },
  conduits: {
    sortKey: "conduitId",
    sortDir: "asc",
    filter: ""
  }
};

worker.onmessage = (event) => {
  const { id, ok, response, error } = event.data;
  const callbacks = pending.get(id);
  if (!callbacks) {
    return;
  }
  pending.delete(id);
  if (ok) {
    callbacks.resolve(response);
  } else {
    callbacks.reject(error);
  }
};

function callSolver(requestJson) {
  requestId += 1;
  const id = requestId;
  return new Promise((resolve, reject) => {
    pending.set(id, { resolve, reject });
    worker.postMessage({ id, requestJson });
  });
}

function updateCsvStatus(message, isError = false) {
  csvStatus.textContent = message;
  csvStatus.style.background = isError ? "rgba(176, 26, 24, 0.12)" : "";
  csvStatus.style.color = isError ? "#7a1c1b" : "";
}

function normalizeHeader(value) {
  return String(value || "").trim().toLowerCase();
}

function parseCsv(text) {
  const rows = [];
  let row = [];
  let current = "";
  let inQuotes = false;

  for (let i = 0; i < text.length; i += 1) {
    const char = text[i];
    if (inQuotes) {
      if (char === "\"") {
        if (text[i + 1] === "\"") {
          current += "\"";
          i += 1;
        } else {
          inQuotes = false;
        }
      } else {
        current += char;
      }
    } else if (char === "\"") {
      inQuotes = true;
    } else if (char === ",") {
      row.push(current);
      current = "";
    } else if (char === "\n") {
      row.push(current);
      rows.push(row);
      row = [];
      current = "";
    } else if (char !== "\r") {
      current += char;
    }
  }

  if (current.length || row.length) {
    row.push(current);
    rows.push(row);
  }

  const trimmedRows = rows.filter((items) => items.some((item) => String(item).trim() !== ""));
  if (!trimmedRows.length) {
    return { headers: [], rawHeaders: [], records: [] };
  }

  const rawHeaders = trimmedRows[0].map((value) => String(value || "").trim());
  const headers = rawHeaders.map(normalizeHeader);
  const records = trimmedRows.slice(1).map((items) => {
    const record = {};
    headers.forEach((header, index) => {
      record[header] = items[index] !== undefined ? String(items[index]).trim() : "";
    });
    return record;
  });
  return { headers, rawHeaders, records };
}

function getValue(record, keys, mapping) {
  if (mapping) {
    for (const key of keys) {
      const mappedHeader = mapping[key];
      if (mappedHeader) {
        const value = record[mappedHeader];
        if (value !== undefined && value !== "") {
          return value;
        }
        return null;
      }
    }
  }

  for (const key of keys) {
    const value = record[key];
    if (value !== undefined && value !== "") {
      return value;
    }
  }
  return null;
}

function getNumber(record, keys, mapping) {
  const value = getValue(record, keys, mapping);
  if (value === null) {
    return null;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function getString(record, keys, mapping) {
  const value = getValue(record, keys, mapping);
  return value !== null ? String(value).trim() : null;
}

function mapInletType(value) {
  if (!value) {
    return "combination";
  }
  const normalized = value.toLowerCase();
  if (normalized === "curb" || normalized === "curb-opening") {
    return "curb-opening";
  }
  if (normalized === "grate") {
    return "grate";
  }
  if (normalized === "slotted") {
    return "slotted";
  }
  return "combination";
}

function mapInletLocation(value) {
  if (!value) {
    return "on-grade";
  }
  const normalized = value.toLowerCase();
  return normalized === "sag" ? "sag" : "on-grade";
}

function mapBarConfig(value) {
  if (!value) {
    return null;
  }
  const normalized = value.toLowerCase();
  return normalized === "parallel" ? "parallel" : "perpendicular";
}

function mapThroatType(value) {
  if (!value) {
    return null;
  }
  const normalized = value.toLowerCase();
  if (normalized === "inclined") {
    return "inclined";
  }
  if (normalized === "vertical") {
    return "vertical";
  }
  return "horizontal";
}

function mapBoundaryCondition(value) {
  if (!value) {
    return "normal-depth";
  }
  const normalized = value.toLowerCase();
  if (normalized === "free") {
    return "free";
  }
  if (normalized === "fixed" || normalized === "fixed-stage") {
    return "fixed-stage";
  }
  if (normalized === "tidal") {
    return "tidal";
  }
  return "normal-depth";
}

function mapLandUse(value) {
  if (!value) {
    return null;
  }
  const normalized = value.toLowerCase();
  const mapping = {
    commercial: "Commercial",
    industrial: "Industrial",
    residential: "Residential",
    "open space": "OpenSpace",
    openspace: "OpenSpace",
    transportation: "Transportation",
    agricultural: "Agricultural",
    mixed: "Mixed"
  };
  return mapping[normalized] || null;
}

function parseNodesCsv(records, mapping) {
  return records.map((record) => {
    const id = getString(record, ["id"], mapping);
    const type = getString(record, ["type", "node_type"], mapping);
    const invertElevation = getNumber(record, ["invert_elev", "invert_elevation", "invert"], mapping);

    if (!id || !type || invertElevation === null) {
      throw new Error("Nodes CSV requires id, type, invert_elev columns.");
    }

    const node = {
      id,
      type: type.toLowerCase(),
      invertElevation
    };

    const rimElevation = getNumber(record, ["rim_elev", "rim_elevation"], mapping);
    if (rimElevation !== null) {
      node.rimElevation = rimElevation;
    }

    const x = getNumber(record, ["x"], mapping);
    const y = getNumber(record, ["y"], mapping);
    if (x !== null || y !== null) {
      node.coordinates = {
        x: x !== null ? x : undefined,
        y: y !== null ? y : undefined
      };
    }

    if (node.type === "inlet") {
      const inlet = {
        inletType: mapInletType(getString(record, ["inlet_type", "inlettype"], mapping)),
        location: mapInletLocation(getString(record, ["inlet_location", "inletlocation"], mapping))
      };

      const localDepression = getNumber(record, ["local_depression"], mapping);
      if (localDepression !== null) {
        inlet.localDepression = localDepression;
      }
      const cloggingFactor = getNumber(record, ["clogging_factor"], mapping);
      if (cloggingFactor !== null) {
        inlet.cloggingFactor = cloggingFactor;
      }
      const bypassTo = getString(record, ["bypass_to"], mapping);
      if (bypassTo) {
        inlet.bypassTo = bypassTo;
      }

      const grateLength = getNumber(record, ["grate_length"], mapping);
      const grateWidth = getNumber(record, ["grate_width"], mapping);
      const barConfiguration = mapBarConfig(getString(record, ["bar_configuration"], mapping));
      if (grateLength !== null || grateWidth !== null || barConfiguration) {
        inlet.grate = {};
        if (grateLength !== null) {
          inlet.grate.length = grateLength;
        }
        if (grateWidth !== null) {
          inlet.grate.width = grateWidth;
        }
        if (barConfiguration) {
          inlet.grate.barConfiguration = barConfiguration;
        }
      }

      const curbLength = getNumber(record, ["curb_opening_length"], mapping);
      const curbHeight = getNumber(record, ["curb_opening_height"], mapping);
      const throatType = mapThroatType(getString(record, ["throat_type"], mapping));
      if (curbLength !== null || curbHeight !== null || throatType) {
        inlet.curbOpening = {};
        if (curbLength !== null) {
          inlet.curbOpening.length = curbLength;
        }
        if (curbHeight !== null) {
          inlet.curbOpening.height = curbHeight;
        }
        if (throatType) {
          inlet.curbOpening.throatType = throatType;
        }
      }

      node.inlet = inlet;
    }

    if (node.type === "junction") {
      const diameter = getNumber(record, ["diameter"], mapping);
      if (diameter !== null) {
        node.junction = { diameter };
      }
    }

    if (node.type === "outfall") {
      node.outfall = {
        boundaryCondition: mapBoundaryCondition(
          getString(record, ["boundary_condition", "boundary"], mapping)
        )
      };
      const tailwater = getNumber(record, ["tailwater_elevation", "tailwater"], mapping);
      if (tailwater !== null) {
        node.outfall.tailwaterElevation = tailwater;
      }
    }

    return node;
  });
}

function parseConduitsCsv(records, mapping) {
  return records.map((record) => {
    const id = getString(record, ["id"], mapping);
    const fromNode = getString(record, ["from_node", "fromnode"], mapping);
    const toNode = getString(record, ["to_node", "tonode"], mapping);
    const length = getNumber(record, ["length"], mapping);
    const conduitType = (getString(record, ["type", "conduit_type"], mapping) || "pipe")
      .toLowerCase();

    if (!id || !fromNode || !toNode || length === null) {
      throw new Error("Conduits CSV requires id, from_node, to_node, length columns.");
    }

    const conduit = {
      id,
      type: conduitType,
      fromNode,
      toNode,
      length
    };

    const slope = getNumber(record, ["slope"], mapping);
    if (slope !== null) {
      conduit.slope = slope;
    }

    const upstreamInvert = getNumber(record, ["upstream_invert", "upstreaminvert"], mapping);
    const downstreamInvert = getNumber(record, ["downstream_invert", "downstreaminvert"], mapping);
    if (upstreamInvert !== null) {
      conduit.upstreamInvert = upstreamInvert;
    }
    if (downstreamInvert !== null) {
      conduit.downstreamInvert = downstreamInvert;
    }

    if (conduitType === "gutter") {
      const crossSlope = getNumber(record, ["cross_slope", "crossslope"], mapping);
      const longSlope = getNumber(record, ["long_slope", "longslope", "slope"], mapping);
      if (crossSlope === null || longSlope === null) {
        throw new Error("Gutter conduits require cross_slope and long_slope (or slope).");
      }
      const manningN = getNumber(record, ["manning_n"], mapping) ?? 0.016;
      conduit.gutter = {
        crossSlope,
        longitudinalSlope: longSlope,
        manningN
      };
    } else {
      const diameter = getNumber(record, ["diameter"], mapping);
      if (diameter === null) {
        throw new Error("Pipe conduits require diameter.");
      }
      const manningN = getNumber(record, ["manning_n"], mapping) ?? 0.013;
      const material = getString(record, ["material"], mapping);
      conduit.pipe = {
        shape: "circular",
        diameter,
        manningN
      };
      if (material) {
        conduit.pipe.material = material.toUpperCase();
      }

      const entranceLoss = getNumber(record, ["entrance_loss"], mapping);
      const exitLoss = getNumber(record, ["exit_loss"], mapping);
      const bendLoss = getNumber(record, ["bend_loss"], mapping);
      if (entranceLoss !== null) {
        conduit.pipe.entranceLoss = entranceLoss;
      }
      if (exitLoss !== null) {
        conduit.pipe.exitLoss = exitLoss;
      }
      if (bendLoss !== null) {
        conduit.pipe.bendLoss = bendLoss;
      }
    }

    return conduit;
  });
}

function parseAreasCsv(records, mapping) {
  return records.map((record) => {
    const id = getString(record, ["id"], mapping);
    const area = getNumber(record, ["area"], mapping);
    const runoff = getNumber(record, ["runoff_coef", "runoff_coefficient"], mapping);
    const tc = getNumber(record, ["time_of_conc", "time_of_concentration"], mapping);
    const outlet = getString(record, ["outlet_node", "outlet"], mapping);

    if (!id || area === null || runoff === null || tc === null || !outlet) {
      throw new Error("Drainage areas CSV requires id, area, runoff_coef, time_of_conc, outlet_node.");
    }

    const drainageArea = {
      id,
      area,
      outlet,
      runoffCoefficient: runoff,
      timeOfConcentration: tc
    };

    const landUse = mapLandUse(getString(record, ["land_use"], mapping));
    if (landUse) {
      drainageArea.landUse = { primary: landUse };
    }

    return drainageArea;
  });
}

function buildRequestFromCsv() {
  if (!csvState.nodes || !csvState.conduits) {
    throw new Error("Please load both nodes and conduits CSV files.");
  }

  const request = {
    network: {
      nodes: parseNodesCsv(csvState.nodes.records, csvState.nodes.mapping),
      conduits: parseConduitsCsv(csvState.conduits.records, csvState.conduits.mapping)
    },
    intensity: Number(intensityInput.value || 0),
    unit_system: unitSelect.value,
    use_inlet_interception: interceptionToggle.checked,
    design_storm_id: "web-import"
  };

  if (csvState.areas) {
    request.drainage_areas = parseAreasCsv(csvState.areas.records, csvState.areas.mapping);
  }

  return request;
}

function downloadFile(filename, content, type) {
  const blob = new Blob([content], { type });
  const link = document.createElement("a");
  link.href = URL.createObjectURL(blob);
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(link.href);
}

function downloadCsv(filename, content) {
  downloadFile(filename, content, "text/csv");
}

function csvEscape(value) {
  const text = value === null || value === undefined ? "" : String(value);
  if (/[",\n\r]/.test(text)) {
    return `"${text.replace(/"/g, "\"\"")}"`;
  }
  return text;
}

function buildIssuesCsv(state, issuesList) {
  const headerKeys = state.headers.length ? state.headers : state.rawHeaders.map(normalizeHeader);
  const displayHeaders = state.rawHeaders.length ? state.rawHeaders : state.headers;
  const outputHeaders = [...displayHeaders, "issues"];
  const rows = issuesList.map(({ record, issues }) => {
    const values = headerKeys.map((key) => (record[key] !== undefined ? record[key] : ""));
    values.push(issues.join("; "));
    return values.map(csvEscape).join(",");
  });
  return [outputHeaders.map(csvEscape).join(","), ...rows].join("\n");
}

function updateExportStatus(message, isError = false) {
  if (!exportStatus) {
    return;
  }
  exportStatus.textContent = message;
  exportStatus.style.background = isError ? "rgba(176, 26, 24, 0.12)" : "";
  exportStatus.style.color = isError ? "#7a1c1b" : "";
}

function updatePlanStatus(message, isError = false) {
  if (!planStatus) {
    return;
  }
  planStatus.textContent = message;
  planStatus.style.background = isError ? "rgba(176, 26, 24, 0.12)" : "";
  planStatus.style.color = isError ? "#7a1c1b" : "";
}

function updateProfileStatus(message, isError = false) {
  if (!profileStatus) {
    return;
  }
  profileStatus.textContent = message;
  profileStatus.style.background = isError ? "rgba(176, 26, 24, 0.12)" : "";
  profileStatus.style.color = isError ? "#7a1c1b" : "";
}

function updateJsonStatus(message, isError = false) {
  if (!jsonStatus) {
    return;
  }
  jsonStatus.textContent = message;
  jsonStatus.style.background = isError ? "rgba(176, 26, 24, 0.12)" : "";
  jsonStatus.style.color = isError ? "#7a1c1b" : "";
}

function isNumber(value) {
  return typeof value === "number" && Number.isFinite(value);
}

function validateRequestObject(data) {
  const errors = [];
  const pushError = (path, message) => {
    errors.push({ path, message });
  };

  if (!data || typeof data !== "object") {
    pushError("$", "Request must be a JSON object.");
    return errors;
  }

  if (data.unit_system !== undefined) {
    const unit = String(data.unit_system);
    if (unit !== "US" && unit !== "SI") {
      pushError("unit_system", "unit_system must be \"US\" or \"SI\".");
    }
  }

  if (data.use_inlet_interception !== undefined && typeof data.use_inlet_interception !== "boolean") {
    pushError("use_inlet_interception", "use_inlet_interception must be true/false.");
  }

  if (data.intensity !== undefined && !isNumber(data.intensity)) {
    pushError("intensity", "intensity must be a number.");
  }

  const network = data.network;
  if (!network || typeof network !== "object") {
    pushError("network", "network is required.");
    return errors;
  }

  const nodes = network.nodes;
  if (!Array.isArray(nodes) || nodes.length === 0) {
    pushError("network.nodes", "network.nodes must be a non-empty array.");
  } else {
    const seen = new Set();
    nodes.forEach((node, index) => {
      const base = `network.nodes[${index}]`;
      if (!node || typeof node !== "object") {
        pushError(base, "node must be an object.");
        return;
      }
      if (!node.id || typeof node.id !== "string") {
        pushError(`${base}.id`, "node id is required.");
      } else if (seen.has(node.id)) {
        pushError(`${base}.id`, `duplicate node id "${node.id}".`);
      } else {
        seen.add(node.id);
      }
      if (!node.type || typeof node.type !== "string") {
        pushError(`${base}.type`, "node type is required.");
      }
      if (!isNumber(node.invertElevation)) {
        pushError(`${base}.invertElevation`, "invertElevation must be a number.");
      }
    });
  }

  const conduits = network.conduits;
  if (!Array.isArray(conduits) || conduits.length === 0) {
    pushError("network.conduits", "network.conduits must be a non-empty array.");
  } else {
    const seen = new Set();
    conduits.forEach((conduit, index) => {
      const base = `network.conduits[${index}]`;
      if (!conduit || typeof conduit !== "object") {
        pushError(base, "conduit must be an object.");
        return;
      }
      if (!conduit.id || typeof conduit.id !== "string") {
        pushError(`${base}.id`, "conduit id is required.");
      } else if (seen.has(conduit.id)) {
        pushError(`${base}.id`, `duplicate conduit id "${conduit.id}".`);
      } else {
        seen.add(conduit.id);
      }
      if (!conduit.fromNode || typeof conduit.fromNode !== "string") {
        pushError(`${base}.fromNode`, "fromNode is required.");
      }
      if (!conduit.toNode || typeof conduit.toNode !== "string") {
        pushError(`${base}.toNode`, "toNode is required.");
      }
      if (!isNumber(conduit.length)) {
        pushError(`${base}.length`, "length must be a number.");
      }
      const conduitType = conduit.type || "pipe";
      if (conduitType === "gutter") {
        if (!conduit.gutter || typeof conduit.gutter !== "object") {
          pushError(`${base}.gutter`, "gutter properties are required for gutter conduits.");
        } else {
          if (!isNumber(conduit.gutter.crossSlope)) {
            pushError(`${base}.gutter.crossSlope`, "crossSlope must be a number.");
          }
          if (!isNumber(conduit.gutter.longitudinalSlope)) {
            pushError(`${base}.gutter.longitudinalSlope`, "longitudinalSlope must be a number.");
          }
        }
      } else {
        if (!conduit.pipe || typeof conduit.pipe !== "object") {
          pushError(`${base}.pipe`, "pipe properties are required for pipe conduits.");
        } else if (!isNumber(conduit.pipe.diameter)) {
          pushError(`${base}.pipe.diameter`, "diameter must be a number.");
        }
      }
    });
  }

  if (Array.isArray(nodes) && Array.isArray(conduits)) {
    const nodeIds = new Set(nodes.map((node) => node && node.id).filter(Boolean));
    conduits.forEach((conduit, index) => {
      if (conduit && conduit.fromNode && !nodeIds.has(conduit.fromNode)) {
        pushError(
          `network.conduits[${index}].fromNode`,
          `fromNode "${conduit.fromNode}" not found in network.nodes.`
        );
      }
      if (conduit && conduit.toNode && !nodeIds.has(conduit.toNode)) {
        pushError(
          `network.conduits[${index}].toNode`,
          `toNode "${conduit.toNode}" not found in network.nodes.`
        );
      }
    });
  }

  if (data.drainage_areas !== undefined) {
    if (!Array.isArray(data.drainage_areas)) {
      pushError("drainage_areas", "drainage_areas must be an array.");
    } else {
      data.drainage_areas.forEach((area, index) => {
        const base = `drainage_areas[${index}]`;
        if (!area || typeof area !== "object") {
          pushError(base, "drainage area must be an object.");
          return;
        }
        if (!area.id || typeof area.id !== "string") {
          pushError(`${base}.id`, "id is required.");
        }
        if (!isNumber(area.area)) {
          pushError(`${base}.area`, "area must be a number.");
        }
        if (!area.outlet || typeof area.outlet !== "string") {
          pushError(`${base}.outlet`, "outlet is required.");
        }
        if (!isNumber(area.runoffCoefficient)) {
          pushError(`${base}.runoffCoefficient`, "runoffCoefficient must be a number.");
        }
        if (!isNumber(area.timeOfConcentration)) {
          pushError(`${base}.timeOfConcentration`, "timeOfConcentration must be a number.");
        }
      });
    }
  }

  const hasConduitFlows = data.conduit_flows && typeof data.conduit_flows === "object";
  const hasNodeInflows = data.node_inflows && typeof data.node_inflows === "object";
  const hasAreas =
    Array.isArray(data.drainage_areas) && data.drainage_areas.length > 0 && isNumber(data.intensity);

  if (!hasConduitFlows && !hasNodeInflows && !hasAreas) {
    pushError(
      "$",
      "Provide conduit_flows, node_inflows, or (drainage_areas + intensity) to run the solver."
    );
  }

  return errors;
}

function getLineColumn(text, index) {
  if (index < 0) {
    return { line: null, column: null };
  }
  const slice = text.slice(0, index);
  const lines = slice.split("\n");
  return { line: lines.length, column: lines[lines.length - 1].length + 1 };
}

function findLineForPath(text, path) {
  if (!path) {
    return null;
  }
  const match = path.match(/([A-Za-z_][\w-]*)(?:\[\d+\])?$/);
  if (!match) {
    return null;
  }
  const key = match[1];
  const target = `"${key}"`;
  const index = text.indexOf(target);
  if (index === -1) {
    return null;
  }
  return getLineColumn(text, index).line;
}

function renderJsonErrors(errors) {
  if (!jsonErrors) {
    return;
  }
  jsonErrors.innerHTML = "";
  if (!errors.length) {
    const ok = document.createElement("div");
    ok.className = "json-error ok";
    ok.textContent = "No validation issues found.";
    jsonErrors.appendChild(ok);
    return;
  }
  errors.forEach((error) => {
    const row = document.createElement("div");
    row.className = "json-error";
    const lineInfo = error.line ? ` (line ${error.line})` : "";
    row.innerHTML = `<span class="json-error-path">${escapeHtml(error.path)}</span>${lineInfo}: ${escapeHtml(error.message)}`;
    jsonErrors.appendChild(row);
  });
}

function renderJsonPreview(text, errorLines) {
  if (!jsonPreview) {
    return;
  }
  if (!text.trim()) {
    jsonPreview.textContent = "";
    return;
  }
  const lines = text.split("\n");
  const html = lines
    .map((line, index) => {
      const lineNumber = index + 1;
      const isError = errorLines.has(lineNumber);
      return `<div class="json-line${isError ? " error" : ""}"><span class="json-lineno">${lineNumber}</span><span>${escapeHtml(line) || " "}</span></div>`;
    })
    .join("");
  jsonPreview.innerHTML = html;
}

function validateJsonInput() {
  const text = requestEl.value || "";
  if (!text.trim()) {
    updateJsonStatus("No JSON provided.", false);
    if (jsonErrors) {
      jsonErrors.innerHTML = "";
    }
    renderJsonPreview("", new Set());
    requestEl.classList.remove("field-error");
    return;
  }

  try {
    const parsed = JSON.parse(text);
    updateProfileOptions(parsed);
    const errors = validateRequestObject(parsed).map((error) => ({
      ...error,
      line: findLineForPath(text, error.path)
    }));
    const errorLines = new Set(errors.map((error) => error.line).filter(Boolean));
    renderJsonErrors(errors);
    renderJsonPreview(text, errorLines);
    if (errors.length) {
      updateJsonStatus(`${errors.length} issue(s) found.`, true);
      requestEl.classList.add("field-error");
    } else {
      updateJsonStatus("JSON looks valid.", false);
      requestEl.classList.remove("field-error");
    }
  } catch (error) {
    const message = String(error.message || error);
    let line = null;
    let column = null;
    const match = message.match(/position (\d+)/);
    if (match) {
      const pos = Number(match[1]);
      const location = getLineColumn(text, pos);
      line = location.line;
      column = location.column;
    }
    const errorMessage = line
      ? `Invalid JSON at line ${line}, column ${column || "?"}.`
      : "Invalid JSON.";
    renderJsonErrors([
      {
        path: "$",
        message: errorMessage,
        line
      }
    ]);
    renderJsonPreview(text, new Set(line ? [line] : []));
    updateJsonStatus("Invalid JSON.", true);
    requestEl.classList.add("field-error");
  }
}

function scheduleValidation() {
  if (validationTimer) {
    clearTimeout(validationTimer);
  }
  validationTimer = setTimeout(validateJsonInput, 250);
}

function ensureRequest() {
  if (lastRequest) {
    return { ok: true, request: lastRequest };
  }
  const text = requestEl.value || "";
  if (!text.trim()) {
    return { ok: false, error: "Provide request JSON to render the network." };
  }
  try {
    const parsed = JSON.parse(text);
    lastRequest = parsed;
    return { ok: true, request: parsed };
  } catch (error) {
    return { ok: false, error: "Request JSON is invalid." };
  }
}

function updateProfileOptions(request) {
  if (!profileStartSelect || !profileEndSelect) {
    return;
  }
  const nodes = request && request.network && Array.isArray(request.network.nodes)
    ? request.network.nodes
    : [];
  const ids = nodes.map((node) => node.id).filter(Boolean);

  const buildOptions = (select) => {
    select.innerHTML = "";
    ids.forEach((id) => {
      const option = document.createElement("option");
      option.value = id;
      option.textContent = id;
      select.appendChild(option);
    });
  };

  buildOptions(profileStartSelect);
  buildOptions(profileEndSelect);

  if (ids.length) {
    profileStartSelect.value = ids[0];
    profileEndSelect.value = ids[ids.length - 1];
  }
}

function getAnalysisData(result) {
  const analysis = result && result.analysis ? result.analysis : null;
  if (!analysis) {
    return {
      analysis: null,
      nodeResults: [],
      conduitResults: [],
      drainageAreaResults: [],
      violations: []
    };
  }
  return {
    analysis,
    nodeResults: analysis.nodeResults || analysis.node_results || [],
    conduitResults: analysis.conduitResults || analysis.conduit_results || [],
    drainageAreaResults: analysis.drainageAreaResults || analysis.drainage_area_results || [],
    violations: analysis.violations || []
  };
}

function getValueByPath(obj, path) {
  if (!obj) {
    return "";
  }
  const parts = path.split(".");
  let current = obj;
  for (const part of parts) {
    if (current && Object.prototype.hasOwnProperty.call(current, part)) {
      current = current[part];
    } else {
      return "";
    }
  }
  return current === null || current === undefined ? "" : current;
}

function buildCsvFromRows(rows, fields) {
  const header = fields.map((field) => field.label);
  const lines = rows.map((row) => {
    const values = fields.map((field) => csvEscape(getValueByPath(row, field.key)));
    return values.join(",");
  });
  return [header.join(","), ...lines].join("\n");
}

function buildMapCsv(mapObj, idLabel, valueLabel) {
  const rows = Object.entries(mapObj || {}).map(([key, value]) => ({
    id: key,
    value
  }));
  return buildCsvFromRows(rows, [
    { key: "id", label: idLabel },
    { key: "value", label: valueLabel }
  ]);
}

function downloadResultsCsvs(result) {
  const { nodeResults, conduitResults, drainageAreaResults, violations } = getAnalysisData(result);

  if (nodeResults.length) {
    const csv = buildCsvFromRows(nodeResults, [
      { key: "nodeId", label: "node_id" },
      { key: "hgl", label: "hgl" },
      { key: "egl", label: "egl" },
      { key: "depth", label: "depth" },
      { key: "velocity", label: "velocity" },
      { key: "flooding", label: "flooding" },
      { key: "pressureHead", label: "pressure_head" },
      { key: "junctionLoss", label: "junction_loss" },
      { key: "gutterSpread", label: "gutter_spread" }
    ]);
    downloadCsv("node_results.csv", csv);
  }

  if (conduitResults.length) {
    const csv = buildCsvFromRows(conduitResults, [
      { key: "conduitId", label: "conduit_id" },
      { key: "flow", label: "flow" },
      { key: "velocity", label: "velocity" },
      { key: "depth", label: "depth" },
      { key: "capacityUsed", label: "capacity_used" },
      { key: "froudeNumber", label: "froude_number" },
      { key: "flowRegime", label: "flow_regime" },
      { key: "headloss.friction", label: "headloss_friction" },
      { key: "headloss.entrance", label: "headloss_entrance" },
      { key: "headloss.exit", label: "headloss_exit" },
      { key: "headloss.bend", label: "headloss_bend" },
      { key: "headloss.total", label: "headloss_total" }
    ]);
    downloadCsv("conduit_results.csv", csv);
  }

  if (drainageAreaResults.length) {
    const csv = buildCsvFromRows(drainageAreaResults, [
      { key: "drainageAreaId", label: "drainage_area_id" },
      { key: "peakFlow", label: "peak_flow" },
      { key: "timeOfPeak", label: "time_of_peak" },
      { key: "totalVolume", label: "total_volume" },
      { key: "intensity", label: "intensity" }
    ]);
    downloadCsv("drainage_area_results.csv", csv);
  }

  if (violations.length) {
    const csv = buildCsvFromRows(violations, [
      { key: "type", label: "type" },
      { key: "severity", label: "severity" },
      { key: "elementId", label: "element_id" },
      { key: "message", label: "message" },
      { key: "value", label: "value" },
      { key: "limit", label: "limit" }
    ]);
    downloadCsv("violations.csv", csv);
  }

  if (result && result.conduit_flows && Object.keys(result.conduit_flows).length) {
    const csv = buildMapCsv(result.conduit_flows, "conduit_id", "flow");
    downloadCsv("conduit_flows.csv", csv);
  }

  if (result && result.node_inflows && Object.keys(result.node_inflows).length) {
    const csv = buildMapCsv(result.node_inflows, "node_id", "flow");
    downloadCsv("node_inflows.csv", csv);
  }

  if (result && result.inlet_results && result.inlet_results.length) {
    const csv = buildCsvFromRows(result.inlet_results, [
      { key: "node_id", label: "node_id" },
      { key: "approach_flow", label: "approach_flow" },
      { key: "intercepted_flow", label: "intercepted_flow" },
      { key: "bypass_flow", label: "bypass_flow" },
      { key: "efficiency", label: "efficiency" },
      { key: "spread", label: "spread" },
      { key: "bypass_to_node", label: "bypass_to_node" },
      { key: "drainage_area", label: "drainage_area" }
    ]);
    downloadCsv("inlet_results.csv", csv);
  }
}

function escapeHtml(value) {
  return String(value)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

function formatNumber(value) {
  if (value === null || value === undefined || Number.isNaN(value)) {
    return "—";
  }
  const number = Number(value);
  if (!Number.isFinite(number)) {
    return "—";
  }
  return number.toFixed(3).replace(/\.?0+$/, "");
}

function formatBool(value) {
  if (value === true) {
    return "Yes";
  }
  if (value === false) {
    return "No";
  }
  return "—";
}

function buildReportTable(headers, rows) {
  const headCells = headers.map((label) => `<th>${escapeHtml(label)}</th>`).join("");
  const bodyRows = rows
    .map(
      (row) =>
        `<tr>${row.map((cell) => `<td>${escapeHtml(cell)}</td>`).join("")}</tr>`
    )
    .join("");
  return `<table><thead><tr>${headCells}</tr></thead><tbody>${bodyRows}</tbody></table>`;
}

function buildReportHtml(result) {
  const { analysis, nodeResults, conduitResults, drainageAreaResults, violations } =
    getAnalysisData(result);

  const nodeCount = nodeResults.length;
  const conduitCount = conduitResults.length;
  const drainageCount = drainageAreaResults.length;
  const floodedCount = nodeResults.filter((node) => node.flooding).length;
  const maxHgl = Math.max(
    ...nodeResults.map((node) => (node.hgl !== null && node.hgl !== undefined ? node.hgl : -Infinity)),
    -Infinity
  );
  const maxEgl = Math.max(
    ...nodeResults.map((node) => (node.egl !== null && node.egl !== undefined ? node.egl : -Infinity)),
    -Infinity
  );
  const supercriticalCount = conduitResults.filter(
    (conduit) => conduit.flowRegime === "supercritical"
  ).length;
  const maxConduitFlow = Math.max(
    ...conduitResults.map((conduit) =>
      conduit.flow !== null && conduit.flow !== undefined ? conduit.flow : -Infinity
    ),
    -Infinity
  );

  const summaryRows = [
    ["Design storm", analysis && analysis.designStormId ? analysis.designStormId : "—"],
    ["Analysis method", analysis && analysis.method ? analysis.method : "—"],
    ["Timestamp", analysis && analysis.timestamp ? analysis.timestamp : "—"],
    ["Nodes", String(nodeCount)],
    ["Conduits", String(conduitCount)],
    ["Drainage areas", String(drainageCount)],
    ["Max HGL", maxHgl === -Infinity ? "—" : formatNumber(maxHgl)],
    ["Max EGL", maxEgl === -Infinity ? "—" : formatNumber(maxEgl)],
    ["Flooded nodes", String(floodedCount)],
    ["Supercritical conduits", String(supercriticalCount)],
    ["Max conduit flow", maxConduitFlow === -Infinity ? "—" : formatNumber(maxConduitFlow)]
  ];

  const nodeTable = nodeResults.length
    ? buildReportTable(
        ["Node", "HGL", "EGL", "Depth", "Velocity", "Flooding", "Junction loss", "Gutter spread"],
        nodeResults.map((node) => [
          node.nodeId || "—",
          formatNumber(node.hgl),
          formatNumber(node.egl),
          formatNumber(node.depth),
          formatNumber(node.velocity),
          node.flooding === true ? "Yes" : node.flooding === false ? "No" : "—",
          formatNumber(node.junctionLoss),
          formatNumber(node.gutterSpread)
        ])
      )
    : "<p>No node results.</p>";

  const conduitTable = conduitResults.length
    ? buildReportTable(
        [
          "Conduit",
          "Flow",
          "Velocity",
          "Depth",
          "Capacity used",
          "Froude",
          "Regime",
          "Headloss total"
        ],
        conduitResults.map((conduit) => [
          conduit.conduitId || "—",
          formatNumber(conduit.flow),
          formatNumber(conduit.velocity),
          formatNumber(conduit.depth),
          formatNumber(conduit.capacityUsed),
          formatNumber(conduit.froudeNumber),
          conduit.flowRegime || "—",
          formatNumber(conduit.headloss && conduit.headloss.total)
        ])
      )
    : "<p>No conduit results.</p>";

  const drainageTable = drainageAreaResults.length
    ? buildReportTable(
        ["Drainage area", "Peak flow", "Time of peak", "Total volume", "Intensity"],
        drainageAreaResults.map((area) => [
          area.drainageAreaId || "—",
          formatNumber(area.peakFlow),
          formatNumber(area.timeOfPeak),
          formatNumber(area.totalVolume),
          formatNumber(area.intensity)
        ])
      )
    : "<p>No drainage area results.</p>";

  const violationTable = violations.length
    ? buildReportTable(
        ["Type", "Severity", "Element", "Message", "Value", "Limit"],
        violations.map((viol) => [
          viol.type || "—",
          viol.severity || "—",
          viol.elementId || "—",
          viol.message || "—",
          formatNumber(viol.value),
          formatNumber(viol.limit)
        ])
      )
    : "<p>No violations reported.</p>";

  const summaryTable = buildReportTable(["Metric", "Value"], summaryRows);

  return `<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>HEC-22 Report</title>
  <style>
    :root {
      --ink: #1f2428;
      --muted: #5b5f63;
      --accent: #0f766e;
      --panel: #f7f3ea;
      --border: #d2c8bb;
    }
    * { box-sizing: border-box; }
    body {
      margin: 0;
      padding: 32px 40px;
      font-family: "Space Grotesk", "Trebuchet MS", sans-serif;
      color: var(--ink);
      background: #fdfbf6;
    }
    header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 16px;
      margin-bottom: 24px;
    }
    h1 {
      margin: 0;
      font-size: 1.8rem;
    }
    .meta {
      color: var(--muted);
      font-size: 0.9rem;
    }
    .button-row {
      display: flex;
      gap: 12px;
      flex-wrap: wrap;
    }
    button {
      border: 1px solid var(--border);
      border-radius: 999px;
      padding: 8px 14px;
      background: #fff;
      cursor: pointer;
      font-family: inherit;
      font-weight: 600;
    }
    section {
      margin-bottom: 28px;
      padding: 16px;
      border: 1px solid var(--border);
      border-radius: 16px;
      background: var(--panel);
    }
    h2 {
      margin: 0 0 12px;
      font-size: 1.1rem;
    }
    table {
      width: 100%;
      border-collapse: collapse;
      font-size: 0.85rem;
    }
    th, td {
      text-align: left;
      padding: 8px;
      border-bottom: 1px solid var(--border);
    }
    th {
      background: rgba(15, 118, 110, 0.08);
    }
    p {
      margin: 0;
      color: var(--muted);
    }
    @media print {
      body { padding: 0; }
      .button-row { display: none; }
      section { break-inside: avoid; }
    }
  </style>
</head>
<body>
  <header>
    <div>
      <h1>HEC-22 Solver Report</h1>
      <div class="meta">Generated ${escapeHtml(new Date().toLocaleString())}</div>
    </div>
    <div class="button-row">
      <button onclick="window.print()">Print report</button>
    </div>
  </header>
  <section>
    <h2>Summary</h2>
    ${summaryTable}
  </section>
  <section>
    <h2>Node Results</h2>
    ${nodeTable}
  </section>
  <section>
    <h2>Conduit Results</h2>
    ${conduitTable}
  </section>
  <section>
    <h2>Drainage Area Results</h2>
    ${drainageTable}
  </section>
  <section>
    <h2>Violations</h2>
    ${violationTable}
  </section>
</body>
</html>`;
}

function ensureResult() {
  if (lastResult) {
    return { ok: true, result: lastResult };
  }
  const text = outputEl.textContent || "";
  if (!text.trim() || text.startsWith("Error")) {
    return { ok: false, error: "Run the solver to generate results." };
  }
  try {
    const parsed = JSON.parse(text);
    lastResult = parsed;
    lastResultRaw = JSON.stringify(parsed, null, 2);
    return { ok: true, result: parsed };
  } catch (error) {
    return { ok: false, error: "Output is not valid JSON." };
  }
}

function normalizeSortValue(value) {
  if (value === null || value === undefined) {
    return null;
  }
  if (typeof value === "number") {
    return value;
  }
  if (typeof value === "boolean") {
    return value ? 1 : 0;
  }
  const text = String(value).trim();
  if (text === "") {
    return null;
  }
  const numeric = Number(text);
  if (Number.isFinite(numeric)) {
    return numeric;
  }
  return text.toLowerCase();
}

function sortRows(rows, sortKey, sortDir) {
  const direction = sortDir === "desc" ? -1 : 1;
  const sorted = [...rows];
  sorted.sort((a, b) => {
    const left = normalizeSortValue(getValueByPath(a, sortKey));
    const right = normalizeSortValue(getValueByPath(b, sortKey));
    if (left === null && right === null) {
      return 0;
    }
    if (left === null) {
      return 1;
    }
    if (right === null) {
      return -1;
    }
    if (left < right) {
      return -1 * direction;
    }
    if (left > right) {
      return 1 * direction;
    }
    return 0;
  });
  return sorted;
}

function rowMatchesFilter(row, columns, filterText) {
  if (!filterText) {
    return true;
  }
  const text = filterText.toLowerCase();
  return columns.some((column) => {
    const value = getValueByPath(row, column.key);
    if (value === null || value === undefined) {
      return false;
    }
    return String(value).toLowerCase().includes(text);
  });
}

function buildDataTable(container, metaEl, rows, columns, stateKey) {
  if (!container) {
    return;
  }
  const state = tableState[stateKey];
  const filtered = rows.filter((row) => rowMatchesFilter(row, columns, state.filter));
  const sorted = sortRows(filtered, state.sortKey, state.sortDir);

  if (metaEl) {
    const total = rows.length;
    const shown = sorted.length;
    metaEl.textContent = total ? `Showing ${shown} of ${total} rows.` : "No rows available.";
  }

  if (!rows.length) {
    container.innerHTML = `<div class="table-empty">No results available.</div>`;
    return;
  }

  if (!sorted.length) {
    container.innerHTML = `<div class="table-empty">No rows match this filter.</div>`;
    return;
  }

  const table = document.createElement("table");
  table.className = "data-table";
  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");

  columns.forEach((column) => {
    const th = document.createElement("th");
    const button = document.createElement("button");
    button.type = "button";
    button.className = "sort-button";
    button.textContent = column.label;
    if (state.sortKey === column.key) {
      const indicator = document.createElement("span");
      indicator.className = "sort-indicator";
      indicator.textContent = state.sortDir === "asc" ? "↑" : "↓";
      button.appendChild(indicator);
    }
    button.addEventListener("click", () => {
      if (state.sortKey === column.key) {
        state.sortDir = state.sortDir === "asc" ? "desc" : "asc";
      } else {
        state.sortKey = column.key;
        state.sortDir = "asc";
      }
      buildDataTable(container, metaEl, rows, columns, stateKey);
    });
    th.appendChild(button);
    headerRow.appendChild(th);
  });

  thead.appendChild(headerRow);
  table.appendChild(thead);

  const tbody = document.createElement("tbody");
  sorted.forEach((row) => {
    const tr = document.createElement("tr");
    columns.forEach((column) => {
      const td = document.createElement("td");
      const raw = getValueByPath(row, column.key);
      const display = column.format ? column.format(raw, row) : raw;
      td.textContent = display === null || display === undefined || display === "" ? "—" : display;
      tr.appendChild(td);
    });
    tbody.appendChild(tr);
  });

  table.appendChild(tbody);
  container.innerHTML = "";
  container.appendChild(table);
}

function renderResultsTables() {
  const resultState = ensureResult();
  if (!resultState.ok) {
    if (nodeTableEl) {
      nodeTableEl.innerHTML = `<div class="table-empty">Run the solver to see results.</div>`;
    }
    if (conduitTableEl) {
      conduitTableEl.innerHTML = `<div class="table-empty">Run the solver to see results.</div>`;
    }
    if (nodeTableMeta) {
      nodeTableMeta.textContent = "";
    }
    if (conduitTableMeta) {
      conduitTableMeta.textContent = "";
    }
    return;
  }

  const { nodeResults, conduitResults } = getAnalysisData(resultState.result);

  const nodeColumns = [
    { key: "nodeId", label: "Node" },
    { key: "hgl", label: "HGL", format: formatNumber },
    { key: "egl", label: "EGL", format: formatNumber },
    { key: "depth", label: "Depth", format: formatNumber },
    { key: "velocity", label: "Velocity", format: formatNumber },
    { key: "flooding", label: "Flooding", format: formatBool },
    { key: "junctionLoss", label: "Junction loss", format: formatNumber },
    { key: "gutterSpread", label: "Gutter spread", format: formatNumber }
  ];

  const conduitColumns = [
    { key: "conduitId", label: "Conduit" },
    { key: "flow", label: "Flow", format: formatNumber },
    { key: "velocity", label: "Velocity", format: formatNumber },
    { key: "depth", label: "Depth", format: formatNumber },
    { key: "capacityUsed", label: "Capacity used", format: formatNumber },
    { key: "froudeNumber", label: "Froude", format: formatNumber },
    { key: "flowRegime", label: "Regime" },
    { key: "headloss.total", label: "Headloss", format: formatNumber }
  ];

  buildDataTable(nodeTableEl, nodeTableMeta, nodeResults, nodeColumns, "nodes");
  buildDataTable(
    conduitTableEl,
    conduitTableMeta,
    conduitResults,
    conduitColumns,
    "conduits"
  );
}

function computeAutoLayout(nodes, conduits) {
  const adjacency = new Map();
  const indegree = new Map();
  nodes.forEach((node) => {
    adjacency.set(node.id, []);
    indegree.set(node.id, 0);
  });
  conduits.forEach((conduit) => {
    if (!adjacency.has(conduit.fromNode) || !adjacency.has(conduit.toNode)) {
      return;
    }
    adjacency.get(conduit.fromNode).push(conduit.toNode);
    indegree.set(conduit.toNode, (indegree.get(conduit.toNode) || 0) + 1);
  });

  const queue = [];
  indegree.forEach((count, nodeId) => {
    if (count === 0) {
      queue.push(nodeId);
    }
  });

  const level = new Map();
  queue.forEach((nodeId) => level.set(nodeId, 0));
  const order = [];
  while (queue.length) {
    const nodeId = queue.shift();
    order.push(nodeId);
    const next = adjacency.get(nodeId) || [];
    next.forEach((downstream) => {
      const nextLevel = Math.max(level.get(downstream) || 0, (level.get(nodeId) || 0) + 1);
      level.set(downstream, nextLevel);
      indegree.set(downstream, (indegree.get(downstream) || 0) - 1);
      if (indegree.get(downstream) === 0) {
        queue.push(downstream);
      }
    });
  }

  const levelGroups = new Map();
  nodes.forEach((node) => {
    const nodeLevel = level.get(node.id) || 0;
    if (!levelGroups.has(nodeLevel)) {
      levelGroups.set(nodeLevel, []);
    }
    levelGroups.get(nodeLevel).push(node.id);
  });

  const positions = new Map();
  const levels = Array.from(levelGroups.keys()).sort((a, b) => a - b);
  levels.forEach((lvl, index) => {
    const ids = levelGroups.get(lvl) || [];
    ids.forEach((id, rowIndex) => {
      positions.set(id, {
        x: index,
        y: rowIndex
      });
    });
  });

  return positions;
}

function buildCoordinateMap(nodes, conduits) {
  const coordsAvailable = nodes.every(
    (node) =>
      node.coordinates &&
      typeof node.coordinates.x === "number" &&
      typeof node.coordinates.y === "number"
  );

  if (coordsAvailable) {
    const positions = new Map();
    nodes.forEach((node) => {
      positions.set(node.id, { x: node.coordinates.x, y: node.coordinates.y });
    });
    return { positions, usedAutoLayout: false };
  }

  const autoPositions = computeAutoLayout(nodes, conduits);
  return { positions: autoPositions, usedAutoLayout: true };
}

function colorScale(value, min, max) {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "#94a3b8";
  }
  if (min === max) {
    return "#22c55e";
  }
  const ratio = Math.min(Math.max((value - min) / (max - min), 0), 1);
  const hue = 210 - ratio * 120;
  return `hsl(${hue}, 70%, 50%)`;
}

function buildPlanLegend(metricLabel, minValue, maxValue) {
  if (!planLegend) {
    return;
  }
  const minLabel = Number.isFinite(minValue) ? formatNumber(minValue) : "—";
  const maxLabel = Number.isFinite(maxValue) ? formatNumber(maxValue) : "—";
  planLegend.innerHTML = `
    <div class="legend-title">${escapeHtml(metricLabel)} scale</div>
    <div class="legend-scale"></div>
    <div class="legend-row"><span>${minLabel}</span><span>${maxLabel}</span></div>
    <div class="legend-title">Flow regime</div>
    <div class="legend-row">
      <span class="legend-chip"><span class="legend-dot" style="background:#2563eb"></span>Subcritical</span>
      <span class="legend-chip"><span class="legend-dot" style="background:#f59e0b"></span>Critical</span>
    </div>
    <div class="legend-row">
      <span class="legend-chip"><span class="legend-dot" style="background:#dc2626"></span>Supercritical</span>
      <span class="legend-chip"><span class="legend-dot" style="background:#94a3b8"></span>Unknown</span>
    </div>
  `;
}

function ensurePlanTooltip() {
  if (!planWrap) {
    return null;
  }
  let tooltip = planWrap.querySelector(".plan-tooltip");
  if (!tooltip) {
    tooltip = document.createElement("div");
    tooltip.className = "plan-tooltip";
    planWrap.appendChild(tooltip);
  }
  return tooltip;
}

function showPlanTooltip(text, event) {
  const tooltip = ensurePlanTooltip();
  if (!tooltip || !planWrap) {
    return;
  }
  tooltip.textContent = text;
  const bounds = planWrap.getBoundingClientRect();
  const x = event.clientX - bounds.left + 12;
  const y = event.clientY - bounds.top + 12;
  tooltip.style.left = `${x}px`;
  tooltip.style.top = `${y}px`;
  tooltip.classList.add("visible");
}

function hidePlanTooltip() {
  const tooltip = ensurePlanTooltip();
  if (!tooltip) {
    return;
  }
  tooltip.classList.remove("visible");
}

function applyPlanViewBox() {
  if (!networkPlan) {
    return;
  }
  networkPlan.setAttribute(
    "viewBox",
    `${planViewBox.x} ${planViewBox.y} ${planViewBox.width} ${planViewBox.height}`
  );
}

function clampPlanScale(scale) {
  return Math.min(Math.max(scale, PLAN_MIN_SCALE), PLAN_MAX_SCALE);
}

function getPlanScale() {
  return PLAN_DEFAULT_VIEWBOX.width / planViewBox.width;
}

function setPlanScale(newScale, anchor) {
  const scale = clampPlanScale(newScale);
  const currentScale = getPlanScale();
  if (!anchor || !networkPlan) {
    planViewBox.width = PLAN_DEFAULT_VIEWBOX.width / scale;
    planViewBox.height = PLAN_DEFAULT_VIEWBOX.height / scale;
    applyPlanViewBox();
    return;
  }

  const rect = networkPlan.getBoundingClientRect();
  const relX = (anchor.x - rect.left) / rect.width;
  const relY = (anchor.y - rect.top) / rect.height;
  const cursorX = planViewBox.x + relX * planViewBox.width;
  const cursorY = planViewBox.y + relY * planViewBox.height;
  const nextWidth = PLAN_DEFAULT_VIEWBOX.width / scale;
  const nextHeight = PLAN_DEFAULT_VIEWBOX.height / scale;

  planViewBox.x = cursorX - relX * nextWidth;
  planViewBox.y = cursorY - relY * nextHeight;
  planViewBox.width = nextWidth;
  planViewBox.height = nextHeight;
  applyPlanViewBox();

  if (Math.abs(scale - currentScale) < 0.0001) {
    return;
  }
}

function resetPlanView() {
  planViewBox.x = PLAN_DEFAULT_VIEWBOX.x;
  planViewBox.y = PLAN_DEFAULT_VIEWBOX.y;
  planViewBox.width = PLAN_DEFAULT_VIEWBOX.width;
  planViewBox.height = PLAN_DEFAULT_VIEWBOX.height;
  applyPlanViewBox();
}

function startPan(event) {
  if (!networkPlan || event.button !== 0) {
    return;
  }
  isPanning = true;
  panStart = { x: event.clientX, y: event.clientY };
  panViewBoxStart = { x: planViewBox.x, y: planViewBox.y };
}

function movePan(event) {
  if (!isPanning || !networkPlan) {
    return;
  }
  const rect = networkPlan.getBoundingClientRect();
  const dx = ((event.clientX - panStart.x) / rect.width) * planViewBox.width;
  const dy = ((event.clientY - panStart.y) / rect.height) * planViewBox.height;
  planViewBox.x = panViewBoxStart.x - dx;
  planViewBox.y = panViewBoxStart.y - dy;
  applyPlanViewBox();
}

function endPan() {
  isPanning = false;
}

function renderPlanView() {
  if (!networkPlan) {
    return;
  }
  const requestState = ensureRequest();
  if (!requestState.ok) {
    networkPlan.innerHTML = "";
    buildPlanLegend("HGL", NaN, NaN);
    updatePlanStatus(requestState.error, true);
    return;
  }
  const resultState = ensureResult();
  if (!resultState.ok) {
    networkPlan.innerHTML = "";
    buildPlanLegend("HGL", NaN, NaN);
    updatePlanStatus("Run the solver to render the plan view.", false);
    return;
  }

  const network = requestState.request.network;
  if (!network || !Array.isArray(network.nodes) || !Array.isArray(network.conduits)) {
    networkPlan.innerHTML = "";
    buildPlanLegend("HGL", NaN, NaN);
    updatePlanStatus("Network data missing in request JSON.", true);
    return;
  }

  const nodes = network.nodes;
  const conduits = network.conduits;

  const { positions, usedAutoLayout } = buildCoordinateMap(nodes, conduits);
  updatePlanStatus(
    usedAutoLayout ? "No coordinates found. Using auto layout." : "Plan view ready.",
    false
  );

  const { nodeResults, conduitResults } = getAnalysisData(resultState.result);
  const metricKey = planMetricSelect ? planMetricSelect.value : "hgl";
  const metricLabel = metricKey.toUpperCase();
  const nodeMetric = new Map();
  nodeResults.forEach((node) => {
    const value = metricKey === "egl" ? node.egl : node.hgl;
    nodeMetric.set(node.nodeId, value);
  });

  const metricValues = Array.from(nodeMetric.values()).filter((value) => Number.isFinite(value));
  const minValue = metricValues.length ? Math.min(...metricValues) : NaN;
  const maxValue = metricValues.length ? Math.max(...metricValues) : NaN;
  buildPlanLegend(metricLabel, minValue, maxValue);

  const flowRegimeMap = new Map();
  conduitResults.forEach((conduit) => {
    flowRegimeMap.set(conduit.conduitId, conduit.flowRegime);
  });

  const width = PLAN_DEFAULT_VIEWBOX.width;
  const height = PLAN_DEFAULT_VIEWBOX.height;
  const padding = 50;
  const coords = Array.from(positions.values());
  const xs = coords.map((pos) => pos.x);
  const ys = coords.map((pos) => pos.y);
  const minX = xs.length ? Math.min(...xs) : 0;
  const maxX = xs.length ? Math.max(...xs) : 1;
  const minY = ys.length ? Math.min(...ys) : 0;
  const maxY = ys.length ? Math.max(...ys) : 1;

  const scaleX = (value) =>
    padding + ((value - minX) / (maxX - minX || 1)) * (width - padding * 2);
  const scaleY = (value) =>
    padding + ((value - minY) / (maxY - minY || 1)) * (height - padding * 2);

  applyPlanViewBox();
  networkPlan.innerHTML = `
    <defs>
      <marker id="arrow" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
        <path d="M 0 0 L 10 5 L 0 10 z" fill="currentColor"></path>
      </marker>
    </defs>
  `;

  const lineGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  const nodeGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");

  const regimeColors = {
    subcritical: "#2563eb",
    critical: "#f59e0b",
    supercritical: "#dc2626"
  };

  conduits.forEach((conduit) => {
    const fromPos = positions.get(conduit.fromNode);
    const toPos = positions.get(conduit.toNode);
    if (!fromPos || !toPos) {
      return;
    }
    const line = document.createElementNS("http://www.w3.org/2000/svg", "line");
    line.setAttribute("x1", scaleX(fromPos.x));
    line.setAttribute("y1", scaleY(fromPos.y));
    line.setAttribute("x2", scaleX(toPos.x));
    line.setAttribute("y2", scaleY(toPos.y));
    const regime = flowRegimeMap.get(conduit.id);
    const stroke = regimeColors[regime] || "#94a3b8";
    line.setAttribute("stroke", stroke);
    line.setAttribute("color", stroke);
    line.setAttribute("stroke-width", "2.5");
    line.setAttribute("marker-end", "url(#arrow)");
    lineGroup.appendChild(line);

    line.addEventListener("mousemove", (event) => {
      const text = `${conduit.id} → ${conduit.toNode} (${regime || "unknown"})`;
      showPlanTooltip(text, event);
    });
    line.addEventListener("mouseleave", hidePlanTooltip);
  });

  nodes.forEach((node) => {
    const pos = positions.get(node.id);
    if (!pos) {
      return;
    }
    const value = nodeMetric.get(node.id);
    const color = colorScale(value, minValue, maxValue);
    const cx = scaleX(pos.x);
    const cy = scaleY(pos.y);

    const circle = document.createElementNS("http://www.w3.org/2000/svg", "circle");
    circle.setAttribute("cx", cx);
    circle.setAttribute("cy", cy);
    circle.setAttribute("r", "6");
    circle.setAttribute("fill", color);
    circle.setAttribute("stroke", "#0f172a");
    circle.setAttribute("stroke-width", "1");
    nodeGroup.appendChild(circle);

    const label = document.createElementNS("http://www.w3.org/2000/svg", "text");
    label.setAttribute("x", cx + 8);
    label.setAttribute("y", cy - 8);
    label.setAttribute("font-size", "10");
    label.setAttribute("fill", "#0f172a");
    label.classList.add("plan-label");
    label.textContent = node.id;
    nodeGroup.appendChild(label);

    circle.addEventListener("mousemove", (event) => {
      const text = `${node.id} ${metricLabel}: ${formatNumber(value)}`;
      showPlanTooltip(text, event);
    });
    circle.addEventListener("mouseleave", hidePlanTooltip);
  });

  networkPlan.appendChild(lineGroup);
  networkPlan.appendChild(nodeGroup);
}

function ensureProfileTooltip() {
  if (!profileWrap) {
    return null;
  }
  let tooltip = profileWrap.querySelector(".profile-tooltip");
  if (!tooltip) {
    tooltip = document.createElement("div");
    tooltip.className = "profile-tooltip";
    profileWrap.appendChild(tooltip);
  }
  return tooltip;
}

function showProfileTooltip(text, event) {
  const tooltip = ensureProfileTooltip();
  if (!tooltip || !profileWrap) {
    return;
  }
  tooltip.textContent = text;
  const bounds = profileWrap.getBoundingClientRect();
  const x = event.clientX - bounds.left + 12;
  const y = event.clientY - bounds.top + 12;
  tooltip.style.left = `${x}px`;
  tooltip.style.top = `${y}px`;
  tooltip.classList.add("visible");
}

function hideProfileTooltip() {
  const tooltip = ensureProfileTooltip();
  if (!tooltip) {
    return;
  }
  tooltip.classList.remove("visible");
}

function buildPath(network, startId, endId) {
  const conduits = network.conduits || [];
  const adjacency = new Map();
  conduits.forEach((conduit) => {
    if (!adjacency.has(conduit.fromNode)) {
      adjacency.set(conduit.fromNode, []);
    }
    adjacency.get(conduit.fromNode).push(conduit);
  });

  const queue = [startId];
  const visited = new Set([startId]);
  const prev = new Map();

  while (queue.length) {
    const current = queue.shift();
    if (current === endId) {
      break;
    }
    const edges = adjacency.get(current) || [];
    edges.forEach((conduit) => {
      const next = conduit.toNode;
      if (!visited.has(next)) {
        visited.add(next);
        prev.set(next, { node: current, conduit });
        queue.push(next);
      }
    });
  }

  if (!visited.has(endId)) {
    return null;
  }

  const nodes = [];
  const conduitsPath = [];
  let cursor = endId;
  while (cursor !== startId) {
    const entry = prev.get(cursor);
    if (!entry) {
      break;
    }
    conduitsPath.push(entry.conduit);
    nodes.push(cursor);
    cursor = entry.node;
  }
  nodes.push(startId);
  nodes.reverse();
  conduitsPath.reverse();
  return { nodes, conduits: conduitsPath };
}

function buildProfilePath(points) {
  let path = "";
  let started = false;
  points.forEach((point) => {
    if (!Number.isFinite(point.y)) {
      started = false;
      return;
    }
    if (!started) {
      path += `M ${point.x} ${point.y}`;
      started = true;
    } else {
      path += ` L ${point.x} ${point.y}`;
    }
  });
  return path;
}

function renderProfileView() {
  if (!profileChart) {
    return;
  }
  const requestState = ensureRequest();
  if (!requestState.ok) {
    profileChart.innerHTML = "";
    updateProfileStatus(requestState.error, true);
    return;
  }
  const resultState = ensureResult();
  if (!resultState.ok) {
    profileChart.innerHTML = "";
    updateProfileStatus("Run the solver to render the profile.", false);
    return;
  }

  const network = requestState.request.network;
  if (!network || !Array.isArray(network.nodes) || !Array.isArray(network.conduits)) {
    profileChart.innerHTML = "";
    updateProfileStatus("Network data missing in request JSON.", true);
    return;
  }

  const startId = profileStartSelect ? profileStartSelect.value : null;
  const endId = profileEndSelect ? profileEndSelect.value : null;
  if (!startId || !endId) {
    profileChart.innerHTML = "";
    updateProfileStatus("Select start and end nodes.", true);
    return;
  }

  const path = buildPath(network, startId, endId);
  if (!path) {
    profileChart.innerHTML = "";
    updateProfileStatus("No path found between selected nodes.", true);
    return;
  }

  const nodeMap = new Map(network.nodes.map((node) => [node.id, node]));
  const { nodeResults } = getAnalysisData(resultState.result);
  const resultMap = new Map(nodeResults.map((node) => [node.nodeId, node]));

  const distances = [0];
  path.conduits.forEach((conduit) => {
    const length = Number(conduit.length || 0);
    distances.push(distances[distances.length - 1] + (Number.isFinite(length) ? length : 0));
  });

  const hglValues = path.nodes.map((id) => resultMap.get(id)?.hgl ?? null);
  const eglValues = path.nodes.map((id) => resultMap.get(id)?.egl ?? null);
  const invertValues = path.nodes.map((id) => nodeMap.get(id)?.invertElevation ?? null);

  const allValues = []
    .concat(hglValues, eglValues, invertValues)
    .filter((value) => Number.isFinite(value));
  if (!allValues.length) {
    profileChart.innerHTML = "";
    updateProfileStatus("No HGL/EGL values available for this path.", true);
    return;
  }

  const minValue = Math.min(...allValues);
  const maxValue = Math.max(...allValues);
  const width = 800;
  const height = 320;
  const paddingLeft = 50;
  const paddingRight = 20;
  const paddingTop = 20;
  const paddingBottom = 40;
  const plotWidth = width - paddingLeft - paddingRight;
  const plotHeight = height - paddingTop - paddingBottom;
  const totalLength = distances[distances.length - 1] || 1;

  const scaleX = (value) => paddingLeft + (value / totalLength) * plotWidth;
  const scaleY = (value) =>
    paddingTop + ((maxValue - value) / (maxValue - minValue || 1)) * plotHeight;

  const hglPoints = distances.map((dist, index) => ({
    x: scaleX(dist),
    y: Number.isFinite(hglValues[index]) ? scaleY(hglValues[index]) : NaN
  }));
  const eglPoints = distances.map((dist, index) => ({
    x: scaleX(dist),
    y: Number.isFinite(eglValues[index]) ? scaleY(eglValues[index]) : NaN
  }));
  const invertPoints = distances.map((dist, index) => ({
    x: scaleX(dist),
    y: Number.isFinite(invertValues[index]) ? scaleY(invertValues[index]) : NaN
  }));

  const gridLines = 5;
  const ticks = [];
  for (let i = 0; i <= gridLines; i += 1) {
    const value = maxValue - (i / gridLines) * (maxValue - minValue || 1);
    ticks.push({ value, y: scaleY(value) });
  }

  profileChart.setAttribute("viewBox", `0 0 ${width} ${height}`);
  profileChart.innerHTML = "";

  const svgNs = "http://www.w3.org/2000/svg";
  const gridGroup = document.createElementNS(svgNs, "g");
  ticks.forEach((tick) => {
    const line = document.createElementNS(svgNs, "line");
    line.setAttribute("x1", paddingLeft);
    line.setAttribute("x2", width - paddingRight);
    line.setAttribute("y1", tick.y);
    line.setAttribute("y2", tick.y);
    line.setAttribute("class", "profile-grid");
    gridGroup.appendChild(line);

    const label = document.createElementNS(svgNs, "text");
    label.setAttribute("x", 8);
    label.setAttribute("y", tick.y + 4);
    label.setAttribute("class", "profile-label");
    label.textContent = formatNumber(tick.value);
    gridGroup.appendChild(label);
  });

  const axisLine = document.createElementNS(svgNs, "line");
  axisLine.setAttribute("x1", paddingLeft);
  axisLine.setAttribute("x2", width - paddingRight);
  axisLine.setAttribute("y1", height - paddingBottom);
  axisLine.setAttribute("y2", height - paddingBottom);
  axisLine.setAttribute("class", "profile-axis");
  gridGroup.appendChild(axisLine);

  const xLabelsGroup = document.createElementNS(svgNs, "g");
  path.nodes.forEach((nodeId, index) => {
    const x = scaleX(distances[index]);
    const label = document.createElementNS(svgNs, "text");
    label.setAttribute("x", x);
    label.setAttribute("y", height - 12);
    label.setAttribute("text-anchor", "middle");
    label.setAttribute("class", "profile-label");
    label.textContent = nodeId;
    xLabelsGroup.appendChild(label);
  });

  const lineGroup = document.createElementNS(svgNs, "g");
  const markerGroup = document.createElementNS(svgNs, "g");

  const hglPath = document.createElementNS(svgNs, "path");
  hglPath.setAttribute("d", buildProfilePath(hglPoints));
  hglPath.setAttribute("class", "profile-line hgl");
  lineGroup.appendChild(hglPath);

  const eglPath = document.createElementNS(svgNs, "path");
  eglPath.setAttribute("d", buildProfilePath(eglPoints));
  eglPath.setAttribute("class", "profile-line egl");
  lineGroup.appendChild(eglPath);

  if (profileInvertToggle && profileInvertToggle.checked) {
    const invertPath = document.createElementNS(svgNs, "path");
    invertPath.setAttribute("d", buildProfilePath(invertPoints));
    invertPath.setAttribute("class", "profile-line invert");
    lineGroup.appendChild(invertPath);
  }

  const xPositions = distances.map((dist) => scaleX(dist));
  const stations = path.nodes.map((nodeId, index) => ({
    id: nodeId,
    distance: distances[index],
    hgl: hglValues[index],
    egl: eglValues[index],
    invert: invertValues[index]
  }));

  stations.forEach((station, index) => {
    const left = index === 0 ? paddingLeft : (xPositions[index - 1] + xPositions[index]) / 2;
    const right =
      index === stations.length - 1
        ? width - paddingRight
        : (xPositions[index] + xPositions[index + 1]) / 2;
    const rect = document.createElementNS(svgNs, "rect");
    rect.setAttribute("x", left);
    rect.setAttribute("y", paddingTop);
    rect.setAttribute("width", right - left);
    rect.setAttribute("height", plotHeight);
    rect.setAttribute("fill", "transparent");
    rect.addEventListener("mousemove", (event) => {
      const parts = [
        `${station.id}`,
        `Sta ${formatNumber(station.distance)}`,
        `HGL ${formatNumber(station.hgl)}`,
        `EGL ${formatNumber(station.egl)}`
      ];
      if (profileInvertToggle && profileInvertToggle.checked) {
        parts.push(`Inv ${formatNumber(station.invert)}`);
      }
      showProfileTooltip(parts.join(" | "), event);
    });
    rect.addEventListener("mouseleave", hideProfileTooltip);
    markerGroup.appendChild(rect);
  });

  profileChart.appendChild(gridGroup);
  profileChart.appendChild(lineGroup);
  profileChart.appendChild(markerGroup);
  profileChart.appendChild(xLabelsGroup);
  updateProfileStatus("Profile rendered.", false);
}

function summarizeCsvState() {
  const parts = [];
  if (csvState.nodes) {
    parts.push(`nodes: ${csvState.nodes.records.length}`);
  }
  if (csvState.conduits) {
    parts.push(`conduits: ${csvState.conduits.records.length}`);
  }
  if (csvState.areas) {
    parts.push(`areas: ${csvState.areas.records.length}`);
  }
  if (!parts.length) {
    return "No CSV files loaded.";
  }
  const ready = csvState.nodes && csvState.conduits ? " Ready to build request." : "";
  return `Loaded ${parts.join(", ")}.${ready}`;
}

function createBadge(text, variant = "default") {
  const badge = document.createElement("span");
  badge.className = `csv-badge csv-badge-${variant}`;
  badge.textContent = text;
  return badge;
}

function findMissingRequired(headers, required, mapping) {
  const headerSet = new Set(headers);
  return required.filter((entry) => {
    const mappedHeader = mapping && mapping[entry.label];
    if (mappedHeader) {
      return !headerSet.has(mappedHeader);
    }
    return !entry.keys.some((key) => headerSet.has(key));
  });
}

function validateNodesRow(record, mapping) {
  const missing = [];
  if (!getString(record, ["id"], mapping)) {
    missing.push("id");
  }
  if (!getString(record, ["type", "node_type"], mapping)) {
    missing.push("type");
  }
  if (getNumber(record, ["invert_elev", "invert_elevation", "invert"], mapping) === null) {
    missing.push("invert_elev");
  }
  return missing;
}

function validateConduitsRow(record, mapping) {
  const missing = [];
  if (!getString(record, ["id"], mapping)) {
    missing.push("id");
  }
  if (!getString(record, ["from_node", "fromnode"], mapping)) {
    missing.push("from_node");
  }
  if (!getString(record, ["to_node", "tonode"], mapping)) {
    missing.push("to_node");
  }
  if (getNumber(record, ["length"], mapping) === null) {
    missing.push("length");
  }

  const conduitType = (getString(record, ["type", "conduit_type"], mapping) || "pipe")
    .toLowerCase();
  if (conduitType === "gutter") {
    if (getNumber(record, ["cross_slope", "crossslope"], mapping) === null) {
      missing.push("cross_slope");
    }
    if (getNumber(record, ["long_slope", "longslope", "slope"], mapping) === null) {
      missing.push("long_slope");
    }
  } else if (getNumber(record, ["diameter"], mapping) === null) {
    missing.push("diameter");
  }

  return missing;
}

function validateAreasRow(record, mapping) {
  const missing = [];
  if (!getString(record, ["id"], mapping)) {
    missing.push("id");
  }
  if (getNumber(record, ["area"], mapping) === null) {
    missing.push("area");
  }
  if (getNumber(record, ["runoff_coef", "runoff_coefficient"], mapping) === null) {
    missing.push("runoff_coef");
  }
  if (getNumber(record, ["time_of_conc", "time_of_concentration"], mapping) === null) {
    missing.push("time_of_conc");
  }
  if (!getString(record, ["outlet_node", "outlet"], mapping)) {
    missing.push("outlet_node");
  }
  return missing;
}

const CSV_SCHEMAS = {
  nodes: {
    label: "Nodes",
    required: [
      { keys: ["id"], label: "id" },
      { keys: ["type", "node_type"], label: "type" },
      { keys: ["invert_elev", "invert_elevation", "invert"], label: "invert_elev" }
    ],
    validateRow: validateNodesRow
  },
  conduits: {
    label: "Conduits",
    required: [
      { keys: ["id"], label: "id" },
      { keys: ["from_node", "fromnode"], label: "from_node" },
      { keys: ["to_node", "tonode"], label: "to_node" },
      { keys: ["length"], label: "length" }
    ],
    validateRow: validateConduitsRow
  },
  areas: {
    label: "Drainage areas",
    required: [
      { keys: ["id"], label: "id" },
      { keys: ["area"], label: "area" },
      { keys: ["runoff_coef", "runoff_coefficient"], label: "runoff_coef" },
      { keys: ["time_of_conc", "time_of_concentration"], label: "time_of_conc" },
      { keys: ["outlet_node", "outlet"], label: "outlet_node" }
    ],
    validateRow: validateAreasRow
  }
};

function renderCsvPreviewCard(container, key, state) {
  const schema = CSV_SCHEMAS[key];
  const card = document.createElement("div");
  card.className = "csv-preview-card";

  const title = document.createElement("h4");
  title.textContent = `${schema.label} CSV`;
  card.appendChild(title);

  if (!state) {
    const empty = document.createElement("p");
    empty.className = "csv-empty";
    empty.textContent = "No file loaded.";
    card.appendChild(empty);
    container.appendChild(card);
    return;
  }

  const { headers, rawHeaders, records, mapping } = state;
  const meta = document.createElement("div");
  meta.className = "csv-meta";
  meta.appendChild(createBadge(`${records.length} rows`, "count"));

  const missingHeaders = findMissingRequired(headers, schema.required, mapping);
  if (missingHeaders.length) {
    meta.appendChild(
      createBadge(
        `Missing columns: ${missingHeaders.map((entry) => entry.label).join(", ")}`,
        "warning"
      )
    );
  } else {
    meta.appendChild(createBadge("Required columns present", "ok"));
  }

  const headerPreview = headers.slice(0, 6).join(", ");
  const headerSuffix = headers.length > 6 ? `, +${headers.length - 6} more` : "";
  const headerNote = document.createElement("span");
  headerNote.textContent = `Headers: ${headerPreview || "none"}${headerSuffix}`;
  meta.appendChild(headerNote);
  card.appendChild(meta);

  const issuesByIndex = new Map();
  const issuesList = [];
  records.forEach((record, index) => {
    const issues = schema.validateRow(record, mapping);
    if (issues.length) {
      issuesByIndex.set(index, issues);
      issuesList.push({ record, issues });
    }
  });
  const errorRowCount = issuesList.length;

  const summary = document.createElement("div");
  summary.className = "csv-meta";
  if (errorRowCount) {
    summary.appendChild(createBadge(`${errorRowCount} rows need fixes`, "warning"));
  } else {
    summary.appendChild(createBadge("All rows valid", "ok"));
  }
  card.appendChild(summary);

  const previewLimit = 5;
  const table = document.createElement("table");
  table.className = "csv-table";
  const thead = document.createElement("thead");
  const headerRow = document.createElement("tr");
  const previewHeaders = headers.length ? headers : ["(no headers)"];
  previewHeaders.forEach((header) => {
    const th = document.createElement("th");
    th.textContent = header;
    headerRow.appendChild(th);
  });
  if (errorRowCount) {
    const th = document.createElement("th");
    th.textContent = "issues";
    headerRow.appendChild(th);
  }
  thead.appendChild(headerRow);
  table.appendChild(thead);

  const tbody = document.createElement("tbody");
  records.slice(0, previewLimit).forEach((record, index) => {
    const row = document.createElement("tr");
    const rowIssues = issuesByIndex.get(index) || [];
    if (rowIssues.length) {
      row.classList.add("row-error");
    }
    previewHeaders.forEach((header) => {
      const td = document.createElement("td");
      td.textContent = record[header] || "";
      row.appendChild(td);
    });
    if (errorRowCount) {
      const td = document.createElement("td");
      td.textContent = rowIssues.length ? `Missing: ${rowIssues.join(", ")}` : "";
      row.appendChild(td);
    }
    tbody.appendChild(row);
  });

  table.appendChild(tbody);
  const tableWrap = document.createElement("div");
  tableWrap.className = "csv-table-wrap";
  tableWrap.appendChild(table);
  card.appendChild(tableWrap);

  if (records.length > previewLimit) {
    const note = document.createElement("p");
    note.className = "csv-empty";
    note.textContent = `Showing ${previewLimit} of ${records.length} rows.`;
    card.appendChild(note);
  }

  if (errorRowCount) {
    const actions = document.createElement("div");
    actions.className = "csv-card-actions";

    const exportButton = document.createElement("button");
    exportButton.type = "button";
    exportButton.className = "ghost csv-export";
    exportButton.textContent = "Download rows with issues";
    exportButton.addEventListener("click", () => {
      const content = buildIssuesCsv(state, issuesList);
      downloadCsv(`${key}_rows_with_issues.csv`, content);
    });

    actions.appendChild(exportButton);
    card.appendChild(actions);
  }

  const mappingSection = document.createElement("div");
  mappingSection.className = "csv-mapping";

  const mappingTitle = document.createElement("p");
  mappingTitle.className = "csv-mapping-title";
  mappingTitle.textContent = "Column mapping overrides";
  mappingSection.appendChild(mappingTitle);

  const headerOptions = headers.map((header, index) => ({
    value: header,
    label: rawHeaders[index] ? `${rawHeaders[index]} (${header})` : header
  }));

  schema.required.forEach((entry) => {
    const row = document.createElement("div");
    row.className = "csv-mapping-row";

    const label = document.createElement("span");
    label.textContent = entry.label;
    row.appendChild(label);

    const select = document.createElement("select");
    select.dataset.csvKey = key;
    select.dataset.field = entry.label;

    const autoMatch = entry.keys.find((keyName) => headers.includes(keyName));
    const autoOption = document.createElement("option");
    autoOption.value = "";
    autoOption.textContent = autoMatch ? `Auto (${autoMatch})` : "Auto";
    select.appendChild(autoOption);

    headerOptions.forEach((optionData) => {
      const option = document.createElement("option");
      option.value = optionData.value;
      option.textContent = optionData.label;
      select.appendChild(option);
    });

    select.value = mapping && mapping[entry.label] ? mapping[entry.label] : "";
    row.appendChild(select);
    mappingSection.appendChild(row);
  });

  card.appendChild(mappingSection);
  container.appendChild(card);
}

function renderCsvPreview() {
  if (!csvPreview) {
    return;
  }
  csvPreview.innerHTML = "";
  renderCsvPreviewCard(csvPreview, "nodes", csvState.nodes);
  renderCsvPreviewCard(csvPreview, "conduits", csvState.conduits);
  renderCsvPreviewCard(csvPreview, "areas", csvState.areas);
}

async function handleCsvInput(event, key, label) {
  const file = event.target.files[0];
  if (!file) {
    csvState[key] = null;
    updateCsvStatus(summarizeCsvState());
    renderCsvPreview();
    return;
  }
  try {
    const text = await file.text();
    const { headers, rawHeaders, records } = parseCsv(text);
    if (!records.length) {
      throw new Error("No data rows found.");
    }
    csvState[key] = { headers, rawHeaders, records, mapping: {} };
    updateCsvStatus(summarizeCsvState());
    renderCsvPreview();
  } catch (error) {
    csvState[key] = null;
    updateCsvStatus(`${label} CSV error: ${String(error.message || error)}`, true);
    renderCsvPreview();
  }
}

fileInput.addEventListener("change", async (event) => {
  const file = event.target.files[0];
  if (!file) {
    return;
  }
  const text = await file.text();
  requestEl.value = text;
  lastRequest = null;
  validateJsonInput();
});

loadSampleButton.addEventListener("click", () => {
  requestEl.value = JSON.stringify(sampleRequest, null, 2);
  lastRequest = null;
  validateJsonInput();
});

nodesCsvInput.addEventListener("change", (event) => {
  handleCsvInput(event, "nodes", "Nodes");
});

conduitsCsvInput.addEventListener("change", (event) => {
  handleCsvInput(event, "conduits", "Conduits");
});

areasCsvInput.addEventListener("change", (event) => {
  handleCsvInput(event, "areas", "Drainage areas");
});

if (csvPreview) {
  csvPreview.addEventListener("change", (event) => {
    const target = event.target;
    if (!(target instanceof HTMLSelectElement)) {
      return;
    }
    const key = target.dataset.csvKey;
    const field = target.dataset.field;
    if (!key || !field || !csvState[key]) {
      return;
    }
    const value = target.value;
    if (value) {
      csvState[key].mapping[field] = value;
    } else {
      delete csvState[key].mapping[field];
    }
    updateCsvStatus(summarizeCsvState());
    renderCsvPreview();
  });
}

if (downloadResultsJsonButton) {
  downloadResultsJsonButton.addEventListener("click", () => {
    const resultState = ensureResult();
    if (!resultState.ok) {
      updateExportStatus(resultState.error, true);
      return;
    }
    const json = lastResultRaw || JSON.stringify(resultState.result, null, 2);
    downloadFile("results.json", json, "application/json");
    updateExportStatus("Downloaded results JSON.");
  });
}

if (downloadResultsCsvButton) {
  downloadResultsCsvButton.addEventListener("click", () => {
    const resultState = ensureResult();
    if (!resultState.ok) {
      updateExportStatus(resultState.error, true);
      return;
    }
    downloadResultsCsvs(resultState.result);
    updateExportStatus("Downloaded results CSV files.");
  });
}

if (openReportButton) {
  openReportButton.addEventListener("click", () => {
    const resultState = ensureResult();
    if (!resultState.ok) {
      updateExportStatus(resultState.error, true);
      return;
    }
    const reportHtml = buildReportHtml(resultState.result);
    const reportWindow = window.open("", "hec22-report");
    if (!reportWindow) {
      updateExportStatus("Unable to open report window (popup blocked).", true);
      return;
    }
    reportWindow.document.open();
    reportWindow.document.write(reportHtml);
    reportWindow.document.close();
    updateExportStatus("Report view opened.");
  });
}

buildFromCsvButton.addEventListener("click", () => {
  try {
    const request = buildRequestFromCsv();
    requestEl.value = JSON.stringify(request, null, 2);
    updateCsvStatus("Request JSON built from CSV files.");
    lastRequest = null;
    validateJsonInput();
  } catch (error) {
    updateCsvStatus(`Build failed: ${String(error.message || error)}`, true);
  }
});

downloadTemplatesButton.addEventListener("click", () => {
  const nodeHeader = [
    "id",
    "type",
    "invert_elev",
    "rim_elev",
    "inlet_type",
    "inlet_location",
    "local_depression",
    "clogging_factor",
    "grate_length",
    "grate_width",
    "bar_configuration",
    "curb_opening_length",
    "curb_opening_height",
    "throat_type",
    "bypass_to",
    "diameter",
    "boundary_condition",
    "tailwater_elevation",
    "x",
    "y"
  ];
  const nodesTemplate = [
    nodeHeader.join(","),
    [
      "N-1",
      "inlet",
      "130",
      "134",
      "grate",
      "on-grade",
      "0.5",
      "0.1",
      "2",
      "1.5",
      "perpendicular",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      ""
    ].join(","),
    [
      "N-2",
      "junction",
      "128",
      "132",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "36",
      "",
      "",
      "",
      ""
    ].join(","),
    [
      "N-3",
      "outfall",
      "120",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "",
      "normal-depth",
      "121",
      "",
      ""
    ].join(",")
  ].join("\n");

  const conduitHeader = [
    "id",
    "type",
    "from_node",
    "to_node",
    "length",
    "diameter",
    "manning_n",
    "upstream_invert",
    "downstream_invert",
    "entrance_loss",
    "exit_loss",
    "bend_loss",
    "slope",
    "cross_slope",
    "long_slope",
    "material"
  ];
  const conduitsTemplate = [
    conduitHeader.join(","),
    [
      "C-1",
      "pipe",
      "N-1",
      "N-2",
      "240",
      "18",
      "0.013",
      "130",
      "120.4",
      "0.5",
      "1.0",
      "0.0",
      "",
      "",
      "",
      "RCP"
    ].join(","),
    [
      "G-1",
      "gutter",
      "N-2",
      "N-3",
      "200",
      "",
      "0.016",
      "",
      "",
      "",
      "",
      "",
      "",
      "0.03",
      "0.005",
      ""
    ].join(",")
  ].join("\n");

  const areasTemplate = [
    ["id", "area", "runoff_coef", "time_of_conc", "outlet_node", "land_use"].join(","),
    ["DA-1", "1.2", "0.85", "10", "N-1", "Residential"].join(",")
  ].join("\n");

  downloadCsv("nodes_template.csv", nodesTemplate);
  downloadCsv("conduits_template.csv", conduitsTemplate);
  downloadCsv("areas_template.csv", areasTemplate);
  updateCsvStatus("Downloaded CSV templates.");
});

runButton.addEventListener("click", async () => {
  statusEl.textContent = "Running solver…";
  outputEl.textContent = "Working…";

  try {
    const parsed = JSON.parse(requestEl.value);
    parsed.intensity = Number(intensityInput.value || parsed.intensity || 0);
    parsed.unit_system = unitSelect.value || parsed.unit_system;
    parsed.use_inlet_interception = interceptionToggle.checked;
    lastRequest = parsed;
    const response = await callSolver(JSON.stringify(parsed));
    lastResult = JSON.parse(response);
    lastResultRaw = JSON.stringify(lastResult, null, 2);
    outputEl.textContent = lastResultRaw;
    statusEl.textContent = "Done";
    updateExportStatus("Results ready for export.");
    renderResultsTables();
    renderPlanView();
    renderProfileView();
  } catch (error) {
    statusEl.textContent = "Error";
    outputEl.textContent = String(error);
    lastResult = null;
    lastResultRaw = "";
    updateExportStatus("No results yet.", false);
    renderResultsTables();
    renderPlanView();
    renderProfileView();
  }
});

copyButton.addEventListener("click", async () => {
  try {
    await navigator.clipboard.writeText(outputEl.textContent);
    statusEl.textContent = "Copied output";
  } catch (error) {
    statusEl.textContent = "Copy failed";
  }
});

worker.addEventListener("error", () => {
  statusEl.textContent = "Worker error";
});

statusEl.textContent = "WASM ready";
updateCsvStatus(summarizeCsvState());
renderCsvPreview();
updateExportStatus("No results yet.");
validateJsonInput();

requestEl.addEventListener("input", () => {
  lastRequest = null;
  scheduleValidation();
});
renderResultsTables();
renderPlanView();
renderProfileView();

if (nodeFilterInput) {
  nodeFilterInput.addEventListener("input", () => {
    tableState.nodes.filter = nodeFilterInput.value || "";
    renderResultsTables();
  });
}

if (conduitFilterInput) {
  conduitFilterInput.addEventListener("input", () => {
    tableState.conduits.filter = conduitFilterInput.value || "";
    renderResultsTables();
  });
}

if (planMetricSelect) {
  planMetricSelect.addEventListener("change", () => {
    renderPlanView();
  });
}

if (planLabelsToggle && networkPlan) {
  planLabelsToggle.addEventListener("change", () => {
    networkPlan.classList.toggle("labels-hidden", !planLabelsToggle.checked);
  });
  networkPlan.classList.toggle("labels-hidden", !planLabelsToggle.checked);
}

if (networkPlan) {
  networkPlan.addEventListener("wheel", (event) => {
    event.preventDefault();
    const direction = event.deltaY < 0 ? 1.1 : 0.9;
    const targetScale = getPlanScale() * direction;
    setPlanScale(targetScale, { x: event.clientX, y: event.clientY });
  });
  networkPlan.addEventListener("mousedown", startPan);
  networkPlan.addEventListener("mousemove", movePan);
  networkPlan.addEventListener("mouseleave", endPan);
  window.addEventListener("mouseup", endPan);
}

if (planZoomInButton) {
  planZoomInButton.addEventListener("click", () => {
    if (planWrap) {
      const rect = planWrap.getBoundingClientRect();
      setPlanScale(getPlanScale() * 1.2, {
        x: rect.left + rect.width / 2,
        y: rect.top + rect.height / 2
      });
    } else {
      setPlanScale(getPlanScale() * 1.2, null);
    }
  });
}

if (planZoomOutButton) {
  planZoomOutButton.addEventListener("click", () => {
    if (planWrap) {
      const rect = planWrap.getBoundingClientRect();
      setPlanScale(getPlanScale() / 1.2, {
        x: rect.left + rect.width / 2,
        y: rect.top + rect.height / 2
      });
    } else {
      setPlanScale(getPlanScale() / 1.2, null);
    }
  });
}

if (planResetButton) {
  planResetButton.addEventListener("click", () => {
    resetPlanView();
  });
}

if (profileBuildButton) {
  profileBuildButton.addEventListener("click", () => {
    renderProfileView();
  });
}

if (profileInvertToggle) {
  profileInvertToggle.addEventListener("change", () => {
    renderProfileView();
  });
}
