Summary

Cleaned the remaining non-guardrail config-file boundary drift in the cargo and clippy families. Cross-family clippy inputs now carry typed `.cargo/config.toml` and typed `Cargo.toml` documents instead of pre-sliced summaries and precomputed booleans, and cargo now uses parser-owned document helpers instead of raw/duplicated manifest facts at the package boundary.

Decisions made

- Moved parser document behavior out of parser `types` crates and into parser `runtime` free functions. Rejected keeping inherent methods on document wrappers because parser `types` must stay passive.
- Replaced clippy's `CLIPPY_CONF_DIR` override summary with typed `CargoConfigToml` state. Rejected preserving the old boolean summary because it kept clippy ingestion responsible for config semantics.
- Replaced clippy's `published_library_policy: bool` with typed cargo root/member manifest states and moved publishability evaluation into clippy config checks. Rejected keeping the precomputed boolean because it hid `Cargo.toml` semantics inside ingestion.
- Reworked cargo package boundaries around `CargoTomlDocument` so cargo checks consume passive parser-owned documents plus runtime document helpers instead of duplicated raw fields on cargo family types.

Key files for context

- `.plans/2026-04-17-153743-config-file-boundary-cleanup.md`
- `packages/parsers/cargo-toml-parser/crates/runtime/src/document.rs`
- `packages/parsers/clippy-toml-parser/crates/runtime/src/document.rs`
- `packages/rs/cargo/g3rs-cargo-types/src/types.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/support.rs`
- `packages/rs/clippy/g3rs-clippy-types/src/types.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs`

Next steps

- Apply the same typed-file handoff rule to `guardrail3-rs.toml` once that family exists instead of adding more family-specific slices.
- Decide whether the remaining parser-package `RS-CODE-SOURCE-04` warnings should stay visible or get exact waiver support.
