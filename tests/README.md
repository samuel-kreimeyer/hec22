# HEC-22 Test Suite

Last updated: 2026-02-27

This directory contains integration and verification tests for hydraulics, routing,
inlet behavior, schema compatibility, and selected HEC-22 worked examples.

## Current Health

- `cargo test --all-targets -q` passes.
- Key Chapter 7 and Chapter 9 regression coverage is present in dedicated test files.

## Test Files

- `chapter5_verification.rs`: Chapter 5 gutter-flow verification cases
- `hec22_chapter5_examples.rs`: additional Chapter 5 worked-example checks
- `hec22_example_7_2_test.rs`: Chapter 7 curb-opening example validation
- `grate_sizing_test.rs`: sag grate sizing and design workflow checks
- `inlet_bypass_test.rs`: inlet interception and bypass behavior
- `hec22_table_9_cases.rs`: Chapter 9 Table 9.6/9.7 regression cases
- `outfall_egl_test.rs`: outfall EGL/HGL behavior
- `network_integration_test.rs`: end-to-end routing and solver integration
- `tributary_flow_test.rs`: tributary flow propagation checks
- `multi_level_tributary_flow_test.rs`: multi-level tributary routing checks
- `json_schema_tests.rs`: schema compatibility and JSON model checks

## Running Tests

```bash
# Full test suite (recommended)
cargo test --all-targets

# Integration tests only
cargo test --tests

# Run one file
cargo test --test hec22_table_9_cases

# Run with printed output
cargo test --test hec22_example_7_2_test -- --nocapture
```

## Guidance

- Add/adjust tests whenever equation behavior changes.
- Prefer explicit expected-value assertions for HEC-22 regression tests.
- For new hydraulic features, include at least:
  - one nominal case
  - one edge/boundary case
  - one regression case tied to a known issue or equation reference
