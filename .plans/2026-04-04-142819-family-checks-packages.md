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
use guardrail3_check_types::G3CheckResult;

pub struct G3ClippyContentChecksInput {
    pub clippy_rel_path: String,
    pub clippy: ClippyToml,
}

pub fn check(input: &G3ClippyContentChecksInput) -> Vec<G3CheckResult>
```

Initial rules in package: RS-CLIPPY-02, 03, 09, 10, 11, 17, 21, 22
Rules in app: RS-CLIPPY-01, 04, 05, 06, 07, 08, 12, 13, 14, 15, 16, 18, 19,
20, 23, 24, 25

Notes:
- `RS-CLIPPY-25` stays app-side as the typed parse/schema gate for `clippy.toml`.
- `RS-CLIPPY-24` stays app-side so the package does not take Cargo config files in
  its initial input contract.
- Profile-sensitive and raw malformed-section rules stay app-side until their
  structural malformed-input ownership is intentionally redesigned.

### Deny
```rust
use deny_toml_parser::DenyToml;
use guardrail3_check_types::G3CheckResult;

pub struct G3DenyContentChecksInput {
    pub deny_rel_path: String,
    pub deny: DenyToml,
}

pub fn check(input: &G3DenyContentChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-DENY-04, 05, 06, 07, 08, 10, 11, 12, 13, 14, 15, 16,
18, 19, 20, 21, 22, 23, 24, 27, 28, 29
Rules in app: RS-DENY-01, RS-DENY-03, RS-DENY-09, RS-DENY-17, RS-DENY-25,
RS-DENY-26, RS-DENY-30

Note: app-side deny orchestration still owns authoritative config selection,
parse-failure routing, coverage/shadowing, and profile resolution. The package
receives a concrete parsed `DenyToml` only.

### Cargo
```rust
use cargo_toml_parser::CargoToml;

pub struct G3CargoChecksInput {
    pub policy_root_rel_path: String,
    pub policy_root_manifest: CargoToml,
    pub member_manifests: Vec<(String, CargoToml)>,
    pub policy_rel_path: Option<String>,
    pub policy_profile: Option<G3CargoPolicyProfile>,
    pub lint_allow_waivers: Vec<G3CargoLintAllowWaiver>,
}

pub fn check(input: &G3CargoChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-CARGO-01..09, RS-CARGO-11..13, RS-CARGO-15
Rules in app: RS-CARGO-10, RS-CARGO-14

Package boundary:

- receives parsed policy-root and member `Cargo.toml` values
- receives normalized profile/waiver policy inputs derived from the root-local guardrail config
- app still owns root/member discovery, missing-member routing, and malformed-input fail-closed behavior

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
use rust_toolchain_toml_parser::RustToolchainToml;
use guardrail3_check_types::G3CheckResult;

pub struct G3ToolchainChannelAndComponentsInput {
    pub toolchain_rel_path: String,
    pub toolchain_toml: RustToolchainToml,
}

pub struct G3ToolchainMsrvConsistencyInput {
    pub toolchain_rel_path: String,
    pub toolchain_toml: RustToolchainToml,
    pub cargo_rel_path: String,
    pub cargo_toml: CargoToml,
}

pub fn check_channel_and_components(
    input: &G3ToolchainChannelAndComponentsInput,
) -> Vec<G3CheckResult>;

pub fn check_msrv_consistency(
    input: &G3ToolchainMsrvConsistencyInput,
) -> Vec<G3CheckResult>;
```

Rules in package: RS-TOOLCHAIN-02, RS-TOOLCHAIN-03
Rules in app: RS-TOOLCHAIN-01, RS-TOOLCHAIN-04

### Garde
```rust
use clippy_toml_parser::ClippyToml;
use cargo_toml_parser::CargoToml;

pub struct G3GardeChecksInput {
    pub source_files: Vec<G3SourceFile>,
    pub clippy_config: Option<ClippyToml>,    // garde checks its bans are present
    pub cargo_manifest: CargoToml,            // garde checks for garde dependency
    pub profile: G3Profile,
}

pub fn check(input: &G3GardeChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-GARDE-01..14 (dep check, clippy bans, derive validation)
Rules in app: none — garde IS the policy owner for its concerns

### Deps
```rust
use cargo_toml_parser::CargoToml;

pub struct G3DepsChecksInput {
    pub workspace_manifest: CargoToml,
    pub member_manifests: Vec<(String, CargoToml)>,
    pub cargo_lock_exists: bool,
    pub cargo_lock_gitignored: bool,
    pub tools: Vec<G3ToolStatus>,
    pub profile: G3Profile,
}

pub fn check(input: &G3DepsChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-DEPS-01..12 (all rules)
Rules in app: none

### Test
```rust
use nextest_toml_parser::NextestToml;
use mutants_toml_parser::MutantsToml;
use cargo_toml_parser::CargoToml;

pub struct G3TestChecksInput {
    pub source_files: Vec<G3SourceFile>,
    pub cargo_manifest: CargoToml,
    pub nextest_config: Option<NextestToml>,
    pub mutants_config: Option<MutantsToml>,
    pub tools: Vec<G3ToolStatus>,
    pub profile: G3Profile,
}

pub fn check(input: &G3TestChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-TEST-01..18 (all rules)
Rules in app: none

### Release
```rust
use cargo_toml_parser::CargoToml;

pub struct G3ReleaseChecksInput {
    pub workspace_manifest: CargoToml,
    pub member_manifests: Vec<(String, CargoToml)>,
    pub tools: Vec<G3ToolStatus>,
    pub profile: G3Profile,
    // release-plz.toml, cliff.toml etc. — need parsers or raw strings
}

pub fn check(input: &G3ReleaseChecksInput) -> Vec<G3CheckResult>
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
pub struct G3SourceFile {
    pub rel_path: String,
    pub content: String,
    pub is_test_context: bool,
}

pub struct G3ToolStatus {
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
