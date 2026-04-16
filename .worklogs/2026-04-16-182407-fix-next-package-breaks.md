Summary

Fixed the next three concrete breakages after the private-doc cleanup. `g3rs-code-ingestion` now imports the live code type surface, `g3rs-clippy-ingestion` now uses the exported filetree run assertions path, and the real-workspace toolchain sweep now accepts both `stable` and pinned stable channels, which matches the current toolchain rule.

Decisions made

- Switched `g3rs-code-ingestion` runtime imports to `g3rs-code-types`.
  - Why: the old imports still pointed at the local ingestion types crate even though the actual shared structs now live in `g3rs-code-types`.
  - Rejected: re-exporting those shared structs through `g3rs-code-ingestion-types` just to keep stale imports working.
- Routed `g3rs-clippy-ingestion` through `g3rs-clippy-filetree-checks-assertions::run::rule::assert_same_root_conflict`.
  - Why: the filetree assertions crate exports the combined run proof there, not raw top-level `assert_findings` helpers on those rule modules.
  - Rejected: patching the filetree assertions crate just to support the stale caller path.
- Relaxed the real-workspace toolchain sweep from `stable` only to `stable or pinned stable`.
  - Why: the live toolchain rule already treats both as valid, and real package fixtures now include pinned stable channels like `1.85.0`.
  - Rejected: forcing the fixtures back to `stable` or keeping a stale test that contradicts the rule.

Key files for context

- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/assertions/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/assertions/src/run/rule.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/ingest_tests/real_workspaces.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule.rs`

Next steps

- Clean `packages/rs/toolchain/g3rs-toolchain-ingestion`; it still has ordinary package debt in filetree, test, release, and deps.
- Continue the package-by-package pass after toolchain ingestion.
- Stop only on the next real contradictory rule.
