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

function downloadCsv(filename, content) {
  const blob = new Blob([content], { type: "text/csv" });
  const link = document.createElement("a");
  link.href = URL.createObjectURL(blob);
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
  URL.revokeObjectURL(link.href);
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

  let errorRowCount = 0;
  records.forEach((record) => {
    const issues = schema.validateRow(record, mapping);
    if (issues.length) {
      errorRowCount += 1;
    }
  });

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
  records.slice(0, previewLimit).forEach((record) => {
    const row = document.createElement("tr");
    const rowIssues = schema.validateRow(record, mapping);
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
});

loadSampleButton.addEventListener("click", () => {
  requestEl.value = JSON.stringify(sampleRequest, null, 2);
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

buildFromCsvButton.addEventListener("click", () => {
  try {
    const request = buildRequestFromCsv();
    requestEl.value = JSON.stringify(request, null, 2);
    updateCsvStatus("Request JSON built from CSV files.");
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
    outputEl.textContent = JSON.stringify(JSON.parse(response), null, 2);
    statusEl.textContent = "Done";
  } catch (error) {
    statusEl.textContent = "Error";
    outputEl.textContent = String(error);
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
