# HGL/EGL Conformance Plan – HEC-22 Chapter 9

Date: 2025-01-21

Purpose: Document the solver changes needed to match FHWA HEC-22 (4th ed., 2024) Chapter 9 guidance for hydraulic grade line (HGL) and energy grade line (EGL) evaluation.

## Scope
- Pipe upstream/downstream grade line calculations (Tables 9.6 and 9.7)
- Junction/access-hole loss application (Step 7)
- Tailwater treatment at outfalls (Step 4)

## Findings (current behavior)
- **Downstream end (Step 5 / Table 9.6):** `solve_pipe` always seeds with `downstream_hgl + V²/2g` and assumes zero downstream velocity; no case logic for submerged, free, or plunging outlets (`src/solver.rs:722-783`).
- **Upstream end (Step 6 / Table 9.7):** Flow condition uses `Froude>1` for supercritical and `flow >= q_full` for surcharge; ignores HGL position vs `BOC+yc`, `BOC+yn`, `TOC` (`src/solver.rs:760-793`).
- **Junction/access-hole losses (Step 7):** Losses are added only to stored EGLs; node HGLs are left unchanged and upstream solves use the pre-loss HGL (`src/solver.rs:317-324,329-356`).
- **Tailwater (Step 4):** Free outfall always uses `(yc + D)/2` above invert and ignores lower/higher receiving stage or the “no influence below invert/critical” guidance (`src/solver.rs:390-417`).
- **Plunging inflow offset:** Access hole plunge height uses the upstream node invert rather than the inflow pipe invert at the junction, overstating `z_k` and plunging losses (Example 9.2 node 41; `src/solver.rs:520-548`).
- **Angled inflow loss:** Access hole angles are hard-coded to 180° for the first inflow and 90° for others, ignoring actual geometry (Example 9.2 structure 42 needs a 90° bend; `src/solver.rs:520-540`).
- **Discharge intensity area:** Access hole discharge intensity uses the flowing area from the HGL solution instead of full outflow pipe area, inflating `DI` and `E_ai` in supercritical partial flow (Example 9.2 structure 41; `src/solver.rs:502-514`).
- **Supercritical fallback:** When provisional HGL suggests surcharge, full-flow losses can yield an upstream HGL below the pipe invert (negative pressure) for supercritical segments, violating the HEC‑22 guidance to stop carrying losses in supercritical flow (`src/solver.rs:880-910`).

## Proposed changes
1) **Implement Table 9.6 downstream cases in `solve_pipe`:**
   - Compute downstream EGL inside the pipe using tailwater/EGLa vs `BOC_o`, `yc`, `yn`, `TOC` with exit loss placement per Cases A–E.
   - Use downstream velocity when estimating exit loss; support plunging (Case E) and steep pipe applicability (Cases A, E).

2) **Implement Table 9.7 upstream classification in `solve_pipe`:**
   - Classify with provisional HGL vs `BOC_i+yc`, `BOC_i+yn`, `TOC_i` (Conditions A–D).
   - Conditions A–C: carry friction and minor losses upstream; Condition D: set `HGL_i = BOC_i + yn` and `EGL_i = HGL_i + V_n²/2g` (no upstream loss carry).
   - Keep Froude for reporting only.

3) **Propagate junction/access-hole losses into HGLs:**
   - After computing access-hole loss, raise both node EGL and HGL by the loss so upstream pipes see the loss-adjusted downstream boundary.

4) **Tailwater handling for free outfalls:**
   - Use the greater of `(yc + D)/2` and any specified receiving-water stage; if tailwater < invert or < critical depth, treat as non-controlling per §9.4 narrative.

5) **Outfall EGLa and exit loss accounting:**
   - Treat the outfall EGLa as the receiving water surface (no velocity head) and compute EGLo via Table 9.6 to avoid double-counting exit loss.

6) **Surcharged (Condition A) flow properties:**
   - When HGL exceeds TOC, compute full-flow velocity head from actual flow (Q/A) rather than full-capacity Manning flow to align with pressurized flow reporting.

7) **Access hole plunging offset:**
   - Use the inflow pipe invert at the junction (downstream end of upstream conduit) when computing `z_k` for plunging classification and losses.

8) **Access hole inflow angles:**
   - Derive inflow angles from node coordinates when available, falling back to the existing 180°/90° heuristic.

9) **Access hole discharge intensity:**
   - Use the full outflow pipe area when computing `DI` for Equations 9.16–9.18.

10) **Supercritical override:**
   - If full-flow HGL falls below the crown in a supercritical segment, treat as Condition D and do not carry losses upstream.

11) **Tests and validation:**
   - Add regression for HEC-22 Example 9.2 including Pipe 41–42 (Condition D, losses not carried upstream).
   - Add a plunging outlet case (Table 9.6 Case E) to verify downstream EGL seeding and exit loss placement.

## Acceptance criteria
- Upstream/downstream computations follow Tables 9.6/9.7 decision logic.
- Junction losses change node HGLs and affect upstream pipes.
- Tailwater selection matches §9.4 guidance for free outfalls.
- Tests reproduce HEC-22 Example 9.2 values within rounding tolerance.
