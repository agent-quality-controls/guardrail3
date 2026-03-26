# Fix Arch Placement Edge Cases

**Date:** 2026-03-26 12:19
**Scope:** `apps/guardrail3/crates/app/rs/placement/src/{classification.rs,overlap.rs,roots.rs}`, `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/adapters/inbound/cli/init.rs`, `apps/guardrail3/crates/app/commands/src/messages.rs`, `apps/guardrail3/crates/domain/modules/guide.rs`, `apps/guardrail3/crates/app/rs/families/arch/src/test_support.rs`, targeted `arch` sidecar tests

## Summary
Fixed the first batch of real `rs/arch` contract bugs after the rewrite: excluded validation roots no longer self-classify as live architecture, overlap detection no longer double-reports ambiguous roots, and forbidden scoped `arch` config is now still surfaced even when global `arch = false`. I also tightened auxiliary metadata handling so only ungoverned roots require Cargo metadata parsing, invalid `arch_role` values fail closed, and the package-side `libarch` owner is now explicit in init/help/config surfaces.

## Context & Problem
The review after the shared-placement rewrite surfaced six issues. The highest-signal ones were behavior bugs rather than documentation drift:

- validating inside `tests/fixtures`, `tests/snapshots`, `target`, or `.claude/worktrees` still treated the local root `Cargo.toml` as live architecture
- `RS-ARCH-04` was still triggered by ambiguous cross-zone roots, which duplicated `RS-ARCH-01` / `RS-ARCH-03`
- scoped `arch` violations disappeared when `[rust.checks].arch = false` because runtime suppressed the entire family
- auxiliary-role parsing was broader than needed and invalid `arch_role` values failed open

There was also a user-surface inconsistency: `libarch` was live in config and ownership logic but still missing from init/config examples for packages.

## Decisions Made

### Treat excluded validation roots as entirely out-of-domain
- **Chose:** Short-circuit placement collection to an empty result when the validation root path itself lives under `tests/fixtures`, `tests/snapshots`, `target`, or `.claude/worktrees`.
- **Why:** The family contract says those subtrees are not live architecture at all. Merely filtering descendant relative paths was not enough because the root `""` Cargo manifest still slipped through.
- **Alternatives considered:**
  - Special-case only the root `Cargo.toml` insertion and keep scanning descendants — rejected because validating inside an excluded subtree should not partially resurrect any of its inner Cargo roots.
  - Leave the logic as-is and narrow the README wording — rejected because the runtime behavior was clearly wrong, not the contract.

### Keep ambiguous roots out of RS-ARCH-04
- **Chose:** Build overlap pairs only from roots classified as exactly `App` and exactly `Package`.
- **Why:** Cross-zone nested roots already belong to `RS-ARCH-01` and `RS-ARCH-03`. Letting ambiguous roots participate in overlap detection created duplicate reporting and muddied the rule split.
- **Alternatives considered:**
  - Keep overlap based on raw candidate presence — rejected because it recreates the duplicate-emission bug the review found.
  - Delete `RS-ARCH-04` immediately — rejected because the rule still expresses a valid contract if truly app-owned and package-owned roots ever overlap.

### Let scoped-arch violations survive global arch disablement
- **Chose:** Special-case `RustValidateFamily::Arch` in runtime selection so the family still runs when scoped app/package `arch` config is present, even if global `arch` is false.
- **Why:** Forbidden scoped `arch` config is an unconditional contract error. A global family toggle must not be able to hide that invalid config.
- **Alternatives considered:**
  - Always run `arch` regardless of config — rejected because it would re-enable all arch findings even when the user intentionally turned the family off.
  - Move scoped-config validation entirely out of the family into generic config parsing — rejected for this checkpoint because the current rule ownership already exists and only needed runtime visibility restored.

### Parse Cargo metadata only for ungoverned roots, and fail closed on invalid arch_role
- **Chose:** Only resolve auxiliary metadata for roots that do not already have an `apps/*` or `packages/*` ownership candidate, and turn malformed/unknown `arch_role` values into required-input failures.
- **Why:** Path classification is already sufficient for governed roots. The README explicitly narrowed Cargo parsing failures to cases where auxiliary metadata matters. Unknown `arch_role` strings silently becoming `None` was a real fail-open bug.
- **Alternatives considered:**
  - Keep parsing every eligible `Cargo.toml` — rejected because it widens failure surface beyond the documented contract.
  - Ignore bad `arch_role` values as if absent — rejected because it hides broken explicit policy.

