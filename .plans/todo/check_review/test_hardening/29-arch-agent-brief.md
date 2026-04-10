# Arch Agent Brief

> Historical note: this file is superseded. For the current `RS-ARCH` contract, use:
> - `apps/guardrail3/crates/app/rs/families/arch/README.md`
> - `apps/guardrail3/crates/app/rs/README.md`
> - `.plans/todo/checks/rs/arch.md`
>
> This brief predates the current family layout and rule inventory.

This is the droppable handoff file for the `rs/arch` implementation and hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/arch.md`
6. `.plans/todo/checks/rs/hexarch.md`
8. `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`

## Primary Code

The family does not exist yet.

Target folder:
- `apps/guardrail3/crates/app/rs/checks/rs/arch/`

Required family shape:
- `mod.rs`
- `facts.rs`
- `inputs.rs`
- one production file per `RS-ARCH-*` rule
- one rule-specific `*_tests/` directory per rule

## Legacy Seed Material

There is no old `arch` family to port.

Use these as structural seed material only:
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/`
- `apps/guardrail3/crates/domain/project_tree.rs`

Do not copy app-internal `hexarch` semantics into `arch`.

## Family Contract

`rs/arch` owns repo-global Rust root placement and architecture ownership.

It is deliberately narrow.

It owns only:
- discovery/classification of all Rust `Cargo.toml` roots
- placement under architecture zones
- root-to-family ownership
- overlap/nesting legality between architecture zones

It does not own:
- app-internal hex structure
- package-internal layered structure
- workspace-member semantics inside an app or package boundary
- generic Cargo manifest policy

## Core Model

Every discovered Rust `Cargo.toml` root must classify as exactly one of:

1. `app`
   - under `apps/*`
2. `package`
   - under `packages/*`
3. `other`
   - anywhere else

Ownership:
- `app` roots are candidates for `rs/hexarch`
- `package` roots are candidates for `rs/arch`
- `other` roots are misplaced when Rust architecture enforcement is active

`rs/arch` is the only family that should emit repo-global misplaced-root findings.

## Conditional Reporting Contract

Discovery/classification always happens.

Reporting is conditional:
- if `hexarch` and/or `arch` is enabled, misplaced `other` roots are errors
- if both are disabled, no misplaced-root finding is emitted

`rs/arch` must not fail open by silently skipping discovery when family enablement changes.

## Fail-Closed Contract

Malformed required inputs must not silently suppress misplaced-root findings.

Required inputs include:
- readable `Cargo.toml` discovery
- readable guardrail config
- readable directory structure for zone classification

## Planned Rules

- `RS-ARCH-01` root classification is unambiguous
- `RS-ARCH-02` no misplaced Rust roots
- `RS-ARCH-03` no dual ownership
- `RS-ARCH-04` no illegal zone overlap
- `RS-ARCH-05` scoped `arch` config is forbidden
- `RS-ARCH-06` owner-family enablement is coherent
- `RS-ARCH-07` required inputs fail closed

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
- app root correctly under `apps/*`
- package root correctly under `packages/*`
- stray Rust root under `other`
- ambiguous or overlapping path ownership
- illegal nesting between app/package architecture boundaries
- config states where only `hexarch` is enabled
- config states where only `arch` is enabled
- config states where both are enabled
- config states where both are disabled
- malformed placement/config inputs that must fail closed

## Mission

Implement and harden `rs/arch`.

Required outcomes:
- create the family in the new checker architecture
- use one rule file per rule and one rule-specific `*_tests/` directory per rule
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- introduce shared placement facts reusable by `hexarch` and `arch`
- update `.plans/todo/checks/rs/arch.md` with:
  - implementation notes
  - gaps closed
  - remaining gaps

## Do Not

- duplicate misplaced-root rule bodies in `hexarch` and `arch`
- move repo-global placement into `cargo`
- let enablement change discovery
- collapse this family back into `hexarch`
- use grouped tests or grouped production files

## Done Means

The pass is not done until:

- `rs/arch` exists under the new architecture
- every planned rule has its own production file
- every rule has a rule-specific `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- misplaced-root signaling is owned only by `rs/arch`
- `arch.md` reflects what changed and what remains

## Suggested Start Order

1. read `arch.md` fully and map the 5 planned rules into likely fact/input shapes
2. inspect `hexarch` and `arch` for reusable root/workspace/dependency facts
3. build `facts.rs` and `inputs.rs` first, especially root classification and ownership facts
4. implement and test the classification/placement rules first:
   - `RS-ARCH-01`
   - `RS-ARCH-02`
   - `RS-ARCH-03`
5. then implement and test overlap/coherence rules:
   - `RS-ARCH-04`
   - `RS-ARCH-05`
