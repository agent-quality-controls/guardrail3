# Rust Generator — `libarch`

## Generated artifacts

### Flat library mode

- `<library_root>/Cargo.toml`
- `<library_root>/src/lib.rs`

### Layered library mode

- `<library_root>/Cargo.toml`
- `<library_root>/src/lib.rs`
- `<library_root>/crates/api/Cargo.toml`
- `<library_root>/crates/api/src/lib.rs`
- `<library_root>/crates/core/Cargo.toml`
- `<library_root>/crates/core/src/lib.rs`
- optional `<library_root>/crates/infra/Cargo.toml`
- optional `<library_root>/crates/infra/src/lib.rs`

## Ownership mode

- scaffold for directories and starter files
- semantic-patch for root workspace/facade wiring in `<library_root>/Cargo.toml`

## Root selection

`libarch` owns one target library root at a time.

That library root may exist:
- as a standalone Rust package root
- inside a mixed repo beside app workspaces
- inside a packages area beside other libraries

When the library root is layered:
- it is both the workspace root and the root facade package
- it owns its own workspace boundary
- it is not treated as an ordinary member of an ancestor workspace
- ancestor workspaces must not include the layered library root or its `crates/api`, `crates/core`, or `crates/infra` as members

The generator must not modify sibling app workspaces or unrelated package roots when generating one library root.

## Required generator contract

### Flat library mode

- flat mode produces a valid single-crate library root
- generated flat roots stay below the `RS-LIBARCH` escalation boundary by default
- root `src/lib.rs` is a facade

### Layered library mode

- root `Cargo.toml` is both:
  - workspace root
  - facade package manifest
- `crates/api` and `crates/core` always exist
- `crates/infra` exists only when requested
- workspace members exactly match the generated layered crates
- root `src/lib.rs` exports public surface from `api`
- generated dependency wiring matches the `RS-LIBARCH` direction rules:
  - `api -> core` allowed
  - `infra -> core` allowed
  - `core -> api` forbidden
  - `core -> infra` forbidden
  - `api -> infra` forbidden
- `infra` does not become the direct public package surface

## Checker target

- `.plans/todo/checks/rs/libarch.md`
- checker family: `RS-LIBARCH`

Generated flat roots must remain compatible with:
- the flat-library side of `RS-LIBARCH`

Generated layered roots must satisfy:
- `RS-LIBARCH-02`
- `RS-LIBARCH-03`
- `RS-LIBARCH-04`
- `RS-LIBARCH-05`
- `RS-LIBARCH-06`
- `RS-LIBARCH-07`
- `RS-LIBARCH-08`
- `RS-LIBARCH-09`
- `RS-LIBARCH-10`
- `RS-LIBARCH-11`

## Parity contract

1. `generate -> validate`
- generate flat and layered library roots
- `RS-LIBARCH` passes for the generated mode

2. `generate twice`
- second generation leaves the scaffold unchanged semantically

3. negative mutation
- removing or miswiring one generated layer, member, dependency edge, or root export surface produces the exact `RS-LIBARCH-*` finding

4. mixed-repo exactness
- a generated layered library remains self-contained even when sibling app workspaces and other package roots exist beside it
