# Hexarch App-Workspace Boundary Handoff

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3`

## What This Handoff Is For

This handoff is only for the immediate structural gap discussed on `2026-03-27`:

1. app root must be the Rust workspace for the app
2. every live `Cargo.toml` inside the app boundary must be a member of that app workspace
3. no nested Rust workspaces may exist under the app root

This is the gap that currently allows the `apps/guardrail3` Rust tree to sprawl under `crates/app/rs/...` without `RS-HEXARCH` catching it.

This is **not** the handoff for solving the broader inner-architecture question. Do **not** use this task to redesign the entire subsystem grammar. The only goal here is to close the obvious workspace-boundary hole.

## Read First

Current contract:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/hexarch.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/README.md`

Current implementation:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/facts.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/inputs.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_07_workspace_members_match_crate_dirs.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_08_app_cargo_is_workspace.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_09_no_extra_workspace_members.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_10_members_within_app_boundary.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/rs_hexarch_11_root_workspace_doesnt_include_apps.rs`

Routing / placement:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/roots.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/classification.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`

## The Actual Gap

Today `RS-HEXARCH` enforces:

- app `Cargo.toml` is a workspace
- workspace members cover discovered hex crate dirs
- no extra members within the app boundary
- repo root must not include app roots

But it does **not** enforce:

- every live Rust `Cargo.toml` under the app is a workspace member
- no nested `[workspace]` under the app root

That is why paths like `crates/app/rs/families/*/Cargo.toml` can become nested workspace roots without `hexarch` treating them as architectural violations.

## Why The Current Implementation Misses It

There are three concrete misses in the live code:

1. `facts.rs` only builds workspace coverage against `discovered_crate_dirs`, which come from hex-structure leaf discovery rather than from all live `Cargo.toml` files under the app boundary.
2. `routed_app_roots()` in `facts.rs` keeps only top-level `apps/<name>` roots, so nested app-scoped Cargo roots inside the app are not treated as owned hex roots for these rules.
3. There is no rule that says “nested workspace under app root is forbidden.” `RS-HEXARCH-08` only checks that the app root itself is a workspace.

## Required End State

After this work:

- `RS-HEXARCH` must error if an app contains any live nested `Cargo.toml` that is not covered by the app-root workspace members
- `RS-HEXARCH` must error if any nested `Cargo.toml` under the app root contains `[workspace]`
- the implementation must work against all live `Cargo.toml` paths inside the app boundary, not just discovered hex leaves

This should make the current `apps/guardrail3` nested family workspace pattern fail clearly under `hexarch`.

## Scope Limits

Do not solve these here:

- whether the right long-term answer is nested hex, slice-by-role, or some other subsystem design
- whether family roots should become pure containers
- whether `runtime/assertions/test_support` is the right package grammar
- broader top-level exact-contents changes such as `bin/` or `shared/`

Those are next-step architecture decisions. This handoff is only for the missing workspace-boundary guardrail.

## Suggested Implementation Direction

The likely correct move is:

1. add app-local Cargo-root discovery facts that collect **all** live `Cargo.toml` files under the routed app boundary
2. classify each discovered nested Cargo root as:
   - package only
   - workspace root
   - malformed manifest
3. compare those discovered Cargo roots against the app-root workspace members
4. emit explicit nested-workspace failures

This may mean:

- broadening `WorkspaceCoverageFacts`
- adding a second fact type specifically for nested Cargo roots inside the app
- adding one or two new `RS-HEXARCH-*` rules instead of trying to overload `07/09/10`

Do **not** hide this inside `RS-ARCH`. This is app-local workspace topology, so it belongs in `RS-HEXARCH`.

## Recommended Rule Shape

The cleanest rule ownership probably looks like:

- keep `RS-HEXARCH-08` as “app Cargo.toml is workspace”
- keep `RS-HEXARCH-09/10` focused on member correctness
- add a new rule for “nested workspace forbidden under app root”
- possibly add a new rule for “all live app-local Cargo roots must be workspace members”

If you can widen `07/09/10` cleanly without muddling ownership, that is acceptable. But do not force one existing rule to own too many distinct failure modes.

## Required Tests

Add focused regressions for:

1. nested package `Cargo.toml` under app root that is not in `[workspace].members`
2. nested package `Cargo.toml` under app root that **is** in `[workspace].members`
3. nested `Cargo.toml` under app root containing `[workspace]`
4. malformed nested `Cargo.toml` under app root
5. wildcard workspace member coverage over nested package paths
6. the current family-like shape:
   - `crates/app/rs/families/deny/Cargo.toml` contains `[workspace]`
   - app workspace points at child crates
   - `hexarch` must fail

Do not limit tests to `crates/{app,domain,ports,adapters}` leafs. The whole point is to catch Cargo roots outside the old hex-leaf discovery path.

## Verification

Minimum expected verification:

- targeted `hexarch` unit tests for the touched rules/facts
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch --lib`
- a binary-level `rs validate ... --family hexarch --inventory --format json` run on `apps/guardrail3` that is clean after the nested workspaces are removed
- a temp-repo attack that reintroduces a nested family-style workspace under `apps/guardrail3` and proves `RS-HEXARCH-07` plus `RS-HEXARCH-27` fire at the top level

If top-level Cargo remains broken for unrelated reasons, do not “fix” the rest of the repo just to make this task pass.

## Acceptance Criteria

This handoff is complete when:

- `RS-HEXARCH` explicitly owns the app-root workspace boundary
- nested app-local workspaces fail
- all live app-local Cargo roots must be workspace members
- tests pin the behavior
- the current `apps/guardrail3` tree no longer contains nested family workspaces, so top-level Cargo metadata succeeds and `apps/guardrail3` validates clean under `RS-HEXARCH`

## Notable Open Follow-Up

Once this gap is closed, the next discussion is the bigger one:

- what the allowed package grammar inside an app should actually be
- whether `rs` should be a role-sliced subsystem, a nested hex, or something else
- how `RS-TEST` package companions interact with that production package grammar

That is deliberately out of scope for this handoff.
