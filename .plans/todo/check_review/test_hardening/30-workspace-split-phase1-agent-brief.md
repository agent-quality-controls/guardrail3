# Workspace Split Phase 1 Agent Brief

You own Phase 1 of the `apps/guardrail3` workspace/crate split.

This is not the whole refactor.
It is the first real substrate cut:

- workspace root
- thin root facade
- shared validation model
- shared project tree
- normalized outbound traits
- shared fs ownership

Do not jump ahead into family-crate splitting.

## Read First

Read these in order:

1. `AGENTS.md`
2. `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
3. `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md`
4. `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`

## Primary Code

Current root package:
- `apps/guardrail3/Cargo.toml`
- `apps/guardrail3/crates/lib.rs`

Primary shared/domain surfaces:
- `apps/guardrail3/crates/domain/report/mod.rs`
- `apps/guardrail3/crates/domain/config/types.rs`
- `apps/guardrail3/crates/domain/project_tree.rs`
- `apps/guardrail3/crates/ports/outbound/traits/mod.rs`
- `apps/guardrail3/crates/fs.rs`

Primary app/core surfaces:
- `apps/guardrail3/crates/app/core/project_walker.rs`
- `apps/guardrail3/crates/app/core/discover.rs`
- `apps/guardrail3/crates/app/core/project_map.rs`

Primary adapter surfaces:
- `apps/guardrail3/crates/adapters/outbound/tool-runner/mod.rs`
- `apps/guardrail3/crates/adapters/outbound/fs/mod.rs`
- `apps/guardrail3/crates/adapters/outbound/report/mod.rs`

Important runtime/CLI context:
- `apps/guardrail3/crates/main.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/cli.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/validate.rs`

## Phase 1 Contract

Your scope is only the Phase 1 substrate.

You are responsible for making these real members:

- `crates/domain/validation-model`
- `crates/domain/project-tree`
- `crates/ports/outbound/traits`
- `crates/adapters/outbound/tool-runner`
- `crates/shared/fs`

You may also thin the root facade and workspace root as needed to make those members real.

You do **not** own yet:

- `domain-config`
- `domain-report`
- `domain-modules`
- `app-core`
- `app-rs/runtime`
- `app-rs/generate`
- `app-rs/families/*`
- `app-hooks`
- inbound CLI promotion
- family crate splitting

## Concrete Owner Decisions

These are already frozen in the plan:

- pure Rust family identity:
  - crate: `crates/domain/validation-model`
  - file: `src/families.rs`
- portable filesystem record types:
  - crate: `crates/ports/outbound/traits`
  - file: `src/fs_types.rs`

Phase 1 must not invent different owners for those concepts.

## Required Outcomes

1. `apps/guardrail3/Cargo.toml` becomes a real workspace root.
2. The root facade package remains present but becomes thin.
3. `domain/validation-model` becomes real and owns only domain validation model concepts.
4. `domain/project-tree` becomes real.
5. `ports/outbound/traits` becomes real only after the trait API is normalized.
6. `shared/fs` becomes the real owner of shared fs behavior needed by later members.
7. No promoted Phase 1 crate is allowed to depend back on root-only `crate::fs`.

## Critical Invariants

### Validation model

`domain/validation-model` must own:
- pure Rust family identity
- domain-only validation model types

It must **not** own:
- `clap::ValueEnum`
- CLI parsing helpers
- report presentation helpers
- help text

### Traits

`ports/outbound/traits` must not freeze adapter-specific OS types into the shared API.

It must define portable fs record types in:
- `src/fs_types.rs`

Do not promote the current raw `std::fs::DirEntry` / `Metadata` coupling unchanged.

### Shared fs

Phase 1 is not done while any promoted Phase 1 substrate still depends on root `crate::fs`.

This includes:
- `app/core`
- outbound fs adapter
- Rust family code later

Your job is to create the clean shared owner, not just another forwarding layer.

### Root facade

The root facade stays only as the shipped product-entry adapter.
Do not use it to preserve bad internal ownership.

## Non-Goals

Do not try to solve these now:

- family crates
- typed applicability
- typed scope
- single Rust report owner
- single Rust write-set owner
- command/text ownership
- module registry ownership
- hook runtime ownership
- root test shrink implementation
- fixture split implementation

Those come later.

## Required Review Lenses

As you work, continuously check:

1. Did this new member remove a real monolith edge, or just move it?
2. Did this change freeze a bad ownership edge into a real crate boundary?
3. Did this change create a reverse dependency from shared/domain back into CLI/root?
4. Did this change leave root `crate::fs` on the hot path?

## Acceptance Criteria

Phase 1 is done only when:

1. `apps/guardrail3` is a real workspace.
2. The Phase 1 substrate members above are real members.
3. The root facade is thin.
4. `domain/validation-model` is domain-only and no longer drags CLI/report concerns with it.
5. `ports/outbound/traits` exposes portable fs types rather than raw adapter-specific OS types.
6. The Phase 1 substrate no longer depends on root `crate::fs`.
7. The CLI still builds through the root facade.
8. The refactor does not accidentally promote `domain-config` or `domain-report` early.

## Suggested Order

1. Convert `apps/guardrail3/Cargo.toml` to a real workspace root while preserving a thin root package.
2. Extract `crates/domain/validation-model`:
   - move pure family identity there
   - keep `clap` / CLI / report concerns out
3. Extract `crates/domain/project-tree`.
4. Normalize `crates/ports/outbound/traits`:
   - add portable fs types
   - stop freezing `std::fs::*` in the shared API
5. Extract `crates/shared/fs`.
6. Rewire the shared Phase 1 surfaces so they no longer depend on root `crate::fs`.
7. Promote `adapters/outbound/tool-runner` only after the trait boundary is clean.
8. Thin the root facade further once the shared members are real.

## Do Not

- do not promote `domain-config` or `domain-report` in this pass
- do not promote inbound CLI in this pass
- do not create coarse Rust subcrates
- do not split families in this pass
- do not leave new shared crates depending on root `crate::fs`
- do not move `clap` derives into `domain/validation-model`

## Recommended Staffing

One implementation owner should do this pass.

Support reviewers should be read-only and attack:
- dependency ownership
- test/build fallout

Do not split Phase 1 across multiple code-writing agents with overlapping write sets.
