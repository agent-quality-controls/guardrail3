# Arch Family Rewrite Agent Brief

> Historical note: this file is superseded. For the current `RS-ARCH` contract, use:
> - `apps/guardrail3/crates/app/rs/families/arch/README.md`
> - `apps/guardrail3/crates/app/rs/README.md`
> - `.plans/todo/checks/rs/arch.md`
>
> This brief reflects an older rewrite phase and may contain stale rule counts, paths, or family shape.

This is the current droppable handoff file for rewriting and hardening the live `rs/arch` family.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/check_review/test_hardening/00-shared-test-story.md`
4. `.plans/todo/check_review/test_hardening/99-family-agent-playbook.md`
5. `apps/guardrail3/crates/app/rs/families/arch/README.md`
6. `.plans/todo/checks/rs/arch.md`
7. `.plans/todo/checks/rs/hexarch.md`
8. `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`

## Source Of Truth

The final contract is:

- `apps/guardrail3/crates/app/rs/families/arch/README.md`

Treat that README as authoritative.

If the live family disagrees with the README:

- the README wins

Do not preserve old behavior just because it already exists.

## Primary Code

The live family is here:

- `apps/guardrail3/crates/app/rs/families/arch/`

Important files:

- `apps/guardrail3/crates/app/rs/families/arch/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/arch/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/arch/src/inputs.rs`
- `apps/guardrail3/crates/app/rs/families/arch/src/rust_root_placement.rs`

Related live surfaces:

- `apps/guardrail3/crates/app/rs/runtime.rs`
- `apps/guardrail3/crates/domain/config/types.rs`
- `apps/guardrail3/crates/domain/report/mod.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/cli.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs`
- `apps/guardrail3/crates/app/commands/src/messages.rs`
- `apps/guardrail3/crates/app/rs/families/hexarch/`

## Goal

Make the live `rs/arch` family match the README exactly.

That includes:

- owned surface
- live-root discovery domain
- global-only config model
- shared placement-substrate target
- per-rule detection logic
- fail-closed behavior
- runtime/user-surface expectations

## Required Outcomes

### 1. Live-root discovery is correct

The family must judge eligible live Rust roots only.

It must not treat these as live architecture:

- `tests/fixtures/**`
- `tests/snapshots/**`
- `target/**`

If fixture manifests currently affect `RS-ARCH`, that is a bug and must be fixed.

### 2. `arch` is truly global-only

The only supported config surface is:

- `[rust.checks].arch`

Scoped `arch` config under app/package sections must not remain as dead accepted surface.

The implementation must make runtime, config, generated config, and help agree on that.

### 3. Placement substrate is not private drift

The README target says root placement is shared Rust architecture substrate, not private `arch`-only logic.

If the implementation stops short of the final shared crate extraction, the code still must move decisively toward one reusable placement source of truth instead of leaving `arch` and `hexarch` with conflicting rediscovery logic.

### 4. Fail-closed behavior is real

Unreadable-present required inputs must not be treated as absent.

That applies at least to:

- eligible live `Cargo.toml`
- `guardrail3.toml`

### 5. Runtime/user-surface coverage exists

At least one runtime-surface test must prove:

- `--family arch` parses and survives selection
- runtime dispatch runs `arch`
- the report section name is `arch`
- generated config includes global `arch = true`
- generated config does not place `arch` under scoped app/package checks

## Rule Inventory

The README contract is:

- `RS-ARCH-01` root classification is unambiguous
- `RS-ARCH-02` no misplaced roots when architecture enforcement is active
- `RS-ARCH-03` no dual ownership
- `RS-ARCH-04` no illegal app/package overlap
- `RS-ARCH-05` global architecture enablement and ownership are coherent

Keep:

- one production file per rule
- one rule-specific `*_tests/` directory per rule

## Required Test Standard

Every rule should have:

1. golden pass
2. direct attack case
3. exact owned hit set
4. exact owned non-hit set
5. fail-closed coverage where applicable
6. exact severity assertions

In addition, the family must have runtime/product tests for the live user-facing `arch` surface.

## Specific Problems Already Known

These are not optional:

- fixture `Cargo.toml` files are currently discoverable as architecture roots
- `guardrail3.toml` unreadable-present state currently fails open
- config model exposes scoped `arch` even though runtime treats it as global-only
- `hexarch` still duplicates root discovery instead of consuming one shared placement source
- stale plan/handoff files still describe dead paths and pre-live status

## Documentation Follow-Through

After the code matches the README, update the stale planning/docs that still contradict live reality:

- `.plans/todo/checks/rs/arch.md`
- `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`
- `.plans/todo/check_review/test_hardening/29-arch-agent-brief.md`

Do not update those first.
The README and live implementation come first.

## Done Means

The work is done when:

- live `rs/arch` matches the README contract
- family-local tests cover all five rules to the stated standard
- runtime/user-surface tests cover live `arch` selection/report/config behavior
- fixture manifests no longer pollute live root placement
- scoped `arch` config is no longer dead or contradictory
- stale plan/docs are reconciled to the new live state
