# HEC-22 Conformance Review

Date: 2025-12-31

Scope: Compare key implementations to FHWA HEC-22 (4th ed., 2024) guidance in docs/reference/FHWA HEC22 and equations in docs/reference/equations.

## Summary

- Core gutter hydraulics (Chapter 5) and pipe hydraulics (Chapter 9) generally follow HEC-22 with good structure and unit handling.
- Several inlet capture formulas (Chapter 7) deviate from HEC-22, notably grate-on-grade frontal/side efficiency and slotted drains. These likely explain observed inlet capture issues.
- Sag inlet capacity uses reasonable weir/orifice forms but needs minor corrections (head definition and perimeter treatment) for closer fidelity.
- Curb-opening on-grade implements Eq. 7.10 and 7.13, and supports depressed sections via Eq. 7.11. Consider adding FHWA’s alternative equation noted in Chapter 7.

## What Matches Well

- Gutter flow equations and composite section logic
  - Uniform gutter Eq. 5.2/5.4 implementation and spread/depth/velocity reporting are consistent: `src/gutter.rs:32` to `src/gutter.rs:114`.
  - Composite gutter uses Eq. 5.8 and the Eo ratio per Eq. 5.7 with robust bisection: `src/gutter.rs:140`, `src/gutter.rs:186`, `src/gutter.rs:226`.
  - Parabolic crown section implemented with equivalent slope approximation: `src/gutter.rs:468`.

- Pipe hydraulics and losses (Chapter 9)
  - Manning full/partial flow, normal/critical depth routines, and velocity head: `src/hydraulics.rs:23`, `src/hydraulics.rs:95`, `src/hydraulics.rs:157`, `src/hydraulics.rs:289`.
  - Loss models (entrance/exit/bend/transition) and an FHWA access hole method are present: `src/hydraulics.rs:410`, `src/hydraulics.rs:520`, `src/hydraulics.rs:616`.

- Curb-opening on grade
  - Lt and efficiency follow Eq. 7.10 and 7.13; depressed gutter uses Eq. 7.11 for effective slope: `src/inlet.rs:391`, `src/inlet.rs:415`.

## Deviations and Issues

- Grate inlets on grade: frontal vs side flow handling
  - The code conflates Eo (frontal flow fraction) with Rf (frontal flow capture efficiency) and uses a non-HEC-22 piecewise form with very low splash-over thresholds.
    - Where: `src/inlet.rs:126` (deriving “ratio_frontal” from Eo/W/T) and `src/inlet.rs:150` (frontal_efficiency) and `src/inlet.rs:183` (total efficiency composition).
  - HEC-22 specifies: E = Eo·Rf + (1−Eo)·Rs, with
    - Rf using splash-over velocity relation (Eq. 7.5) with Vo = Ku·√(gL) and Ku≈0.09 (US), and
    - Rs using Eq. 7.6, which depends on velocity, length and cross slope. Current `side_efficiency` omits the velocity term and uses only Kx·(L/T)^1.8: `src/inlet.rs:166`.
  - The constants used for “v0” (0.49–1.79 ft/s) are orders of magnitude below HEC-22 examples (e.g., ~8 ft/s for P‑1‑7/8 in Figure 7.8). This will severely underpredict Rf.

- Grate type handling for on-grade capture
  - On-grade capture doesn’t parameterize by grate type to obtain Vo; only bar orientation is used. HEC-22 requires grate-type-dependent Vo or the Eq. 7.5 relation with L.
    - Where: `src/inlet.rs:146` (BarConfiguration-driven v0 only).

- Slotted drains on grade
  - Currently hard-coded to 80% efficiency without equations: `src/solver.rs:1234`.
  - HEC-22 states slotted drains behave like curb openings on grade (length weir), with efficiency vs Lt and allows depression; they should use the curb-opening on-grade framework.

