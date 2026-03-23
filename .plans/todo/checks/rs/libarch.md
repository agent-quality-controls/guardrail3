# RS-LIBARCH — Rust library architecture checker

**Input:** package roots + Cargo.toml files + workspace membership + crate directory structure + crate dependency edges
**Parser:** TOML + directory structure
**Current code:** none yet

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
  Cargo.toml              # workspace root
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
| RS-LIBARCH-01 | Error | Library package exceeds any library-complexity threshold but is not a layered workspace | Planned |
| RS-LIBARCH-02 | Error | Layered library package root Cargo.toml must be a workspace | Planned |
| RS-LIBARCH-03 | Error | `crates/` must exist at layered library root | Planned |
| RS-LIBARCH-04 | Error | Required layered crates are exactly `{api, core}` with optional `infra` | Planned |
| RS-LIBARCH-05 | Error | Workspace members must match layered crate dirs | Planned |
| RS-LIBARCH-06 | Error | No extra workspace members outside library boundary | Planned |
| RS-LIBARCH-07 | Error | `core` must not depend on `api` | Planned |
| RS-LIBARCH-08 | Error | `core` must not depend on `infra` | Planned |
| RS-LIBARCH-09 | Error | `api` may depend only on `core` and allowed external deps, not on `infra` | Planned |
| RS-LIBARCH-10 | Error | `infra` may depend on `core`, but must not be re-exported directly as public package surface | Planned |
| RS-LIBARCH-11 | Error | Root package facade must export public surface from `api`, not directly from `core` / `infra` | Planned |

## Rule intent

### RS-LIBARCH-01 — Escalation required

If a library package crosses any escalation threshold and is still a flat single-crate package, that is an error.

This is the core rule that forces architectural split.

### RS-LIBARCH-02..06 — Workspace shape

Once layered mode is required:
- package root must be a workspace
- `crates/` must exist
- only `api` and `core` are required
- only `infra` is optional
- workspace membership must match actual layered crates

This mirrors the shape-enforcement style from `RS-HEXARCH`, but for libraries.

### RS-LIBARCH-07..09 — Dependency direction

Layer rules:
- `core` is the bottom layer
- `api` is the public surface layer
- `infra` is the integration layer

Allowed:
- `api -> core`
- `infra -> core`

Forbidden:
- `core -> api`
- `core -> infra`
- `api -> infra`

Rationale:
- public surface should not depend on integration glue
- core should stay pure of upper/external layers

### RS-LIBARCH-10..11 — Public surface discipline

- `infra` must not become the public package surface directly
- root package facade should export from `api`

This keeps `api` as the stable public contract.

## Relationship to other families

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
- workspace members
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
- workspace members
- inter-crate dependency edges
- root facade export targets

## Suggested inputs

- one package root with complexity facts
- one layered-workspace root
- one workspace-membership comparison
- one inter-crate dependency edge
- one root-facade export surface

## Open questions

- Whether `api -> infra` should ever be allowed for facade/wiring convenience.
  - Current answer in this plan: no.
- Whether `infra` should be allowed to be absent even after escalation.
  - Current answer: yes, optional.
- Whether the root package itself is a published facade crate or a pure workspace root.
  - Current answer: either is acceptable, as long as public surface exports from `api`.