### Make libarch explicit in config surfaces, not validate-family lists
- **Chose:** Add `libarch` to init/help/config examples and generated package config, but keep it out of the `rs validate --family` family list because it is not a runnable family.
- **Why:** The user-surface bug was about config/ownership clarity, not about creating a fake validate family.
- **Alternatives considered:**
  - Add `libarch` to the validate family list — rejected because it would advertise a CLI surface that does not exist.
  - Leave `libarch` implicit for packages — rejected because package-side ownership was still hidden in generated config.

## Architectural Notes
The placement crate is still the right owner for all root-discovery and classification policy. These fixes reinforce that boundary rather than poking holes in it:

- root exclusion now treats validation-root context as part of placement policy
- auxiliary metadata parsing is now conditional on placement state, not unconditional Cargo semantics
- overlap detection now respects final classification rather than raw candidate bags

Runtime remains responsible only for family selection and result filtering. The `arch` special case is intentionally narrow: runtime now preserves the family when there is forbidden scoped `arch` config to report, but still allows normal `arch = false` behavior to suppress placement reporting.

## Information Sources
- Review findings from this session, especially the concrete file/line critiques for:
  - `placement/src/roots.rs`
  - `placement/src/overlap.rs`
  - `app/rs/runtime.rs`
  - `app/commands/src/messages.rs`
  - `adapters/inbound/cli/init.rs`
- Prior arch rewrite worklog:
  - `.worklogs/2026-03-26-115735-rewrite-arch-family-global-placement.md`
- Live contract reference:
  - `apps/guardrail3/crates/app/rs/families/arch/README.md`

## Open Questions / Future Considerations
- `RS-ARCH-04` is now much narrower, and in the current segment-based model it may rarely or never emit on realistic nested cross-zone examples because those become ambiguous first. That is acceptable for now, but the rule’s long-term value should be revisited once the broader architecture contracts settle.
- The arch test helper still offers convenience assertions that collapse findings to file-path sets. I added targeted case coverage this round, but deeper duplicate-emission assertions across the whole family are still worth tightening later.
- Product-level `cargo run ... rs validate` proof was not rerun in this batch because unrelated branch churn still exists outside the arch/runtime/init slice. The crate-level proofs touched here are green.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/placement/src/roots.rs` — validation-root exclusion, conditional auxiliary parsing, invalid `arch_role` failure behavior
- `apps/guardrail3/crates/app/rs/placement/src/overlap.rs` — overlap restricted to final `App`/`Package` classifications
- `apps/guardrail3/crates/app/rs/runtime.rs` — `arch` selection special case for forbidden scoped config
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — runtime proof that scoped `arch` still emits with global `arch = false`
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs` — package-side `libarch` generation and init proof
- `apps/guardrail3/crates/app/commands/src/messages.rs` — config/help text updated without advertising a fake validate family
- `apps/guardrail3/crates/app/rs/families/arch/src/rs_arch_02_no_misplaced_roots_tests/false_positives.rs` — excluded-root and contextual app-root proofs
- `apps/guardrail3/crates/app/rs/families/arch/src/rs_arch_07_required_inputs_fail_closed_tests/fail_closed.rs` — malformed auxiliary metadata and conditional Cargo parsing proofs
- `.worklogs/2026-03-26-115735-rewrite-arch-family-global-placement.md` — prior rewrite that this bugfix batch corrects

## Next Steps / Continuation Plan
1. Revisit the remaining valid review items that were not fully closed here:
   - duplicate-emission-proof gaps in `arch` helpers/tests
   - broader user-surface coherence around `libarch` outside the files touched here
2. If the branch is stabilized enough for product proof, rerun:
   - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate . --family arch --format json`
   and verify excluded roots stay absent while auxiliary roots appear as `RS-ARCH-08`.
3. Consider tightening `arch` sidecar assertions with explicit count helpers so future duplicate-emission regressions are caught directly rather than inferred from file-path sets.
4. Keep future placement-policy changes in `guardrail3-app-rs-placement`; do not reintroduce family-local discovery or overlap logic under `arch`.
