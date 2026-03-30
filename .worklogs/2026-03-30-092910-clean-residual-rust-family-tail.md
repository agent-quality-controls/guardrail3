# Clean Residual Rust Family Tail

**Date:** 2026-03-30 09:29
**Scope:** `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_06_owner_family_enablement_coherence_tests/ownership_coherence.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_01_exists_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_02_settings_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_03_extra_settings_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_06_edition_mismatch.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_06_edition_mismatch_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_07_ignore_escape_hatch_tests/mod.rs`, `apps/guardrail3/crates/domain/modules/cspell.rs`, `apps/guardrail3/crates/domain/modules/eslint.rs`, `apps/guardrail3/crates/domain/modules/cspell/tests.rs`, `apps/guardrail3/crates/domain/modules/eslint/tests.rs`

## Summary
Committed the remaining non-clippy dirty tail that was left behind by earlier family sweeps. The substantive fixes are module-owned test relocation for `domain/modules` and stronger fmt test fixture messages/helpers; the arch file is only formatting, and the lockfile now matches the tested crate graph.

## Context & Problem
After the main family cleanup, the repo was still dirty in a few narrow places:
- `domain/modules` still had legacy sibling test files that no longer matched the module-owned shape after the code cleanup
- `fmt` had helpful-but-uncommitted test fixture/message hardening
- `Cargo.lock` had drifted to reflect new assertion/test-support crates and dev-dependencies used by the now-green tree
- one `arch` test file had formatting-only changes

The user asked for the repo to be reviewed as a whole, not just the originally owned lane. Leaving these changes unstaged would preserve a dirty tree even though the code and tests were already depending on them.

## Decisions Made

### Move domain-module tests into owned module files
- **Chose:** Replace `cspell_tests.rs` and `eslint_tests.rs` with `cspell/tests.rs` and `eslint/tests.rs`, and switch each module to `#[cfg(test)] mod tests;`.
- **Why:** This preserves module-local ownership without using another `#[path]` escape hatch and fixes the compilation break caused by the old sibling-file arrangement after the previous cleanup.
- **Alternatives considered:**
  - Reintroduce `#[path = "..."]` from the module root — rejected because it would recreate avoidable `RS-CODE-24` noise.
  - Leave the broken sibling-file layout — rejected because `guardrail3-domain-modules` would not compile cleanly.

### Keep the fmt changes as test-quality hardening
- **Chose:** Commit the fmt test helper/message changes as-is.
- **Why:** They improve fixture parsing clarity and `.expect(...)` signal quality without changing rule semantics. The family tests stayed green.
- **Alternatives considered:**
  - Drop them as unrelated noise — rejected because they are legitimate cleanup in already-touched family test files.

### Commit the lockfile with the tested graph
- **Chose:** Include `apps/guardrail3/Cargo.lock` in this residual cleanup commit.
- **Why:** The tree now contains additional assertion/test-support crates and dev-dependencies that were already compiled during validation. Keeping the lockfile stale would make the repo inconsistent with the tested state.
- **Alternatives considered:**
  - Leave the lockfile dirty for a later catch-all commit — rejected because the user asked for the repo to be cleaned and committed now.

## Architectural Notes
The `domain/modules` test relocation is the meaningful architectural change here. It preserves the stricter “owned tests live under the module they exercise” direction without introducing more path-based exceptions. That keeps the modules aligned with the rest of the family-local test-ownership cleanup done elsewhere in the repo.

The fmt and arch edits do not change rule behavior. They tighten test readability and fixture diagnostics only.

## Information Sources
- `.worklogs/2026-03-30-051054-finish-non-clippy-rs-code-cleanup.md`
- `.worklogs/2026-03-30-091513-clippy-full-sweep-cleanup.md`
- Package tests run against this exact tree:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-domain-modules --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-fmt --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-arch --lib`
- `git diff --check`

## Open Questions / Future Considerations
- The nested `rustfmt.toml` warnings in the repo are still intentional `RS-FMT-05` warnings for self-hosted family roots; this commit does not change that policy.
- If `domain/modules` gains more generated-config modules later, they should follow the same `module/tests.rs` pattern rather than reviving sibling `*_tests.rs` files.

## Key Files for Context
- `apps/guardrail3/crates/domain/modules/cspell.rs` — module wiring for the cspell config generator
- `apps/guardrail3/crates/domain/modules/cspell/tests.rs` — owned tests for generated cspell config shape
- `apps/guardrail3/crates/domain/modules/eslint.rs` — module wiring for the eslint config generator
- `apps/guardrail3/crates/domain/modules/eslint/tests.rs` — owned tests for generated eslint config shape
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_04_nightly_keys_on_stable_tests/mod.rs` — representative fmt fixture-message hardening
- `apps/guardrail3/Cargo.lock` — tested dependency graph snapshot
- `.worklogs/2026-03-30-051054-finish-non-clippy-rs-code-cleanup.md` — prior context for the earlier code cleanup that made the domain-modules relocation necessary

## Next Steps / Continuation Plan
1. Commit the remaining untracked planning/docs files so the repo is fully clean.
2. Re-run final repo-root validation for `code`, `clippy`, and `arch` after the last commit.
3. Keep any subsequent family work in separate commits; this commit is the cleanup tail, not a new policy sweep.
