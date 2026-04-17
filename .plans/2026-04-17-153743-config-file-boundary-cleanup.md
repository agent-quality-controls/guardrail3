Goal

- Remove the remaining non-guardrail config slicing and mixed raw-boundary handoffs.
- Family packages should receive typed file state, not ingestion summaries or raw `toml::Value`.
- Parser packages should own file-specific inspection logic when rules need more than the main typed model exposes.

Approach

- Fix `.cargo/config.toml -> clippy` first.
- Replace the current `CLIPPY_CONF_DIR` override summary with a typed cargo-config state built from `cargo-config-toml-parser`.
- Update clippy ingestion, types, config checks, and tests to consume that typed state directly.
- Clean `clippy.toml -> clippy` next.
- Add parser-owned document state in `clippy-toml-parser` for the file facts the clippy rules currently reconstruct from raw TOML:
  - top-level keys
  - ban-section entries and malformed-entry facts
  - direct field-state helpers needed by current rules
- Pass that typed document through clippy ingestion and remove the raw TOML boundary from `g3rs-clippy-types`.
- Clean `Cargo.toml -> cargo` last.
- Add parser-owned document/state helpers in `cargo-toml-parser` for the file facts cargo rules currently reconstruct from raw TOML:
  - workspace/member edition state
  - rust-version state
  - `[lints] workspace` state
  - explicit allow inventory and lint-level inspection
- Pass typed cargo document state through cargo ingestion and remove `raw_cargo` from `g3rs-cargo-types`.

Key decisions

- Do not add more slicing in ingestion.
- File-specific inspection belongs in the parser package for that file, not in the family ingestion layer.
- Do not keep backward-compat aliases or dual old/new inputs.
- Do not touch `guardrail3-rs.toml` in this slice.
- Keep the change staged as one isolated architectural cleanup slice even though the worktree is otherwise dirty.

Alternatives considered

- Leave raw `toml::Value` in family boundaries and call it "typed enough".
  - Rejected because it keeps document-shape knowledge in family support code and preserves the mixed boundary.
- Add more ingestion summaries for specific rules.
  - Rejected because that makes ingestion own family semantics and grows the spaghetti the user called out.

Files to modify

- `packages/rs/clippy/g3rs-clippy-types/src/types.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_20_forbid_clippy_conf_dir_override.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/test_support/src/input.rs`
- `packages/parsers/cargo-config-toml-parser/...`
- `packages/parsers/clippy-toml-parser/...`
- `packages/rs/cargo/g3rs-cargo-types/src/types.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/support.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/test_support/src/input.rs`
- `packages/parsers/cargo-toml-parser/...`
