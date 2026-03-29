# Continue RS-CODE Allow Cleanup

**Date:** 2026-03-29 21:26
**Scope:** `apps/guardrail3/crates/app/rs/families/{arch,deny,garde,hexarch,hooks-shared,test}`

## Summary
Finished the second `RS-CODE-03` sweep by removing the remaining undocumented test-harness `#[allow(...)]` surface instead of converting it into reasoned exceptions. This cleared repo-root `RS-CODE-03` to zero and also removed now-proven-dead helper wrappers in `deny` and `hexarch` that had only been surviving behind `dead_code` suppressions.

## Context & Problem
After the first allow-cleanup commit (`ec69c20d`), repo-root `RS-CODE` was no longer dominated by copied suppressions, but the tree still carried an uncommitted follow-up slice across `arch`, `deny`, `garde`, `hexarch`, `hooks-shared`, and `test`. The remaining pattern was narrow:
- `arch` sidecar files still had file-level `#[allow(unused_imports)]` copied from a broad fixture-import template
- `test` had two stale `#[allow(dead_code)]` wrappers
- `deny` and `hexarch` still had old test helper wrappers that compiled only because `dead_code` had not yet been fully challenged

The user explicitly wanted the allow surface reduced without copping out with more reason comments.

## Decisions Made

### Remove leftover file-level sidecar suppressions instead of documenting them
- **Chose:** Delete the file-level `#[allow(unused_imports)]` lines in `arch` sidecar tests and let `cargo fix --tests` shrink each import list to what the test file actually uses.
- **Why:** These were not architectural exceptions. They were simply broad `use super::{...}` lists kept alive by a copied top-of-file suppression.
- **Alternatives considered:**
  - Add `// reason:` comments to the file-level allows — rejected because the imports were mostly unnecessary.
  - Leave the broad imports and accept permanent inventory noise — rejected because it preserves the dead seam instead of removing it.

### Delete orphaned deny/hexarch test wrappers once the compiler proved they were dead
- **Chose:** Remove test-only wrappers such as `run_check_with_profile`, `run_forbidden`, `run_same_root_conflict`, `run_family`, and `expected_ban_wrappers_for_test` where the family tests no longer call them.
- **Why:** The allow sweep surfaced the real state: many of these helpers had no remaining callers. Restoring `dead_code` would only hide that the sidecars had already moved on to narrower helpers.
- **Alternatives considered:**
  - Restore `#[allow(dead_code)]` around the wrappers — rejected because it would directly undo the cleanup.
  - Keep the wrappers “just in case” — rejected because there was no caller evidence and the compiler was already proving them dead.

### Trim shared `tree_at` exports instead of re-suppressing the mod-level warning
- **Chose:** Keep `tree_at` exported only in `rs_arch_02_no_misplaced_roots_tests/mod.rs`, the one arch rule that still uses it.
- **Why:** Once the child-file suppressions were gone, the remaining warning source was the shared mod-level re-export itself.
- **Alternatives considered:**
  - Add a `reason` comment or restore `unused_imports` at the mod level — rejected because six of the seven modules did not need the export.
  - Keep `tree_at` exported everywhere for uniformity — rejected because that uniformity was precisely what forced the suppressions.

## Architectural Notes
- This pass keeps the same policy as the first code cleanup: a suppressible seam should exist only if a live test actually needs it.
- `cargo fix --tests` worked well once the blanket allows were removed, because the remaining child imports became mechanically reducible.
- The `deny` family ended up as the strongest proof that the cleanup is real rather than cosmetic: removing suppressions exposed helper wrappers that were truly dead, and deleting them still left the family tests green.

## Information Sources
- Repo-root code validation:
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json`
- Prior worklog:
  - `.worklogs/2026-03-29-211331-trim-rs-code-allow-surface.md`
- Family test runs:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-arch --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-garde --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hooks-shared --lib`

## Open Questions / Future Considerations
- `RS-CODE` is still blocked mainly by `RS-CODE-24` (`#[path]` without same-line reason) and `RS-CODE-32` (weak `.expect(...)` messages). This commit does not start those buckets.
- There are still unrelated dirty files outside this commit scope, especially `Cargo.lock`, `ast`, `code` family tests, `hooks-rs`, and `crates/domain/project-tree/src/lib.rs`.
- `RS-CODE-05` in `crates/domain/project-tree/src/lib.rs` remains untouched and should be handled separately from the allow cleanup.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_02_no_misplaced_roots_tests/mod.rs` — the one arch rule that still legitimately exports `tree_at`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_09_ban_baseline_complete.rs` — representative deny rule where unused test wrappers were removed
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_24_cross_app_boundary.rs` — representative hexarch rule where orphaned test helpers were deleted
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_10_input_failures.rs` — test family helper with stale `dead_code` removed
- `.worklogs/2026-03-29-211331-trim-rs-code-allow-surface.md` — first allow-cleanup slice and the rationale this one extends

## Next Steps / Continuation Plan
1. Commit only the cleaned family directories from this slice: `arch`, `deny`, `garde`, `hexarch`, `hooks-shared`, and the two `test` runtime files; keep unrelated dirty files unstaged.
2. Start the next repo-root `RS-CODE` pass on `RS-CODE-24`, because that is now the largest remaining bucket at `269` errors.
3. After the `#[path]` sweep, tackle `RS-CODE-32` `.expect(...)` message quality, which is largely independent string cleanup.
4. Handle `crates/domain/project-tree/src/lib.rs` `RS-CODE-05` separately from the family cleanup so the `garde(skip)` documentation decision stays explicit.
