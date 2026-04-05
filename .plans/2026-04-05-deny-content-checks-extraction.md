# Deny Content Checks Extraction

## Package

- `packages/g3-deny-content-checks`

This package owns only typed `deny.toml` content validation. It does not own
file discovery, root selection, profile resolution, coverage, or same-root
conflict analysis.

## Public Interface

```rust
use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub struct G3DenyContentChecksInput {
    pub deny_rel_path: String,
    pub deny: DenyToml,
}

pub fn check(input: &G3DenyContentChecksInput) -> Vec<G3CheckResult>;
```

## App / Package Split

### Stays in app

- `RS-DENY-01` — coverage
- `RS-DENY-03` — same-root shadowing
- parse-failure routing for deny config files
- authoritative config selection from routed family files
- profile resolution from `guardrail3.toml`

### Moves into package

- `RS-DENY-04`
- `RS-DENY-05`
- `RS-DENY-06`
- `RS-DENY-07`
- `RS-DENY-08`
- `RS-DENY-10`
- `RS-DENY-11`
- `RS-DENY-12`
- `RS-DENY-13`
- `RS-DENY-14`
- `RS-DENY-15`
- `RS-DENY-16`
- `RS-DENY-18`
- `RS-DENY-19`
- `RS-DENY-20`
- `RS-DENY-21`
- `RS-DENY-22`
- `RS-DENY-23`
- `RS-DENY-24`
- `RS-DENY-27`
- `RS-DENY-28`
- `RS-DENY-29`

### Deferred for parser / policy-context reasons

- `RS-DENY-09`
- `RS-DENY-17`
- `RS-DENY-25`
- `RS-DENY-26`
- `RS-DENY-30`

These stay app-side. `RS-DENY-09`, `25`, `26`, and `30` depend on
app-resolved profile context. `RS-DENY-17` remains app-side even after the
parser expansion because the current extraction boundary keeps license
exception policy handling with the app-owned rules.

## Internal Layout

```text
packages/g3-deny-content-checks/
  Cargo.toml
  README.md
  src/
    lib.rs

  crates/
    types/
      Cargo.toml
      src/
        lib.rs

    runtime/
      Cargo.toml
      src/
        lib.rs
        run.rs
        advisories/
          mod.rs
        bans/
          mod.rs
        licenses/
          mod.rs
        sources/
          mod.rs

    assertions/
      Cargo.toml
      src/
        lib.rs
```

## Rule Organization

- `advisories/` owns `RS-DENY-04..08`
- `bans/` owns `RS-DENY-10..13`, `21`, `22`, `27`
- `licenses/` owns `RS-DENY-14..16`
- `sources/` owns `RS-DENY-18..20`, `23`, `24`, `28`, `29`

The runtime `check(&G3DenyContentChecksInput)` entrypoint fans out into these
areas. Internal rule functions should take direct params, not extra typed
wrapper structs.

## Parser Dependency

`g3-deny-content-checks` depends on `deny-toml-parser` only. The package
receives an already parsed `DenyToml` from the app orchestrator.

## Current Status

Completed:
1. `g3-deny-content-checks` is scaffolded and compiles.
2. The package owns `RS-DENY-04`, `05`, `06`, `07`, `08`, `10`, `11`, `12`,
   `13`, `14`, `15`, `16`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `27`,
   `28`, and `29`.
3. The app deny family delegates those rules to the package and keeps
   `RS-DENY-01`, `03`, `09`, `17`, `25`, `26`, and `30` locally.
