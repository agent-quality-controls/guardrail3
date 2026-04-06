# Split Clippy Library Type-Ban Ownership

**Date:** 2026-03-27 21:22
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/{runtime,assertions}/src/{clippy_support.rs,rs_clippy_05_missing_type_ban.rs,rs_clippy_05_missing_type_ban_tests/library_profile.rs,rs_clippy_05_missing_type_ban.rs}`

## Summary
Removed an ownership overlap between `RS-CLIPPY-05` and `RS-CLIPPY-14`. The base missing-type rule no longer inventories or errors on library-only global-state bans; those remain owned solely by `RS-CLIPPY-14`, which matches the family plan and avoids double-reporting.

## Context & Problem
Continuing the adversarial review of `rs/clippy`, I noticed a contract drift in the profile-aware type-ban logic:

- `.plans/todo/checks/rs/clippy.md` says `RS-CLIPPY-05` owns the base type-ban set
- the same plan says `RS-CLIPPY-14` owns the library-only global-state type bans

But the implementation of `RS-CLIPPY-05` called `expected_type_bans(profile, garde_enabled)`, which already included the library-only extras when `profile == library`. That meant a library config missing `LazyLock`/`OnceLock`/`once_cell` bans could be reported under both `RS-CLIPPY-05` and `RS-CLIPPY-14`.

That overlap makes the rule inventory noisier and weakens ownership clarity. It is exactly the kind of “green but semantically wrong” drift the attack pass is supposed to flush out.

## Decisions Made

### Make `RS-CLIPPY-05` own only the base required type set
- **Chose:** Introduce `expected_required_type_bans(garde_enabled)` and have `RS-CLIPPY-05` use it instead of the profile-aware `expected_type_bans(...)`.
- **Why:** The plan assigns library-only global-state bans to `RS-CLIPPY-14`. `RS-CLIPPY-05` should stay the generic required type-ban rule.
- **Alternatives considered:**
  - Leave the overlap and accept duplicate reporting — rejected because it obscures which rule actually owns the library-only policy.
  - Remove `RS-CLIPPY-14` and let `05` own everything — rejected because the plan intentionally split the library-profile concern into its own rule.

### Keep profile-aware expectations for other consumers
- **Chose:** Preserve `expected_type_bans(profile_name, garde_enabled)` for rules that really need the profile-expanded set (`RS-CLIPPY-07`, `RS-CLIPPY-13`, etc.), while adding the narrower helper for `RS-CLIPPY-05`.
- **Why:** The family still needs profile-expanded sets for baseline-completeness and “extra ban” logic. Only the base missing-type rule needed narrowing.
- **Alternatives considered:**
  - Rename everything and refactor all callers in one big pass — rejected because the concrete bug was isolated and did not justify broader churn.

### Turn the old overlapping tests into anti-overlap regressions
- **Chose:** Rewrite the `library_profile` tests under `RS-CLIPPY-05` so they prove library-only global-state bans are *not* inventoried or double-reported there.
- **Why:** The old tests were encoding the wrong contract. Keeping them would just preserve the overlap.
- **Alternatives considered:**
  - Delete the tests entirely and rely on `RS-CLIPPY-14` coverage — rejected because the negative ownership boundary is important here.

## Architectural Notes
- `RS-CLIPPY-05` now owns the base required type-ban surface only.
- `RS-CLIPPY-14` remains the sole owner of library-only global-state type bans.
- `RS-CLIPPY-07` still uses the profile-expanded set so library-only bans do not get mislabeled as “extra” in library roots.
- `RS-CLIPPY-13` still uses the profile-expanded set so local library policy roots must remain self-contained.

This is the cleaner rule split:
- generic baseline presence in `05`
- library-profile specialization in `14`

## Information Sources
- `.plans/todo/checks/rs/clippy.md`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_14_library_global_state.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban_tests/library_profile.rs`
- prior clippy attack checkpoint:
  - `.worklogs/2026-03-27-211613-fix-clippy-policy-root-gaps.md`

## Open Questions / Future Considerations
- There may be similar ownership overlap elsewhere in the family, especially between:
  - `RS-CLIPPY-08` vs `RS-CLIPPY-15` for missing-vs-trivial reason quality
  - `RS-CLIPPY-13` vs the per-rule threshold/bool checks for local policy roots
- The next attack pass should keep looking for duplicated responsibility, not just parser holes.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/clippy_support.rs` — now exposes both base and profile-expanded type-ban helpers
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban.rs` — base missing-type rule after the ownership split
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_14_library_global_state.rs` — still owns library-only global-state requirements
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban_tests/library_profile.rs` — anti-overlap regression
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/rs_clippy_05_missing_type_ban.rs` — updated assertions surface for the anti-overlap tests
- `.worklogs/2026-03-27-211613-fix-clippy-policy-root-gaps.md` — previous semantic clippy fix in the same attack stream

## Next Steps / Continuation Plan
1. Continue the `RS-CLIPPY` attack pass on remaining policy rules:
   - `RS-CLIPPY-16`
   - `RS-CLIPPY-CONFIG-15`
   - `RS-CLIPPY-19`
   - `RS-CLIPPY-20`
2. Look specifically for:
   - rule overlap
   - fail-open behavior on malformed-but-active policy surfaces
   - false positives around user-owned Clippy keys
3. Once the unrelated outer-workspace break is fixed, rerun top-level `RS-TEST` on the clippy family to make sure the structural cleanup and semantic hardening still compose correctly.
