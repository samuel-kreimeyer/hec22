# AGENTS.md

## Purpose

Shared Rust crate for HEC-22 drainage analysis, hydraulic solving, and schema-compatible data models.

## Invariants

- Keep CLI binaries out of this package.
- Keep the Rust crate name as `hec22`.
- Keep the extracted schema in `schemas/drainage/network/` as the canonical contract.
- Preserve source reference material for wasm/web until those interfaces are intentionally promoted.

## Commands

- Format: `cargo fmt --manifest-path packages/hec22/Cargo.toml`
- Tests: `cargo test --manifest-path packages/hec22/Cargo.toml --tests -q`
