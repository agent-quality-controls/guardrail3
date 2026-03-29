# Clean RS-TEST App-Local Assertions And Clippy

**Date:** 2026-03-29 18:46
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/ast`, `apps/guardrail3/crates/app/rs/generate`, `apps/guardrail3/crates/domain/project-tree`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/*parity.rs`

## Summary
Completed the outstanding app-local `RS-TEST` migrations for `ast`, `generate`, and `project-tree` by converting legacy flat/ad hoc sidecars into owned sidecar directories and adding the sibling assertions crates those runtimes now require. Also removed the remaining `RS-TEST-03` parity-sidecar violations in the `clippy` family and fixed a real `ast` runtime bug that the newly compiled unit tests exposed for nested `cfg_attr(..., cfg_attr(..., allow(...)))` detection on foreign modules.

## Context & Problem
After finishing the family-heavy `RS-TEST` sweep, the remaining repo-root backlog still included non-family app crates and four `clippy` parity sidecars. Those crates had been partly migrated structurally but were still missing the companion assertions crates that `RS-TEST-03` expects once owned sidecars exist. `project-tree` also still used the old root-sidecar shape, and the live worktree contained unrelated `release`, `deps`, and lockfile changes that needed to stay out of this commit.

## Decisions Made

### Convert App-Local Crates To The Same Runtime + Assertions Pattern
- **Chose:** Add sibling `assertions` crates for `app/rs/ast`, `app/rs/generate`, and `domain/project-tree`, and rewrite the sidecars into owned `*_tests/mod.rs` or `lib_tests/mod.rs` directories.
- **Why:** The checker already treats this as the minimal correct `RS-TEST-03` shape for internal unit-sidecar ownership. Keeping these crates on a looser pattern would leave a permanent exception lane outside the family system.
- **Alternatives considered:**
  - Keep the old flat `*_tests.rs` files — rejected because `RS-TEST-02`/`03` explicitly forbid that shape.
  - Add `test_support` too — rejected because these crates only needed proof-bearing assertions, not reusable fixture infrastructure.

### Make The New Assertions Crates Real Dependencies
- **Chose:** Use actual helper functions from the new assertions crates in `generate` and `project-tree` instead of silencing `unused-crate-dependencies` with dummy imports.
- **Why:** The runtime crates deny unused crate dependencies, and the new assertions crates should represent real proof ownership rather than a staged-but-dead dependency.
- **Alternatives considered:**
  - Add `use ... as _;` in crate roots — rejected because it would satisfy Cargo while leaving the assertions crates architecturally fake.
  - Drop the assertions crates — rejected because that would re-open the `RS-TEST-03` gap we were fixing.

### Fix The Nested `cfg_attr` Detection Bug In Production
- **Chose:** Extend `item_attrs()` in `app/rs/ast` to expose `syn::Item::ForeignMod` attributes and keep the nested `cfg_attr` test alive.
- **Why:** Once the `ast` crate actually compiled its unit tests again, `nested_cfg_attr_allow_found` exposed that foreign-module attrs were silently ignored. This was a real production bug, not just test glue fallout.
- **Alternatives considered:**
  - Weaken or delete the test — rejected because the failure was valid and would have hidden a genuine parsing gap.
  - Add a special-case test helper without fixing production — rejected because the returned result set itself was wrong.

### Inline Clippy Parity Baselines Instead Of Importing Domain Modules
- **Chose:** Replace the remaining direct `guardrail3_domain_modules::clippy::*` parity imports with local baseline literals in the four `clippy` parity sidecars.
- **Why:** `RS-TEST-03` forbids sidecars from reaching into local crates directly, and parity baselines are small enough to keep local without losing intent.
- **Alternatives considered:**
  - Allow those imports as a parity exception — rejected because the user explicitly asked not to relax `RS-TEST`.
  - Move the baselines into another helper crate immediately — rejected because the local literal form was the smallest correct fix for these four files.

## Architectural Notes
This commit keeps the `RS-TEST` packaging rule uniform across both family crates and app-local helper crates: runtime crate plus sibling assertions crate, with owned sidecar directories in `src/`. `project-tree` now follows the same root-sidecar pattern already used successfully in `hooks-rs`, which matters because repo infrastructure crates were becoming a blind spot in the refactor. The production `ast` bug fix is intentionally bundled here because the new test layout forced that crate to compile again and exposed a genuine AST coverage hole.

## Information Sources
- Existing `RS-TEST` migrated patterns in `apps/guardrail3/crates/app/rs/families/hooks-rs`
- Live repo-root validator snapshot from `/tmp/rs-test-root.json`
- Prior worklogs:
  - `.worklogs/2026-03-29-175039-finish-rs-test-hooks-shared-family.md`
  - `.worklogs/2026-03-29-182711-finish-rs-test-hooks-rs-family.md`
- Local crate tests and focused validator runs:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-ast --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-generate --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-domain-project-tree --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
  - `apps/guardrail3/target/debug/guardrail3 rs validate ... --family test`

## Open Questions / Future Considerations
- Repo-root `RS-TEST` is still not zero after this chunk. Remaining errors are concentrated in `hexarch`, `code`, `garde`, and two `test` family self-fixture files.
- `RS-TEST-14` still warns at repo root because there is no executable `cargo mutants` hook step yet.
- The `ast` assertions helper currently uses primitive field arguments instead of the struct type because unit-test compilation sees the local test crate copy and the dependency-crate copy as distinct nominal types.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — workspace membership additions for the new assertions crates
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs` — owned sidecar path plus the foreign-module attr fix
- `apps/guardrail3/crates/app/rs/ast/assertions/src/ast_helpers.rs` — proof-bearing cfg-attr helper used by the sidecar
- `apps/guardrail3/crates/app/rs/generate/src/owned_artifacts_tests/mod.rs` — sidecar now delegates content proofs to its assertions crate
- `apps/guardrail3/crates/domain/project-tree/src/lib_tests/mod.rs` — root-sidecar pattern for a non-family crate
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_04_missing_method_ban_tests/parity.rs` — specimen for the clippy parity cleanup
- `.worklogs/2026-03-29-182711-finish-rs-test-hooks-rs-family.md` — prior family-level pattern that informed the app-local migration

## Next Steps / Continuation Plan
1. Commit this app-local/clippy chunk without staging unrelated `release`, `deps`, or `Cargo.lock` changes.
2. Tackle the remaining `RS-TEST-16` self-fixtures in `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs` and `.../rs_test_10_input_failures_tests/golden.rs`.
3. Sweep `apps/guardrail3/crates/app/rs/families/hexarch` next, focusing first on `RS-TEST-03` imports of `guardrail3_domain_report` and `guardrail3_domain_project_tree`, then the two remaining `RS-TEST-16` files in `rs_hexarch_01_crates_exists_tests`.
4. Sweep `garde`, then `code`, using the same rule: remove direct local-crate imports from sidecars first, then move remaining semantic guardrail assertions into owned assertions crates/modules.
