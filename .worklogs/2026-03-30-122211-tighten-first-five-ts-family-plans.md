# Tighten First Five TS Family Plans

**Date:** 2026-03-30 12:22
**Scope:** `.plans/by_family/ts/README.md`, `.plans/by_family/ts/arch.md`, `.plans/by_family/ts/eslint.md`, `.plans/by_family/ts/tsconfig.md`, `.plans/by_family/ts/npmrc.md`, `.plans/by_family/ts/package.md`

## Summary
Updated the first five TypeScript family plan files after an adversarial Rust-vs-TS comparison pass. The edits do not add new TS rules blindly; they make the missing design boundaries explicit, especially around fail-closed behavior, family ownership, and overloaded rule surfaces.

## Context & Problem
The new `.plans/by_family/ts/**` tree had a real rule inventory, but the first five families still reflected a softer and less disciplined design than the Rust families they are supposed to learn from. The user asked to compare the first five TS families against Rust counterparts because the Rust families are more complete, then to turn that comparison into direct plan edits.

The main recurring weakness across TS was not missing rule IDs by itself. It was mixed ownership:
- `TS-ARCH` lacked the Rust-style global placement/fail-closed model.
- `TS-TSCONFIG` mixed toolchain-floor and local strictness/inheritance concerns.
- `TS-PACKAGE` mixed root policy, local manifest policy, and dependency/tool ownership.
- `TS-ESLINT` mixed config policy with package presence and some architecture enforcement.
- `TS-NPMRC` was comparatively clean but still lacked explicit fail-closed language.

## Decisions Made

### Make Rust counterparts explicit in the TS family plans
- **Chose:** Add explicit references and comparison notes to the relevant Rust families inside each TS family plan.
- **Why:** The user’s goal was not a generic TS cleanup. It was to make the TS family design catch up to the already-hardened Rust family design. Making those counterpart relationships explicit prevents the next pass from reintroducing fuzzy ownership.
- **Alternatives considered:**
  - Keep the comparison only in chat output — rejected because the design conclusion would be lost instead of becoming part of the plan surface.
  - Add a separate comparison doc — rejected because the family files themselves are the current planning surface and should carry the consequence directly.

### Add missing design obligations without pretending the implementation already has them
- **Chose:** Expand the TS family plans with explicit missing semantics such as fail-closed handling, global-only ownership, exempt-root handling, and applicability matrices.
- **Why:** The plans need to describe the correct destination even when current TS code has not implemented it yet. Avoiding those additions would preserve the same under-specification that caused the drift.
- **Alternatives considered:**
  - Only document what current code does today — rejected because it would freeze design debt into the new primary planning surface.
  - Invent a full new rule set everywhere — rejected because the request was to reconcile against current code carefully, not to explode the inventory speculatively.

### Keep the edits focused on the first five families rather than rewriting the whole TS tree
- **Chose:** Update only `arch`, `eslint`, `tsconfig`, `npmrc`, `package`, plus the TS family index README.
- **Why:** These are the five families the user asked to audit first, and they are enough to set the standard for the rest of the TS family reconciliations.
- **Alternatives considered:**
  - Start editing other TS families immediately — rejected because it would blur the source of the conclusions and make it harder to see what came from the first Rust comparison pass.

## Architectural Notes
- `TS-ARCH` now explicitly points toward a Rust-style global placement family with governed roots, excluded roots, exempt roots, global-only config ownership, and fail-closed required inputs.
- `TS-TSCONFIG` now records the most important design correction: it should be split conceptually into a base/runtime floor and local strictness/inheritance, even if it remains one family.
- `TS-PACKAGE` now records the need for a root-kind applicability matrix and fail-closed manifest handling.
- `TS-ESLINT` now states plainly that package presence and architecture-adjacent checks are mixed into the family and need boundary cleanup.
- `TS-NPMRC` now stays intentionally narrow and calls out fail-closed handling as the main missing piece rather than inventing unnecessary redesign.

## Information Sources
- `.plans/by_family/ts/README.md`
- `.plans/by_family/ts/arch.md`
- `.plans/by_family/ts/eslint.md`
- `.plans/by_family/ts/tsconfig.md`
- `.plans/by_family/ts/npmrc.md`
- `.plans/by_family/ts/package.md`
- `.plans/by_family/rs/arch.md`
- `.plans/by_family/rs/clippy.md`
- `.plans/by_family/rs/cargo.md`
- `.plans/by_family/rs/deny.md`
- `.plans/by_family/rs/deps.md`
- `.plans/by_family/rs/toolchain.md`
- subagent adversarial comparison reports for `TS-ARCH`, `TS-TSCONFIG`, and `TS-PACKAGE`

## Open Questions / Future Considerations
- `TS-ARCH-05` still needs a final decision on owner-family coherence versus optional capability implications like `i18n` and `seo`.
- `TS-TSCONFIG` still needs a concrete decision on whether `target`, `module`, and `moduleResolution` remain in this family or become the seed of a future TS toolchain split.
- `TS-PACKAGE` still needs a real decision on whether dependency policy stays manifest-local here or becomes a future TS dependency family.
- `TS-ESLINT` still needs a firm split between lint policy and TS architecture enforcement.

## Key Files for Context
- `.plans/by_family/ts/README.md` — current TS planning index and current trust ordering for the first five families
- `.plans/by_family/ts/arch.md` — TS architecture family target contract after Rust comparison
- `.plans/by_family/ts/tsconfig.md` — TS compiler-config family with the new base-floor vs local-inheritance split recorded
- `.plans/by_family/ts/package.md` — TS package policy family with fail-closed and applicability-matrix requirements recorded
- `.plans/by_family/ts/eslint.md` — TS lint-policy family with boundary-mixing called out explicitly
- `.plans/by_family/ts/npmrc.md` — TS package-manager config family, currently the cleanest of the first five
- `.plans/by_family/rs/arch.md` — Rust reference for global placement/fail-closed architecture ownership
- `.plans/by_family/rs/cargo.md` — Rust reference for root-policy ownership and lint/dependency boundary discipline
- `.plans/by_family/rs/clippy.md` — Rust reference for lint-config family ownership
- `.plans/by_family/rs/toolchain.md` — Rust reference for repo/runtime floor semantics
- `.worklogs/2026-03-30-111654-create-by-family-rust-plan-surface.md` — background for the by-family planning cutover
- `.worklogs/2026-03-30-114340-add-typescript-family-placeholders.md` — background for the TS by-family tree creation

## Next Steps / Continuation Plan
1. Continue the Rust-vs-TS adversarial comparison family by family, starting with the next highest-leverage TS architecture families: `hexarch`, `code`, and `content`.
2. After each comparison pass, update the corresponding `.plans/by_family/ts/*.md` files before touching old ledgers, so the primary planning surface stays ahead of the cleanup.
3. Once a TS family summary stabilizes, add a superseded-primary banner to the matching `.plans/todo/checks/ts/*.md` ledger pointing back to the by-family file.
4. Delay implementation changes until the family plan clearly separates root ownership, fail-closed behavior, and sibling-family boundaries; otherwise the TS code will keep encoding mixed policy.
