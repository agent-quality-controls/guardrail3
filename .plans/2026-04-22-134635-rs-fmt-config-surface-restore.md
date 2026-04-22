Goal

Restore `rs/fmt` to the intended config-family architecture:
- ingestion selects and parses the relevant config files
- `g3rs-fmt-types` carries the parsed config surfaces intact
- `g3rs-fmt-config-checks` interprets those parsed config documents
- ingestion does not slice parsed config into rule-shaped facts

Approach

1. Re-read the pre-`51705f423` `rs/fmt` shape and restore the config boundary in:
   - `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
   - `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
   - `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/inputs.rs`
   - `packages/rs/fmt/g3rs-fmt-config-checks/crates/test_support/src/input.rs`
2. Keep the proving `run.rs` sidecar test infrastructure added in the last commit, but replace any assumption that ingestion must pre-slice config facts.
3. Update config rules back to parser-surface interpretation if any still depend on the temporary fact structs.
4. Run `rustfmt --check` on touched files, `cargo test` for the three fmt packages, and `g3rs validate` for the touched fmt slice.
5. Write a worklog and commit this correction as a standalone architecture fix.

Key decisions

- Revert only the config-surface drift introduced by `51705f423`.
  - Why: the mistake was specific to `rs/fmt` config handling, not the whole fmt family or other Rust seam repairs.
- Preserve the red-proof discipline, but do not force a fake "minimal config input" model.
  - Why: config-family tests should prove the right boundary, not the wrong one more rigorously.

Alternatives considered

- Keep the ingestion-owned fact structs and treat them as the new contract.
  - Rejected: that directly contradicts the intended config-family architecture for this repo.
- Revert the entire previous commit wholesale.
  - Rejected: the commit also added useful run-sidecar test structure that can remain once the config surface is restored.

Files to modify

- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-types/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/inputs.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_*.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/test_support/src/input.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/test_support/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/types/src/lib.rs`
