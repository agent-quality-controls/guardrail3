# Summary

Added fixture3 coverage for serialized `g3rs-code-ingestion` outputs. The new suite runs a Rust fixture-output binary over the existing behavior fixtures and approves the resulting JSON.

# Decisions

- Serialized the real ingestion return values from Rust instead of adding Python adapters or duplicate fixture structs.
- Added `serde::Serialize` to `G3RsCodeIngestionError` because fixture output must include lane errors as owned Rust data.
- Added a private fixture-output crate under `g3rs-code-ingestion` rather than a public `g3rs` command because this is behavior harness infrastructure.
- Kept command parsing local and exact for `--path <workspace-root>` because adding `clap` only for one internal argument added unnecessary lockfile and dependency surface.
- Reapproved existing fixture3 suite metadata because adding the new suite changed the `fixture3.yaml` manifest hash while existing approved output stayed matched.

# Key Files

- `packages/rs/code/g3rs-code-ingestion/crates/fixture-output/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/error.rs`
- `scripts/behavior/fixture3-g3rs-code-ingestion.py`
- `fixture3.yaml`
- `behavior/golden/g3rs-code-ingestion/approved.normalized.json`
- `.plans/2026-05-15-172324-code-ingestion-fixture-output.md`
- `.plans/2026-05-15-172324-code-ingestion-fixture-output.md.manifest.toml`

# Verification

- `cargo check --workspace --all-targets --all-features` in `packages/rs/code/g3rs-code-ingestion`
- `cargo test --workspace --all-targets --all-features` in `packages/rs/code/g3rs-code-ingestion`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` in `packages/rs/code/g3rs-code-ingestion`
- `g3rs validate --path packages/rs/code/g3rs-code-ingestion`
- `fixture3 check --all --json`
- `scripts/behavior/verify-all.sh`
- `g3rs validate-repo`

# Next Steps

- Apply the same serialize-first fixture output pattern to the next ingestion family only after identifying the exact owned output structs that need snapshot coverage.
