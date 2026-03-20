# Arch Helpers Unification + TS-ARCH-01 Rule 01 Tests

## Summary
Extracted shared hex arch utilities into `arch_helpers.rs`, migrated both RS and TS checks to use them, and built comprehensive TS-ARCH-01 rule_01 tests (38 tests, 3 adversarial rounds converged).

## Decisions

### Shared arch_helpers.rs
Extracted 6 language-agnostic functions: `list_dir_names`, `list_file_names`, `has_gitkeep`, `check_loose_files`, `check_exact_subdirs`, `check_container_not_empty`. Parameterized by `id` ("R-ARCH-01"/"T-ARCH-01") and `entity` ("Service"/"TS app"). RS helpers.rs became thin re-exports. Rejected alternative of keeping duplicate implementations — same logic, different bugs (TS had std::fs::read_dir bypass).

### Double-fire fix
`check_container_not_empty` now handles loose files internally. When container is empty with files: reports "empty container" (with file listing in message), does NOT also fire "loose files". When container has subdirs: calls `check_loose_files` for stray files alongside valid crates. One error per problem, not two.

### check_12 src/ ban — kept list_dir behavior
Plan proposed switching to metadata() to catch empty src/ dirs. User decided: file named `src` should NOT trigger the ban (it's not a directory of code). Final version uses `list_dir` + `is_dir()` fallback — catches empty dirs but not files.

### TS rule_01 test rigor
38 tests after 3 adversarial rounds (4 agents each). Every test uses exact `assert_eq!` counts. `assert_warn_for_app` helper validates title content + file field path. Symlink tests `#[cfg(unix)]`-gated at function level. Coverage: golden baseline, missing modules, modules-as-file/empty/.gitkeep, symlinks (broken/valid/dev-null), discovery edge cases (ts-only/tsx-only/deep nesting/node_modules excluded/.next excluded/dual-stack/package.json variants), unicode/spaces/casing/typos, permissions, isolation, idempotency.

## Key files
- `crates/app/arch_helpers.rs` — NEW shared module
- `crates/app/rs/validate/arch/rs_arch_01/helpers.rs` — thin re-exports
- `crates/app/rs/validate/arch/rs_arch_01/check_02..05` — use arch_helpers
- `crates/app/ts/validate/ts_arch_checks.rs` — uses arch_helpers, deleted list_ts_dir_names + check_ts_loose_files
- `tests/unit/ts_arch_01/rule_01.rs` — 38 tests, 3 adversarial rounds

## Next steps
- TS-ARCH-01 rules 02-07 tests (same adversarial pattern)
- Fix pre-existing RS-ARCH-01 failures (rules 07-12)
- `has_ts_source_files` still uses WalkDir directly — fix when testing rule_06 leaf validation
