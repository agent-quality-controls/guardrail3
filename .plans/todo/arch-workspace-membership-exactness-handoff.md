# Arch Workspace Membership Exactness Handoff

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3`

## What This Handoff Is For

This handoff is only for one ownership change:

1. workspace membership exactness is one repo-global Rust topology concept
2. that concept should live in `RS-ARCH`, not in `RS-HEXARCH`
3. the rule should mean: every governed workspace's declared member set must exactly equal its real owned child-crate set

This means:

- every real owned child crate must be declared
- every declared workspace member must resolve to a real owned child crate

This is one topology invariant with two failure directions.

This is **not** the handoff for redesigning crate public surfaces, facade policy, or generic dependency architecture. Do not use this task to widen `arch` into crate API semantics.

## Read First

Current `arch` contract:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/arch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/arch.md`

Current `hexarch` contract:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/hexarch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/hexarch.md`

Current implementation:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/inputs.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/inputs.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/workspace_policy/rs_hexarch_07_workspace_members_match_crate_dirs.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/workspace_policy/rs_hexarch_09_no_extra_workspace_members.rs`

Shared routing / placement:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/roots.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`

## The Actual Problem

Today the exact-workspace-membership concept is split across families:

- `RS-ARCH-12` checks the missing-member direction globally
- `RS-HEXARCH-07` checks the missing-member direction locally for apps
- `RS-HEXARCH-09` checks the extra-member direction locally for apps

That is the wrong ownership.

Exact workspace membership is repo-global Rust topology, not app-local hex structure.

If `arch` owns:

- which live Rust roots exist
- which workspaces own which child crates
- whether the Rust topology is legal

then `arch` should also own:

- whether each governed workspace's `[workspace].members` is exactly correct

`hexarch` should not duplicate that local topology concept.

## Required End State

After this work:

- `RS-ARCH-12` is redefined as one exactness rule:
  - every governed workspace's declared member set must exactly equal its real owned child-crate set
- `RS-ARCH-12` emits both failure directions under one rule ID:
  - missing declared member for a real owned child crate
  - extra declared member with no matching real owned child crate
- `RS-HEXARCH-07` and `RS-HEXARCH-09` no longer own workspace membership exactness
- `RS-HEXARCH` continues to own app-local structure and dependency semantics, but not this topology duplication

This should make workspace membership exactness a single global topology concept instead of one global rule plus local duplicates.

## Important Non-Goals

Do not change these in this task:

- `RS-ARCH-13` escaping member paths
- `RS-ARCH-11` nested workspace prohibition
- `RS-HEXARCH-10` app-boundary ownership of member paths
- crate public-surface / facade policy
- dependency-edge API enforcement

Those are different concepts and must stay separate.

Especially important:

- `RS-ARCH-13` stays separate even after this migration
- a member pattern can be illegal because it escapes the root even if it still participates in membership matching

## Rule Ownership After The Change

### `RS-ARCH-12`

Owns one exactness invariant:

- all real owned child crates are declared
- all declared members correspond to real owned child crates

This is one topology concept.

It is acceptable for one rule ID to emit two message shapes:

- missing real child crate from membership
- extra declared member with no owned real child crate

### `RS-HEXARCH`

Must stop owning this concept.

After the change, `hexarch` should still own:

- app root is workspace
- app-local boundary legality
- app-local exact directory shape
- dependency direction
- cross-app boundary rules

But it should no longer own:

- exact membership equivalence between workspace members and real child crates

## Suggested Implementation Direction

The cleanest path is to redesign `RS-ARCH-12`, not add a brand new rule ID.

### 1. Redefine `RS-ARCH-12`

Current file:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only.rs`

Keep the rule ID.

Change the rule meaning from:

- only "live lower-level Rust crates must be declared members"

to:

- "workspace membership must exactly match real owned child crates"

### 2. Expand `arch` facts

In:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/facts.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/inputs.rs`

collect enough information for each governed workspace root to compare:

- declared `[workspace].members` resolved child dirs
- real owned child crate dirs under that workspace

The comparison must be workspace-root-local, but owned by `arch` globally.

Do not rebuild separate app-only logic.

### 3. Compare exact sets

For each governed workspace root:

- missing set:
  - real child crate dir exists
  - not covered by resolved member dirs
- extra set:
  - resolved member dir exists in membership set
  - does not correspond to a real owned child crate for that workspace

Emit one result per mismatch, not one giant aggregated repo blob.

The rule still needs file-local attribution:

- missing child case should point at the child crate `Cargo.toml`
- extra member case should point at the parent workspace `Cargo.toml`

### 4. Remove local duplicate ownership from `hexarch`

Stop running:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/workspace_policy/rs_hexarch_07_workspace_members_match_crate_dirs.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/workspace_policy/rs_hexarch_09_no_extra_workspace_members.rs`

Likely changes:
- remove the two calls from `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`
- delete the rule files and their test directories if they become fully dead
- simplify `WorkspaceCoverageFacts` if some fields become unnecessary

Do not remove app-boundary logic from `RS-HEXARCH-10`.

## Test Plan

### `arch` must gain the exactness tests

Add or rewrite tests under:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/rs_arch_12_declared_workspace_members_only_tests/`

Required cases:

1. real owned child crate exists but is omitted from `[workspace].members`
2. declared member exists but no owned child crate matches it
3. both directions in one workspace at once
4. exact match stays clean
5. normalized / glob member patterns that legitimately cover owned child crates
6. nested-path runtime proof that `arch` still sees the whole repo topology

### `hexarch` must lose the duplicate tests

Remove or rewrite tests under:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/workspace_policy/rs_hexarch_07_workspace_members_match_crate_dirs_tests/`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/workspace_policy/rs_hexarch_09_no_extra_workspace_members_tests/`

If the rules are deleted, their tests must be deleted too.

If some fixtures are still useful, move them into `arch` and retarget them there.

### Ownership split tests to preserve

Keep explicit proofs that:

- `RS-ARCH-12` owns exact workspace membership
- `RS-HEXARCH-10` still owns app-boundary violations
- `RS-ARCH-13` still owns escaping member-path legality

The goal is to remove duplicate ownership, not flatten everything into one mega-rule.

## Verification

Minimum expected verification:

- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-arch --lib`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-runtime -- --nocapture`
- lean binary run:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-arch -- rs validate apps/guardrail3 --family arch --format json`
- lean binary run:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-hexarch -- rs validate apps/guardrail3 --family hexarch --format json`

The important product proof is:

- exact membership failures still appear under `arch`
- they no longer appear under `hexarch`

## Acceptance Criteria

This handoff is complete when:

- `RS-ARCH-12` owns exact workspace membership globally
- both mismatch directions are covered under that one rule concept
- `RS-HEXARCH-07` and `RS-HEXARCH-09` no longer own or emit that concept
- tests prove the new ownership split
- docs/plans clearly state that workspace membership exactness is `arch`, not `hexarch`

## Documentation Updates Required

Update these after code lands:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/arch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/arch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/hexarch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/hexarch.md`

The docs must stop describing `RS-HEXARCH-07/09` as live ownership of workspace membership exactness if those rules are removed.

## Recommended Execution Order

1. redesign `arch` facts/inputs for exact workspace membership
2. upgrade `RS-ARCH-12` to emit both mismatch directions
3. add `arch` tests that prove both directions and exact-match success
4. remove duplicate `hexarch` rule execution
5. delete or migrate `hexarch` duplicate tests
6. run family and runtime verification
7. update docs/plans

## Notable Follow-Up

Once this is done, the next architecture question is separate:

- whether `arch` should stay pure topology/governance
- whether broader crate API / facade dependency policy needs a separate family

That is deliberately out of scope for this handoff.
