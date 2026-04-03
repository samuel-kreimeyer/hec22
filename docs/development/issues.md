# Development Issues

Last updated: 2026-02-27

This file tracks current, actionable engineering issues. It intentionally excludes
stale static-analysis output dumps.

## Current Status

- `cargo test --all-targets -q`: passing
- `cargo check --all-targets -q`: passing
- `cargo clippy --all-targets -q`: passing (no current warnings)

## Known Issues

### 1) Production Readiness Gaps (Roadmap)

These are planned but not implemented yet:

- Design automation (pipe sizing, inlet spacing, cost optimization)
- Advanced Chapter 10-12 engines (detention/retention, BMPs, pump stations)
- Interoperability converters (SWMM/Civil3D/GeoJSON, etc.)
- Packaging and CI/CD automation

Reference: `README.md` Phase 3-7 roadmap sections.

### 2) Documentation Drift Risk

Some development docs can become stale after implementation changes. The current
policy is:

- Prefer curated issue summaries over pasted tool output
- Include command + date when reporting build/test health
- Update `tests/README.md` when test coverage changes materially

### 3) Lint Hygiene Baseline

Current baseline is warning-free under `cargo clippy --all-targets -q`.
Maintain this by keeping low-risk style cleanups in normal feature work.

## Notes

- Historical static analysis snapshots should live in dedicated artifacts, not this
  issue tracker file.
- If a critical solver discrepancy is found, document it here with:
  - affected equation/section
  - repro command
  - expected vs actual behavior
  - proposed fix scope
