# Rust Generator Families

This directory mirrors the Rust checker families under `.plans/todo/checks/rs` and `.plans/todo/checks/hooks`, but from the generator side.

The family files in this directory are pure target-state specs.
Current implementation mapping and gaps live in:
- `gap-analysis.md`

## Root taxonomy

Generator contracts use these root kinds:

### Validation root
- the repository root being validated

### Rust policy root
- a root that owns Rust policy files for itself and the Rust units beneath it
- kinds:
  - validation root when it is itself a Rust workspace root or a standalone Rust package root
  - nested Rust workspace root
  - standalone Rust package root that is not a member of any workspace

### Workspace member root
- a crate/package that is a member of a workspace
- never owns local policy files unless the checker family explicitly says it does

### Hex structural root
- a directory that owns a `crates/` tree for hex architecture
- may be:
  - an app workspace root
  - a nested inner hex root inside an outer app workspace
- an inner hex structural root does not become a policy root just because it contains `crates/`

### Layered library root
- a library root that is both:
  - a workspace root
  - the root facade package

Mixed repositories may contain all of these at once:
- root packages workspace
- nested app workspaces
- standalone Rust package roots
- inner hex structural roots
- layered library roots

Family specs must remain correct in that mixed shape.

## Family list

- `fmt.md`
- `toolchain.md`
- `clippy.md`
- `deny.md`
- `cargo.md`
- `release.md`
- `hooks.md`
- `hexarch.md`
- `libarch.md`

## Required questions per family

Every family file freezes:

1. generated artifacts
- exact filenames or exact filename patterns

2. ownership mode
- exact-owned
- semantic-patch
- scaffold

3. root selection
- which roots the family owns
- which roots it must never touch

4. checker target
- which checker family must accept the generated result

5. parity proof
- `generate -> validate`
- `generate twice`
- negative mutation
- scope exactness
