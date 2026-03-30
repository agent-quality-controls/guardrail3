# RS-ARCH — Rust root placement and architecture ownership checker

> Superseded as the primary family plan by [`.plans/by_family/rs/arch.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/arch.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** all discovered Rust `Cargo.toml` roots + guardrail config + owned workspace/package classification
**Parser:** TOML + directory structure
**Current code:** `apps/guardrail3/crates/app/rs/families/arch/`, `apps/guardrail3/crates/app/rs/placement/`

## Why this family exists

`RS-HEXARCH` owns app architecture inside `apps/*`.
`RS-LIBARCH` owns library/package architecture inside `packages/*`.

What neither family should own is the repo-global question:

- where Rust roots are allowed to live
- which architecture family owns a given Rust root
- whether a Rust root is misplaced or ambiguously owned

That is a separate concern. It should not be duplicated in both `hexarch` and `libarch`, and it should not be buried in `cargo`.

`RS-ARCH` exists to own that global placement and ownership contract once.

Current source of truth:
- `apps/guardrail3/crates/app/rs/families/arch/README.md` for family-local behavior
- `apps/guardrail3/crates/app/rs/README.md` for shared routed architecture
- this file for the live rule inventory

Older handoffs are historical only.

## Scope

This family is deliberately narrow.

It owns only:
- Rust root discovery/classification
- root placement under architecture zones
- root-to-family ownership
- overlap/nesting legality between architecture zones
- repo-global Rust root topology policy

It does **not** own:
- app-internal hex structure
- library-internal layered structure
- workspace-member semantics inside an app or package boundary
- generic Cargo manifest policy

Those remain in `RS-HEXARCH`, `RS-LIBARCH`, and `RS-CARGO`.

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
- `package` roots are candidates for `RS-LIBARCH`
- `other` roots are misplaced when Rust architecture enforcement is active

The topology policy above is orthogonal to zone ownership:

- a root may be correctly zoned and still be illegal if it is a loose top-level package
- a root may be correctly zoned and still be illegal if it declares a nested workspace
- a lower-level crate may be correctly zoned and still be illegal if it is live but omitted from the parent workspace

## Ownership model

This family owns repo-global placement findings.

That means:
- `RS-HEXARCH` must not emit “Cargo root misplaced globally”
- `RS-LIBARCH` must not emit it either
- `RS-ARCH` emits it once

This avoids:
- double signaling
- drift between `hexarch` and `libarch`
- hidden policy duplication

## Discovery / classification model

The family must discover every Rust `Cargo.toml` root in the repo and classify each root by zone.

Classification is path-based:
- roots under `apps/<name>/...` belong to the `app` zone
- roots under `packages/<name>/...` belong to the `package` zone
- everything else is `other`

The family must also detect illegal overlap:
- one Rust root must not be simultaneously treated as both app-owned and package-owned
- illegal nesting between architecture roots must be surfaced explicitly

## Conditional applicability

Placement reporting depends on architecture-family enablement.

The intended behavior is:

- if both `hexarch` and `libarch` are enabled
  - misplaced `other` roots are `Error`
- if only `hexarch` is enabled
  - misplaced `other` roots are `Error`
- if only `libarch` is enabled
  - misplaced `other` roots are `Error`
- if both are disabled
  - no placement finding is emitted

Discovery/classification still happens either way.
Enablement changes reporting, not root discovery.
When reporting is inactive, `RS-ARCH-02` should still emit an inventory/info result saying misplaced-root enforcement is suppressed.

## Input integrity / fail-closed expectations

The family depends on:
- readable `Cargo.toml` discovery
- readable guardrail config
- readable directory structure for zone classification

Malformed required placement/config inputs must not silently suppress misplaced-root findings.

## Planned rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-ARCH-01 | Error | Every discovered Rust root must classify as `app`, `package`, or `other` without ambiguity | Implemented |
| RS-ARCH-02 | Error | When Rust architecture enforcement is active, no discovered Rust root may live in `other` | Implemented |
| RS-ARCH-03 | Error | No Rust root may be simultaneously owned by both app and package zones | Implemented |
| RS-ARCH-04 | Error | Illegal nesting/overlap between app roots and package roots is forbidden | Implemented |
| RS-ARCH-05 | Error | Scoped `arch` config is forbidden; `arch` is global-only | Implemented |
| RS-ARCH-06 | Error | Governed roots must stay coherent with owner-family enablement | Implemented |
| RS-ARCH-07 | Error | Required `arch` inputs fail closed when unreadable or malformed | Implemented |
| RS-ARCH-08 | Info | Declared auxiliary roots are surfaced explicitly in reports | Implemented |
| RS-ARCH-09 | Error | Every live top-level Rust root must be a workspace root | Planned |
| RS-ARCH-10 | Error | Loose top-level package roots are forbidden | Planned |
| RS-ARCH-11 | Error | Nested workspaces are forbidden even when excluded or not referenced by the parent workspace | Planned |
| RS-ARCH-12 | Error | Live lower-level Rust crates under a governed workspace must be declared workspace members | Planned |
| RS-ARCH-13 | Error | Workspace member paths must not escape the workspace root | Planned |
| RS-ARCH-14 | Error | Auxiliary top-level Rust roots must obey the same top-level workspace rule | Planned |
| RS-ARCH-15 | Policy | Hybrid top-level workspace roots need an explicit allow/forbid decision | Planned |

## Rule intent

### RS-ARCH-01 — Root classification is unambiguous

Every discovered Rust root must resolve to one clear zone classification.

This catches:
- ambiguous path ownership
- broken zone-classification logic
- silently skipped roots

### RS-ARCH-02 — No misplaced Rust roots

When either `hexarch` or `libarch` is enabled, an unexpected Rust root in `other` is an error.

This is the missing global-placement rule.

### RS-ARCH-03 — No dual ownership

A single Rust root must not be governed by both app and package architecture zones.

### RS-ARCH-04 — No illegal zone overlap

App and package architecture boundaries must not overlap or nest in a way that makes ownership unclear.

This is about root-zone legality, not member legality inside a single workspace.

Treat this as a layout-level rule, not a duplicate root-level one:
- `RS-ARCH-01` owns ambiguous per-root classification
- `RS-ARCH-03` owns per-root dual ownership
- `RS-ARCH-04` owns the illegal app/package containment pair itself

### RS-ARCH-05 — Scoped `arch` config is forbidden

`arch` is repo-global and must be configured only under `[rust.checks]`.

### RS-ARCH-06 — Owner-family enablement is coherent

For a governed root:
- app-zone ownership maps to `hexarch`
- package-zone ownership maps to `libarch`
- app-scoped `hexarch` overrides win over the global `hexarch` default for every root under that app

The family should surface impossible or contradictory ownership states explicitly.

### RS-ARCH-07 — Required inputs fail closed

Unreadable-present or malformed required `arch` inputs must surface explicit errors instead of silently degrading into absence.

That includes:
- malformed governed app/package `Cargo.toml`
- governed roots that declare `arch_role`
- malformed auxiliary metadata on out-of-zone roots

### RS-ARCH-08 — Declared auxiliary roots are explicit

Roots outside governed zones may opt into `auxiliary` status explicitly.
That exemption should stay visible as inventory/info output.

### RS-ARCH-09 — Every live top-level Rust root is a workspace

For any live Rust root that is not nested beneath another live Rust workspace root:

- `[workspace]` must be present
- pure top-level package manifests are not enough

This is the main topology hardening rule that turns "workspace" into the only allowed top-level Rust product shape.

### RS-ARCH-10 — Loose top-level packages are forbidden

Any top-level Rust root that contains `[package]` but not `[workspace]` is forbidden.

This rule exists separately from `RS-ARCH-09` so the family can emit a direct "loose package root" failure rather than only an indirect "workspace missing" failure.

### RS-ARCH-11 — Nested workspaces are forbidden

Any discovered live Rust root beneath a governed workspace root must not declare `[workspace]`.

This must still fail when the nested workspace is:

- listed in the parent workspace members
- excluded from the parent workspace
- not referenced by the parent workspace at all

Discovery is structural, not membership-based.

### RS-ARCH-12 — Live lower-level Rust crates must be declared members

If a live lower-level Rust crate exists beneath a governed workspace root and is not itself excluded by structural discovery rules, it must be declared in the parent workspace membership set.

This forbids shadow crates that exist on disk but are quietly omitted from the workspace.

### RS-ARCH-13 — Workspace member paths must stay inside the root

Parent workspace manifests must not use member paths that escape the workspace root, including `../` traversal.

Workspace membership must not become a backdoor for cross-root ownership drift.

### RS-ARCH-14 — Auxiliary roots obey the same top-level workspace rule

If out-of-zone Rust roots remain allowed through explicit `auxiliary` declaration, they should still be required to be top-level workspace roots rather than loose standalone packages.

Otherwise auxiliary roots become an escape hatch that reintroduces the same unstable topology this family is trying to forbid.

### RS-ARCH-15 — Hybrid top-level roots need an explicit policy

The family needs an explicit decision on whether a top-level root may contain both:

- `[workspace]`
- `[package]`

Options:

- allow hybrid top roots only
- forbid hybrid roots entirely

Do not leave this implicit in parser behavior or in downstream family assumptions.

## Relationship to other families

### RS-HEXARCH

`RS-HEXARCH` owns:
- app structure
- app workspace shape
- app member coverage
- app dependency direction

It does **not** own repo-global misplaced-root detection.

### RS-LIBARCH

`RS-LIBARCH` owns:
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

Those shared facts may later be consumed by:
- `RS-HEXARCH`
- `RS-LIBARCH`

But `RS-ARCH` remains the only family that emits repo-global placement findings.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Moving misplaced-root detection into `cargo` | Placement is architecture policy, not Cargo policy |
| Moving top-level workspace enforcement into `cargo` | Root topology is architecture policy; `cargo` should only run after `arch` has accepted the root shape |
| Duplicating misplaced-root rules in both `hexarch` and `libarch` | Causes double signaling and drift |
| Letting enable/disable change discovery | Discovery must stay complete; only reporting is conditional |

## Current family shape

The live family now follows the self-hosted workspace split used by the other stabilized Rust families.

```text
apps/guardrail3/crates/app/rs/families/arch/
├── Cargo.toml
├── README.md
├── crates/
│   ├── runtime/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── facts.rs
│   │       ├── inputs.rs
│   │       ├── rs_arch_01_root_classification.rs
│   │       └── rs_arch_01_root_classification_tests/
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

- `rs/arch` is implemented as a repo-global family with one production file per rule and one rule-specific `*_tests/` directory per rule.
- Shared Rust root discovery/classification lives under `rs/placement/` so future `hexarch` and `libarch` work can reuse the same root inventory instead of rediscovering `Cargo.toml` roots independently.
- Classification is segment-based rather than top-level-only. This intentionally makes illegal nested shapes such as `apps/<app>/packages/<pkg>` and `packages/<pkg>/apps/<app>` visible as:
  - `RS-ARCH-01` ambiguous roots
  - `RS-ARCH-03` dual ownership
  - `RS-ARCH-04` illegal layout overlap pairs
- `RS-ARCH-02` resolves reporting from `hexarch` / `libarch` enablement only. Discovery always runs even when both owner families are disabled.
- `RS-ARCH-07` owns fail-closed input integrity for this family:
  - malformed `guardrail3.toml`
  - unreadable-present `guardrail3.toml`
  - unreadable discovered eligible live `Cargo.toml` content in the cached tree
  - malformed governed app/package `Cargo.toml`
  - malformed eligible live out-of-zone `Cargo.toml`
  - governed app/package roots that declare `arch_role`
- The next `arch` hardening pass should move "workspace only at the top, packages only below" into explicit root-topology rules here rather than leaving it split across `hexarch`, `libarch`, and `cargo`.
- Nested-workspace prohibition must be repo-global, not only app-local. `hexarch` already forbids nested workspaces inside app roots, but the harsher contract belongs at the shared placement/topology layer.

## Gaps closed

- Added the new `rs/arch` checker family under the current `ProjectTree` architecture.
- Added reusable root-placement facts for all discovered Rust `Cargo.toml` roots, owner-family candidates, and illegal app/package overlap pairs.
- Added golden and attack-vector coverage for all eight live rules using rule-specific test directories.
- Wired `arch` into Rust runtime family selection and user-facing help/guide family lists.

## Remaining gaps

- Shared placement facts are still consumed directly by too few families; `rs/hexarch` and future `rs/libarch` still need to finish migrating onto the same routed root substrate.
- `libarch` is still not an implemented runtime family, even though `rs/arch` now understands `libarch` enablement for ownership coherence and misplaced-root reporting.
- Full `cargo test -p guardrail3` verification is currently blocked by unrelated existing test-callsite signature drift in other families (`code`, `garde`, and `test`) that predates this family.
- The family still does not implement the intended repo-global Rust topology contract:
  - top-level roots must be workspaces
  - loose top-level packages must be rejected
  - nested workspaces must be rejected everywhere, not only in app-local `hexarch`
  - shadow lower-level crates omitted from workspace membership must be rejected
  - workspace member path escape must be rejected
  - auxiliary roots still need an explicit topology decision
- Hybrid top-level root policy remains intentionally unresolved and must be decided before implementing the topology rules above.

## Hexarch topology migration audit

The current `hexarch` family still owns several workspace/topology rules that are no longer app-shape-specific once `arch` becomes the universal Rust root-topology family.

These rules should move into `arch` or be subsumed by generalized `arch` rules:

- `RS-HEXARCH-07` — workspace members cover all live app-local Cargo roots
  - keep the intent
  - generalize from "app-local Cargo roots" to "all live lower-level Cargo roots beneath a governed workspace root"
  - this becomes the universal "if it is nested and live, it is a member" rule
- `RS-HEXARCH-08` — app Cargo.toml is workspace
  - move to `arch`
  - generalize from app roots to all top-level governed Rust roots
- `RS-HEXARCH-09` — no extra workspace members
  - move to `arch`
  - generalize from app-local exactness to universal workspace member exactness under a governed root
- `RS-HEXARCH-10` — members within app boundary
  - move to `arch`
  - generalize from app boundary to owning workspace root boundary
  - this should also cover explicit `../` escape, not only app-relative path resolution
- `RS-HEXARCH-27` — nested workspace forbidden under app root
  - move to `arch`
  - generalize from app-only to repo-global "no nested workspaces anywhere"

The following `hexarch` rule is not app-shape-specific either and should be revisited during the same migration:

- `RS-HEXARCH-11` — root workspace does not include apps
  - under the stricter topology model, this is really a repo-global root-topology constraint
  - likely outcome:
    - either move it into `arch` as part of the generalized "repo-root workspace cannot claim nested governed workspaces" rule
    - or delete it after broader `arch` rules make the special-case check redundant

These rules should remain in `hexarch` because they are app-shape-specific rather than universal root-topology policy:

- `RS-HEXARCH-01` through `RS-HEXARCH-06` — `crates/` tree shape, container shape, leaf validity
- `RS-HEXARCH-12` — app-level `src/` banned
- `RS-HEXARCH-13` through `RS-HEXARCH-26` — dependency direction, dependency integrity, cross-app dependency policy, and source-surface enforcement

Migration notes:

- do not port the current app-specific wording directly; rewrite these rules around governed workspace roots and lower-level package roots
- `arch` should own the shared facts needed to express:
  - top-level workspace roots
  - lower-level live package roots beneath each top-level workspace
  - workspace-member coverage exactness
  - nested workspace detection
  - workspace-member path escape
- after migration, `hexarch` should consume the accepted app workspace boundary from `arch` instead of re-owning generic workspace legality
