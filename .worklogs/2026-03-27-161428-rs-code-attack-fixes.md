# RS-CODE Attack Fixes

**Date:** 2026-03-27 16:14
**Scope:** `apps/guardrail3/crates/app/rs/families/code/**`, `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `.plans/todo/checks/rs/code.md`

## Summary
Ran the first real adversarial pass against `rs/code`, fixed the concrete false-positive and fail-closed holes it exposed, and re-verified the family against `RS-CODE`, `RS-TEST`, and the top-level Rust runtime. The slice tightened parser coverage, corrected test-path classification and canonical `#[path]` handling, scoped config-derived `RS-CODE` results to the active routed roots, and fixed `RS-CODE-30` fail-closed behavior for malformed or unreadable active inputs.

## Context & Problem
`rs/code` had already been structurally stabilized and was green against its own family checks, but it had not yet been attacked the way `rs/test` and `rs/hexarch` were. The attack pass surfaced two classes of problems:

- real false positives and false greens in the rule implementations
- architecture drift between the family README and the runtime/test helper shape

The highest-signal concrete bugs were:

- `_tests/` sidecar directories were not treated as test files, producing bogus `RS-CODE-01`, `RS-CODE-09`, and `RS-CODE-15` hits on repo-standard sidecars
- `RS-CODE-24` exempted the wrong canonical `#[path]` wiring and missed `cfg_attr(..., path = ...)`
- `RS-CODE-14` and `RS-CODE-15` still missed UFCS and alias forms
- `RS-CODE-30` was self-tested but still dead on the live runtime path for malformed `guardrail3.toml`
- `scoped_files` only narrowed Rust source checks; config inventories and fail-closed config/root results still leaked across every routed root

## Decisions Made

### Treat repo-standard sidecar trees as test files
- **Chose:** extend `is_test_path()` so `_tests/` directories count as test code, not only conventional `tests/` roots.
- **Why:** the repo-standard family sidecar layout uses `<rule>_tests/`, and the code family was false-positiveing that exact shape.
- **Alternatives considered:**
  - special-case only the `code` family sidecar directories — rejected because the path shape is now repo-standard across self-hosted families
  - relax the affected rules instead of fixing path classification — rejected because the bug was discovery, not policy

### Exempt only the documented canonical `#[path]` test wiring, but recognize both live shapes
- **Chose:** keep `RS-CODE-24` strict, but exempt the two documented sidecar forms:
  - `#[cfg(test)] #[path = "<current_rule>_tests/mod.rs"] mod <current_rule>_tests;`
  - `#[cfg(test)] #[path = "<current_rule>_tests/mod.rs"] mod tests;`
- **Why:** the family and the repo both use canonical test-sidecar wiring, but there were two real shapes in flight. The rule needed to recognize both without weakening generic `#[path]` enforcement.
- **Alternatives considered:**
  - allow all `#[path]` in test modules — rejected because it would gut the rule
  - keep only the self-hosted `mod <rule>_tests;` exemption — rejected because it still false-positiveed repo-standard `mod tests;`

### Harden parser coverage instead of carving out self-exemptions
- **Chose:** extend the parser to catch:
  - `cfg_attr(..., path = ...)`
  - UFCS and fully-qualified `unwrap` / `expect`
  - `use std as alias; alias::fs::...`
- **Why:** the attack findings were concrete bypasses, not documentation mismatches.
- **Alternatives considered:**
  - teach the family to ignore its own parser blind spots — rejected because that would preserve live false greens
  - depend on clippy/lints to catch the same shapes — rejected because `RS-CODE` exists specifically to close those structural bypasses

### Make fail-closed behavior real at the top-level runtime
- **Chose:** include `Code` in the “config parse failure may still produce family findings” runtime allowlist and make `facts.rs` validate `guardrail3.toml` through typed config parsing before falling back to TOML inspection.
- **Why:** `RS-CODE-30` was supposed to surface malformed active inputs, but explicit `--family code` runs still aborted on malformed `guardrail3.toml`.
- **Alternatives considered:**
  - leave the mismatch and rely only on family-local unit tests — rejected because the live CLI behavior was wrong
  - loosen `RS-CODE-30` so malformed config is not part of the contract — rejected because fail-closed behavior is one of the family’s core jobs

### Scope config-derived results to active routed roots
- **Chose:** derive active root dirs from `route.scoped_files` and only emit code-family config inventories / root-manifest failures for those active roots.
- **Why:** `scoped_files` previously filtered only Rust source-file execution. `RS-CODE-07`, `RS-CODE-12`, and root-manifest `RS-CODE-30` findings still leaked from unrelated routed roots during scoped runs.
- **Alternatives considered:**
  - change the route type so mapper removes inactive roots entirely — rejected because it was a larger cross-family route-shape change than needed for this bug
  - leave config inventories global even on scoped runs — rejected because it produced concrete cross-root leakage the attack pass identified

