# Rust Family Audit Fix Plan

**Date:** 2026-03-31

## Summary

This plan covers the gaps found while auditing every active Rust family against:

- `.plans/by_family/rs/<family>.md`
- `apps/guardrail3/crates/app/rs/families/<family>/README.md`
- `.plans/todo/checks/rs/<family>.md`
- live production code

The main result of the audit is:

- most families have most or all numbered rule files
- several families are still wrong on the new routed surface
- some families still rediscover their own inputs instead of consuming shared ownership output
- some workspace-local families lose shared ancestor inputs before rules run
- `toolchain` is not fully implemented in production
- docs/specs are badly out of sync in several places, which makes the live contract hard to trust

This is not one bug. It is a boundary failure.

The intended architecture is:

```text
Project walker
  -> ProjectTree
  -> shared structure discovery
  -> shared legality / ownership
  -> family mapper
  -> routed family surface
  -> family facts
  -> typed rule inputs
  -> pure rules
```

The live code is still leaking back toward:

```text
ProjectTree
  -> family
  -> family rediscovers files / walk-up behavior / scope
```

That is the core thing that must be fixed.

## What Is Going Wrong

### 1. Family ingress is still too powerful

Families are supposed to consume routed family input, not rediscover their universe.

That is still violated in multiple places:

- `code` gets `RsProjectSurface::from_tree(ctx.tree)` and then rescans `.rs` files itself
- `test` still discovers files from a broad surface instead of consuming a pre-owned test slice
- `hexarch` gets scoped-file routing metadata but ignores it and scans the whole routed app anyway
- some family test helpers still bypass routed surfaces and run on full-tree surfaces

Consequence:

- ownership is not actually centralized
- families can silently create their own exclusions and scope behavior
- legality can be bypassed by rediscovery
- subtree behavior becomes inconsistent family by family

### 2. Workspace-local routing is dropping ancestor/shared inputs

Several local families correctly stopped seeing the whole tree, but the replacement surface is too narrow and drops inputs they still legitimately need.

Confirmed examples:

- `clippy` loses repo `guardrail3.toml`
- `clippy` loses applicable ancestor `.cargo/config.toml` / `.cargo/config`
- `deny` loses repo `guardrail3.toml`
- `deps` loses ancestor `.gitignore`
- `release` loses repo/ancestor release files and workflows in practice

Consequence:

- rules exist but can fail open because required context never reaches them
- local-family routing is currently stricter than the family contracts

### 3. Global families are still half-implemented as root-driven families

A global family is not supposed to get raw repo authority and rediscover its own inputs.
It is supposed to receive a repo-global family-owned slice.

Confirmed examples:

- `code` still acts as if Cargo roots determine whether code exists
- `test` still acts as if routed roots are the main entry, with scoped-file support wired but not actually fed
- `arch` is global, but some of its reporting is still incorrectly scope-filtered

Consequence:

- global families are not truly global in the right way
- they can still disappear findings or over-scan unrelated files depending on route shape

### 4. Specs and live behavior are no longer aligned

There are three layers of truth for many families:

- old detailed rule ledger
- newer family README / by-family spec
- live runtime behavior

Those layers often disagree about:

- whether the family is global or workspace-local
- whether standalone package roots still exist
- whether placement is owned by the family or by `arch`
- exact severity levels
- whether a rule is implemented vs still planned

Consequence:

- auditability is poor
- tests can be written against obsolete contracts
- developers cannot tell whether a mismatch is a bug or a stale document

## Fix Principles

1. No family decides ownership for itself.
2. No family gets raw repo authority just because it is global.
3. Global families get repo-global owned slices, not the raw tree.
4. Workspace-local families get per-workspace slices plus explicit ancestor/shared attachments that the shared layer owns.
5. Placement legality stays outside the local families.
6. Family tests must stop cheating with full-tree helper surfaces where production uses routed surfaces.
7. Runtime attack tests must prove mapper + runner + family behavior, not just family-local rule logic.

## Workstream 1: Enforce the Family Ingress Boundary

### Goal

No Rust family runtime should be able to treat the full repo as its discovery authority.

### Required change

Define a stricter contract for family inputs:

- shared pipeline may read `ProjectTree`
- family mapper may derive routed slices from shared legality/ownership
- family runtime may read only its routed family surface / routed facts
- pure rules may read only typed rule inputs

### Concrete follow-up

1. Audit all family runners and remove any use of whole-tree family ingress where a routed surface should be used.
2. Audit family test helpers and stop building full-tree surfaces for tests that are meant to prove routed behavior.
3. Replace family-local file discovery that decides ownership with pre-owned routed inventories from the shared layer.

### First targets

