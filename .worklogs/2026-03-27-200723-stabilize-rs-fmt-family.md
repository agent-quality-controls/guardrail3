# Stabilize RS-FMT Family

**Date:** 2026-03-27 20:07
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/fmt/**`

## Summary
Migrated `RS-FMT` from the old single-crate layout into the stabilized family workspace shape with `crates/runtime`, `crates/assertions`, and `test_support`. Fixed the family’s self-hosting gaps so it is clean under `RS-ARCH`, `RS-TEST`, and `RS-FMT`, and tightened `RS-FMT-CONFIG-03` / `RS-FMT-CONFIG-04` so malformed root toolchain or Cargo inputs fail closed instead of silently disabling those checks.

## Context & Problem
The handoff at `.plans/todo/family-stabilization-handoffs/fmt.md` described a small Rust family that was structurally behind the newer `arch` / `cargo` / `code` / `test` families. The existing state matched that snapshot:

- `families/fmt` was still a single crate with flat `*_tests.rs` sidecars
- `RS-TEST` reported the old sidecar shape as 16 errors
- `RS-FMT` on the family root failed because there was no family-local `rustfmt.toml`
- `RS-FMT-CONFIG-03` and `RS-FMT-CONFIG-04` still silently downgraded missing or malformed root `rust-toolchain.toml` / `Cargo.toml` into no finding

The goal was to make the family structurally self-hosted and then attack the detector itself rather than treating wider repo formatting debt as the task.

## Decisions Made

### Convert `fmt` into the same workspace family shape as the stabilized specimens
- **Chose:** Move the implementation into `crates/runtime`, add sibling `crates/assertions` and `test_support`, add a family README, and rewire the top-level workspace to point at `crates/runtime`.
- **Why:** That is the established self-hosted family shape in this repo, and it is what `RS-TEST` expects.
- **Alternatives considered:**
  - Keep `fmt` as a single crate and only rename sidecars — rejected because the handoff explicitly called for the stabilized family structure.
  - Move only runtime code and skip `assertions` / `test_support` — rejected because the family would still not match the intended self-hosted pattern.

### Make `RS-FMT-CONFIG-03` and `RS-FMT-CONFIG-04` fail closed on active secondary inputs
- **Chose:** Add explicit Cargo/toolchain state in `facts.rs` and have the rules emit errors when required root inputs are missing, malformed, or missing the specific value they need.
- **Why:** The plan says malformed root `Cargo.toml` / `rust-toolchain.toml` must not silently disable those rules.
- **Alternatives considered:**
  - Keep `Option<String>` facts and rely on tests for the happy path only — rejected because it preserved the false-green gap.
  - Add a separate family-wide input failure rule — rejected because the current plan assigns those failures to `RS-FMT-CONFIG-03` and `RS-FMT-CONFIG-04`.

### Tighten `RS-FMT-01` so a compliant family root is quiet
- **Chose:** Stop inventorying the “config exists” success path and keep `RS-FMT-01` as a missing-root error only.
- **Why:** The handoff’s expected end state for the family root is zero `arch` / `test` / `fmt` findings under the verification commands. Keeping success inventory on `RS-FMT-01` made that impossible.
- **Alternatives considered:**
  - Leave the inventory behavior and accept a permanent info result — rejected because it contradicts the handoff’s end-state target.
  - Special-case the family root only — rejected because it would make the rule behavior context-sensitive for the validator itself.

### Refactor sidecar tests to obey the stricter `RS-TEST` boundary rules
- **Chose:** Keep all sidecars rule-local, but move test helpers into their owning production modules and use the `test_support` crate under the allowed `test_support` alias.
- **Why:** `RS-TEST` forbids sidecars from reaching into sibling runtime modules or importing local helper crates through ad hoc paths.
- **Alternatives considered:**
  - Keep direct imports from `facts`, `inputs`, and `check_test_tree` — rejected because `RS-TEST` flagged those as boundary escapes.
  - Move the new attack cases into broader integration tests — rejected because the family pattern here is rule-specific sidecars, not family-wide integration harnesses.

## Architectural Notes
`fmt` remains a repo-root family. This migration did not add a routed family input or reintroduce family-local root placement; the family still consumes only `ProjectTree` and performs formatting-policy discovery inside that repo snapshot. The main architectural change is packaging and test organization:

- `crates/runtime` owns facts, typed inputs, rule orchestration, and rule-local test helpers
- `crates/assertions` owns reusable result-shape assertions for `RS-FMT-*`
- `test_support` owns generic tempdir / file-writing / `ProjectTree` walking helpers

This keeps `fmt` aligned with the stabilized family pattern without widening its runtime boundary.

## Information Sources
- `.plans/todo/family-stabilization-handoffs/fmt.md`
- `.plans/todo/checks/rs/fmt.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `apps/guardrail3/crates/app/rs/families/test/**` as the packaging and sidecar-shape specimen
- recent worklogs:
  - `.worklogs/2026-03-27-192542-add-toolchain-and-fmt-handoffs.md`
  - `.worklogs/2026-03-27-183815-merge-rs-code-28-into-27.md`
  - `.worklogs/2026-03-27-174353-rs-code-attack-round-2.md`

## Open Questions / Future Considerations
- `RS-FMT-01` now treats “config exists” as silent success to match the handoff’s zero-finding end state. If the project still wants positive inventory for compliant roots, that should be reconciled explicitly in the plan and README rather than drifting back in through implementation.
- The family now has concrete fail-closed coverage for root Cargo/toolchain inputs, but further adversarial review could still probe parser behavior around malformed root `rustfmt.toml` values that are syntactically TOML yet semantically odd.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/fmt/README.md` — current family contract and self-hosted shape
- `apps/guardrail3/crates/app/rs/families/fmt/Cargo.toml` — nested family workspace root
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/lib.rs` — family orchestrator and test-only entrypoints
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/facts.rs` — root config discovery plus Cargo/toolchain state collection
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/inputs.rs` — typed family inputs
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_01_exists.rs` — missing-root rule, now quiet on success
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_config_03_nightly_keys_on_stable.rs` — fail-closed nightly-key handling
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/rs_fmt_config_04_edition_mismatch.rs` — fail-closed Cargo edition handling
- `apps/guardrail3/crates/app/rs/families/fmt/crates/assertions/src/lib.rs` — reusable assertion surface for the rule sidecars
- `apps/guardrail3/crates/app/rs/families/fmt/test_support/src/lib.rs` — allowed generic sidecar helpers

## Next Steps / Continuation Plan
1. If `RS-TOOLCHAIN` is next, use this `fmt` family as the packaging and `RS-TEST` boundary specimen: nested workspace, rule-local helpers, assertions crate, and `test_support` alias usage.
2. If future `fmt` work continues in attack-review mode, start by re-reading `README.md`, `facts.rs`, `rs_fmt_config_03_nightly_keys_on_stable.rs`, and `rs_fmt_config_04_edition_mismatch.rs`, then compare any new findings against `.plans/todo/checks/rs/fmt.md`.
3. If the project wants positive inventory back for `RS-FMT-01`, resolve that policy in the plan first, then update the rule, README, and sidecar assertions together so self-validation expectations stay coherent.
