# Refactor Canonical Clippy Generator

**Date:** 2026-03-22 16:27
**Scope:** `apps/guardrail3/crates/domain/modules/clippy/**`, `apps/guardrail3/crates/domain/modules/mod.rs`

## Summary
Replaced the old monolithic generator-side `domain/modules/clippy.rs` file with a split `clippy/` module tree and aligned the generated canonical baseline with the frozen clippy contract. The generator now renders the full managed scalar settings, updated method/type bans, and required macro bans from smaller testable pieces.

## Context & Problem
The user explicitly called out the old generator module as an outdated jumble that was becoming its own source-of-truth blob. That was a real problem for the migration:
- the new checker family was being built rule-by-rule in isolated files
- the generator baseline still lived in one dense file
- generator/checker drift had already shown up in the cargo family and was starting to show up in clippy too

The goal of this refactor was not to finish every future generator concern, but to make the clippy generator readable, modular, and auditable enough that the checker could safely align to it.

## Decisions Made

### Split the generator by policy slice instead of keeping one big file
- **Chose:** Replace `domain/modules/clippy.rs` with `domain/modules/clippy/{mod,thresholds,settings,methods,types,macros,render}.rs`.
- **Why:** The clippy baseline is naturally decomposed into independent policy slices. That makes the canonical contract visible and testable instead of hiding it in one rendering blob.
- **Alternatives considered:**
  - Keep the monolith and only patch values — rejected because it preserves the same auditability problem.
  - Split only render logic and leave the baseline data monolithic — rejected because the baseline content is what needed decomposition.

### Add the missing managed scalar settings and macro baseline to generated output
- **Chose:** Render:
  - all 7 frozen thresholds
  - explicit boolean settings (`avoid-breaking-exported-api`, `allow-dbg-in-tests`, `allow-print-in-tests`)
  - required `disallowed-macros`
- **Why:** The clippy checker is supposed to validate upstream enforcement knobs. The canonical generated file must therefore contain the full managed baseline, not only methods and types.
- **Alternatives considered:**
  - Leave booleans/macros checker-only — rejected because generator/checker drift is exactly what we are removing.

### Export structured baseline constants for reuse
- **Chose:** Add exported path/value slices such as `SERVICE_METHOD_PATHS`, `BASE_TYPE_PATHS`, `LIBRARY_EXTRA_TYPE_PATHS`, `THRESHOLD_VALUES`, and `EXPECTED_MACRO_BANS`.
- **Why:** The checker can now import the same canonical baseline instead of retyping lists by hand.
- **Alternatives considered:**
  - Keep duplicated constants in checker and generator — rejected because it recreates drift risk.

### Harden the canonical ban set while refactoring
- **Chose:** Add the previously agreed hardening items:
  - `std::process::abort`
  - `std::any::Any`
  - required macro bans
  - `max-fn-params-bools`
  - `excessive-nesting-threshold`
- **Why:** There was no value in preserving a knowingly stale canonical baseline while doing a structural refactor.
- **Alternatives considered:**
  - Refactor first, harden later — rejected because it would create an extra drift window immediately after the split.

## Architectural Notes
The generator is now organized the same way the checker migration wants the rest of the codebase to look:
- policy data in narrow files
- rendering isolated in one place
- tests exercising both composition and rendered output

This is still generator code, not the checker family architecture, but it now exposes the same kind of independently reviewable slices instead of one opaque block.

## Information Sources
- `.plans/todo/checks/rs/clippy.md` — frozen clippy contract
- `.plans/by_file/rs/clippy-toml.md` — generation/merge behavior and allowed user-owned keys
- old `apps/guardrail3/crates/domain/modules/clippy.rs` — previous monolithic generator implementation
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` — generator call sites for `build_clippy_toml`
- `.worklogs/2026-03-22-162712-clippy-policy-freeze.md` — policy freeze committed immediately before this refactor

## Open Questions / Future Considerations
- The generator still uses the existing override model from `generate_helpers.rs`; that broader generate-path cleanup is separate from this clippy baseline refactor.
- Garde-conditional clippy baselines are still not threaded through this canonical generator split. The current baseline remains the hardened default.

## Key Files for Context
- `apps/guardrail3/crates/domain/modules/clippy/mod.rs` — canonical clippy module entrypoint and re-exports
- `apps/guardrail3/crates/domain/modules/clippy/render.rs` — canonical clippy.toml renderer
- `apps/guardrail3/crates/domain/modules/clippy/thresholds.rs` — exact numeric threshold contract
- `apps/guardrail3/crates/domain/modules/clippy/settings.rs` — managed boolean settings
- `apps/guardrail3/crates/domain/modules/clippy/macros.rs` — macro-ban baseline
- `apps/guardrail3/crates/domain/modules/clippy/clippy_tests.rs` — generator-side contract tests
- `.worklogs/2026-03-22-162712-clippy-policy-freeze.md` — prior policy freeze worklog

## Next Steps / Continuation Plan
1. Commit this generator refactor independently so the canonical baseline change is isolated from checker behavior changes.
2. Commit the checker family completion next:
   - `apps/guardrail3/crates/app/rs/checks/rs/clippy/**`
   - `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`
3. Keep all unrelated dirty files out of the commit; they belong to other ongoing work in the repo.
