# Workspace-local family checks packages

## Architecture

Each family becomes a checks package that validates ONE workspace root's
files against that family's policy. The app handles topology (which roots
exist, coverage, file discovery) and calls the package per-root.

### Split by concern, not by file

Each family checks its own concerns across MULTIPLE files. The files it
needs are passed as parsed types from the parser crates.

Examples:
- Clippy package checks: clippy.toml (thresholds, ban quality, settings) +
  Cargo.toml (disallowed_macros lint) + .cargo/config.toml (CLIPPY_CONF_DIR)
- Garde package checks: .rs files (derive, field constraints) +
  clippy.toml (garde-specific bans present) + Cargo.toml (garde dep)
- Cargo package checks: Cargo.toml (lints, edition, resolver, members)

No family checks another family's concerns. Garde checks "are my bans in
clippy.toml" — clippy doesn't know about garde.

### What stays in the app

- Topology: root discovery, workspace membership, zone classification
- Coverage: "does every root have a clippy.toml" (RS-CLIPPY-01, RS-DENY-01)
- File discovery: finding and reading files from the project tree
- Parsing: calling parser crates, constructing typed inputs
- Iteration: calling the checks package once per workspace root
- Result collection: merging results from all packages

### What moves to the package

- All content validation rules
- Policy knowledge (expected thresholds, required bans, etc.)
- Profile-aware behavior (Application vs Library differences)

## Per-family input types

The app parses all files once and passes typed structs. Every family
defines its own input struct using types from the parser crates.

### Clippy
```rust
use clippy_toml_parser::ClippyToml;
use cargo_toml_parser::CargoToml;
use cargo_config_toml_parser::CargoConfig;

pub struct GrdzClippyChecksInput {
    pub clippy_config: Option<ClippyToml>,
    pub clippy_config_rel_path: Option<String>,
    pub cargo_manifest: CargoToml,
    pub cargo_config: Option<CargoConfig>,
    pub cargo_config_rel_path: Option<String>,
    pub profile: GrdzProfile,
}

pub fn check(input: &GrdzClippyChecksInput) -> Vec<GrdzCheckResult>
```

Rules in package: RS-CLIPPY-02..22, 25 (thresholds, bans, settings, typos, parse)
Rules in app: RS-CLIPPY-01 (coverage), RS-CLIPPY-12 (placement), RS-CLIPPY-23 (guardrail3.toml)

### Deny
```rust
use deny_toml_parser::DenyToml;

pub struct GrdzDenyChecksInput {
    pub deny_config: Option<DenyToml>,
    pub deny_config_rel_path: Option<String>,
    pub profile: GrdzProfile,
}

pub fn check(input: &GrdzDenyChecksInput) -> Vec<GrdzCheckResult>
```

Rules in package: RS-DENY-04..30 (all content validation)
Rules in app: RS-DENY-01 (coverage), RS-DENY-03 (shadowing)

### Cargo
```rust
use cargo_toml_parser::CargoToml;
use guardrail3_toml::Guardrail3Config;

pub struct GrdzCargoChecksInput {
    pub workspace_manifest: CargoToml,
    pub workspace_manifest_rel_path: String,
    pub member_manifests: Vec<(String, CargoToml)>,  // (rel_path, manifest)
    pub guardrail3_config: Option<Guardrail3Config>,
    pub profile: GrdzProfile,
}

pub fn check(input: &GrdzCargoChecksInput) -> Vec<GrdzCheckResult>
```

Rules in package: RS-CARGO-01..15 (all lint, edition, resolver rules)
Rules in app: none — all cargo rules are content validation

### Fmt
```rust
use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

pub struct G3FmtContentChecksInput {
    pub rustfmt_rel_path: String,
    pub rustfmt: RustfmtToml,
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
    pub toolchain_rel_path: String,
    pub toolchain: RustToolchainToml,
}

pub fn check(input: &G3FmtContentChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-FMT-02, RS-FMT-03, RS-FMT-04, RS-FMT-06
Rules in app: RS-FMT-01, RS-FMT-05, RS-FMT-07, RS-FMT-08

Note: the package does not discover or choose files. The app/orchestrator
selects the authoritative `rustfmt.toml`, `Cargo.toml`, and
`rust-toolchain.toml` to compare, parses them, reports missing/malformed-file
failures itself, and calls the package only with concrete typed parsed inputs.
Inside the package, `check(&G3FmtContentChecksInput)` can fan out to smaller
rule-local inputs, but the package boundary stays one typed aggregate input.

### Toolchain
```rust
use cargo_toml_parser::CargoToml;

