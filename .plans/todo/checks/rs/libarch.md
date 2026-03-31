# RS-LIBARCH — Rust library architecture checker

> Superseded as the primary family plan by [`.plans/by_family/rs/libarch.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/libarch.md).
> Keep this file as the detailed rule ledger and implementation-history reference.

**Input:** package roots + Cargo.toml files + crate directory structure + crate dependency edges
**Parser:** TOML + directory structure
**Current code:** `apps/guardrail3/crates/app/rs/families/libarch/`

## Implementation mapping contract

- exactly one `RS-LIBARCH-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `facts.rs` and `inputs.rs` may contain shared discovery, typed inputs, and normalization helpers only

Forbidden:

- grouped family test files such as `libarch_tests.rs`
- helper files that hide multiple unrelated rule predicates behind one API

## Why this family exists

`RS-CODE` and `RS-DEPS` can force library code quality, but they do not force library architecture.

That leaves libraries as the main junk-drawer risk:
- flat single-crate packages grow too large
- public API, core logic, and external integration collapse together
- agents keep adding files and dependencies without ever being forced to split

`RS-LIBARCH` exists to solve that.

## Core model

There are two valid library modes.

### Mode A — Flat library

Allowed only while the package stays below the library-complexity thresholds.

Shape:

```text
package/
  Cargo.toml
  src/
    lib.rs
    ...
```

This mode relies on:
- `RS-CODE` for facade/API quality
- `RS-DEPS` for dependency pressure

### Mode B — Layered library workspace

Required once the package exceeds any library-complexity threshold.

Shape:

```text
package/
  Cargo.toml              # workspace root + root facade package
  src/
    lib.rs
  crates/
    api/
    core/
    infra/                # optional
```

Intent:
- `api` = public surface
- `core` = pure/internal logic
- `infra` = external integration / IO / glue

This is deliberately smaller and more boring than hexarch.

## Discovery / ownership model

Owned package roots are local `Cargo.toml` package roots that define a library target.

Library classification is:
- root has `[package]`
- and either:
  - `src/lib.rs` exists
  - or `[lib]` is declared

The family does not apply to:
- binary-only packages
- workspaces with no root/package library target

Once a flat library crosses a threshold, the owned package root is still the package root at the top of that library package. In layered mode, that same package root becomes:
- the workspace root
- and the root facade package

## Exact measurement inputs

Escalation measurements are per crate, from that crate’s own `Cargo.toml`.

They mean:
- direct dependencies: unique direct dependency names on that crate
- module depth: nested Rust module/source path depth from that crate root
- sibling subdirectories: subdirectories in one Rust source/module directory
- sibling `.rs` files: `.rs` files in one Rust source/module directory

Future verification must not reinterpret these as repo-wide totals.

## Input integrity / fail-closed expectations

The family depends on:
- readable package/workspace `Cargo.toml` files
- readable crate directory structure
- readable dependency-edge facts
- readable root facade source when facade/export rules apply

Malformed required inputs must not silently downgrade a too-large library into a “flat library allowed” clean pass.

## Escalation thresholds

A library package may remain flat only while it stays under all of these thresholds.

If it exceeds **any** of them, it must adopt the layered library workspace shape:

- direct dependencies `> 12`
- module depth `> 3`
- sibling subdirectories in one Rust source directory `> 4`
- sibling `.rs` files in one Rust source directory `> 6`

Notes:
- these are lower than the generic anti-sprawl caps on purpose
- the point is to force architecture early, not only catch extreme garbage late
- measurements are per crate, from that crate’s `Cargo.toml`

## Planned rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-LIBARCH-01 | Error | Library package exceeds any library-complexity threshold but is not a layered workspace | Implemented |
| RS-LIBARCH-02 | Error | Layered library package root Cargo.toml must be a workspace | Implemented |
| RS-LIBARCH-03 | Error | `crates/` must exist at layered library root | Implemented |
| RS-LIBARCH-04 | Error | Required layered crates are exactly `{api, core}` with optional `infra` | Implemented |
| RS-LIBARCH-07 | Error | `core` must not depend on `api` | Implemented |
| RS-LIBARCH-08 | Error | `core` must not depend on `infra` | Implemented |
| RS-LIBARCH-09 | Error | `api` may depend only on `core` and allowed external deps, not on `infra` | Implemented |
| RS-LIBARCH-10 | Error | `infra` may depend on `core`, but must not be re-exported directly as public package surface | Implemented |
| RS-LIBARCH-11 | Error | Root package facade must export public surface from `api`, not directly from `core` / `infra` | Implemented |

## Rule intent

### RS-LIBARCH-01 — Escalation required

If a library package crosses any escalation threshold and is still a flat single-crate package, that is an error.

This is the core rule that forces architectural split.

### RS-LIBARCH-02 — Layered root is a workspace + facade package

Once layered mode is required, the package root `Cargo.toml` must be:
- a workspace root
- and the root facade package

This is what allows `RS-LIBARCH-11` to own a real root facade.

### RS-LIBARCH-03 — `crates/` exists

Once layered mode is required, the package root must contain `crates/`.

### RS-LIBARCH-04 — Exact layered crate set

Inside `crates/`:
- `api` is required
- `core` is required
- `infra` is optional
- no other layer crate dirs are allowed

### RS-LIBARCH-07 — `core` must not depend on `api`

`core` is the bottom layer and must not depend upward on the public-surface layer.

### RS-LIBARCH-08 — `core` must not depend on `infra`

`core` must stay free of integration/external glue.

### RS-LIBARCH-09 — `api` must not depend on `infra`

Allowed:
- `api -> core`
- `infra -> core`

Forbidden:
- `api -> infra`

Rationale:
- the public surface should not be built on top of integration glue

### RS-LIBARCH-10 — `infra` must not become the public package surface

`infra` may depend on `core`, but it must not be re-exported directly as the public package surface.

### RS-LIBARCH-11 — Root facade exports from `api`

The root facade package must export public surface from `api`, not directly from `core` or `infra`.

## Relationship to other families

### RS-ARCH

`RS-ARCH` owns:
- repo-global Rust root placement
- zone classification (`apps/*`, `packages/*`, `other`)
- misplaced-root reporting
- overlap/ownership legality between app/package zones
- workspace-membership exactness for governed workspaces

`RS-LIBARCH` does not emit repo-global misplaced-root findings.
It assumes `RS-ARCH` has already answered whether a Rust root belongs in the package zone at all.

### RS-CODE

`RS-CODE` still owns:
- facade-only `lib.rs`
- public-field bans
- bad public error forms
- generic caps
- source-tree sprawl caps
- string dispatch cap

`RS-LIBARCH` does not replace those.
It uses some of them as escalation inputs.

### RS-DEPS

`RS-DEPS` still owns:
- direct dependency count cap
- allowlist policy
- dependency tool enforcement

`RS-LIBARCH-01` should reuse the direct dependency count as one escalation signal.

### RS-HEXARCH

`RS-LIBARCH` is the library equivalent of “force architecture once complexity exists”.

It should reuse the same style of checks where possible:
- workspace shape
- member matching
- dependency direction

But it is not hexarch:
- no adapters/ports split
- only `api/core/infra`

## Explicitly rejected for now

| Finding | Why rejected |
|---------|-------------|
| Every library must always be multi-crate | Too much ceremony for genuinely small libraries |
| Intra-crate module dependency enforcement instead of subcrates | Not robust enough for hard guardrails |
| Additional optional layers beyond `infra` | Too much freedom weakens the architecture |
| Public re-export graph analysis beyond facade path checks | Too brittle for v1 |

## Implementation notes

This family should be crate/workspace-based, not AST-heavy.

Suggested facts:
- package roots classified as libraries
- package complexity facts reused from `RS-CODE` / `RS-DEPS`
- layered crate dirs
- inter-crate dependency edges

This probably wants the same style of dependency-edge resolution already built for `RS-HEXARCH`.

## Target family shape

This family should follow the same new checker architecture as the other Rust families.

Target folder:

```text
apps/guardrail3/crates/app/rs/checks/rs/libarch/
├── mod.rs
├── facts.rs
├── inputs.rs
├── rs_libarch_01_*.rs
├── ...
└── rs_libarch_11_*.rs
```

Rules must be:
- one production file per rule
- pure functions over minimal typed inputs

Tests must be:
- one rule-specific sidecar test module directory per rule
- named `rs_libarch_xx_*_tests/`
- split by attack class, not lumped into one family file

Example:

```text
rs_libarch_01_escalation_required.rs
rs_libarch_01_escalation_required_tests/
  mod.rs
  golden.rs
  threshold_crossing.rs
  false_positives.rs
  severity_exactness.rs
```

Forbidden:
- grouped production files
- family-wide grouped test files
- inline production-file test modules as the default
- rules that receive `ProjectTree` directly instead of typed inputs

## Suggested facts

- package roots classified as libraries
- crate complexity facts:
  - direct dependency count
  - module depth
  - sibling directory count
  - sibling `.rs` file count
- layered crate directories present at one package root
- inter-crate dependency edges
- root facade export targets

## Suggested inputs

- one package root with complexity facts
- one layered-workspace root
- one inter-crate dependency edge
- one root-facade export surface

## Open questions

- Whether `api -> infra` should ever be allowed for facade/wiring convenience.
  - Current answer in this plan: no.
- Whether `infra` should be allowed to be absent even after escalation.
  - Current answer: yes, optional.
- Whether the layered root should be anything other than the workspace root + root facade package.
  - Current answer: no. Layered mode requires the root package to remain the facade package so `RS-LIBARCH-11` owns a concrete root export surface.
