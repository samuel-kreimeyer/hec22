# hec22

Shared Rust library for urban drainage network analysis following FHWA HEC-22.

The reusable crate lives in this package. The CLI binaries now live in the sibling app:

- `apps/hec22-cli`

The canonical drainage-network schema extracted from the standalone repo lives in `schemas/drainage/network/`.

The source repository also included wasm and static-web materials. Those are preserved here as source reference while the monorepo keeps the promoted interface surface limited to the CLI.