pub struct GrdzToolchainChecksInput {
    pub toolchain_config: Option<String>,         // raw rust-toolchain.toml content
    pub toolchain_config_rel_path: Option<String>,
    pub legacy_toolchain_exists: bool,
    pub cargo_rust_version: Option<String>,       // pre-extracted from CargoToml
    pub profile: GrdzProfile,
}

pub fn check(input: &GrdzToolchainChecksInput) -> Vec<GrdzCheckResult>
```

Rules in package: RS-TOOLCHAIN-01..04
Rules in app: none

Note: toolchain receives raw content (uses rust-toolchain-file crate
internally) and pre-extracted rust-version.

### Garde
```rust
use clippy_toml_parser::ClippyToml;
use cargo_toml_parser::CargoToml;

pub struct GrdzGardeChecksInput {
    pub source_files: Vec<GrdzSourceFile>,
    pub clippy_config: Option<ClippyToml>,    // garde checks its bans are present
    pub cargo_manifest: CargoToml,            // garde checks for garde dependency
    pub profile: GrdzProfile,
}

pub fn check(input: &GrdzGardeChecksInput) -> Vec<GrdzCheckResult>
```

Rules in package: RS-GARDE-01..14 (dep check, clippy bans, derive validation)
Rules in app: none — garde IS the policy owner for its concerns

### Deps
```rust
use cargo_toml_parser::CargoToml;

pub struct GrdzDepsChecksInput {
    pub workspace_manifest: CargoToml,
    pub member_manifests: Vec<(String, CargoToml)>,
    pub cargo_lock_exists: bool,
    pub cargo_lock_gitignored: bool,
    pub tools: Vec<GrdzToolStatus>,
    pub profile: GrdzProfile,
}

pub fn check(input: &GrdzDepsChecksInput) -> Vec<GrdzCheckResult>
```

Rules in package: RS-DEPS-01..12 (all rules)
Rules in app: none

### Test
```rust
use nextest_toml_parser::NextestToml;
use mutants_toml_parser::MutantsToml;
use cargo_toml_parser::CargoToml;

pub struct GrdzTestChecksInput {
    pub source_files: Vec<GrdzSourceFile>,
    pub cargo_manifest: CargoToml,
    pub nextest_config: Option<NextestToml>,
    pub mutants_config: Option<MutantsToml>,
    pub tools: Vec<GrdzToolStatus>,
    pub profile: GrdzProfile,
}

pub fn check(input: &GrdzTestChecksInput) -> Vec<GrdzCheckResult>
```

Rules in package: RS-TEST-01..18 (all rules)
Rules in app: none

### Release
```rust
use cargo_toml_parser::CargoToml;

pub struct GrdzReleaseChecksInput {
    pub workspace_manifest: CargoToml,
    pub member_manifests: Vec<(String, CargoToml)>,
    pub tools: Vec<GrdzToolStatus>,
    pub profile: GrdzProfile,
    // release-plz.toml, cliff.toml etc. — need parsers or raw strings
}

pub fn check(input: &GrdzReleaseChecksInput) -> Vec<GrdzCheckResult>
```

## Global families (NOT workspace-local)

These are called once for the entire project, not per-root. Different
extraction pattern — deferred for now.

- **Arch**: needs full directory tree + all Cargo.toml + all .rs facades
- **Code**: needs all .rs files across project
- **Hexarch**: needs directory tree + all Cargo.toml + entrypoint .rs files
- **Topology**: reports legality findings — may stay in app
- **Hooks-shared, Hooks-rs**: project-wide hook validation

## Shared types needed in check-types

```rust
pub struct GrdzSourceFile {
    pub rel_path: String,
    pub content: String,
    pub is_test_context: bool,
}

pub struct GrdzToolStatus {
    pub name: String,
    pub installed: bool,
}
```

## Migration order

Extract workspace-local families first (simplest, per-root):
1. toolchain — simplest (4 rules, no .rs parsing, minimal inputs)
2. fmt — simple (8 rules, pre-extracted cross-domain fields)
3. deny — moderate (27 rules, single config file)
4. clippy — moderate (20+ rules, multi-file, no garde coupling)
5. cargo — moderate (15 rules, workspace + members)
6. deps — moderate (12 rules, needs tool status)
7. garde — complex (14 rules, .rs AST + cross-file)
8. test — complex (18 rules, .rs AST + configs + tools)
9. release — complex (29 rules, multi-config)

Then global families:
10. arch, code, hexarch — need different input pattern
11. topology, hooks — may stay in app

## Key principle

Each package owns a CONCERN, not a file. It receives parsed types from
whatever files it needs. The app discovers, reads, parses, routes. The
package validates.
