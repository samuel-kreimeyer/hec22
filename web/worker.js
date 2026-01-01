import init, { solve_from_json } from "../wasm/web/pkg/hec22_wasm.js";

const ready = init();

self.onmessage = async (event) => {
  const { id, requestJson } = event.data;
  try {
    await ready;
    const response = solve_from_json(requestJson);
    self.postMessage({ id, ok: true, response });
  } catch (error) {
    self.postMessage({ id, ok: false, error: String(error) });
  }
};