- `code`
- `test`
- `hexarch`
- any helper tests that call `RsProjectSurface::from_tree(tree)` where production uses a routed workspace surface

## Workstream 2: Define Global vs Local Surface Types Explicitly

### Goal

Stop overloading one family surface shape for incompatible use cases.

### Required change

Make the mapper output reflect actual family scope:

- global families:
  - repo-global owned file / config slices
  - optional scoped-file subset where the contract requires scoped activation
- workspace-local families:
  - one legal workspace route at a time
  - attached local files
  - attached ancestor/shared files explicitly allowed for that family

### Implication

The question is not “does a family get the tree.”
The question is “what exact owned surface does the family get.”

That boundary must be made impossible to blur again.

## Workstream 3: Fix Shared Attachment of Ancestor / Shared Inputs

### Goal

When a local family legitimately depends on ancestor or repo-level inputs, that dependency must be declared and attached by shared routing, not reimplemented inside the family.

### Required change

Extend the shared routing/ownership layer so family slices can explicitly carry:

- repo `guardrail3.toml`
- applicable ancestor `.cargo/config.toml` / `.cargo/config`
- ancestor `.gitignore`
- repo or ancestor release files/workflows when the family contract requires them

### Constraint

This must not reopen direct tree access. The family should receive those files because routing attached them, not because the family walks upward on its own.

## Workstream 4: Fix Spec Drift and Make One Contract Authoritative

### Goal

After behavior is fixed, docs must stop contradicting runtime.

### Required change

For each family:

- keep one current contract in `.plans/by_family/rs/<family>.md` plus family `README.md`
- demote stale ledger text to historical detail only
- update the detailed ledgers where they are still referenced as live truth

### Rule

No family README should claim ownership or scope that production no longer has.

## Family Fix Queue

### 1. `toolchain`

Status:

- production incomplete
- docs/spec claim `g3rs-toolchain/root-toolchain-config-exists..07`
- live runtime only has `01..04`

Required fixes:

- implement `RS-TOOLCHAIN-05`
- implement `RS-TOOLCHAIN-06`
- implement `RS-TOOLCHAIN-07`
- remove or rewrite tests that currently assert those findings belong nowhere / stay silent
- align mapper/runtime/family contract on whether placement is fully `arch`-owned or partially `toolchain`-owned

Why first:

- this is the only family that is plainly not implemented rather than merely miswired

### 2. `release`

Status:

- rule inventory exists
- workspace-local routing and family logic fundamentally disagree
- family still hardcodes repo-root assumptions

Confirmed problems:

- repo-root `Cargo.toml` assumptions inside a workspace-local family
- repo-root `release-plz.toml` / `cliff.toml` assumptions
- repo-root workflow assumptions
- family tests use full-tree surfaces and can miss runtime routing bugs

Required fixes:

- decide exact local release contract
- attach required shared ancestor files in routing
- make fact collection use routed workspace root and routed attachments, not literal repo-root paths
- add runtime tests for non-root workspace release inputs

### 3. `code`

Status:

- rule inventory present
- ingress boundary wrong
- zero-root case can fail open

Confirmed problems:

- runner gives full-tree surface
- family rescans `.rs` files itself
- root gating can return empty facts when no routed roots exist
- hidden local fixture carveout exists

Required fixes:

- introduce shared owned Rust source inventory for `code`
- route repo-global owned source slice into `code`
- stop using routed roots as the existence gate for code files
- decide where fixture exclusions belong and remove family-local hidden carveouts if they are not shared policy
- keep root/context facts only for the small subset of rules that actually need root metadata

### 4. `test`

Status:

- rule inventory present
- production never actually passes scoped files
- family still does too much discovery itself

Required fixes:

- route scoped-file subset for subtree runs if that is the intended contract
- route repo-global owned test surface instead of broad root-driven rediscovery
- prove same-root scoped narrowing in runtime tests

### 5. `hexarch`

Status:

- rule inventory present
- scoped-file contract exists but is ignored by collectors

Required fixes:

- make dependency facts honor scoped files
- make source facts honor scoped files
- add runtime tests for same-app subtree narrowing, not just cross-app isolation

### 6. `clippy`

Status:

- rule inventory present
- routed surface is missing required shared inputs

Required fixes:

- attach repo `guardrail3.toml`
- attach applicable ancestor cargo-config files
- add runtime-path tests for both channels
- align severity/docs for `RS-CLIPPY-08`

### 7. `deny`

Status:

- rule inventory mostly present
- routed surface is missing required profile context

Required fixes:

- attach repo `guardrail3.toml`
- prove routed profile-sensitive behavior for rules that depend on it
- remove any stale docs claiming old placement ownership

### 8. `deps`