- Sag inlet capacity details
  - Curb opening orifice head uses ponding depth directly; HEC-22 defines head to the opening centroid. Consider using d_c = ponding_depth − h/2 in orifice term.
    - Where: `src/inlet.rs:818`.
  - Grate weir perimeter uses all sides; when adjacent to a curb in a sag configuration, the curb side may not act as a free weir. Confirm configuration and reduce P if appropriate.
    - Where: `src/inlet.rs:704` (perimeter calculation).

- Alternative curb-opening equation
  - Chapter 7 notes Eq. 7.10 underestimates Lt for long curb inlets and references an FHWA alternative approach (FHWA 2022a). Not yet implemented as an option.
    - Where: `docs/reference/FHWA HEC22/chapter-07-inlet-design.md:302`.

## Recommendations

- Correct grate on-grade capture per Eq. 7.3–7.9
  - Compute Eo from gutter result (composite if available) or Eq. 7.3; do not reuse Eo as Rf.
  - Implement Rf from Eq. 7.5 using Vo = Ku·√(g·L). Use Ku=0.09 (US) / 0.295 (SI), g from unit system, L = grate length. Cap Rf to 1.0 when V ≤ Vo.
  - Implement Rs from Eq. 7.6 with its velocity dependence (not just Kx·(L/T)^1.8). Use HEC-22 Ku for Rs and include Sx and L/T terms as specified.
  - Compose total efficiency as E = Eo·Rf + (1−Eo)·Rs. Clamp to [0,1].
  - Add grate type or direct Vo input so different grate geometries map to different splash-over behavior (align with Figure 7.8). Prefer a lookup per grate type with L-scaling.

- Replace fixed 80% for slotted drains
  - Route slotted drains on grade through the curb-opening on-grade path (Eq. 7.10/7.13, with depression support), using slot length as L and slot height as h.
  - For sag slotted drains, use weir/orifice with appropriate coefficients, consistent with curb openings.

- Curb-opening refinements
  - Implement the FHWA alternative to Eq. 7.10 as a selectable method for long inlets; expose via a config flag and document applicability.
  - For sag orifice flow, use head to centroid (d_c) for the orifice term, and ensure clogging and opening ratio are applied consistently.

- Combination inlets (sweeper behavior)
  - Current sequence (grate then curb from bypass) is acceptable for equal-length side-by-side; add an option for “sweeper” configuration where curb opening upstream reduces spread over the grate (apply reduced spread when computing grate Eo).

- Units and constants
  - Centralize and use unit-consistent Ku/coefficients for Eqs. 7.5–7.6 with both US/SI support, similar to `GUTTER_K_US`/`GUTTER_K_SI` in `src/gutter.rs:608`.

## Suggested Implementation Plan

1) Grate on-grade: refactor to E = Eo·Rf + (1−Eo)·Rs with correct Vo and Rs(V). Update tests to replicate Example 7.1 magnitudes.
2) Slotted drains: reuse curb-opening on-grade path and add basic slotted geometry to `node.rs`/CSV.
3) Curb opening: add optional “Eq. 7.10 alternative” path and centroid head for sag.
4) Combination: add sweeper option that reduces T for the grate step.
5) Coefficients: define unit-aware constants for Eq. 7.5/7.6 in a shared module.

## Notable File References

- Gutter: `src/gutter.rs:32`, `src/gutter.rs:140`, `src/gutter.rs:608`
- Grate on-grade: `src/inlet.rs:126`, `src/inlet.rs:150`, `src/inlet.rs:166`, `src/inlet.rs:183`
- Curb on-grade: `src/inlet.rs:351`, `src/inlet.rs:391`, `src/inlet.rs:415`
- Sag capacity: `src/inlet.rs:680`, `src/inlet.rs:704`, `src/inlet.rs:818`
- Slotted drains: `src/solver.rs:1234`
- HEC-22 references: `docs/reference/FHWA HEC22/chapter-07-inlet-design.md:200`, `docs/reference/equations/inlet_design.md:1`

## Closing

Addressing the grate on-grade equations and replacing the fixed slotted drain efficiency should resolve most inlet capture discrepancies. I can implement the grate-on-grade refactor and unit constants next; confirm if you want the FHWA alternative Eq. 7.10 in the same pass.

