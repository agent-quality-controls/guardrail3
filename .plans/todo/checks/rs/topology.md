# RS-TOPOLOGY — Rust root placement and architecture ownership checker

> Superseded as the primary family plan by [`.plans/by_family/rs/topology.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/topology.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** all discovered Rust `Cargo.toml` roots + guardrail config + owned workspace/package classification
**Parser:** TOML + directory structure
**Current code:** `apps/guardrail3/crates/app/rs/families/topology/`, `apps/guardrail3/crates/app/rs/placement/`

## Why this family exists

`RS-HEXARCH` owns app architecture inside `apps/*`.
`RS-ARCH` owns package/library architecture inside `packages/*`.

What neither family should own is the repo-global question:

- where Rust roots are allowed to live
- which architecture family owns a given Rust root
- whether a Rust root is misplaced or ambiguously owned

That is a separate concern. It should not be duplicated in both `hexarch` and `arch`, and it should not be buried in `cargo`.

`RS-TOPOLOGY` exists to own that global placement and ownership contract once.

Current source of truth:
- `apps/guardrail3/crates/app/rs/families/topology/README.md` for family-local behavior
- `apps/guardrail3/crates/app/rs/README.md` for shared routed architecture
- this file for the live rule inventory

Older handoffs are historical only.

## Scope

This family is deliberately global and structural.

It owns only:
- Rust root discovery/classification
- root placement under architecture zones
- root-to-family ownership
- overlap/nesting legality between architecture zones
- repo-global Rust root topology policy
- repo-global placement legality for workspace-local family artifacts

It does **not** own:
- app-internal hex structure
- library-internal layered structure
- workspace-member semantics inside an app or package boundary
- generic Cargo manifest policy
- family-local config content validation

Those remain in `RS-HEXARCH`, `RS-ARCH`, and `RS-CARGO`.

Important architecture split:

- shared Rust structure facts are computed before families
- shared Rust legality is then derived before family slicing
- `RS-TOPOLOGY` is the reporting surface for that legality stage
- workspace-local families consume only legal local surfaces after that stage

So `topology` should not be modeled as "a family that must run first."
It should be modeled as the visible report surface for legality facts the mapper and runners also rely on.

## Test split required by this architecture

The legality-first architecture implies a strict test split.

What must be true:

- `topology` owns tests for illegal topology
- `topology` owns tests for illegal workspace-local family-file placement
- workspace-local family routed tests use legal workspace fixtures only
- pure rule logic is tested against the rule's typed input directly

What must stop:

- workspace-local family test helpers rebuilding fake routes when legality-aware routing returns no legal workspaces
- local family tests using illegal standalone roots or illegal nested roots just to reach content rules
- local family tests asserting placement/topology findings that now belong to `topology`

So for the remaining migration work:

- illegal root-shape fixtures must move to `topology` or shared legality tests
- workspace-local family tests must be rewritten to legal app/package workspace fixtures
- rule sidecar tests must be preferred for pure semantic checks over direct typed inputs
- no test-only routing bypass is allowed as a permanent architecture compromise

## Target topology policy

The intended end state is harsher than the currently implemented rule set.

The target repo-wide Rust topology is:

- every live top-level Rust root is a workspace root
- no loose top-level package roots
- no nested workspaces anywhere
- any live lower-level Rust crate under a governed workspace root must be a package, not a workspace
- any live lower-level package under a governed workspace root must be a declared member of that workspace
- no workspace member path may escape the workspace root with `../`
- auxiliary/tool/fuzz/xtask Rust roots should follow the same top-level workspace rule rather than reintroducing standalone package escape hatches

In this model:

- top-level means a discovered live Rust root that is not inside another live Rust workspace root
- lower-level means any discovered live Rust root beneath a governed top-level workspace root
- repo-root workspace is allowed only if it is the only Rust workspace in the repository
- repo-root workspace plus nested app/package workspaces is forbidden because nested workspaces are forbidden full stop

Open policy decision:

- whether a top-level workspace root may be a hybrid manifest containing both `[workspace]` and `[package]`
  - allowing hybrid top roots preserves root facade crates
  - forbidding hybrid roots is stricter and simpler
  - current decision is intentionally left open until a follow-up policy pass resolves it

## Core model

Every discovered Rust `Cargo.toml` root is classified as exactly one of:

- `app`
  - under `apps/*`
- `package`
  - under `packages/*`
- `auxiliary`
  - outside governed zones but explicitly marked in Cargo metadata
- `other`
  - anywhere else that is neither governed nor declared auxiliary

Architecture families then apply by zone:

- `app` roots are candidates for `RS-HEXARCH`
- `package` roots are candidates for `RS-ARCH`
- `other` roots are misplaced when Rust architecture enforcement is active

The topology policy above is orthogonal to zone ownership:

- a root may be correctly zoned and still be illegal if it is a loose top-level package
- a root may be correctly zoned and still be illegal if it declares a nested workspace
- a lower-level crate may be correctly zoned and still be illegal if it is live but omitted from the parent workspace

## Ownership model

This family owns repo-global topology and placement findings.

That means:
- `RS-HEXARCH` must not emit “Cargo root misplaced globally”
- `RS-ARCH` must not emit it either
- workspace-local families must not emit repo-global misplaced-file findings either
- `RS-TOPOLOGY` emits those structural failures once

This avoids:
- double signaling
- drift between `hexarch` and `arch`
- hidden policy duplication
- workspace-local families re-growing global placement logic

The intended split is:

- shared structure pass: discovers roots/files and attaches files to Rust structure
- shared legality pass: decides whether each root/file placement is legal
- `RS-TOPOLOGY`: reports those legality failures
- family mapper: routes only legal local surfaces to workspace-local families
- family runners: invoke workspace-local families once per legal workspace

## Discovery / classification model

The shared structure pass must discover every Rust `Cargo.toml` root in the repo and classify each root by zone.

Classification is path-based:
- roots under `apps/<name>/...` belong to the `app` zone
- roots under `packages/<name>/...` belong to the `package` zone
- everything else is `other`

The shared structure pass must also detect illegal overlap candidates:
- one Rust root must not be simultaneously treated as both app-owned and package-owned
- illegal nesting between architecture roots must be surfaced explicitly

`RS-TOPOLOGY` then consumes those shared facts and turns them into legality results.

The same model should eventually apply to workspace-local family artifacts:

- shared structure discovers the file and attaches it to nearby Rust structure
- shared legality determines whether that placement is allowed
- `RS-TOPOLOGY` reports the illegal placement if not
- the local family never sees the illegal placement as one of its legal invocation inputs

## Conditional applicability

Placement reporting depends on architecture-family enablement.

The intended behavior is:

- if both `hexarch` and `arch` are enabled
  - misplaced `other` roots are `Error`
- if only `hexarch` is enabled
  - misplaced `other` roots are `Error`
- if only `arch` is enabled
  - misplaced `other` roots are `Error`
- if both are disabled
  - no placement finding is emitted

Discovery/classification still happens either way.
Enablement changes reporting, not root discovery.
When reporting is inactive, `RS-TOPOLOGY-02` should still emit an inventory/info result saying misplaced-root enforcement is suppressed.

## Input integrity / fail-closed expectations

The family depends on:
- readable `Cargo.toml` discovery
- readable guardrail config
- readable directory structure for zone classification

Malformed required placement/config inputs must not silently suppress misplaced-root findings.

## Planned rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TOPOLOGY-01 | Error | Every discovered Rust root must classify as `app`, `package`, or `other` without ambiguity | Implemented |
| RS-TOPOLOGY-02 | Error | When Rust architecture enforcement is active, no discovered Rust root may live in `other` | Implemented |
| RS-TOPOLOGY-03 | Error | No Rust root may be simultaneously owned by both app and package zones | Implemented |
| RS-TOPOLOGY-04 | Error | Illegal nesting/overlap between app roots and package roots is forbidden | Implemented |
| RS-TOPOLOGY-05 | Error | Scoped `topology` config is forbidden; `topology` is global-only | Implemented |
| RS-TOPOLOGY-06 | Error | Governed roots must stay coherent with owner-family enablement | Implemented |
| RS-TOPOLOGY-07 | Error | Required `topology` inputs fail closed when unreadable or malformed | Implemented |
| RS-TOPOLOGY-08 | Info | Declared auxiliary roots are surfaced explicitly in reports | Implemented |
| RS-TOPOLOGY-09 | Error | Every live top-level Rust root must be a workspace root | Planned |
| RS-TOPOLOGY-10 | Error | Loose top-level package roots are forbidden | Planned |
| RS-TOPOLOGY-11 | Error | Nested workspaces are forbidden even when excluded or not referenced by the parent workspace | Planned |
| RS-TOPOLOGY-12 | Error | Workspace membership must exactly match real owned child Rust crates beneath a governed workspace | Implemented |
| RS-TOPOLOGY-13 | Error | Workspace member paths must not escape the workspace root | Planned |
| RS-TOPOLOGY-14 | Error | Auxiliary top-level Rust roots must obey the same top-level workspace rule | Planned |
| RS-TOPOLOGY-15 | Policy | Hybrid top-level workspace roots need an explicit allow/forbid decision | Planned |
| RS-TOPOLOGY-16 | Error | Workspace-local family-owned files must be legally placed before local family validation | Planned |

## Rule intent

### RS-TOPOLOGY-01 — Root classification is unambiguous

Every discovered Rust root must resolve to one clear zone classification.

This catches:
- ambiguous path ownership
- broken zone-classification logic
- silently skipped roots

### RS-TOPOLOGY-02 — No misplaced Rust roots

When either `hexarch` or `arch` is enabled, an unexpected Rust root in `other` is an error.

This is the missing global-placement rule.

### RS-TOPOLOGY-03 — No dual ownership

A single Rust root must not be governed by both app and package architecture zones.

### RS-TOPOLOGY-04 — No illegal zone overlap

App and package architecture boundaries must not overlap or nest in a way that makes ownership unclear.

This is about root-zone legality, not member legality inside a single workspace.

Treat this as a layout-level rule, not a duplicate root-level one:
- `RS-TOPOLOGY-01` owns ambiguous per-root classification
- `RS-TOPOLOGY-03` owns per-root dual ownership
- `RS-TOPOLOGY-04` owns the illegal app/package containment pair itself

### RS-TOPOLOGY-05 — Scoped `topology` config is forbidden

`topology` is repo-global and must be configured only under `[rust.checks]`.

### RS-TOPOLOGY-06 — Owner-family enablement is coherent

For a governed root:
- app-zone ownership maps to `hexarch`
- package-zone ownership maps to `arch`
- app-scoped `hexarch` overrides win over the global `hexarch` default for every root under that app

The family should surface impossible or contradictory ownership states explicitly.

### RS-TOPOLOGY-07 — Required inputs fail closed

Unreadable-present or malformed required `topology` inputs must surface explicit errors instead of silently degrading into absence.

That includes:

## Active migration note

The current cargo/deny/clippy cleanup must follow the test split above.

Specifically:

- `RS-CARGO` and `RS-DENY` still contain older routed-family tests built around illegal fixture shapes
- those tests must be rewritten, not masked with synthetic routes
- when a historical test is really about illegal placement or illegal topology, it must move to `RS-TOPOLOGY`
- when a historical test is really about rule semantics, it should become a direct typed-input sidecar test
- malformed governed app/package `Cargo.toml`
- governed roots that declare `arch_role`
- malformed auxiliary metadata on out-of-zone roots

### RS-TOPOLOGY-08 — Declared auxiliary roots are explicit

Roots outside governed zones may opt into `auxiliary` status explicitly.
That exemption should stay visible as inventory/info output.

### RS-TOPOLOGY-09 — Every live top-level Rust root is a workspace

For any live Rust root that is not nested beneath another live Rust workspace root:

- `[workspace]` must be present
- pure top-level package manifests are not enough

This is the main topology hardening rule that turns "workspace" into the only allowed top-level Rust product shape.

### RS-TOPOLOGY-10 — Loose top-level packages are forbidden

Any top-level Rust root that contains `[package]` but not `[workspace]` is forbidden.

This rule exists separately from `RS-TOPOLOGY-09` so the family can emit a direct "loose package root" failure rather than only an indirect "workspace missing" failure.

### RS-TOPOLOGY-11 — Nested workspaces are forbidden

Any discovered live Rust root beneath a governed workspace root must not declare `[workspace]`.

This must still fail when the nested workspace is:

- listed in the parent workspace members
- excluded from the parent workspace
- not referenced by the parent workspace at all

Discovery is structural, not membership-based.

### RS-TOPOLOGY-12 — Workspace membership must exactly match real owned child crates

For each governed workspace root:

- every real owned child Rust crate beneath that workspace must be declared in `[workspace].members`
- every declared workspace member must resolve to a real owned child Rust crate for that workspace

This is one topology concept with two failure directions:

- missing real child crate from membership
- extra declared member with no matching owned child crate

### RS-TOPOLOGY-13 — Workspace member paths must stay inside the root

Parent workspace manifests must not use member paths that escape the workspace root, including `../` traversal.

Workspace membership must not become a backdoor for cross-root ownership drift.

### RS-TOPOLOGY-14 — Auxiliary roots obey the same top-level workspace rule

If out-of-zone Rust roots remain allowed through explicit `auxiliary` declaration, they should still be required to be top-level workspace roots rather than loose standalone packages.

Otherwise auxiliary roots become an escape hatch that reintroduces the same unstable topology this family is trying to forbid.

### RS-TOPOLOGY-15 — Hybrid top-level roots need an explicit policy

The family needs an explicit decision on whether a top-level root may contain both:

- `[workspace]`
- `[package]`

Options:

- allow hybrid top roots only
- forbid hybrid roots entirely

Do not leave this implicit in parser behavior or in downstream family assumptions.

### RS-TOPOLOGY-16 — Workspace-local family files are globally placement-checked before local validation

Workspace-local family-owned files must be judged globally before local families run.

That means:

- shared structure attaches `clippy.toml`, `rust-toolchain*`, `deny.toml`, workspace-local Cargo policy files, and similar artifacts to Rust topology
- shared legality decides whether those files are legally placed
- illegal placement is reported through `RS-TOPOLOGY`
- local families only receive legal local files when they are invoked per workspace

This rule is not "local family placement by another name."
It is the global architectural assertion that placement legality is settled before local family content validation begins.

## Relationship to other families

### RS-HEXARCH

`RS-HEXARCH` owns:
- app structure
- app workspace shape
- app dependency direction

It does **not** own repo-global misplaced-root detection.

### RS-ARCH

`RS-ARCH` owns:
- layered library/package structure
- package workspace/member rules
- package dependency direction

It does **not** own repo-global misplaced-root detection either.

### RS-CARGO

`RS-CARGO` owns Cargo policy at allowed Rust roots.

It does not decide:

- whether a Rust root is in the correct architecture zone
- whether a top-level Rust root is allowed to be a standalone package
- whether nested workspaces are allowed at all

## Shared-facts expectation

This family should introduce shared architecture-placement facts that other architecture families can reuse:

- all discovered Rust roots
- root zone classification
- zone ownership
- overlap/nesting facts
- attached workspace-local family artifacts
- legality facts for root and workspace-local file placement

Those shared facts may later be consumed by:
- `RS-HEXARCH`
- `RS-ARCH`

But `RS-TOPOLOGY` remains the only family that emits repo-global placement findings.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Moving misplaced-root detection into `cargo` | Placement is architecture policy, not Cargo policy |
| Moving top-level workspace enforcement into `cargo` | Root topology is architecture policy; `cargo` should only run after `topology` has accepted the root shape |
| Duplicating misplaced-root rules in both `hexarch` and `arch` | Causes double signaling and drift |
| Letting enable/disable change discovery | Discovery must stay complete; only reporting is conditional |

## Current family shape

The live family now follows the self-hosted workspace split used by the other stabilized Rust families.

```text
apps/guardrail3/crates/app/rs/families/topology/
├── Cargo.toml
├── README.md
├── crates/
│   ├── runtime/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── facts.rs
│   │       ├── inputs.rs
│   │       ├── rs_topology_01_root_classification.rs
│   │       └── rs_topology_01_root_classification_tests/
│   └── assertions/
│       ├── Cargo.toml
│       └── src/
└── test_support/
    ├── Cargo.toml
    └── src/
```

Shared placement substrate:

```text
apps/guardrail3/crates/app/rs/placement/
```

## Implementation notes

- `rs/topology` is implemented as a repo-global family with one production file per rule and one rule-specific `*_tests/` directory per rule.
- Shared Rust root discovery/classification lives under `rs/placement/` so `hexarch` and `arch` can reuse the same root inventory instead of rediscovering `Cargo.toml` roots independently.
- Classification is segment-based rather than top-level-only. This intentionally makes illegal nested shapes such as `apps/<app>/packages/<pkg>` and `packages/<pkg>/apps/<app>` visible as:
  - `RS-TOPOLOGY-01` ambiguous roots
  - `RS-TOPOLOGY-03` dual ownership
  - `RS-TOPOLOGY-04` illegal layout overlap pairs
- `RS-TOPOLOGY-02` resolves reporting from `hexarch` / `arch` enablement only. Discovery always runs even when both owner families are disabled.
- `RS-TOPOLOGY-07` owns fail-closed input integrity for this family:
  - malformed `guardrail3.toml`
  - unreadable-present `guardrail3.toml`
  - unreadable discovered eligible live `Cargo.toml` content in the cached tree
  - malformed governed app/package `Cargo.toml`
  - malformed eligible live out-of-zone `Cargo.toml`
  - governed app/package roots that declare `arch_role`
- The next `topology` hardening pass should move "workspace only at the top, packages only below" into explicit root-topology rules here rather than leaving it split across `hexarch`, `arch`, and `cargo`.
- Nested-workspace prohibition must be repo-global, not only app-local. `hexarch` already forbids nested workspaces inside app roots, but the harsher contract belongs at the shared placement/topology layer.
- The next architectural pass should stop thinking in terms of "run `topology` first as a family."
  Instead:
  - shared structure is built once
  - shared legality is derived once
  - `RS-TOPOLOGY` reports that legality
  - mapper/runners consume the same legality to build legal family surfaces and per-workspace invocations

## Gaps closed

- Added the new `rs/topology` checker family under the current `ProjectTree` architecture.
- Added reusable root-placement facts for all discovered Rust `Cargo.toml` roots, owner-family candidates, and illegal app/package overlap pairs.
- Added golden and attack-vector coverage for all eight live rules using rule-specific test directories.
- Wired `topology` into Rust runtime family selection and user-facing help/guide family lists.

## Remaining gaps

- Shared placement facts are still consumed directly by too few families; `rs/hexarch` and `rs/arch` still need to finish migrating onto the same routed root substrate.
- `arch` now owns package-root architecture enablement and the old package-only layered family no longer exists as a runtime family.
- Full `cargo test -p guardrail3` verification is currently blocked by unrelated existing test-callsite signature drift in other families (`code`, `garde`, and `test`) that predates this family.
- The family still does not implement the intended repo-global Rust topology contract:
  - top-level roots must be workspaces
  - loose top-level packages must be rejected
- nested workspaces must be rejected everywhere, not only in app-local `hexarch`
- workspace membership exactness beneath governed workspaces must be enforced
- workspace member path escape must be rejected
  - auxiliary roots still need an explicit topology decision
- Hybrid top-level root policy remains intentionally unresolved and must be decided before implementing the topology rules above.

## Hexarch topology migration audit

The current `hexarch` family still owns several workspace/topology rules that are no longer app-shape-specific once `topology` becomes the universal Rust root-topology family.

These rules should move into `topology` or be subsumed by generalized `topology` rules:

- `RS-HEXARCH-08` — app Cargo.toml is workspace
  - move to `topology`
  - generalize from app roots to all top-level governed Rust roots
- `RS-HEXARCH-10` — members within app boundary
  - move to `topology`
  - generalize from app boundary to owning workspace root boundary
  - this should also cover explicit `../` escape, not only app-relative path resolution
- `RS-HEXARCH-27` — nested workspace forbidden under app root
  - move to `topology`
  - generalize from app-only to repo-global "no nested workspaces anywhere"

The following `hexarch` rule is not app-shape-specific either and should be revisited during the same migration:

- `RS-HEXARCH-11` — root workspace does not include apps
  - under the stricter topology model, this is really a repo-global root-topology constraint
  - likely outcome:
    - either move it into `topology` as part of the generalized "repo-root workspace cannot claim nested governed workspaces" rule
    - or delete it after broader `topology` rules make the special-case check redundant

These rules should remain in `hexarch` because they are app-shape-specific rather than universal root-topology policy:

- `RS-HEXARCH-01` through `RS-HEXARCH-06` — `crates/` tree shape, container shape, leaf validity
- `RS-HEXARCH-12` — app-level `src/` banned
- `RS-HEXARCH-13` through `RS-HEXARCH-26` — dependency direction, dependency integrity, cross-app dependency policy, and source-surface enforcement

Migration notes:

- do not port the current app-specific wording directly; rewrite these rules around governed workspace roots and lower-level package roots
- `topology` should own the shared facts needed to express:
  - top-level workspace roots
  - lower-level live package roots beneath each top-level workspace
  - workspace-member coverage exactness
  - nested workspace detection
  - workspace-member path escape
- after migration, `hexarch` should consume the accepted app workspace boundary from `topology` instead of re-owning generic workspace legality

## Implementation surface

The concrete work surface for the next migration pass is:

1. merge the current root/file discovery concepts into one shared Rust structure stage
2. add a shared Rust legality stage before family mapping
3. make `family_mapper` legality-aware so it maps legal family surfaces rather than raw discovered files
4. make `runtime/src/runners.rs` own invocation fan-out:
   - one invocation for global families
   - one invocation per legal workspace for workspace-local families
5. remove repo-global placement judgment from workspace-local families and leave them with content-only local validation
