Summary
- Cleaned another batch of packages that still used old test sidecar ownership.
- All fixes were package-local: move `run_tests` off `lib.rs`, move `rule_tests` off facade `mod.rs`, and fix helper imports after the move.

Decisions made
- Did not touch rules in this batch because every failure was satisfiable by reshaping the package.
- Kept `run.rs` as the owner of runtime entry tests in ingestion packages.
- Kept `rule.rs` as the owner of nested sidecar tests in nested rule packages.
- Fixed one stale workspace-crawl test call and one stale `#[path]` fixture so those packages stayed green after earlier rule changes.

Key files for context
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/runtime/src/run/rule.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-config-checks/crates/runtime/src/rs_deps_config_01_dependencies_allowlisted/rule.rs`
- `packages/rs/deps/g3rs-deps-ingestion/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_01_settings/rule.rs`

Next steps
- Commit this verified batch.
- Continue scanning package-by-package until the next real contradictory rule appears.
