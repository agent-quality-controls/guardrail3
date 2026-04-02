# Fix all test compilation errors across guardrail3 workspace

## Summary
Fixed ~231 compilation errors across 9+ crates caused by the FamilyMapper/structure/legality API changes. All test binaries now compile (`cargo test --no-run` succeeds).

## Decisions

### FamilyView::from_tree removal
- Replaced `FamilyView::from_tree(tree)` with `FamilyView::build(root, structure, content, &["".to_owned()], &[], &[], None)` for ProjectTree sources
- Where tree was already a FamilyView, simply passed it through (from_tree on a FamilyView was a no-op clone)

### FamilyMapper::new -> from_legality pipeline
- Test helpers that previously called `FamilyMapper::new(tree, &scope, config, &selected, scoped_files)` now build the full pipeline: `structure::collect(tree, &[]) -> legality::collect(structure) -> FamilyMapper::from_legality(&legality, config, &selected, scoped_files)`
- For FamilyView-based test helpers: construct a temporary ProjectTree from FamilyView data to feed the pipeline
- Added `root_path()` accessor to FamilyView for this purpose

### Module restructuring
- Rule mod.rs files: renamed `mod rs_XXX_tests;` to `mod tests;` (tests directory already existed)
- Category mod.rs files: removed stale `#[cfg(test)] mod rs_XXX_tests;` declarations (tests are declared in rule's own mod.rs)
- hooks-shared test golden.rs: fixed imports from `crate::hook_shared_XX::` to `super::` and added re-exports in tests/mod.rs
- Changed `pub(super) fn` to `pub(crate) fn` on test helpers to fix visibility through test child modules

### Dependency changes
- Added `guardrail3_app_rs_legality` and `guardrail3_domain_project_tree` as dev-dependencies to 13 family runtime crates and family_mapper
- Fixed cargo/test_support to return real ProjectTree (was aliased FamilyView)
- Kept deps/test_support returning FamilyView (callers expect it)
- eslint_plugin_checks_tests.rs moved to proper submodule directory

## Key files
- `crates/app/rs/family_view/src/lib.rs` - added `root_path()` accessor
- `crates/app/rs/families/*/crates/runtime/Cargo.toml` - added legality + project-tree dev-deps
- `crates/app/rs/families/cargo/test_support/src/lib.rs` - returns real ProjectTree now
- `crates/app/rs/families/cargo/crates/assertions_common/src/lib.rs` - reference implementation of new pattern

## Next steps
- Run `cargo test` to verify tests pass at runtime (not just compilation)
- The `FamilyView as ProjectTree` alias pattern is widespread - consider a larger cleanup to use proper type names
