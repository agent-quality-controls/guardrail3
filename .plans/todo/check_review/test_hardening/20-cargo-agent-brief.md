# Cargo Agent Brief

This is the droppable handoff file for the `rs/cargo` reconciliation and hardening pass.

## Status

This pass has since been implemented in code.

Use `.plans/todo/checks/rs/cargo.md` as the live contract and current-status source of truth.

Treat the rest of this brief as historical implementation guidance for what this pass was meant to close.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/cargo.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/rs/cargo/`

Important family files:
- `mod.rs`
- `discover.rs`
- `facts.rs`
- `inputs.rs`
- `lint_support.rs`
- `test_support.rs`
- `discover_tests.rs`

Rules:
- `rs_cargo_config_01_workspace_lints.rs`
- `rs_cargo_config_02_lint_levels.rs`
- `rs_cargo_03_allow_inventory.rs`
- `rs_cargo_04_lint_inheritance.rs`
- `rs_cargo_config_04_workspace_metadata.rs`
- `rs_cargo_06_no_weakened_overrides.rs`
- `rs_cargo_config_05_priority_order.rs`
- `rs_cargo_config_06_resolver.rs`
- `rs_cargo_09_member_edition_drift.rs`
- `rs_cargo_10_missing_member_cargo.rs`
- `rs_cargo_config_07_disallowed_macros_deny.rs` (planned)
- `rs_cargo_12_unapproved_allow_entries.rs` (planned)
- `rs_cargo_13_member_local_allows_forbidden.rs` (planned)
- `rs_cargo_14_input_failures.rs` (planned)
- `rs_cargo_15_rust_version_policy.rs` (planned)

## Family State You Must Assume

The `cargo` family is implemented, but its current discovery is still under-reconciled and effectively root-only.

Do not assume the current orchestrator shape is correct just because rule logic exists.

The active plan is ahead of the implementation on purpose.

## Family Contract

`rs/cargo` must be a multi-root family.

Owned Rust policy roots:
- workspace roots
- standalone package roots that are not members of a workspace

For workspace roots:
- the root `Cargo.toml` owns workspace lint policy
- member manifests are checked relative to that workspace root

For standalone package roots:
- the package `Cargo.toml` itself owns lint policy
- there is no workspace/member split

This family must not collapse to repo-root-only behavior.

## Rule Applicability By Root Kind

### Policy-root rules

Apply to:
- workspace roots
- standalone package roots

Rules:
- `RS-CARGO-CONFIG-01`
- `RS-CARGO-CONFIG-02`
- `RS-CARGO-03`
- `RS-CARGO-CONFIG-04`
- `RS-CARGO-CONFIG-05`
- `RS-CARGO-15`

### Workspace-only rules

Apply only to owned workspace roots:
- `RS-CARGO-04`
- `RS-CARGO-06`
- `RS-CARGO-CONFIG-06`
- `RS-CARGO-09`
- `RS-CARGO-10`

Standalone packages must not be forced through workspace-member semantics they do not have.

### Input-failure rule

`RS-CARGO-14` is not a root-kind applicability rule.

It owns malformed required inputs for whichever rule/root combination is being evaluated:
- malformed owned policy-root `Cargo.toml`
- malformed member `Cargo.toml` for workspace-member checks
- malformed root-local `guardrail3.toml` when profile-sensitive expectations are needed

## Fail-Closed Contract

Required inputs:
- each owned policy-root `Cargo.toml`
- member `Cargo.toml` files for workspace-member rules
- `guardrail3.toml` only when profile-sensitive lint expectations matter

Malformed required inputs must surface explicitly.

They must not silently produce:
- “no workspace found”
- “no policy root found”
- or accidental repo-root-only behavior

The current code fails open here. The pass should harden against that.

## Cross-Family Dependencies

`rs/cargo` owns Cargo lint-policy configuration, including the manifest-side switches that make many `rs/clippy` bans enforceable.

It does not own:
- dependency direction
- dependency allowlists
- release metadata
- toolchain file content

Specifically relevant:
- `RS-CARGO-CONFIG-07` should require `clippy::disallowed_macros = "deny"` so `RS-CLIPPY-20` is not toothless
- `RS-CARGO-15` overlaps intentionally with `RS-TOOLCHAIN-CONFIG-02` around MSRV/toolchain compatibility

## Known Clean Gaps

These are already accepted as real family work:

- current discovery is still effectively repo-root-only
- malformed `guardrail3.toml` profile parsing fails open
- the family still uses old rule sidecars like `*_tests.rs`
- rule/test architecture still needs conversion to rule-specific `*_tests/` directories
- additional planned rules are not yet implemented:
  - `RS-CARGO-CONFIG-07`
  - `RS-CARGO-12`
  - `RS-CARGO-13`
  - `RS-CARGO-14`
  - `RS-CARGO-15`

## Main Mission

This family needs both architectural reconciliation and test hardening.

Required outcomes:
- verify the plan/code mismatch directly
- redesign discovery/facts/input shaping so owned policy roots are truly multi-root
- preserve rule purity; fix orchestrator/discovery, not by bloating rule inputs
- convert rule tests from `*_tests.rs` to rule-specific `*_tests/` directories
- harden rule coverage under the attack-vector model
- update `.plans/todo/checks/rs/cargo.md` with what was closed and what remains

## Highest-Risk Areas

Start by attacking these:

1. discovery / ownership
- multiple workspace roots
- standalone package roots outside workspaces
- nested roots and non-owned roots

2. fail-closed behavior
- malformed owned-root `Cargo.toml`
- malformed member `Cargo.toml`
- malformed `guardrail3.toml`

3. rule applicability by root kind
- workspace-only rules must not fire on standalone packages
- policy-root rules must work on both workspace and standalone roots

4. clippy-enforcement dependency
- ensure cargo policy truly enforces the manifest-side switches clippy needs

## Required Attack Classes

Every rule should move toward:

1. golden pass
2. multi-root attack
3. owned hit set
4. owned non-hit set
5. precedence / inheritance / shadowing
6. fail-closed coverage where applicable
7. exact severity assertions

For this family, the highest-signal attack classes are:
- multiple owned policy roots in one repo
- workspace roots plus standalone package roots in one repo
- malformed root and member manifests
- profile-sensitive policy behavior via `guardrail3.toml`
- inheritance and weakening attacks at member level
- non-owned root isolation

## Do Not

- keep repo-root-only discovery and just widen tests around it
- push oversized “whole workspace” bags into rule functions
- leave rule tests as `*_tests.rs`
- add grouped family tests
- silently change policy semantics to make current code pass

## Done Means

The pass is not done until:

- the family actually follows the multi-root policy-root model from `cargo.md`
- rule applicability by root kind is enforced correctly
- fail-closed behavior is explicit for required inputs
- every rule has a rule-specific `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- `cargo.md` is updated with closed and remaining gaps

## Suggested Start Order

1. read `cargo.md` and map current code against the intended multi-root contract
2. inspect `discover.rs`, `facts.rs`, and `inputs.rs` first; this is the most likely source of wrong architecture
3. write down the exact current root-only assumptions before editing
4. fix discovery/ownership model before trying to harden individual rules
5. convert rule sidecars from `*_tests.rs` to `*_tests/`
6. then harden the highest-signal rules first:
   - `RS-CARGO-CONFIG-01`
   - `RS-CARGO-04`
   - `RS-CARGO-06`
   - `RS-CARGO-CONFIG-06`
   - `RS-CARGO-10`
7. if time allows, implement and harden:
   - `RS-CARGO-CONFIG-07`
   - `RS-CARGO-12`
   - `RS-CARGO-13`
