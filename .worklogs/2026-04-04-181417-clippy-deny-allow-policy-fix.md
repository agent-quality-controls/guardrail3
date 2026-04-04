# Clippy And Deny Allow-Policy Fix

**Date:** 2026-04-04 18:14
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_15_trivial_reason/**`, `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_26_ban_reason_inventory/**`

## Summary
Adjusted the clippy and deny families so deny-only baseline surfaces no longer warn just because entries are documented. `RS-CLIPPY-15` now stays silent for ban-only surfaces, and `RS-DENY-26` now inventories only extra deny bans beyond the managed baseline instead of treating canonical deny entries as missing/weak-reason violations.

## Context & Problem
Scoped validation on `packages/rustfmt-toml` was producing warning noise for legitimate `clippy.toml` and `deny.toml` baselines. The user clarified that reasons only matter for actual weakening surfaces: matching required deny entries should stay quiet, extra deny entries should be inventory, and allow-shaped weakenings are the suspicious cases. The current implementations of `RS-CLIPPY-15` and `RS-DENY-26` were keyed to "documented reason exists" instead of "policy is being weakened", so they were warning on correct harder-or-equal policy.

## Decisions Made

### Retire `RS-CLIPPY-15` From Deny-Only Ban Hygiene
- **Chose:** Make `RS-CLIPPY-15` emit no findings for `disallowed-methods`, `disallowed-types`, and `disallowed-macros`.
- **Why:** Those ban surfaces are already owned by the baseline and extra-ban rules. Treating documented ban entries as suspicious was a false positive against the user's policy.
- **Alternatives considered:**
  - Keep warning on documented ban entries — rejected because it continues to flag canonical and stricter configs.
  - Downgrade documented entries to inventory — rejected because the user explicitly said matching and stricter denies should not matter here at all.

### Reframe `RS-DENY-26` Around Extra Deny Inventory
- **Chose:** Compare `[bans].deny` entries against the profile-resolved baseline and inventory only extra deny bans.
- **Why:** Canonical deny entries are part of the required baseline, not escape hatches. Extra deny entries are useful to surface, but only as inventory.
- **Alternatives considered:**
  - Preserve reason validation for every deny entry — rejected because it warns/errors on legitimate baseline policy.
  - Make the rule fully silent — rejected because extra deny bans are still useful inventory for local stricter policy.

### Update Tests To Match The New Ownership Boundary
- **Chose:** Rewrite the rule tests to assert silence for canonical deny-only surfaces and inventory for extra deny bans.
- **Why:** The old tests encoded the false-positive behavior. Keeping them would pin the wrong contract.
- **Alternatives considered:**
  - Delete tests entirely — rejected because the new behavior still needs explicit regression coverage.
  - Leave partial legacy expectations in place — rejected because that would keep the ownership boundary ambiguous.

## Architectural Notes
The underlying rule boundary is now clearer:
- baseline completeness remains owned by the existing clippy and deny baseline rules
- extra stronger bans remain inventory
- reason hygiene should only matter on actual weakening / allow-like escape hatches, not on deny-only baseline surfaces

This change is intentionally narrow. It does not fix the separate `code` subtree-scope bug or the duplicated allow ownership in the cargo family.

## Information Sources
- `packages/rustfmt-toml/Cargo.toml` and `packages/rustfmt-toml/deny.toml` for the real package behavior that triggered the investigation
- `packages/rustfmt-toml/clippy.toml` for the scoped validation false positives
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_15_trivial_reason/rule.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_26_ban_reason_inventory/rule.rs`
- `.plans/todo/checks/rs/clippy.md`
- `.plans/todo/checks/rs/deny.md`

## Open Questions / Future Considerations
- `RS-CLIPPY-15` is now effectively retired for current ban-only surfaces; decide whether to delete it or repurpose it for a real weakening surface later.
- Cargo-family allow handling still has duplicated ownership between `RS-CARGO-03` and `RS-CARGO-12`.
- Extra deny additions in the cargo-family clippy lint baseline are still silent; if desired, add a dedicated inventory rule there.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_15_trivial_reason/rule.rs` — now-silent rule for deny-only clippy ban surfaces
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_26_ban_reason_inventory/rule.rs` — deny baseline comparison and extra-ban inventory behavior
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_08_reason_quality/rule.rs` — neighboring rule that still owns actual ban-entry reason formatting/shape
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/bans/rs_deny_09_ban_baseline_complete/rule.rs` — deny baseline completeness owner
- `.worklogs/2026-04-04-181417-clippy-deny-allow-policy-fix.md` — this worklog for the policy change rationale

## Next Steps / Continuation Plan
1. Decide whether `RS-CLIPPY-15` should be deleted or repurposed now that ban-only surfaces are quiet.
2. Clean up cargo-family ownership so `RS-CARGO-03` inventories approved allows and `RS-CARGO-12` alone owns unapproved allow errors.
3. Fix the `code` family subtree-scoping leak so package-scoped validation stops reporting unrelated repo files.
