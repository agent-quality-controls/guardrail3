Goal

Clean `packages/rs/deny/g3rs-deny-ingestion` without changing rules.

Approach

- Make publish intent explicit across the workspace and mark the package unpublished.
- Add `guardrail3-rs.toml` with the package profile, allowed dependencies, and structural waivers for the multi-module runtime and shared assertions/types crates.
- Re-run workspace tests and validator. If new findings appear, fix package debt only and stop on the next real rule contradiction.

Key decisions

- Keep the local `types` crate because it owns the ingestion error type.
- Do the smallest package-local change first, because the current findings are only release and missing deps policy.

Files to modify

- `packages/rs/deny/g3rs-deny-ingestion/Cargo.toml`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/deny/g3rs-deny-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/deny/g3rs-deny-ingestion/crates/types/Cargo.toml`
- `packages/rs/deny/g3rs-deny-ingestion/guardrail3-rs.toml`
