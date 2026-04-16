Goal

Make `packages/rs/g3rs-workspace-crawl` validate clean while keeping `crates/types` passive.

Approach

- Prove the query API can live outside `crates/types`.
  - Add runtime-level tests for `entry`, `root_file`, and `files_with_extension`.
  - Keep the tests focused on behavior, not method syntax.
- Move query behavior out of `crates/types/src/crawl.rs`.
  - Remove the inherent `impl G3RsWorkspaceCrawl`.
  - Add runtime free functions that query `&G3RsWorkspaceCrawl`.
  - Re-export those functions from the runtime crate and the root facade.
- Update this package to the current package shape.
  - Make assertions depend on runtime instead of types.
  - Replace local query-method calls with runtime free-function calls.
  - Add an owned shared assertions file for crawl behavior if needed.
  - Add missing root policy files and explicit publish settings.
- Update repo call sites that currently use method syntax on `G3RsWorkspaceCrawl`.
  - Replace `.entry(...)`, `.root_file(...)`, and `.files_with_extension(...)` with runtime free functions.
  - Keep the change mechanical and local.
- Verify with workspace tests and full package validation.

Key decisions

- Keep `G3RsWorkspaceCrawl` as passive data.
  - Why: `crates/types` should be contract only.
  - Rejected: keeping read-only query methods in shared types just because they are convenient.
- Use free functions in runtime, not a new query wrapper type.
  - Why: this is the smallest change that moves behavior out of types without inventing new structure.
  - Rejected: adding a separate query object or trait layer.
- Keep repo-wide call-site updates mechanical.
  - Why: the architectural change is in `g3rs-workspace-crawl`; callers should only switch syntax.

Files to modify

- `packages/rs/g3rs-workspace-crawl/Cargo.toml`
- `packages/rs/g3rs-workspace-crawl/src/lib.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/lib.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/run.rs`
- `packages/rs/g3rs-workspace-crawl/crates/types/src/crawl.rs`
- `packages/rs/g3rs-workspace-crawl/crates/assertions/src/lib.rs`
- `packages/rs/g3rs-workspace-crawl/crates/assertions/src/common.rs`
- `packages/rs/g3rs-workspace-crawl/crates/assertions/src/workspace_queries.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/*.rs`
- repo call sites that use the old method syntax
