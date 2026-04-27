# Goal
Fix the two remaining decided issues in `packages/rs/clippy/g3rs-clippy-config-checks`: stop tripping `g3rs-code/ast-31-public-struct-named-fields` by making the assertions helper DTO opaque, and stop tripping `g3rs-release/publish-dry-run` by marking the package workspace unpublished.

# Approach
1. Add a focused regression in the assertions crate that exercises the public `Finding` API through the helper constructors and assertion functions, so the package still works after field visibility is tightened.
2. Make `Finding` fields private in `crates/assertions/src/common.rs` and keep construction/comparison routed through the existing helper functions and macro-generated wrappers.
3. Mark the clippy config-checks workspace unpublished.
   - Set `publish = false` on the root package and all three subcrates.
   - Use the existing release-family semantics rather than adding any new release special case.
4. Verify with targeted package tests and CLI validation on `packages/rs/clippy/g3rs-clippy-config-checks`.

# Key Decisions
- Fix `g3rs-code/ast-31-public-struct-named-fields` in the package rather than weakening the rule because this crate already has an explicit helper API and does not need field-bag access.
- Use `publish = false` rather than changing release-family logic because `g3rs-release/accidentally-publishable/18` already define that as the correct unpublished contract.

# Alternatives Considered
- Relax `g3rs-code/ast-31-public-struct-named-fields` for assertion-helper DTOs: rejected because this package already has a better opaque API shape.
- Add a release-family exception for package workspaces: rejected because the family already supports unpublished crates via `publish = false`.

# Files To Modify
- `.plans/2026-04-14-200643-clippy-assertions-opaque-and-unpublished-release.md`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/common.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/src/common_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/assertions/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/types/Cargo.toml`
