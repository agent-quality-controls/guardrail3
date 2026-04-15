Summary

Cleaned `packages/rs/fmt/g3rs-fmt-filetree-checks` so the package now validates with no findings under the active rules. The main work was deleting the useless local `types` wrapper, moving test helpers into a sibling `test_support` crate, and reshaping runtime and assertions to the nested `mod.rs` plus `rule.rs` pattern.

Decisions made

- Reused the cleaned `g3rs-clippy-filetree-checks` shape instead of inventing a fmt-only filetree pattern.
- Deleted the local `crates/types` wrapper because it only re-exported `g3rs-fmt-types` and created unnecessary arch, release, and apparch noise.
- Moved the old runtime-local `test_support.rs` helper into a sibling `crates/test_support` crate so sidecar tests stop reaching through sibling local modules.
- Moved flat assertions files into nested `mod.rs` plus `rule.rs` directories and added a shared `run` assertions surface for the combined filetree scenario.
- Marked the workspace and child crates unpublished with explicit `publish = false`, so release checks stop treating this package as a publish unit.

Key files for context

- `.plans/2026-04-15-185708-fmt-filetree-checks-package-cleanup.md`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/Cargo.toml`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/guardrail3-rs.toml`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/run/rule.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/run/rule_tests/cases.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/assertions/src/run/rule.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/test_support/src/input.rs`

Next steps

- Continue package-by-package in the fmt family.
- The next likely package is `packages/rs/fmt/g3rs-fmt-ingestion`.
- Keep the same loop:
  - run full validation
  - fix clear package debt
  - stop only on a real rule contradiction
