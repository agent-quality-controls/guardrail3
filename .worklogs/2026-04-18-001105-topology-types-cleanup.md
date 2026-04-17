Summary
- Normalized `g3rs-topology-types` into the standard internal single-crate workspace shape used by the cleaned shared and family `types` packages.
- Added the missing root policy surface and feature-gated the facade exports so the package validates clean under topology, cargo, fmt, clippy, deny, toolchain, release, apparch, and deps.

Decisions Made
- Kept `g3rs-topology-types` as a single shared crate because it is only a cross-family type boundary; introducing runtime/assertions crates would have added artificial structure.
- Reused the standard root workspace shell from the cleaned `types` packages rather than inventing topology-specific metadata.
- Waived `module_name_repetitions` narrowly in `guardrail3-rs.toml` because the repeated `G3RsTopology...` naming is the package's intended public API.

Key Files For Context
- `packages/rs/topology/g3rs-topology-types/Cargo.toml`
- `packages/rs/topology/g3rs-topology-types/src/lib.rs`
- `packages/rs/topology/g3rs-topology-types/src/types.rs`
- `packages/rs/topology/g3rs-topology-types/guardrail3-rs.toml`

Next Steps
- Clean `packages/rs/topology/g3rs-topology-file-tree-checks`.
- Clean `packages/rs/topology/g3rs-topology-ingestion`.
- Run a fresh full-repo validate sweep and confirm only the previously accepted parser warning-only packages remain non-clean.
