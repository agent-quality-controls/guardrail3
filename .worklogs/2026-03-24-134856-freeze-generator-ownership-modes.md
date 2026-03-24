# Freeze Generator Ownership Modes

**Date:** 2026-03-24 13:48
**Scope:** `.plans/todo/generate/rs/clippy.md`, `.plans/todo/generate/rs/deny.md`, `.plans/todo/generate/rs/gap-analysis.md`

## Summary
Adjusted the Rust generator planning contracts so `clippy` and `deny` are semantic-patch families rather than exact-owned file families. This freezes the product stance that guardrail3 must preserve user-added hardening while still normalizing canonical baseline policy and refusing any weakening.

## Context & Problem
The first generator planning pass treated `clippy.toml` and `deny.toml` as exact-owned files. That was internally coherent, but it clashed with the product decision the user then made:
- guardrail3 should generate the same canonical baseline everywhere it owns policy
- user-added extra bans are acceptable
- nothing may weaken guardrail policy, locally or globally
- generator reruns should not wipe user work

Those requirements are incompatible with exact-owned whole-file generation. If the user can legitimately add more bans inside the same file and those additions must survive reruns, the generator owns a canonical baseline inside a user-extended file, not the entire file bytes.

## Decisions Made

### Make `clippy` a semantic-patch generator family
- **Chose:** change `clippy` from exact-owned to semantic-patch.
- **Why:** the generator must preserve user-added hardening while continuing to normalize the canonical baseline on every run.
- **Alternatives considered:**
  - Keep `clippy` exact-owned and wipe local additions on rerun — rejected because it destroys legitimate user-added hardening.
  - Allow arbitrary local edits to survive without normalization — rejected because that would preserve policy weakening and drift.

### Restrict preserved user entries to additive ban surfaces
- **Chose:** preserve only additive local ban surfaces for `clippy` and additive ban/feature-ban surfaces for `deny`.
- **Why:** additive hardening is compatible with the guardrail contract; relaxing surfaces are not.
- **Alternatives considered:**
  - Preserve all user edits — rejected because skip/allow/license/source relaxations would become stable escape hatches.
  - Preserve no user edits — rejected because the user explicitly wants additive local hardening to survive.

### Keep generator-owned baseline surfaces canonical on every run
- **Chose:** specify that thresholds, booleans, and required baseline bans (`clippy`), plus relaxing policy surfaces (`deny`), normalize back to canonical on every run.
- **Why:** this is the core anti-bypass behavior. User additions can survive, but user weakening cannot.
- **Alternatives considered:**
  - Let stronger and weaker local edits both survive — rejected because weakening is exactly what guardrail3 exists to block.

## Architectural Notes
- The Rust generator family split is now:
  - exact-owned: `fmt`, `toolchain`, `release`, `hooks`
  - semantic-patch: `clippy`, `deny`, `cargo`
  - scaffold: `hexarch`, `libarch`
- This is a stronger product split than “everything semantic-patch” because some generated files are safety-critical enough that partial user edits would be harmful (`hooks`, `release`) or the file itself is the policy root and local variation is an escape hatch (`fmt`, `toolchain`).
- `clippy` and `deny` now align with the product rule:
  - preserve extra hardening
  - normalize away weakening

## Information Sources
- `.plans/todo/generate/rs/clippy.md`
- `.plans/todo/generate/rs/deny.md`
- `.plans/todo/generate/rs/gap-analysis.md`
- `.plans/todo/generate/README.md`
- `.plans/todo/generate/rs/README.md`
- `.plans/todo/checks/rs/clippy.md`
- `.plans/todo/checks/rs/deny.md`
- same-session discussion about additive local bans versus exact-owned files
- `.worklogs/2026-03-24-132341-freeze-generator-specs.md`

## Open Questions / Future Considerations
- The long-term transport for root-local additive bans is still open. The plan intentionally freezes the semantic contract, not the permanent UX. Current override-file plumbing remains current-state only in `gap-analysis.md`.
- If future product decisions allow additional additive surfaces in `deny`, the semantic-patch contract can expand, but it should not expand to any relaxing surface.

## Key Files for Context

- `.plans/todo/generate/rs/clippy.md` — semantic-patch contract for root-local clippy generation.
- `.plans/todo/generate/rs/deny.md` — semantic-patch contract for root-local deny generation.
- `.plans/todo/generate/rs/gap-analysis.md` — current-state note that now records the ownership-mode mismatch against current code.
- `.plans/todo/generate/README.md` — top-level generator ownership-mode doctrine.
- `.plans/todo/checks/rs/clippy.md` — checker-side clippy contract the generator must satisfy.
- `.plans/todo/checks/rs/deny.md` — checker-side deny contract the generator must satisfy.
- `.worklogs/2026-03-24-132341-freeze-generator-specs.md` — prior worklog that froze the broader Rust generator planning set.

## Next Steps / Continuation Plan

1. Commit this ownership-mode refinement separately from any future generator-planning changes.
2. Run adversarial review on the generator planning set, focusing on:
   - whether the contracts are actionable
   - whether checker/generator ownership is reconciled
   - whether mixed-root and nested-hex cases are fully covered
3. If adversarial review finds real contract gaps, patch the family specs or `gap-analysis.md` rather than reintroducing current-code notes into the family files.
