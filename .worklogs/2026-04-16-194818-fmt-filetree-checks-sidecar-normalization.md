## Summary

Normalized stale test sidecars in `packages/rs/fmt/g3rs-fmt-filetree-checks` so real file modules own their `rule_tests/` sidecars instead of facade `mod.rs` files. Updated the affected sidecar case files to call sibling `check(...)` functions directly.

## Decisions made

- Moved test ownership from facade modules to `rule.rs` files.
  - Why: facade `mod.rs` files should stay wiring-only and should not own logic tests.
  - Rejected: keeping sidecars on `mod.rs`, because that violates the current sidecar ownership rule.
- Kept the exact owned-sidecar `#[path]` shape.
  - Why: file modules need the narrow `#[path = "rule_tests/mod.rs"]` bridge to own sibling sidecar folders without turning into directory modules.
  - Rejected: inline tests or directory-module reshaping, because both degrade the chosen file-module layout.
- Repointed sidecar cases from `super::super::rule::check(...)` to `super::super::check(...)`.
  - Why: once tests belong to `rule.rs`, the sibling function is the direct target.

## Key files for context

- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/rs_fmt_filetree_01_exists/rule.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/rs_fmt_filetree_05_per_crate_override/rule.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/rs_fmt_filetree_08_dual_file_conflict/rule.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/run/rule.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/test_support/src/input.rs`

## Next steps

- Apply the same stale-sidecar cleanup to remaining packages still attaching `run_tests` to `lib.rs` or `rule_tests` to facade `mod.rs` files.
- Stop only if the next failing package exposes a real rule contradiction instead of stale package shape.
