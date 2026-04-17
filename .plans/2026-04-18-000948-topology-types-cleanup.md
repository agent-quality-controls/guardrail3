Goal
- Make `packages/rs/topology/g3rs-topology-types` validate clean under all active families using the current single-crate shared-types package shape.

Approach
- Normalize the root manifest into the standard internal single-crate workspace form with shared lints, explicit non-publish intent, docs metadata, and feature-gated API exports.
- Add the missing root policy files and `guardrail3-rs.toml` so topology, cargo, release, apparch, fmt, clippy, deny, and toolchain families all see the expected package surface.
- Keep the package single-crate and shared; do not introduce runtime/assertions crates because this package is only a shared types surface.

Key Decisions
- Reuse the cleaned `g3rs-test-types` / `guardrail3-check-types` pattern instead of inventing a topology-specific shape.
- Keep `module_name_repetitions` waived narrowly in `guardrail3-rs.toml` because the repeated `G3RsTopology...` naming is the package's public type surface.

Files To Modify
- `packages/rs/topology/g3rs-topology-types/Cargo.toml`
- `packages/rs/topology/g3rs-topology-types/src/lib.rs`
- `packages/rs/topology/g3rs-topology-types/guardrail3-rs.toml`
