# Rename Shared Check Types And Extract Deny Content Checks

**Date:** 2026-04-05 13:19
**Scope:** `packages/guardrail3-check-types`, `packages/g3-deny-content-checks`, `packages/deny-toml-parser`, `packages/g3-fmt-content-checks`, `packages/g3-toolchain-content-checks`, `packages/rust-toolchain-toml-parser`, `apps/guardrail3/crates/app/rs/families/{deny,fmt,toolchain}`, `.plans/2026-04-04-142819-family-checks-packages.md`, `.plans/2026-04-04-143500-toolchain-checks-extraction.md`, `.plans/2026-04-05-deny-content-checks-extraction.md`

## Summary
Renamed the shared extracted-check result/profile/severity types from `Grdz*` to `G3*` across the extracted package surface, then completed the `deny` content-check extraction. `g3-deny-content-checks` now owns the agreed pure `deny.toml` content rules, and the app deny family delegates those rules while keeping coverage, profile-sensitive, and shadowing logic in-app.

## Context & Problem
The extracted package surface had drifted: the shared types crate still exposed `Grdz*` names while the newer package naming and public contract had already moved toward `G3*`. At the same time, `g3-deny-content-checks` was only a scaffold. The user explicitly wanted the shared types renamed and the deny content package finished, but also wanted the deny extraction done cleanly rather than by brute-force copying the old app raw-TOML rules.

The main architectural constraint was the already agreed split:
- parser packages represent files faithfully
- app families own discovery, authoritative file choice, and malformed-file routing
- content packages receive typed parsed files only
- remaining app-side deny rules continue to own profile-context and coverage behavior

## Decisions Made

### Rename The Shared Package Surface To `G3*`
- **Chose:** Rename `GrdzCheckResult`, `GrdzSeverity`, and `GrdzProfile` to `G3CheckResult`, `G3Severity`, and `G3Profile` in the shared types crate and downstream extracted packages.
- **Why:** The extracted package family names and public API had already standardized around `G3*`. Leaving the shared types under the older prefix kept the public contract inconsistent and forced conversion layers to carry legacy naming.
- **Alternatives considered:**
  - Leave the old names in `guardrail3-check-types` and only rename package-local wrappers — rejected because it preserves public inconsistency.
  - Add temporary aliases and defer the real rename — rejected because the user asked for the rename to be completed rather than prolonged.

### Build `g3-deny-content-checks` Against The Typed Parser Model
- **Chose:** Implement the deny package rules directly against `deny_toml_parser::DenyToml` and its typed sections instead of porting the app’s raw `toml::Value` helpers unchanged.
- **Why:** The parser had already been expanded into a file-faithful representation. Rebuilding the package around raw TOML would have duplicated obsolete app assumptions and ignored the actual package boundary.
- **Alternatives considered:**
  - Copy the old app rule code and support helpers as-is — rejected because it would keep the package coupled to raw-TOML shape handling instead of the typed parser contract.
  - Push more policy logic back into the app and keep the package thin — rejected because the agreed extracted rule set was explicitly pure `deny.toml` content validation.

### Keep Malformed-Shape App Rules Alive Even When Typed Package Input Is Missing
- **Chose:** Preserve raw TOML parsing in the app deny facts alongside optional typed `DenyToml`, and only delegate to the package when typed parsing succeeds.
- **Why:** The remaining app-owned deny rules (`RS-DENY-17`, `RS-DENY-25`, etc.) still intentionally inspect malformed-but-TOML-valid shapes. Treating typed parser failure as a family parse error suppressed those rules and broke existing tests.
- **Alternatives considered:**
  - Fail closed on typed parse failure and skip all later app rules — rejected because it erased existing app-owned malformed-shape diagnostics.
  - Remove raw TOML from the app entirely in this change — rejected because it would have forced a larger rewrite of the still-app-owned rules.

