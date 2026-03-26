# Tighten RS-TEST Validator Holes

**Date:** 2026-03-26 20:53
**Scope:** `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove_tests/proof_bearing.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic_tests/boundaries.rs`

## Summary
Hardened `RS-TEST` so it no longer false-greens several deterministic bypasses found by adversarial review: name-based proof credit, `extern crate` alias escapes, root-layout blind spots, and shallow `test_support` helper analysis. The stricter validator still passes the self-hosted `test`, `arch`, `cargo`, and `hexarch` families after the earlier fallout fixes.

## Context & Problem
Recent attack passes showed that the rewritten Rust test architecture was structurally cleaner, but the validator still had several precise holes:
- proof-bearing assertions could be inferred from `assert_*` names instead of only from resolved owned assertions symbols
- sidecar semantic leakage was only caught when it used known assertion macros, not `panic!`/`unwrap`/`expect`
- `extern crate ... as ...` was invisible to the boundary checker
- `test_support` genericity only looked at direct public helper bodies
- the validator only recognized `test_support/src`, not `crates/test_support/src`

Those holes mattered because they allowed families to appear compliant while still circumventing the ownership contract through narrow syntactic tricks.

## Decisions Made

### Remove proof-by-name heuristics
- **Chose:** deleted the public re-export and imported-helper name heuristics from the proof catalog and kept proof recognition tied to resolved owned assertions calls.
- **Why:** the bypass is deterministic and should be blocked deterministically.
- **Alternatives considered:**
  - keep the heuristics for convenience — rejected because they create false greens
  - extend the allowlist of helper names — rejected because the problem is name-based proof credit itself

### Parse more of the real import surface
- **Chose:** recorded `extern crate ... as ...` aliases in the parsed import model and used that richer model in boundary enforcement.
- **Why:** alias imports are a standard Rust path shape and were a trivial escape hatch.
- **Alternatives considered:**
  - ignore `extern crate` as legacy syntax — rejected because the rules are supposed to be robust, not style-dependent
  - special-case only known project aliases — rejected because the family must stay portable

### Treat failure enforcement as sidecar-owned proof
- **Chose:** extended sidecar semantic-proof detection to count `panic!`, `unwrap`, `expect`, `unwrap_err`, and `expect_err`, not only explicit assertion macros.
- **Why:** those operators are still concrete proof sites when a sidecar uses them to check result semantics.
- **Alternatives considered:**
  - keep only macro-based detection — rejected because it leaves an obvious bypass
  - attempt broader semantic inference over arbitrary expressions — rejected because the failure-enforcement proxy is cleaner and deterministic

### Follow helper chains in `test_support`
- **Chose:** made `RS-TEST-18` follow local helper calls so a public generic-looking wrapper cannot hide a private semantic selector over `CheckResult`.
- **Why:** the direct-body-only check was too shallow and easy to game.
- **Alternatives considered:**
  - keep direct-body checking only — rejected because it misses transitive leaks
  - ban all private helpers in `test_support` — rejected because that is too rigid and would create noise

### Support both valid `test_support` locations
- **Chose:** recognized both root-local `test_support/src` and README-shaped `crates/test_support/src` as `test_support`.
- **Why:** the validator should match the architecture it documents.
- **Alternatives considered:**
  - force one layout immediately — rejected because the current family shape already admits both during migration
  - ignore the crates-based layout — rejected because it would create false positives against the documented structure

## Architectural Notes
This checkpoint keeps `RS-TEST` generic by relying on local structural roles instead of project-specific package names:
- owned assertions proof is tracked from resolved local symbol identity
- route-construction checks are keyed off structural `FamilyMapper` usage rather than hardcoded guardrail crate names
- `test_support` is identified by local component position, not package naming conventions

The practical contract is now tighter:
- sidecars only get proof credit from owned assertions or direct proof macros
- sidecars that enforce semantics through panic/unwrap-style operators are treated as owning proof
- `test_support` cannot hide semantic selectors behind local helper chains

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- adversarial findings gathered in the session’s `test-attack` pass
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs`
- `.worklogs/2026-03-26-194521-tighten-rs-test-validator-checkpoint.md`

## Open Questions / Future Considerations
- `RS-TEST-18` is stricter now, but it is still a structural proxy for genericity rather than a full semantic theorem.
- Broader Rust routing/scope issues outside `RS-TEST` remain for later passes; this checkpoint only hardens the test family itself.
- If future attacks find more helper-shape bypasses, the next likely work area is deeper local call resolution inside assertions/test-support modules.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — proof catalog and test-support layout detection.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs` — parsed import/function facts now used for the stricter checks.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — boundary enforcement for sidecars and assertions modules.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — sidecar semantic-proof detection.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_18_test_support_generic.rs` — transitive `test_support` genericity enforcement.
- `.worklogs/2026-03-26-194521-tighten-rs-test-validator-checkpoint.md` — previous validator-tightening checkpoint this work builds on.

## Next Steps / Continuation Plan
1. Re-run adversarial review against the tightened validator and collect the next concrete false-green or false-positive class.
2. If a family newly fails under the harder contract, repair that family by moving proof back into owned assertions instead of weakening the validator.
3. Continue the broader Rust architecture cleanup separately, especially the remaining routing/scope questions outside `RS-TEST`.
