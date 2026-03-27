# Tighten RS-CODE Include And Result Rules

**Date:** 2026-03-27 19:31
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/analysis_helpers.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_23_include_bypass_tests/direct.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_25_public_result_error_type_tests/direct.rs`

## Summary
Tightened two `RS-CODE` rules after adversarial review. `RS-CODE-23` no longer exempts arbitrary `include!` patterns that merely mention `OUT_DIR`, and `RS-CODE-25` now detects weak public `Result` error types on public trait methods in addition to free functions and inherent methods.

## Context & Problem
The ongoing `RS-CODE` attack pass is focused on validator correctness, not repo cleanup. Two concrete detector gaps surfaced:

- `RS-CODE-23` treated any `include!` expression that referenced `env!("OUT_DIR")` somewhere inside the macro arguments as the allowed build-script inventory pattern. That created a false green for arbitrary nested macro shapes that still bypass file-boundary analysis.
- `RS-CODE-25` only scanned public free functions and public inherent methods. Public trait methods returning `Result<_, String>` or `Result<_, Box<dyn Error>>` were silently ignored even though they are part of the public API surface.

Both issues were deterministic parser/classification bugs rather than policy ambiguity, so they were good checkpoint candidates before continuing the wider rule attack.

## Decisions Made

### Narrow The Allowed `include!` Shape
- **Chose:** treat only the exact documented build-script pattern `include!(concat!(env!("OUT_DIR"), ...))` as the inventory exemption.
- **Why:** that is the intended safe carveout. A generic "mentions `OUT_DIR` somewhere" test is too broad and lets unrelated include bypasses pass cleanly.
- **Alternatives considered:**
  - Recurse through arbitrary macro expressions and allow any branch containing `env!("OUT_DIR")` — rejected because it recreates the false green.
  - Remove the build-script exemption entirely — rejected because the family already documents that pattern as allowed inventory.

### Treat Public Trait Methods As Public Result Surface
- **Chose:** extend the existing visitor to inspect `pub trait` items and report weak `Result` error types on public trait methods.
- **Why:** trait methods are part of the public library contract and the rule intent is about public API error typing, not only free functions.
- **Alternatives considered:**
  - Leave trait methods to Clippy or rustdoc-style tooling — rejected because this family already owns the public API surface rule.
  - Add a separate rule just for trait methods — rejected because the policy is identical and the gap is in the detector, not the inventory.

## Architectural Notes
The changes stay inside the existing AST analysis layer for `RS-CODE`. No route, family orchestrator, or self-host architecture changed. The important boundary decision was to keep the rule deterministic by matching a structural macro shape rather than doing heuristic token search over arbitrary nested expressions.

## Information Sources
- `.worklogs/2026-03-27-174353-rs-code-attack-round-2.md` — prior `RS-CODE` detector attack checkpoint.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_23_include_bypass.rs` — rule intent for include bypasses.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_25_public_result_error_type.rs` — rule intent for weak public error types.
- Local adversarial findings from the active `RS-CODE` audit.

## Open Questions / Future Considerations
- `RS-CODE-20` still appears to have a nested `cfg_attr` hole on `extern` blocks.
- `RS-CODE-07` likely misses trailing inline `EXCEPTION:` comments because collection is line-prefix based.
- `RS-CODE-08` still needs a closer look for multiline attribute line-number matching.
- `RS-CODE-12` may still miss table-shaped `unsafe_code = { level = "deny" }` lint configuration.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs` — shared AST visitors for multiple `RS-CODE` rules.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/analysis_helpers.rs` — expression-shape helpers used by include/path analysis.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_23_include_bypass.rs` — include-boundary rule.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_25_public_result_error_type.rs` — weak public result error rule.
- `.worklogs/2026-03-27-174353-rs-code-attack-round-2.md` — previous `RS-CODE` attack baseline.

## Next Steps / Continuation Plan
1. Continue the `RS-CODE` attack pass on the next concrete detector candidates: `RS-CODE-07`, `RS-CODE-08`, `RS-CODE-12`, and `RS-CODE-20`.
2. After the remaining obvious parser/classification gaps are closed, decide whether any open issues are real policy questions rather than rule bugs.
3. Once `RS-CODE` detector behavior is more stable, start `RS-CLIPPY` stabilization with a README, self-host workspace split, and `RS-TEST` migration.
