# Toolchain Agent Brief

This is the droppable handoff file for the `rs/toolchain` hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/toolchain.md`
6. `.plans/todo/checks/rs/cargo.md`

## Primary Code

- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/`

Important family files:
- `mod.rs`
- `discover.rs`
- `facts.rs`
- `inputs.rs`

Rules:
- `rs_toolchain_01_exists.rs`
- `rs_toolchain_config_01_channel_components.rs`
- `rs_toolchain_config_02_msrv_consistency.rs`
- `rs_toolchain_04_legacy_file.rs`

## Legacy Seed Material

Use these as seed material only:

- `apps/guardrail3/crates/app/rs/validate/config_files.rs`
- `apps/guardrail3/crates/app/rs/validate/toolchain_check.rs`

Do not port old compatibility shortcuts mechanically.

## Family Contract

`rs/toolchain` is intentionally a repository-root family.

It owns:
- root toolchain file presence
- root channel and component policy
- root MSRV consistency against root `Cargo.toml`
- legacy `rust-toolchain` compatibility and ambiguity

It does not own:
- per-workspace or per-package local toolchain roots
- inferred toolchain policy from nested manifests

The family is about one repo toolchain contract.

## Fail-Closed Contract

Required inputs include:
- root `rust-toolchain.toml` or legacy `rust-toolchain`
- root `Cargo.toml` when MSRV consistency is needed

Malformed required inputs must not silently weaken enforcement.

Important policy details:
- plain `stable` is accepted
- pinned stable versions are informationally tolerated
- `beta` is an error
- `nightly` is an error
- pinned-nightly forms are errors
- components must include `clippy` and `rustfmt`

## Known Live Gaps

The family is implemented but still structurally behind the test standard.

Highest-signal remaining gaps:
- every rule still uses `*_tests.rs`, not `*_tests/`
- channel normalization and pinned-nightly/beta coverage needs more adversarial depth
- dual-file ambiguity needs exact-result coverage
- malformed Cargo MSRV inputs need explicit fail-closed verification

## Current Test Shape

The family still uses:
- `rs_toolchain_*_tests.rs`

This pass should move it to:
- one rule-specific `*_tests/` directory per rule
- one test file per attack vector

## Required Attack Classes

Every rule should move toward:

1. golden pass
2. attack-vector test
3. exact owned hit set
4. exact owned non-hit set
5. false-positive control
6. fail-closed coverage where applicable
7. exact severity assertions

Highest-signal attacks for this family:
- missing root toolchain file
- wrong channel and missing components
- pinned stable vs nightly/beta distinctions
- legacy `rust-toolchain` ambiguity
- toolchain/Cargo MSRV mismatch
- malformed root Cargo or toolchain files

## Mission

Harden `rs/toolchain` only.

Required outcomes:
- verify root-only ownership against `toolchain.md`
- convert every rule to a rule-specific `*_tests/` directory
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- fix real semantic bugs you find
- update `.plans/todo/checks/rs/toolchain.md` with:
  - gaps closed
  - gaps remaining
  - policy questions, if any

## Do Not

- reinterpret the family as multi-root
- infer policy from nested toolchain files
- leave tests as `*_tests.rs`
- collapse channel distinctions just to simplify tests

## Done Means

The pass is not done until:

- every toolchain rule has a rule-specific `*_tests/` directory
- every toolchain rule has golden coverage
- every toolchain rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- semantic bugs are fixed or written down explicitly
- `toolchain.md` reflects what changed and what remains

## Suggested Start Order

1. read `toolchain.md` and map all 4 rules to current files
2. audit `discover.rs` / `facts.rs` / `inputs.rs` for root-only ownership
3. convert all `rs_toolchain_*_tests.rs` files to `*_tests/`
4. harden the highest-risk rules first:
   - `RS-TOOLCHAIN-CONFIG-01`
   - `RS-TOOLCHAIN-CONFIG-02`
   - `RS-TOOLCHAIN-04`
5. finish malformed-input and exact-severity coverage before stopping
