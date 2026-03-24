# Libarch Agent Brief

This is the droppable handoff file for the `rs/libarch` implementation and hardening pass.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `.plans/todo/checks/rs/libarch.md`
6. `.plans/todo/checks/rs/hexarch.md`
7. `.plans/todo/checks/rs/deps.md`
8. `.plans/todo/checks/rs/code.md`

## Primary Code

The family does not exist yet.

Target folder:
- `apps/guardrail3/crates/app/rs/checks/rs/libarch/`

Required family shape:
- `mod.rs`
- `facts.rs`
- `inputs.rs`
- one production file per `RS-LIBARCH-*` rule
- one rule-specific `*_tests/` directory per rule

## Legacy Seed Material

There is no old `libarch` family to port.

Use these as structural seed material only:
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/`
- `apps/guardrail3/crates/app/rs/checks/rs/deps/`
- `apps/guardrail3/crates/app/rs/checks/rs/code/`

Do not copy hexarch semantics wholesale. `libarch` is smaller and library-specific.

## Family Contract

`rs/libarch` owns Rust library architecture escalation and layered library workspace shape.

Two valid modes exist:

1. flat library
2. layered library workspace

Flat library is allowed only while the package stays under all thresholds.

Layered mode is required once any threshold is crossed.

Layered shape:
- package root `Cargo.toml` is workspace root
- package root is also the root facade package
- root `src/lib.rs` exists and owns the package-level public surface
- `crates/api`
- `crates/core`
- optional `crates/infra`

It does not apply to:
- binary-only packages
- workspaces with no root/package library target

## Escalation Contract

Measurements are per crate from that crate’s own `Cargo.toml`.

Thresholds:
- direct dependencies `> 12`
- module depth `> 3`
- sibling subdirectories in one Rust source dir `> 4`
- sibling `.rs` files in one Rust source dir `> 6`

Important split:
- `rs/code` and `rs/deps` still own the underlying quality/sprawl rules
- `rs/libarch` owns when that complexity forces architecture
- the root package facade must export from `api`, not directly from `core` or `infra`

## Fail-Closed Contract

Malformed required inputs must not silently downgrade a too-large library into “flat allowed”.

Required inputs include:
- readable package/workspace `Cargo.toml` files
- readable crate directory structure
- readable dependency-edge facts
- readable root facade source when facade/export rules apply

## Planned Rules

- `RS-LIBARCH-01` escalation required
- `RS-LIBARCH-02` layered root is workspace + facade package
- `RS-LIBARCH-03` `crates/` exists
- `RS-LIBARCH-04` exact layered crate set
- `RS-LIBARCH-05` workspace members match layered crate dirs
- `RS-LIBARCH-06` no extra workspace members outside layered boundary
- `RS-LIBARCH-07` `core` must not depend on `api`
- `RS-LIBARCH-08` `core` must not depend on `infra`
- `RS-LIBARCH-09` `api` must not depend on `infra`
- `RS-LIBARCH-10` `infra` must not become public package surface
- `RS-LIBARCH-11` root facade exports from `api`

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
- threshold crossing just over each cap
- flat libraries that should still remain allowed
- layered root missing workspace/facade semantics
- wrong crate set under `crates/`
- workspace member drift
- dependency-direction violations between `api` / `core` / `infra`
- root facade re-exporting from `core` or `infra`
- malformed Cargo/workspace inputs that must fail closed

## Mission

Implement and harden `rs/libarch`.

Required outcomes:
- create the family in the new checker architecture
- use one rule file per rule and one rule-specific `*_tests/` directory per rule
- add golden coverage for every rule
- add at least one real attack-vector test for every rule
- use exact owned hit/non-hit assertions
- reuse hexarch-style workspace/member/dependency facts where appropriate without inheriting hexarch-only semantics
- update `.plans/todo/checks/rs/libarch.md` with:
  - implementation notes
  - gaps closed
  - remaining gaps

## Do Not

- collapse `libarch` back into `code` or `deps`
- invent extra optional layers beyond `infra`
- enforce every library as multi-crate from day one
- use intra-crate AST/module dependency checking instead of crate boundaries
- leave the family on grouped tests or grouped production files

## Done Means

The pass is not done until:

- `rs/libarch` exists under the new architecture
- every planned rule has its own production file
- every rule has a rule-specific `*_tests/` directory
- every rule has golden coverage
- every rule has at least one real attack-vector test
- exact-result assertions replace loose presence checks
- `libarch.md` reflects what changed and what remains

## Suggested Start Order

1. read `libarch.md` fully and map the 11 planned rules into likely fact/input shapes
2. inspect `hexarch`, `deps`, and `code` for reusable fact and dependency-edge patterns
3. build `facts.rs` and `inputs.rs` first, especially escalation measurements and layered-workspace facts
4. implement and test the shape rules first:
   - `RS-LIBARCH-01`
   - `RS-LIBARCH-02`
   - `RS-LIBARCH-03`
   - `RS-LIBARCH-04`
   - `RS-LIBARCH-05`
5. then implement and test dependency/facade rules:
   - `RS-LIBARCH-07`
   - `RS-LIBARCH-08`
   - `RS-LIBARCH-09`
   - `RS-LIBARCH-10`
   - `RS-LIBARCH-11`
