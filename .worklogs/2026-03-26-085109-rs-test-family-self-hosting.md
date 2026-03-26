# Make RS-TEST Family Validate Against Itself

**Date:** 2026-03-26 08:51
**Scope:** `apps/guardrail3/crates/app/rs/families/test/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/test/src/rs_test_02_owned_sidecar_shape.rs`, `apps/guardrail3/crates/app/rs/families/test/src/rs_test_02_owned_sidecar_shape_tests/**`, `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split.rs`, `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split_tests/**`

## Summary
Taught `RS-TEST-02` and `RS-TEST-03` to recognize the accepted guardrail-family implementation shape used by the `rs/test` family itself, while preserving fail-closed behavior for ordinary unmapped harnesses. Added explicit regression tests and revalidated the family crate directly until `guardrail3 rs validate ... --family test` returned zero findings on the family root.

## Context & Problem
After the previous checkpoint, `RS-TEST-03` no longer silently skipped ordinary root-local harnesses. That fixed the false-negative class, but it exposed the second half of the design tension: the `rs/test` family’s own accepted implementation shape was still being flagged as if it were an ordinary application crate that should use the `runtime/assertions` split.

There were two separate self-hosting failures:

- `RS-TEST-02` rejected the family’s accepted rule-local sidecar wiring because it only accepted cfg-test declarations named after the owner module, not the `mod tests;` plus `#[path = "..._tests/mod.rs"]` form used in the family.
- `RS-TEST-03` treated the family’s rule-specific sidecar harness directories as unmapped ordinary harnesses and reported them as being outside the `runtime/assertions` split.

The user asked to fix the family itself after recording the validator gotcha, so the next step was to make the validator distinguish the family’s own accepted rule-test architecture from the application-crate architecture it enforces elsewhere.

## Decisions Made

### Detect guardrail-family implementation roots explicitly
- **Chose:** Add `is_guardrail_family_implementation_root(files)` in the family orchestrator layer, using the concrete shape already present in this repo:
  - root-local files only
  - `lib.rs`
  - `test_support.rs`
  - source files owned by `rs_*`
  - sidecar mod directories owned by `rs_*`
  - no external harnesses
- **Why:** The validator already needs to distinguish two architectures. Making that distinction explicit is clearer than relying on accidental skips or forcing the family into an application-crate architecture it was never intended to use.
- **Alternatives considered:**
  - Restructure the family crate into a literal `runtime/assertions` pair — rejected because that conflicts with the accepted family implementation shape and would be architecture theater inside the checker itself.
  - Leave the family failing itself and treat that as acceptable debt — rejected because the user explicitly asked to fix the family itself.

### Accept family-local sidecar declarations in `RS-TEST-02`
- **Chose:** For detected guardrail-family implementation roots, allow:
  - `#[cfg(test)] #[path = "<owner>_tests/mod.rs"] mod tests;` on `rs_*` rule files
  - `#[cfg(test)] mod test_support;` in `lib.rs`
- **Why:** Those are the accepted shapes already documented for the family implementation. `RS-TEST-02` should not reject the checker’s own sanctioned sidecar wiring.
- **Alternatives considered:**
  - Rename every rule file to use long module names like `mod rs_test_01_inline_test_bodies_tests;` — rejected because the current `mod tests;` form is valid and readable once the validator understands the implementation shape.
  - Special-case only the exact family crate path — rejected because the structural distinction is the important bit, not the path string.

### Skip `RS-TEST-03` unmapped-harness errors for detected family implementation roots
- **Chose:** Preserve the ordinary unmapped-harness error path, but disable it when the root is a detected guardrail-family implementation root.
- **Why:** The family’s own rule-sidecar directories are not evidence of a missing `runtime/assertions` split in an application crate. They are the accepted internal test architecture for this checker family.
- **Alternatives considered:**
  - Disable the unmapped-harness error globally again — rejected because that would reintroduce the false-negative class fixed in the previous checkpoint.
  - Force the family to fabricate an `assertions` owner just to satisfy the validator — rejected because that would not reflect the actual accepted architecture.

## Architectural Notes
- The validator now models two distinct shapes:
  - ordinary crate harnesses that must map to `runtime/assertions`
  - guardrail-family implementation roots with rule-specific sidecar test directories and `test_support.rs`
- This is still repo-specific logic, but it is explicit and test-covered instead of being an accidental omission.
- The family crate is now self-validating under `RS-TEST` without weakening the fail-closed behavior for normal roots.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `apps/guardrail3/crates/app/rs/families/test/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_02_owned_sidecar_shape.rs`
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split.rs`
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_02_owned_sidecar_shape_tests/family_impl.rs`
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split_tests/family_impl.rs`
- `.plans/todo/validator/2026-03-26-rs-test-self-hosting-gotchas.md`
- `.worklogs/2026-03-26-084820-validator-rs-test-self-hosting-gotcha.md`
- `CARGO_TARGET_DIR=/tmp/guardrail3-test-family-selfhost cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
- `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/test --family test --inventory --format json`

## Open Questions / Future Considerations
- The family-root detector is intentionally conservative, but it is still heuristic. If more checker families adopt similar rule-sidecar patterns, it may be worth promoting this into shared validator vocabulary rather than keeping it local to `RS-TEST`.
- There are unrelated dirty files elsewhere in the repo that are intentionally excluded from this commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/src/lib.rs` — root-shape detection helper
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_02_owned_sidecar_shape.rs` — sidecar-shape allowances for family implementation roots
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split.rs` — self-hosting exemption from ordinary unmapped-harness errors
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_02_owned_sidecar_shape_tests/family_impl.rs` — regression proof for accepted family sidecar wiring
- `apps/guardrail3/crates/app/rs/families/test/src/rs_test_03_runtime_assertions_split_tests/family_impl.rs` — regression proof that the family root does not require `runtime/assertions`
- `.plans/todo/validator/2026-03-26-rs-test-self-hosting-gotchas.md` — design note explaining the tension this commit resolves

## Next Steps / Continuation Plan
1. Keep using direct self-validation on `apps/guardrail3/crates/app/rs/families/test` whenever `RS-TEST-02` or `RS-TEST-03` changes.
2. If other checker families adopt the same rule-sidecar pattern, consider extracting the family-root detection into shared validator support instead of duplicating or inlining similar logic.
3. Revisit whether the broader validator should expose an explicit “family implementation root” concept in typed facts rather than heuristic detection over analyzed files.
