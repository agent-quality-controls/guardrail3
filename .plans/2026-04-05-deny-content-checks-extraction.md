# Deny Content Checks Extraction

## Package

- `packages/g3rs-deny-config-checks`

This package owns only typed `deny.toml` content validation. It does not own
file discovery, root selection, profile resolution, coverage, or same-root
conflict analysis.

## Public Interface

```rust
use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub struct G3RsDenyConfigChecksInput {
    pub deny_rel_path: String,
    pub deny: DenyToml,
}

pub fn check(input: &G3RsDenyConfigChecksInput) -> Vec<G3CheckResult>;
```

## App / Package Split

### Stays in app

- `RS-DENY-01` — coverage
- `RS-DENY-03` — same-root shadowing
- parse-failure routing for deny config files
- authoritative config selection from routed family files
- profile resolution from `guardrail3.toml`

### Moves into package

- `g3rs-deny/deprecated-advisories`
- `g3rs-deny/advisories-baseline`
- `g3rs-deny/stricter-advisories-inventory`
- `g3rs-deny/graph-all-features`
- `g3rs-deny/graph-no-default-features`
- `g3rs-deny/highlight-inventory`
- `g3rs-deny/allow-wildcard-paths`
- `g3rs-deny/wildcards-inventory`
- `g3rs-deny/license-allow-baseline`
- `g3rs-deny/confidence-threshold`
- `g3rs-deny/copyleft-allowlist`
- `g3rs-deny/unknown-sources-policy`
- `g3rs-deny/allow-git-inventory`
- `g3rs-deny/tokio-full-ban`
- `g3rs-deny/extra-feature-bans-inventory`
- `g3rs-deny/skip-hygiene`
- `g3rs-deny/ignore-hygiene`
- `g3rs-deny/duplicate-entries`
- `g3rs-deny/unknown-keys`
- `g3rs-deny/license-exceptions-inventory`
- `g3rs-deny/allow-override-channel`
- `g3rs-deny/extra-deny-bans-inventory`

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
packages/g3rs-deny-config-checks/
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

- `advisories/` owns `g3rs-deny/deprecated-advisories..08`
- `bans/` owns `g3rs-deny/highlight-inventory..13`, `21`, `22`, `27`
- `licenses/` owns `g3rs-deny/confidence-threshold..16`
- `sources/` owns `g3rs-deny/allow-git-inventory..20`, `23`, `24`, `28`, `29`

The runtime `check(&G3RsDenyConfigChecksInput)` entrypoint fans out into these
areas. Internal rule functions should take direct params, not extra typed
wrapper structs.

## Parser Dependency

`g3rs-deny-config-checks` depends on `deny-toml-parser` only. The package
receives an already parsed `DenyToml` from the app orchestrator.

## Current Status

Completed:
1. `g3rs-deny-config-checks` is scaffolded and compiles.
2. The package owns `g3rs-deny/deprecated-advisories`, `05`, `06`, `07`, `08`, `10`, `11`, `12`,
   `13`, `14`, `15`, `16`, `18`, `19`, `20`, `21`, `22`, `23`, `24`, `27`,
   `28`, and `29`.
3. The app deny family delegates those rules to the package and keeps
   `RS-DENY-01`, `03`, `09`, `17`, `25`, `26`, and `30` locally.
