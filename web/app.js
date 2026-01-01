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
