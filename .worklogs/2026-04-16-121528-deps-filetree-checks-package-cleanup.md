Summary
- Cleaned `packages/rs/deps/g3rs-deps-filetree-checks` to the current workspace shape and removed the fake local `types` wrapper crate.
- Moved all runtime test proof into the shared assertions crate and reshaped every sidecar `mod.rs` into a pure dispatcher.

Decisions made
- Switched the facade and runtime to depend on `g3rs-deps-types` directly because the local `crates/types` crate was only a reexport layer and added false arch and apparch edges.
- Kept the `x_tests/mod.rs` sidecar shape with `#[path = "..._tests/mod.rs"] mod x_tests;` because plain Rust module resolution for `run.rs` and the rule files does not support sibling `x_tests/` directories without the path redirect.
- Marked the workspace and both child crates unpublished so release checks stand down for this package.

Key files for context
- `packages/rs/deps/g3rs-deps-filetree-checks/Cargo.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/guardrail3-rs.toml`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/runtime/src/rs_deps_filetree_09_cargo_lock_present.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/runtime/src/rs_deps_filetree_10_gitignore_not_ignoring_cargo_lock.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/assertions/src/lib.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/assertions/src/rs_deps_filetree_09_cargo_lock_present.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/assertions/src/rs_deps_filetree_10_gitignore_not_ignoring_cargo_lock.rs`
- `packages/rs/deps/g3rs-deps-filetree-checks/crates/assertions/src/run.rs`

Next steps
- Commit this package cleanup as its own slice.
- Continue to the next `deps` package and stop only if a rule is clearly wrong or contradictory.
