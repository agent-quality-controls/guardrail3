Summary

Repaired the `rs/fmt` config boundary so `g3rs-fmt-config-checks` now consumes ingestion-owned family facts instead of parser-owned TOML documents. Added a proving run test first, then moved fact derivation into ingestion, removed the parser adapter layer from checks, and kept the repo green on the touched slice.

Decisions made

- Replaced parser-leaking parsed states with family-owned facts in `g3rs-fmt-types` because the family boundary must not expose parser documents.
- Kept the repair scoped to the config lane only. File-tree and source lanes were already smaller and were not part of the confirmed defect.
- Removed `inputs.rs` from `g3rs-fmt-config-checks` instead of keeping a translation shim there. That shim was the wrong boundary and would have preserved the same defect under a different name.
- Added a run-sidecar proving test on `run.rs` rather than `lib.rs`. The earlier `lib_tests` placement was wrong for the behavior being proved and violated the package's test-sidecar ownership pattern.
- Formatted and verified the touched `rs/fmt` files directly because workspace-level `cargo fmt --check` for those manifests pulls in unrelated workspace formatting debt outside this repair slice.

Key files for context

- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/test_support/src/input.rs`
- `.plans/2026-04-22-130428-rs-fmt-boundary-repair.md`

Next steps

- Continue the same seam audit on the next remaining Rust family that still exposes parser-owned config documents or bag-heavy config inputs.
- Keep preferring red proving dispatch tests before the next boundary repair so the check package cannot silently keep rebuilding local facts.
