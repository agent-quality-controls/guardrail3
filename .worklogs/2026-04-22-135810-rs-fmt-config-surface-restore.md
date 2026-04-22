Summary

Reverted the `rs/fmt` config boundary drift introduced by `51705f423`. `fmt` config ingestion once again selects and parses `rustfmt.toml`, `Cargo.toml`, and `rust-toolchain.toml`, while `g3rs-fmt-config-checks` interprets those parsed config documents directly instead of ingestion-owned fact structs.

Decisions made

- Restored parsed config documents as the config-family contract in `g3rs-fmt-types`.
  - Why: `fmt` is a plain config family. The intended boundary is "ingestion parses, config checks interpret", not "ingestion slices rule-shaped facts".
- Restored `inputs.rs` in `g3rs-fmt-config-checks`.
  - Why: config checks need a local interpretation layer over parsed config surfaces. Deleting it forced rule semantics into ingestion.
- Removed the proving `run.rs` sidecar test and assertions helper added in the previous commit.
  - Why: that proof was validating the wrong architecture. Keeping it would preserve the same mistake under stronger test coverage.
- Kept the correction scoped to `rs/fmt`.
  - Why: this was a local architectural reversion, not a reason to rewrite other families blindly before they are reviewed against the correct config-family rule.

Key files for context

- `.plans/2026-04-22-134635-rs-fmt-config-surface-restore.md`
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/inputs.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_01_settings/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_03_nightly_keys_on_stable/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/test_support/src/input.rs`

Next steps

- Review the recent Rust config-family repairs against the corrected rule:
  - config ingestion may select and parse config files
  - config checks should interpret those parsed config surfaces
  - ingestion should not pre-slice rule-shaped config facts
- Do not apply the `rs/fmt` correction mechanically to graph/source/file-tree families. Those need separate review because their owned subjects are not plain config documents.
