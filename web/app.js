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
  card.appendChild(table);

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
  validateJsonInput();
});

loadSampleButton.addEventListener("click", () => {
  requestEl.value = JSON.stringify(sampleRequest, null, 2);
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
    const response = await callSolver(JSON.stringify(parsed));
    lastResult = JSON.parse(response);
    lastResultRaw = JSON.stringify(lastResult, null, 2);
    outputEl.textContent = lastResultRaw;
    statusEl.textContent = "Done";
    updateExportStatus("Results ready for export.");
  } catch (error) {
    statusEl.textContent = "Error";
    outputEl.textContent = String(error);
    lastResult = null;
    lastResultRaw = "";
    updateExportStatus("No results yet.", false);
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

requestEl.addEventListener("input", scheduleValidation);
