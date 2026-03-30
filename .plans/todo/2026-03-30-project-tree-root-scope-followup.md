# ProjectTree Root Scope Follow-up

**Date:** 2026-03-30

## Goal

Make `ProjectTree` the full repository snapshot for validation and move scope
judgment out of crawling.

The walker change already established the first half of the contract:

- fast ignore-based baseline walk
- tracked ignored file recovery
- targeted recovery of ignored-but-relevant files

This follow-up finishes the contract:

- validation should build one `ProjectTree` from the real project root
- placement should classify roots from that full tree
- families should scope from placement outputs and policy resolution
- no family should depend on subtree-local crawling to decide what exists

## Desired End State

1. `ProjectTree` is built from the true project root.
2. `ProjectTree` contains relevant files even when ignored.
3. placement sees the full root set and classifies it.
4. family mappers route from placement, not from ad hoc subtree assumptions.
5. policy lookup is not tied to the local validation subtree.

## Why This Is Needed

Current failures are not primarily walker failures anymore.

The remaining problems come from later narrowing:

- validating a nested workspace can hide repo-root `guardrail3.toml`
- parent `.gitignore` files above the chosen subtree disappear
- placement excludes roots before families can reason about them
- deps policy attachment still assumes `apps/*` and `packages/*`

That means the repository snapshot exists, but later layers still behave as if
validation starts from an isolated subtree.

## Work Plan

### 1. Canonical Project Root Resolution

Add one explicit project-root resolution step before walking.

Requirements:

- CLI validation should resolve one canonical project root
- normal validation should walk from that root, not from the user’s arbitrary
  current subtree
- scoped validation should become a later filter, not a different crawl root

Likely touch points:

- CLI / command entrypoint that currently accepts the validate path
- code that calls `walk_project(...)`

Questions to settle:

- is the canonical root the git repo root, the nearest `guardrail3.toml`, or a
  resolved `rust.workspace_root` target?
- what should happen when those disagree?

Recommendation:

- use repo root as the physical `ProjectTree` root
- resolve policy roots separately

### 2. Placement Should Classify, Not Erase

Placement should operate on the full tree and classify roots instead of making
them disappear too early.

Current exclusions in placement such as:

- `tests/fixtures`
- `tests/snapshots`
- `target`

should become explicit classifications where possible, not silent absence.

Desired shape:

- live root
- fixture/snapshot root
- generated/build root
- invalid/unexpected root

Families can then choose which classes they own.

Primary file:

- `apps/guardrail3/crates/app/rs/placement/src/roots.rs`

### 3. Add Scoped Validation As Filtering, Not Re-rooting

If the user validates a subpath, keep the full-root `ProjectTree` and add a
scoped filter layer instead of rebuilding the tree from that subpath.

That means:

- full root snapshot stays available
- parent config remains visible
- family mappers can filter routed roots/files to the requested subpath

This is the right place to use `scoped_files` / routed roots, not the walker.

### 4. Separate Policy Root From Validation Scope

Families that depend on policy files must not assume policy lives exactly at the
local validation subtree root.

Needed follow-up:

- introduce explicit policy-root resolution
- let families read policy from that governing root
- surface an input failure if policy cannot be resolved confidently

This is especially important for:

- `RS-DEPS`
- cargo policy families
- future TypeScript families that depend on repo-root config

### 5. Keep Family Routing Thin

Family mappers should continue routing from placement outputs, but they should
not be responsible for rediscovering filesystem truth.

Good contract:

- walker: what exists
- placement: what roots/classes exist
- family mapper: which classes/roots/files this family wants
- family facts: parse and normalize owned semantics

### 6. Add Regression Coverage For Full-root Behavior

Add tests proving:

- validating a nested subpath still retains repo-root config visibility
- parent `.gitignore` remains available for lockfile checks
- fixture/snapshot roots are classified, not erased
- scoped validation filters outputs without shrinking the underlying tree

## Suggested Execution Order

1. add canonical project-root resolution
2. thread full-root walking through validation entrypoints
3. add scoped filtering layer for subpath validation
4. refactor placement exclusions into classifications
5. update policy-dependent families to use policy-root resolution
6. add regression tests for nested validation and root visibility

## Immediate Next Target

The next concrete change after the walker work should be:

1. identify the validation entrypoint that chooses the walk root
2. switch it to full-root walking
3. preserve subpath requests as scope filters only

That unlocks the rest of the architecture cleanup without changing every family
at once.
