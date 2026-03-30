# Reconcile Third TS Family Comparison Wave

**Date:** 2026-03-30 12:53
**Scope:** `.plans/by_family/ts/README.md`, `.plans/by_family/ts/fmt.md`, `.plans/by_family/ts/spelling.md`, `.plans/by_family/ts/size.md`, `.plans/by_family/ts/jscpd.md`, `.plans/by_family/ts/css.md`

## Summary
Ran the third Rust-vs-TypeScript comparison wave across the remaining narrower TS families and updated the by-family plans to reflect the results. This pass clarified which families are already coherent narrow tool/config families (`jscpd`, `css`, `spelling`) and which still need fundamental design decisions (`fmt`, `size`).

## Context & Problem
After reconciling the architecture-heavy TS families, the remaining unfinished picture was the smaller tool/config families. These were easier to overlook because they are narrower, but they still needed the same treatment:
- compare them to the nearest Rust design standards
- separate genuinely coherent families from bucket families
- record fail-closed, ownership, and applicability gaps directly in the by-family plans

The major risk here was leaving these as “small enough to ignore,” which would recreate the same planning drift that happened in the larger TS families.

## Decisions Made

### Treat `TS-FMT` as still fundamentally underdesigned
- **Chose:** Mark `TS-FMT` as substantially weaker than `RS-FMT` and call out the missing config ownership, fail-closed behavior, and root-vs-local policy decision.
- **Why:** The current TS family still only really enforces package presence. Without clarifying whether formatting is root-only policy or local-config policy, the family cannot become implementation-ready.
- **Alternatives considered:**
  - Treat `TS-FMT` as already “basically fine” because the surface is small — rejected because the actual contract is much larger than the live runtime.

### Keep `TS-SPELLING` narrow rather than inflating it
- **Chose:** Describe `TS-SPELLING` as a narrow tool/capability family with missing ownership and fail-closed details, not as a broad text-quality family.
- **Why:** The current implementation already points toward a small coherent family. The right move is to sharpen ownership and config behavior, not to let it sprawl.
- **Alternatives considered:**
  - Expand it immediately into a broader content/text semantics family — rejected because that would blur it with `content`, `i18n`, and `code`.

### Call out `TS-SIZE` as still bucket-shaped
- **Chose:** Record that `TS-SIZE` still needs an explicit applicability decision: content/public-web capability family versus general app/package budget family.
- **Why:** The main problem is not missing tool checks. It is that the family does not yet know what kinds of roots it actually governs.
- **Alternatives considered:**
  - Assume the current content-profile bias is the final answer — rejected because the old contract is broader and the family plan should not accidentally freeze an unresolved design question.

### Treat `TS-JSCPD` as mostly coherent once spillover is removed
- **Chose:** Frame `TS-JSCPD` as a reasonably well-shaped duplication family whose main remaining problem is content spillover, not core family incoherence.
- **Why:** This is one of the TS families where the live rule surface is already narrow and understandable; the planning job is to preserve that narrowness.
- **Alternatives considered:**
  - Recast it as a broader content/code policy family — rejected because the duplication core is stronger when kept small.

### Treat `TS-CSS` as a real policy family with remaining boundary work
- **Chose:** Describe `TS-CSS` as reasonably coherent already, but still split at the package/config boundary and unclear on exact root ownership.
- **Why:** This matches the current code and avoids pretending the family is either fully mature or still merely aspirational.
- **Alternatives considered:**
  - Call it planning-led like the weakest TS families — rejected because the config/rule half is already meaningfully real.

## Architectural Notes
- `TS-FMT` now clearly records the missing Rust-style ideas:
  - canonical config ownership
  - parseability/fail-closed behavior
  - override/shadowing semantics
  - root-only versus local policy
- `TS-SPELLING` is now positioned as a small tool/config family, not a broad semantic family.
- `TS-SIZE` now records that its main problem is family applicability, not just missing implementation.
- `TS-JSCPD` is now explicitly protected from reabsorbing content-family policy into duplication policy.
- `TS-CSS` now records that package-presence spillover and Tailwind/ESLint bridge ownership are the main remaining design questions.

## Information Sources
- `.plans/by_family/ts/README.md`
- `.plans/by_family/ts/fmt.md`
- `.plans/by_family/ts/spelling.md`
- `.plans/by_family/ts/size.md`
- `.plans/by_family/ts/jscpd.md`
- `.plans/by_family/ts/css.md`
- `.plans/by_family/rs/fmt.md`
- `.plans/by_family/rs/code.md`
- `.plans/by_family/rs/arch.md`

## Open Questions / Future Considerations
- `TS-FMT` still needs a concrete decision on root-only versus nearest-local formatting policy.
- `TS-SIZE` still needs a family-level applicability decision before implementation work should continue.
- `TS-CSS` still needs a clean answer on whether Tailwind-related bridge rules stay in CSS or remain partially ESLint-owned.
- `TS-SPELLING` still needs explicit malformed-config and nearest-config semantics.

## Key Files for Context
- `.plans/by_family/ts/README.md` — current TS family index with accumulated comparison-wave conclusions
- `.plans/by_family/ts/fmt.md` — TS formatting family, still the clearest weak spot in the narrower tool families
- `.plans/by_family/ts/spelling.md` — TS spelling family, currently a narrow tool/config family
- `.plans/by_family/ts/size.md` — TS size/budget family, still bucket-shaped and applicability-unclear
- `.plans/by_family/ts/jscpd.md` — TS duplication family, narrow and mostly coherent once spillover is removed
- `.plans/by_family/ts/css.md` — TS CSS/stylelint family, relatively coherent but still boundary-split
- `.plans/by_family/rs/fmt.md` — Rust reference for a hardened formatting family
- `.plans/by_family/rs/code.md` — Rust reference for narrow source-policy/fail-closed expectations
- `.plans/by_family/rs/arch.md` — Rust reference for applicability and owner-family discipline
- `.worklogs/2026-03-30-122211-tighten-first-five-ts-family-plans.md` — first TS comparison wave
- `.worklogs/2026-03-30-125031-reconcile-second-ts-family-comparison-wave.md` — second TS comparison wave

## Next Steps / Continuation Plan
1. Start adding superseded-primary banners to the old TS ledgers for the families whose by-family files are now strong enough: `npmrc`, `package`, `jscpd`, `css`, and likely `spelling`.
2. Hold off on demoting the old ledgers for the still-underdesigned families until their remaining ownership questions are resolved: `arch`, `tsconfig`, `fmt`, `size`, `content`, `libarch`, `seo`.
3. If the user wants to continue the TS planning cleanup, the next useful pass is not another family comparison wave; it is a controlled demotion of the old `.plans/todo/checks/ts/*.md` ledgers family by family, using the now-reconciled by-family docs as the primary surface.
