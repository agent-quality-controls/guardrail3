# Finish RS-TEST Hooks-Shared Family

**Date:** 2026-03-29 17:50
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/hooks-shared/**`

## Summary
Migrated `hooks-shared` onto the owned `RS-TEST` sidecar shape, added the missing sibling assertions crate, and rewired the rule sidecars so they stay within owned module boundaries. The family now validates clean under `--family test`, its library tests pass, and the repo-root `RS-TEST` backlog dropped materially once this legacy family stopped leaking flat sidecars and direct semantic assertions.

## Context & Problem
After the `clippy` repo-root cleanup, the largest remaining `RS-TEST` bucket was still the legacy `hooks-shared` family. It was in the old single-crate shape with:
- flat `*_tests.rs` sidecars
- ad hoc `#[cfg(test)] mod tests;` declarations
- no sibling `assertions` crate even though owned sidecars existed
- sidecars importing sibling production modules like `facts`, `inputs`, and `hook_shell`
- semantic assertions still living directly in the sidecars

That meant the family was tripping all three main `RS-TEST` boundaries at once: `RS-TEST-02`, `RS-TEST-03`, and `RS-TEST-16`.

## Decisions Made

### Convert every internal sidecar to the owned directory shape
- **Chose:** move every flat `*_tests.rs` file to `<module>_tests/golden.rs`, add sibling `mod.rs` entrypoints, and rename production test modules from generic `mod tests;` to explicit `mod <module>_tests;`.
- **Why:** `RS-TEST-02` keys off both the directory shape and the declaration name. Using the explicit `<module>_tests` name matches the validator’s ownership model and removes the remaining “ad hoc cfg(test) module declaration” errors.
- **Alternatives considered:**
  - Keep flat files and only change `#[path = ...]` strings — rejected because `RS-TEST-02` still rejects flat sidecars.
  - Keep `mod tests;` and rely on the path attribute alone — rejected because `RS-TEST-02` expects the declaration name to match the owned sidecar directory.

### Add a sibling assertions crate instead of relaxing sidecar imports
- **Chose:** add `hooks-shared/assertions` with one assertions module per rule plus `hook_shell.rs`.
- **Why:** the family already had real owned sidecars, so the correct fix was to give them the sibling assertions surface that `RS-TEST-03` expects, not to carve out exemptions.
- **Alternatives considered:**
  - Relax `RS-TEST-03` for legacy hook families — rejected because the user explicitly asked not to relax any test-family rules.
  - Push semantic helpers into generic `test_support` — rejected because that collapses semantic proof back into generic helpers, which is exactly what `RS-TEST` is trying to prevent.

### Move scenario construction back behind owned production modules
- **Chose:** add `#[cfg(test)]` helper functions like `run_case(...)` inside the owning production modules for rules that previously imported sibling modules such as `facts`, `inputs`, and `hook_shell`.
- **Why:** owned sidecars may reach their owned production module subtree, but not sibling production modules. Small test-only helpers in the owner module keep that boundary strict while still letting the sidecar express the scenario it needs.
- **Alternatives considered:**
  - Keep constructing `DispatcherSyntaxInput` / `ExecutableCommandContextInput` directly inside sidecars — rejected because `RS-TEST-03` correctly flags those sibling-module imports.
  - Rebuild those constructors in `mod.rs` sidecar glue — rejected because the sidecar module itself would still need the same forbidden sibling imports.

### Move result semantics into the assertions crate
- **Chose:** replace direct sidecar checks on `result.id`, `result.title`, `result.inventory`, `result.message`, and `result.line` with assertions-crate helpers.
- **Why:** this is the core `RS-TEST-16` contract: sidecars own setup, assertions modules own guardrail-result proof. I extended the hooks-shared assertions common helper with optional `line` matching so the hook families could prove exact line attribution without falling back to sidecar-local assertions.
- **Alternatives considered:**
  - Leave direct `assert_eq!(results[0]....)` assertions in place because the family tests were already passing — rejected because the family validator still correctly flagged them as architecture violations.
  - Add blanket custom helper layers in `hook_shell_tests` only — rejected because the semantic ownership problem was per rule, not just in the shell parser tests.

## Architectural Notes
This checkpoint makes `hooks-shared` behave like the cleaned Rust families:
- one owned sidecar directory per rule
- one sibling assertions module per sidecar
- owned production modules may expose small `#[cfg(test)]` scenario helpers
- sidecars no longer tunnel through sibling runtime modules
- semantic proof now lives in assertions, not in the sidecars

The family is still a single runtime crate plus sibling assertions crate, not the newer `runtime/assertions/test_support` tri-split. That is acceptable for the current `RS-TEST` target because the actual issues were ownership and semantic-boundary issues, not missing generic fixture support.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_02_owned_sidecar_shape.rs` — exact shape constraints for owned sidecar directories and declaration names
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — runtime/assertions boundary rules that dictated the new helper placement
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — semantic assertion ownership boundary
- `.worklogs/2026-03-29-170630-fix-rs-test-clippy-repo-root.md` — prior repo-root `RS-TEST` checkpoint that left `hooks-shared` as the next dominant bucket
- local validation and tests:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hooks-shared --lib`
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/hooks-shared --family test --inventory --format json`
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family test --inventory --format json`

## Open Questions / Future Considerations
- `hooks-rs` is still in the old flat-sidecar/single-package shape and is the obvious next family in the repo-root `RS-TEST` backlog.
- The repo still has unrelated dirty work outside this checkpoint (`release`, `deps`, `project-tree`, `Cargo.lock`). Keep those out of the hooks-focused commits.
- `RS-TEST` is not repo-root clean yet; this checkpoint only removes the `hooks-shared` portion.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hooks-shared/Cargo.toml` — runtime crate now wired with sibling assertions dev-dependency
- `apps/guardrail3/crates/app/rs/families/hooks-shared/assertions/src/lib.rs` — owned assertions surface for all hooks-shared rules plus `hook_shell`
- `apps/guardrail3/crates/app/rs/families/hooks-shared/assertions/src/common.rs` — shared rule-result assertion helper with line-aware matching
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shared_18_executable_command_context_only.rs` — representative owned helper pattern for content-driven hook rules
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shared_18_executable_command_context_only_tests/golden.rs` — representative sidecar now staying inside the owned module + assertions boundary
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell.rs` — parser tests migrated to owned `hook_shell_tests/` plus owned assertions module presence
- `.worklogs/2026-03-29-170630-fix-rs-test-clippy-repo-root.md` — prior repo-root `RS-TEST` checkpoint that explains why `hooks-shared` was next

## Next Steps / Continuation Plan
1. Commit only the `hooks-shared` family changes plus the workspace member addition in `apps/guardrail3/Cargo.toml`; do not sweep in unrelated dirty files.
2. Rerun repo-root `RS-TEST` after the commit and use that updated backlog to confirm `hooks-rs` is now the dominant remaining family.
3. Apply the same owned-sidecar migration to `hooks-rs`: flatten its flat sidecars into `<module>_tests/mod.rs`, add a sibling assertions crate, and fix the special `hook_rs_13_cargo_dupes_excludes_tests.rs` naming trap before validating.
4. After `hooks-rs`, sweep the smaller residual `RS-TEST-16` tails in `garde`, `hexarch`, `code`, and the non-family app-local crates (`ast`, `generate`, `project-tree`) until the repo-root `RS-TEST` count reaches zero.
