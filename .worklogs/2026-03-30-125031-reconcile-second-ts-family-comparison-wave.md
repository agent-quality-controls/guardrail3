# Reconcile Second TS Family Comparison Wave

**Date:** 2026-03-30 12:50
**Scope:** `.plans/by_family/ts/README.md`, `.plans/by_family/ts/hexarch.md`, `.plans/by_family/ts/code.md`, `.plans/by_family/ts/content.md`, `.plans/by_family/ts/libarch.md`, `.plans/by_family/ts/tests.md`, `.plans/by_family/ts/i18n.md`, `.plans/by_family/ts/seo.md`

## Summary
Ran the second Rust-vs-TypeScript family comparison wave across the next TS architecture-heavy and capability families, then updated the by-family TS plans to record the design conclusions directly. The resulting plan surface now makes explicit which TS families are real owner families, which are capability families, and where Rust-style fail-closed and ownership discipline are still missing.

## Context & Problem
After the first comparison wave (`arch`, `eslint`, `tsconfig`, `npmrc`, `package`), the remaining high-leverage TS families still had unresolved design ambiguity:
- `hexarch` and `code` looked real, but weaker than their Rust counterparts.
- `content` and `libarch` were still planning-led and boundary-unclear.
- `tests` risked being misread as a counterpart to `RS-TEST` even though it is really a test-quality family.
- `i18n` and `seo` were capability families, but that dependency was not spelled out strongly enough.

The user asked to keep sending adversarial agents and continue planning, so this pass was focused on design hardening rather than implementation changes.

## Decisions Made

### Mark TS owner families versus capability families explicitly
- **Chose:** Update the family plans so `hexarch`, `code`, and `tests` are described more precisely, and `i18n`/`seo` are explicitly described as capability families rather than peer owner families.
- **Why:** The biggest risk in the TS planning surface was not missing rule IDs; it was readers treating all families as equivalent peers. Rust is more disciplined because ownership families and dependent/capability surfaces are not blurred.
- **Alternatives considered:**
  - Keep the distinction only implicit — rejected because the same ambiguity would keep resurfacing as implementation drift.
  - Invent a full TS owner/capability taxonomy in a separate meta-doc — rejected because the family files themselves should carry the consequence directly.

### Push fail-closed and anti-bypass language into the TS plans now
- **Chose:** Add explicit notes and next-step requirements for fail-closed handling and anti-bypass coverage where the Rust comparison showed they were missing.
- **Why:** Rust became trustworthy only after hardening around silent degradation and bypasses. TS families were still too likely to fail open or remain vague about that expectation.
- **Alternatives considered:**
  - Wait until runtime implementations exist — rejected because the design should demand fail-closed semantics before the next implementation pass, not after.

### Treat `TS-TESTS` as a product test-quality family, not a clone of `RS-TEST`
- **Chose:** Make the family file state explicitly that `TS-TESTS` is not the TS analogue of `RS-TEST`.
- **Why:** The Rust `test` family is a family-hardening/meta-architecture family. The TS family is currently about product test quality. Pretending they are equivalent would create the wrong next-step pressure.
- **Alternatives considered:**
  - Keep comparing them as if they should converge fully — rejected because it would distort the intended TS family role.

## Architectural Notes
- `TS-HEXARCH` now records the key missing Rust-like ideas:
  - routed inputs from a stronger TS arch layer
  - fail-closed required architecture inputs
  - separation of config prerequisites from true architecture semantics
  - explicit evasion categories such as alias/barrel/dynamic-import bypasses
- `TS-CODE` now records that it is much narrower than `RS-CODE`, still lacks an explicit input-failure rule, and should not own installed dependency policy (`T59`) without a stronger rationale.
- `TS-CONTENT` now states more directly that content-root discovery belongs in `TS-ARCH`, while the family should center on pipeline/model/artifact ownership rather than generic import-policy drift.
- `TS-LIBARCH` now records that it needs a concrete smallest-input model before it is implementation-ready.
- `TS-I18N` now explicitly records the overloaded one-rule problem and the need for fail-closed locale/config behavior.
- `TS-SEO` now explicitly records that it is a capability family depending on routed owner-family scope rather than a standalone root-owner family.

## Information Sources
- `.plans/by_family/ts/README.md`
- `.plans/by_family/ts/hexarch.md`
- `.plans/by_family/ts/code.md`
- `.plans/by_family/ts/content.md`
- `.plans/by_family/ts/libarch.md`
- `.plans/by_family/ts/tests.md`
- `.plans/by_family/ts/i18n.md`
- `.plans/by_family/ts/seo.md`
- `.plans/by_family/rs/hexarch.md`
- `.plans/by_family/rs/code.md`
- `.plans/by_family/rs/test.md`
- `.plans/by_family/rs/arch.md`
- subagent adversarial comparison reports for `hexarch`, `code`, `content`, `libarch`, `tests`, and `i18n`/`seo`

## Open Questions / Future Considerations
- `TS-HEXARCH` still needs a final decision on whether route-wrapper enforcement stays there or remains a lint-side bridge rule.
- `TS-CODE` still needs a decision on whether `any` stays inventory-only and whether a future TS dependency family exists.
- `TS-CONTENT` still needs a precise definition of “content purity” versus generic architecture boundaries.
- `TS-LIBARCH` still needs a concrete applicability/input model before it is stable enough to replace the old ledger.
- `TS-TESTS` still needs a decision on assertion-bearing surface, unfinished-test policy, and how much tool/package coherence stays in the family.

## Key Files for Context
- `.plans/by_family/ts/README.md` — current TS by-family index and accumulated comparison state
- `.plans/by_family/ts/hexarch.md` — TS service/extension architecture family after Rust comparison
- `.plans/by_family/ts/code.md` — TS source-policy family, now with explicit narrowness and fail-closed gaps recorded
- `.plans/by_family/ts/content.md` — TS content-family boundary after separating it from root discovery and generic import policy
- `.plans/by_family/ts/libarch.md` — TS library architecture family, still planning-led and needing a concrete input model
- `.plans/by_family/ts/tests.md` — TS product test-quality family, explicitly not a counterpart to Rust family-hardening semantics
- `.plans/by_family/ts/i18n.md` — TS capability-family contract with the overloaded rule problem recorded
- `.plans/by_family/ts/seo.md` — TS capability-family contract with owner-family dependence made explicit
- `.plans/by_family/rs/hexarch.md` — Rust reference for hardened architecture-family ownership
- `.plans/by_family/rs/code.md` — Rust reference for source-policy breadth and fail-closed expectations
- `.plans/by_family/rs/test.md` — Rust reference for what a truly hardened test/meta family looks like
- `.worklogs/2026-03-30-122211-tighten-first-five-ts-family-plans.md` — prior TS comparison wave for the first five canonical families

## Next Steps / Continuation Plan
1. Continue the Rust-vs-TS comparison on the remaining TS families that are likely to be real owner/config families next: `fmt`, `spelling`, `size`, `jscpd`, and `css`.
2. After each comparison wave, update the matching `.plans/by_family/ts/*.md` files immediately before touching any old ledger docs.
3. Once a family file has both rule inventory and comparison-based boundary notes, add a superseded-primary banner to the matching `.plans/todo/checks/ts/*.md` ledger.
4. Do not start TS implementation work for `content`, `libarch`, or `seo` until their routed-root applicability and sibling-family boundaries are explicit in the by-family docs.