### Reduce production-surface route-construction drift without inventing a new test harness architecture mid-slice
- **Chose:** move the route-construction dependencies in `crates/runtime/src/lib.rs` behind `#[cfg(test)]` and update the README wording so the contract speaks about the production runtime surface, not a broader shared test harness that does not exist yet.
- **Why:** the attack pass correctly pointed out that the runtime crate still constructs routes for its own tests. The smallest truthful improvement was to keep that logic out of the production surface and stop overstating the family-local contract.
- **Alternatives considered:**
  - build a brand new shared routed-family test harness crate in this slice — rejected because it would have widened the work far beyond the concrete bugs found
  - leave the README claiming the drift was already solved — rejected because it was no longer true once the attack pass surfaced it

### Keep `test_support` generic by hiding raw temp-root path generation
- **Chose:** make `temp_root()` private and expose `create_temp_dir(slug) -> TempDir` instead.
- **Why:** the new `RS-TEST-18` enforcement correctly treated public raw path factories as too specific / canned for generic `test_support`.
- **Alternatives considered:**
  - special-case `temp_root` in `RS-TEST-18` — rejected because that would weaken the generic-support boundary
  - keep returning `PathBuf` publicly — rejected because it immediately broke self-hosting under `RS-TEST`

## Architectural Notes
This slice kept the existing routed-family architecture intact:

- shared root scope still comes from `placement`
- `FamilyMapper` still maps that scope into `RsCodeRoute`
- `rs/code` still owns family-local discovery inside the routed roots

What changed is the boundary fidelity inside that shape:

- active root selection now respects `scoped_files` for config-derived code-family facts, not only Rust source-file checks
- runtime-only route construction for self-tests is still present, but it no longer leaks into the production runtime surface
- `test_support` went back to a more clearly generic API

The remaining architectural debt is explicit now: if routed-family tests are supposed to stop constructing routes locally, that needs a broader shared test-harness design rather than one more family-local patch.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/README.md`
- `.plans/todo/checks/rs/code.md`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/discover.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/attrs.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/fs_visitors.rs`
- `apps/guardrail3/crates/app/rs/runtime.rs`
- `apps/guardrail3/crates/app/rs/runtime_tests.rs`
- attack findings from explorer agents in this session (`Laplace`, `Bohr`, plus earlier `Jason`, `Faraday`, `Singer`)
- prior worklogs:
  - `.worklogs/2026-03-27-132300-start-rs-code-stabilization.md`
  - `.worklogs/2026-03-27-140627-rs-code-rs-test16-slice.md`
  - `.worklogs/2026-03-27-142821-rs-code-parse-shape-cleanup.md`
  - `.worklogs/2026-03-27-144818-finish-rs-code-stabilization.md`

## Open Questions / Future Considerations
- `rs/code` still uses a test-only family-local route harness in `crates/runtime/src/lib.rs`. That is no longer part of the production surface, but if the long-term contract is “no family-local test route construction at all,” the next step is a broader shared routed-family harness design.
- Repo-wide `RS-CODE-24` counts dropped materially, but there are still many real `#[path]` hits in the repo. That remaining bucket should be sampled before declaring the rule fully debugged across all families.
- `RS-CODE-14` remains the dominant repo-wide finding by far. The attack slice fixed concrete parser misses, but not the larger question of whether the remaining count is all real or still contains cluster-shaped false positives.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/discover.rs` — test-path classification, including `_tests/` handling
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs` — active-root scoping for config-derived code-family facts
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs` — production runtime entrypoint plus test-only route harness
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/fs_visitors.rs` — extracted std::fs visitor logic including alias handling
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_24_path_attr.rs` — canonical `#[path]` exemption logic
- `apps/guardrail3/crates/app/rs/families/code/test_support/src/lib.rs` — generic tempdir/fixture helpers after the `temp_root` tightening
- `apps/guardrail3/crates/app/rs/runtime.rs` — top-level fail-closed runtime behavior for `--family code`
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — runtime-level malformed-config and scoped-files regressions
- `apps/guardrail3/crates/app/rs/families/code/README.md` — current family contract after attack fixes
- `.plans/todo/checks/rs/code.md` — live rule inventory and `RS-CODE-24` exemption wording
- `.worklogs/2026-03-27-144818-finish-rs-code-stabilization.md` — prior stabilization checkpoint

## Next Steps / Continuation Plan
1. Sample the remaining repo-wide `RS-CODE-24` and `RS-CODE-14` hits to decide whether more attack fixes are needed or whether the remaining inventory is now mostly real.
2. Decide whether to tackle the broader routed-family test harness debt now or leave it as a shared Rust architecture cleanup after more families are stabilized.
3. Move to the next high-value family stabilization target (`release` is the most likely next candidate) only after this `rs/code` attack-fix checkpoint is committed and pushed.