Status:

- rule inventory present
- routed surface drops ancestor `.gitignore`

Required fixes:

- attach applicable ancestor `.gitignore`
- prove `RS-DEPS-10` on routed workspace slices
- decide whether tool-presence rules should deduplicate globally or intentionally repeat per workspace

### 9. `arch`

Status:

- much further implemented than docs say
- one real global-reporting hole remains

Required fixes:

- stop scope-filtering `RS-ARCH-16` placement findings that are supposed to be repo-global
- add runtime tests for `09..14` and `16`
- update README and inventory docs to match live implementation

### 10. `fmt`

Status:

- runtime largely aligned
- dead rule and stale ownership docs remain

Required fixes:

- decide whether to delete `RS-FMT-05` from the family or keep it as a permanently dead historical stub
- move all nested rustfmt placement ownership text fully to `arch`
- add runtime proof that subtree runs do not localize `fmt`

### 11. `cargo`

Status:

- rule inventory present
- main issue is contract drift and stale standalone-root logic

Required fixes:

- remove stale standalone-root discovery branches if they are truly dead
- update docs to workspace-local-only truth
- add runtime tests for routed malformed inputs and member-level rules

### 12. `garde`

Status:

- rule inventory present
- smaller correctness and contract issues remain

Required fixes:

- tighten fail-closed behavior for unreadable routed root Cargo manifests
- decide whether fixture exclusions are shared policy or a family-local bug
- align severity/docs for `RS-GARDE-AST-04`

### 13. package-scoped `arch`

Status:

- rule inventory present
- zone ownership wording and one fail-closed test are wrong

Required fixes:

- settle whether package-scoped `arch` is intentionally `packages/*`-only
- fix the incorrect quiet-path test for broken layered roots
- add runtime proof for the zone boundary

## Execution Order

This should not be done alphabetically.

Recommended order:

1. shared ingress and shared-attachment architecture
2. `toolchain`
3. `release`
4. `code`
5. `test`
6. `hexarch`
7. `clippy`
8. `deny`
9. `deps`
10. `arch`
11. `fmt`
12. `cargo`
13. `garde`
14. package-scoped `arch`
15. docs reconciliation pass

Reason:

- the shared boundary and attachment model must be fixed first
- otherwise local family repairs will keep getting redone
- `toolchain` and `release` are the highest-signal correctness failures after that
- `code` / `test` / `hexarch` are the core ingress-boundary violations

## Test-Attack Requirements Before Calling This Done

Every family fix must be proved in three layers.

### 1. Family-local rule tests

These prove rule semantics on minimal routed inputs.

### 2. Runtime routing tests

These prove:

- global families actually receive repo-global owned surfaces
- local families run once per legal workspace
- scoped runs narrow only where they should
- ancestor/shared attachments survive routing
- illegal placements still report where the contract says they should

### 3. Mutation / attack tests

For each family, mutate the golden fixture so that:

- every supposed owned target breaks at once
- sibling workspaces also exist
- ancestor-shared files exist
- malformed configs appear in the routed path
- illegal placements exist outside the currently scoped workspace

The proof is not “green tests.”
The proof is:

- correct hits appear everywhere they should
- no owned target disappears
- no non-owned target leaks in

## Specific Architecture Decisions That Must Be Made Explicitly

These are the decisions the codebase needs to encode, not leave implicit:

- whether global families ever receive a full raw surface instead of an owned slice
- whether `tests/fixtures` exclusions are shared policy or forbidden family-local carveouts
- whether ancestor attachments are modeled per family by declarative shared contracts
- whether some families should consume routed file inventories directly instead of rediscovering from routed structure
- whether local-family profile context always comes from explicit shared attachment rather than ad hoc file reads

## Definition of Done

This audit follow-up is only done when all of the below are true:

1. No Rust family runtime can decide its own ownership universe from broad repo structure.
2. No workspace-local family loses required shared ancestor inputs.
3. Global families consume repo-global owned slices, not raw tree authority.
4. `toolchain` is complete in production.
5. `release` works correctly for non-root workspaces.
6. `code`, `test`, and `hexarch` honor the shared routing contract.
7. `arch` reports global placement legality globally.
8. Family tests no longer cheat with full-tree surfaces where production uses routed surfaces.
9. Runtime attack tests exist for every family.
10. `.plans/by_family/rs/*` and family `README.md` files match live behavior.

## Immediate Next Step

Before touching family-specific fixes, define the strict family ingress contract in code:

- who may read `ProjectTree`
- who may only read routed family surfaces
- what a global family surface actually is
- how shared ancestor attachments are declared and carried

Without that, fixing individual families will keep reintroducing the same boundary failure.
