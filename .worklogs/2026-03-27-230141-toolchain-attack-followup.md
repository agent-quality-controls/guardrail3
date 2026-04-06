# Toolchain Attack Followup

**Date:** 2026-03-27 23:01
**Scope:** `apps/guardrail3/crates/app/rs/families/toolchain/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/inputs.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_01_exists.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_01_exists_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_04_legacy_file.rs`

## Summary
Ran additional adversarial passes on the stabilized `RS-TOOLCHAIN` family and fixed more fail-open behavior. The main changes were: explicit invalid `[toolchain]` table handling, explicit invalid `Cargo.toml` `rust-version` type handling, and a small family-level `ProjectTree` smoke harness so discovery and cross-rule interaction are tested rather than only rule-local helper inputs.

## Context & Problem
After the initial stabilization commit, the family had good rule-local coverage, but further attacks still showed two trust gaps:

- `rust-toolchain.toml` with no usable `[toolchain]` table still only produced soft “missing channel/components” signals instead of an input-integrity failure.
- `Cargo.toml` `rust-version` that existed but was not a string collapsed into the same broad “not declared” or generic invalid-version bucket, which made discovery semantics less explicit than they should be.

There was also a structural testing gap: most tests instantiated `ToolchainRootInput` directly, which meant `discover.rs` and the orchestrator path could still drift without immediate coverage.

## Decisions Made

### Treat missing or non-table `[toolchain]` as an explicit `RS-TOOLCHAIN-CONFIG-01` input failure
- **Chose:** Add dedicated errors for missing `[toolchain]` and non-table `toolchain = ...` shapes.
- **Why:** A present but structurally invalid root toolchain file is not the same as “policy keys absent”; it is malformed active input and should fail closed.
- **Alternatives considered:**
  - Keep emitting “channel missing” and “components missing” warnings — rejected because that weakens malformed active input into a softer policy nudge.
  - Push this into `RS-TOOLCHAIN-01` instead — rejected because the file does exist; the structural content failure belongs with the channel/components rule.

### Distinguish invalid `rust-version` type from missing `rust-version`
- **Chose:** Track `cargo_rust_version_invalid` through facts and inputs, and let `RS-TOOLCHAIN-CONFIG-02` emit a dedicated error when the field exists but is not a string.
- **Why:** Missing metadata and malformed metadata are different states. The rule should not blur them because malformed declared MSRV is strictly worse than absent declared MSRV.
- **Alternatives considered:**
  - Keep treating both as “not declared” inventory — rejected because it hides an active manifest bug.
  - Infer invalidity only inside `RS-TOOLCHAIN-CONFIG-02` from the final string parse — rejected because the type information is lost once discovery collapses non-string values to `None`.

### Add family-level `ProjectTree` smoke tests
- **Chose:** Add a tiny `ProjectTree` builder and `crate::check(...)` entrypoint helper in `rs_toolchain_01_exists.rs`, then use that harness for a few cross-rule adversarial cases.
- **Why:** The family should not rely solely on direct rule-helper tests. Discovery plus orchestrator plumbing is small enough here that a few smoke tests materially increase trust.
- **Alternatives considered:**
  - Keep coverage purely rule-local — rejected because discovery bugs can still slip through.
  - Build a separate test-support crate — rejected because this family does not need that much shared machinery for three smoke cases.

### Give the family workspace its own lint contract
- **Chose:** Copy the local `workspace.lints` block into `families/toolchain/Cargo.toml`.
- **Why:** The family workspace should be directly testable on its own, especially while unrelated top-level workspace issues exist elsewhere in the repo.
- **Alternatives considered:**
  - Keep relying on the top-level workspace only — rejected because current repo state made local family verification unnecessarily brittle.
  - Remove `workspace = true` lint inheritance from the family crates — rejected because that would diverge from the established family pattern.

## Architectural Notes
The family still follows the same core architecture:

- `discover.rs` produces one root-level fact set
- `inputs.rs` projects a single `ToolchainRootInput`
- `lib.rs` fans the shared input into four pure rule modules

What changed is the fidelity of discovery state:

- root `Cargo.toml` presence is still explicit
- root `Cargo.toml` parse failure is still explicit
- root `Cargo.toml` `rust-version` type invalidity is now also explicit

The small `ProjectTree` test harness deliberately lives near the simplest rule entrypoint instead of becoming a new shared layer. That keeps the addition proportional to this family’s size.

## Information Sources
- `.plans/todo/checks/rs/toolchain.md`
- `apps/guardrail3/crates/domain/project-tree/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/*`
- `apps/guardrail3/crates/app/rs/families/deny/Cargo.toml` — used as the nearest specimen for family-local `workspace.lints`
- prior worklogs:
  - `.worklogs/2026-03-27-203046-stabilize-toolchain-family.md`
  - `.worklogs/2026-03-27-192542-add-toolchain-and-fmt-handoffs.md`

## Open Questions / Future Considerations
- Top-level cargo verification through `apps/guardrail3/Cargo.toml` is currently blocked by an unrelated nested-workspace issue in `families/deny`. This follow-up work therefore verified the family directly through `families/toolchain/Cargo.toml`.
- The family now has a few `ProjectTree`-level smoke tests, but not yet a broad matrix of end-to-end family cases. More of those would be useful if future attacks keep surfacing discovery bugs rather than pure rule bugs.
- The positive self-inventory outputs under `--inventory` remain expected behavior; they should not be confused with self-validation debt.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs` — discovery state, including `rust-version` type invalidity tracking
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/facts.rs` — new `cargo_rust_version_invalid` fact
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/inputs.rs` — projected input carrying the new invalidity state
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components.rs` — explicit invalid/missing `[toolchain]` table handling
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency.rs` — explicit invalid `rust-version` type handling
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_01_exists_tests/mod.rs` — family-level `ProjectTree` smoke cases
- `apps/guardrail3/crates/app/rs/families/toolchain/Cargo.toml` — family-local lint inheritance for direct verification
- `.worklogs/2026-03-27-203046-stabilize-toolchain-family.md` — prior stabilization and earlier attack-hardening context

## Next Steps / Continuation Plan
1. Commit this follow-up attack-hardening batch with only the toolchain-family files and this worklog.
2. After the commit, run another couple of adversarial rounds focused on edge interactions still not covered by the family-level smoke tests, especially mixed malformed modern+legacy surfaces and stable/unpinned behavior through discovery.
3. If those new rounds do not reveal another detector bug, stop expanding the family and move to the next handoff-safe family instead of inventing speculative complexity here.
