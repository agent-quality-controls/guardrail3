Summary

Cleaned `packages/rs/g3rs-workspace-crawl` until workspace tests and full `validate` returned clean. The shared crawl snapshot is now passive data, the query behavior lives in runtime free functions, and the repo callers that used the old method syntax were moved onto the new runtime API.

Decisions made

- Moved `entry`, `root_file`, and `files_with_extension` out of `crates/types`.
  - Why: `types` should stay contract-only, and these are shared query helpers, not data.
  - Rejected: keeping read-only methods in `types` just because many packages already used them.
- Kept the replacement API as free functions, not a new wrapper type.
  - Why: this is the smallest architectural fix that moves behavior out of `types` without inventing new layers.
  - Rejected: adding a query object or trait surface.
- Switched runtime users from `features = ["types"]` to `features = ["api"]` only where they actually use the query behavior.
  - Why: those packages now need the runtime side of `g3rs-workspace-crawl`, not just the passive types.
  - Rejected: reintroducing behavior on the shared data type to avoid manifest changes.
- Moved all final crawl proof into `crates/assertions/src/crawl.rs`.
  - Why: runtime sidecars must not prove behavior by calling local runtime helpers directly.
  - Rejected: leaving `crate::entry(...)` calls inside sidecar tests.
- Marked the whole workspace unpublished.
  - Why: this package is internal repo infrastructure and does not need release burden.
  - Rejected: making the workspace carry release files and publishable child crates.

Key files for context

- `packages/rs/g3rs-workspace-crawl/Cargo.toml`
- `packages/rs/g3rs-workspace-crawl/guardrail3-rs.toml`
- `packages/rs/g3rs-workspace-crawl/src/lib.rs`
- `packages/rs/g3rs-workspace-crawl/crates/types/src/crawl.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/query.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/lib.rs`
- `packages/rs/g3rs-workspace-crawl/crates/assertions/src/crawl.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/Cargo.toml`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/roots.rs`

Next steps

- Commit this workspace-crawl slice as a standalone cleanup.
- Continue the package-by-package pass from the next failing package.
- The first broader compile failure after this slice is in `packages/rs/clippy/g3rs-clippy-ingestion`, inside `crates/assertions/src/run.rs`; it looks unrelated to workspace-crawl and should be handled as the next package debt item unless a real rule contradiction appears first.
