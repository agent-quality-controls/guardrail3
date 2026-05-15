# Goal

Add fixture3 coverage that compares serialized code-family ingestion data, not only `g3rs validate` text output.

# Approach

- Add a small Rust verifier binary under `packages/rs/code/g3rs-code-ingestion/crates/fixture-output`.
- The binary accepts `--path <workspace-root>`.
- The binary crawls the workspace with `g3-workspace-crawl`.
- The binary calls the real `g3rs-code-ingestion` entry points:
  - `ingest_for_config_checks`
  - `ingest_for_source_checks`
  - `ingest_for_file_tree_checks`
- The binary prints JSON with `serde_json`.
- The JSON contains only:
  - a schema version string
  - the three serialized ingestion outputs as `Result<actual-owned-type, actual-owned-error-type>`
- Add `serde::Serialize` to `G3RsCodeIngestionError` so ingestion failures are serialized as owned Rust data.
- Add a small Python fixture3 script that only:
  - copies fixture repos like the existing replay harness
  - runs the Rust verifier binary
  - parses the Rust verifier JSON without remapping family fields
  - emits a fixture3 envelope with fixture id/hash/exit code/stdout/stderr/payload
- Add a fixture3 suite for serialized code ingestion output.
- Approve the initial output after reviewing that it is produced from Rust JSON, not Python family-specific remapping.

# Files To Modify

- `packages/rs/code/g3rs-code-ingestion/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/guardrail3-rs.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/types/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/error.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/fixture-output/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/fixture-output/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/fixture-output/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/fixture-output/src/main.rs`
- `scripts/behavior/fixture3-g3rs-code-ingestion.py`
- `fixture3.yaml`
- `behavior/golden/g3rs-code-ingestion/approved.normalized.json`
- `behavior/golden/g3rs-code-ingestion/approved.meta.json`
- affected `Cargo.lock` files

# Non-Goals

- Do not add adapters, exporters, ingestion suites, replay suites, replay record maps, or duplicated fixture-only copies of owned family structs.
- A small command-output envelope is allowed because fixture3 needs one stable JSON document, but the family data inside it must be the real ingestion return values.
- Do not parse Rust, TOML, or guardrail family data in Python.
- Do not hand-select fields from the ingestion structs.
- Do not add this as a public `g3rs` user command.

# Verification

- `cargo check --workspace --all-targets --all-features` in `packages/rs/code/g3rs-code-ingestion`.
- `cargo test --workspace --all-targets --all-features` in `packages/rs/code/g3rs-code-ingestion`.
- `g3rs validate --path packages/rs/code/g3rs-code-ingestion`.
- `fixture3 check --suite g3rs-code-ingestion`.
- `scripts/behavior/verify-all.sh`.