### Shrink App Deny Ownership Instead Of Keeping Dead Extracted Modules Live
- **Chose:** Remove the extracted rule modules from the app deny family module graph and trim `deny_support` to only the helpers still needed by app-owned rules.
- **Why:** Once the package became the single owner of the extracted rules, keeping the old app modules compiled only created dead-code noise and duplicate ownership.
- **Alternatives considered:**
  - Leave the old app modules compiled but unused — rejected because the crate is built with warnings as errors and it obscures the actual ownership boundary.
  - Delete all old rule files immediately — rejected because removing them from the module graph was sufficient for this change and less disruptive.

## Architectural Notes
- `g3-deny-content-checks` now owns:
  - `RS-DENY-04`, `05`, `06`, `07`, `08`
  - `RS-DENY-10`, `11`, `12`, `13`
  - `RS-DENY-14`, `15`, `16`
  - `RS-DENY-18`, `19`, `20`
  - `RS-DENY-21`, `22`
  - `RS-DENY-23`, `24`
  - `RS-DENY-27`, `28`, `29`
- The app deny family still owns:
  - `RS-DENY-01`, `03`
  - `RS-DENY-09`
  - `RS-DENY-17`
  - `RS-DENY-25`, `26`, `30`
- The app deny facts now keep both:
  - raw `toml::Value` for app-owned malformed-shape rules
  - optional typed `DenyToml` for package delegation
- `fmt` and `toolchain` package/app bridges were also updated to consume the current parser field-style APIs after the parser packages dropped getter methods.

## Information Sources
- `.worklogs/2026-04-05-125907-deny-toml-parser-full-schema-handoff.md`
- `.worklogs/2026-04-05-112641-fmt-toolchain-content-checks-boundaries.md`
- `.plans/2026-04-05-deny-content-checks-extraction.md`
- `.plans/2026-04-04-142819-family-checks-packages.md`
- `.plans/2026-04-04-143500-toolchain-checks-extraction.md`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/run.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts/mod.rs`
- `packages/g3-deny-content-checks/crates/runtime/src/support.rs`
- `packages/deny-toml-parser/crates/parser/types/src/*.rs`

## Open Questions / Future Considerations
- The deny package currently has compile coverage but not dedicated package-side rule tests. Behavior is still primarily verified through the app deny family tests.
- The app deny family still keeps raw TOML for its remaining local rules. If those rules are extracted later, the family can likely drop the raw parsed representation entirely.
- Historical worklogs still mention `Grdz*` names. Those were left intact as historical records.

## Key Files for Context
- `packages/guardrail3-check-types/crates/guardrail3-check-types/src/lib.rs` — shared `G3*` result/profile/severity exports
- `packages/g3-deny-content-checks/crates/runtime/src/run.rs` — deny package entrypoint and rule fanout
- `packages/g3-deny-content-checks/crates/runtime/src/support.rs` — typed deny helpers and baseline knowledge
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/run.rs` — app/package split for deny
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts/mod.rs` — raw+typed parse handling for deny config facts
- `packages/deny-toml-parser/crates/parser/types/src/deny_toml.rs` — typed deny file contract consumed by the package
- `.plans/2026-04-05-deny-content-checks-extraction.md` — deny extraction boundary and moved/staying rule inventory
- `.worklogs/2026-04-05-125907-deny-toml-parser-full-schema-handoff.md` — parser expansion rationale that this extraction now relies on

## Next Steps / Continuation Plan
1. Add dedicated rule tests inside `packages/g3-deny-content-checks` so deny package behavior is directly verified without depending only on app-family coverage.
2. Revisit the remaining app-owned deny rules (`RS-DENY-17`, `25`, `26`, `30`) and decide whether any can move once their policy-context or malformed-shape dependencies are clarified.
3. Continue the extraction sequence after deny, using the same pattern: typed parser input at the package boundary, app-owned malformed-file routing, and no raw-TOML leakage into content packages.
