# WASM/Web App Skeleton

Date: 2025-01-21

## Summary
This adds a WASM wrapper crate plus a minimal web UI to run the solver in a browser. The WASM API accepts a JSON request (network + flow inputs), runs routing and the HGL/EGL solver, and returns JSON results.

## Build steps
1) Install the WASM target:
   - `rustup target add wasm32-unknown-unknown`
2) Build the WASM bundle:
   - `wasm-pack build wasm --target web --out-dir web/pkg`
   - If you build without `--out-dir`, the default output is `wasm/web/pkg` and the UI is configured to load from there.
3) Serve the UI (from the repo root):
   - `python3 -m http.server 8000`
   - Open `http://localhost:8000/web/`

## Request JSON (WASM API)
`solve_from_json(request_json)` accepts an object with:
- `network` (required): `Network` JSON.
- `conduit_flows` (optional): map of conduit ID -> flow (cfs/cms).
- `node_inflows` (optional): map of node ID -> flow (cfs/cms).
- `drainage_areas` + `intensity` (optional): used to compute `node_inflows`.
- `unit_system` (optional): `US` or `SI`.
- `use_inlet_interception` (optional): defaults to `true`.
- `design_storm_id` (optional): string label.

If `conduit_flows` is provided, routing is skipped. Otherwise routing uses:
- `node_inflows`, or
- `drainage_areas` + `intensity`.

## Response JSON
- `analysis`: full `Analysis` object with node/conduit results.
- `conduit_flows`: routed flows used by the solver.
- `node_inflows`: returned when computed from drainage areas or provided directly.
- `inlet_results`: only returned when inlet interception was used.

## Notes
- CSV parsing and visualization modules are disabled in the WASM build.
- Timestamp generation is disabled unless the `timestamps` feature is enabled.
