# Rust Generator — `hexarch`

## Generated artifacts

For a generated app workspace root:
- `<app_root>/Cargo.toml`
- `<app_root>/crates/adapters/`
- `<app_root>/crates/app/`
- `<app_root>/crates/domain/`
- `<app_root>/crates/ports/`
- optional `<app_root>/crates/macros/`

For every generated leaf crate:
- `<leaf_root>/Cargo.toml`
- `<leaf_root>/src/lib.rs`

For empty container directories:
- `.gitkeep` only where required to keep the structural container present

For a generated nested inner hex structural root:
- `<inner_hex_root>/crates/adapters/`
- `<inner_hex_root>/crates/app/`
- `<inner_hex_root>/crates/domain/`
- `<inner_hex_root>/crates/ports/`
- optional `<inner_hex_root>/crates/macros/`

Generated inner hex leaf crates follow the same `<leaf_root>/Cargo.toml` + `src/lib.rs` pattern.

## Ownership mode

- scaffold for structural directories and starter crate files
- semantic-patch for the owning app workspace `Cargo.toml` member set

## Root selection

`hexarch` is a structural family, not a Rust policy-root family.

The generator owns:
- each requested app workspace root
- each requested inner hex structural root inside an existing app workspace

An inner hex structural root:
- contains `crates/`
- does not become its own workspace root
- does not get its own local root `Cargo.toml`
- remains governed by the nearest owning app workspace

The generator must support mixed repositories containing:
- multiple app workspaces
- nested inner hex roots
- library packages outside apps
- non-Rust apps beside Rust app roots

## Required generator contract

- every generated app root satisfies the `RS-HEXARCH` structural contract exactly
- required top-level crate dirs are exactly:
  - `adapters`
  - `app`
  - `domain`
  - `ports`
- `macros` is optional and created only when requested
- `adapters/` and `ports/` contain `inbound/` and `outbound/`
- loose files are not created in structural/container dirs except `.gitkeep`
- every generated leaf is either:
  - a valid Rust crate
  - or a valid nested inner hex structural root
- the owning app workspace member set matches all generated leaf crates, including leaves inside generated inner hex roots
- generator never invents root `src/` at an app workspace root

## Checker target

- `.plans/todo/checks/rs/hexarch.md`
- checker family: `RS-HEXARCH`

The generated result must satisfy the structural and workspace-side `RS-HEXARCH` contract for:
- required containers
- optional macros
- leaf validity
- workspace root shape
- workspace member alignment
- boundary containment

## Parity contract

1. `generate -> validate`
- generate app and nested-inner-hex scaffolds
- `RS-HEXARCH` passes

2. `generate twice`
- second generation leaves the scaffold unchanged semantically

3. negative mutation
- removing or misplacing one generated container, leaf, or workspace member produces the exact `RS-HEXARCH-*` finding

4. mixed-root exactness
- multiple app workspaces can coexist
- inner hex roots remain under their owning app workspace and do not become accidental policy or workspace roots
