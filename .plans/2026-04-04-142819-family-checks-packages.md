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

pub struct G3RsClippyConfigChecksInput {
    pub clippy_rel_path: String,
    pub clippy: ClippyToml,
}

pub fn check(input: &G3RsClippyConfigChecksInput) -> Vec<G3CheckResult>
```

Initial rules in package: RS-CLIPPY-CONFIG-01, 03, 09, 10, 11, 17, 21, 22
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

pub struct G3RsDenyConfigChecksInput {
    pub deny_rel_path: String,
    pub deny: DenyToml,
}

pub fn check(input: &G3RsDenyConfigChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-DENY-CONFIG-01, 05, 06, 07, 08, 10, 11, 12, 13, 14, 15, 16,
18, 19, 20, 21, 22, 23, 24, 27, 28, 29
Rules in app: RS-DENY-01, RS-DENY-03, RS-DENY-09, RS-DENY-17, RS-DENY-25,
RS-DENY-26, RS-DENY-30

Note: app-side deny orchestration still owns authoritative config selection,
parse-failure routing, coverage/shadowing, and profile resolution. The package
receives a concrete parsed `DenyToml` only.

### Cargo
```rust
use cargo_toml_parser::CargoToml;
use guardrail3_check_types::G3CheckResult;

pub struct G3RsCargoConfigChecksInput {
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
}

pub fn check(input: &G3RsCargoConfigChecksInput) -> Vec<G3CheckResult>
```

Single-file rules in package: RS-CARGO-CONFIG-01, 02, 05, 07, 08, 11
Rules in app for now: RS-CARGO-03, 04, 06, 09, 10, 12, 13, 14, 15

Package boundary:

- receives one parsed `Cargo.toml`
- determines workspace/package applicability from the file itself
- does not receive member sets, profile enums, waiver subsets, or other
  derived policy helper types
- app still owns routing, workspace/member relationship rules, missing-member detection, cross-file comparison
  rules, and malformed-input fail-closed behavior

### Fmt
```rust
use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

pub struct G3RsFmtConfigChecksInput {
    pub rustfmt_rel_path: String,
    pub rustfmt: RustfmtToml,
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
    pub toolchain_rel_path: String,
    pub toolchain: RustToolchainToml,
}

pub fn check(input: &G3RsFmtConfigChecksInput) -> Vec<G3CheckResult>
```

Rules in package: RS-FMT-CONFIG-01, RS-FMT-CONFIG-02, RS-FMT-CONFIG-03, RS-FMT-CONFIG-04
Rules in app: RS-FMT-01, RS-FMT-05, RS-FMT-07, RS-FMT-08

Note: the package does not discover or choose files. The app/orchestrator
selects the authoritative `rustfmt.toml`, `Cargo.toml`, and
`rust-toolchain.toml` to compare, parses them, reports missing/malformed-file
failures itself, and calls the package only with concrete typed parsed inputs.
Inside the package, `check(&G3RsFmtConfigChecksInput)` can fan out to smaller
rule-local inputs, but the package boundary stays one typed aggregate input.

### Toolchain
```rust
use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;
use guardrail3_check_types::G3CheckResult;

pub struct G3RsToolchainConfigChannelComponentsInput {
    pub toolchain_rel_path: String,
    pub toolchain_toml: RustToolchainToml,
}

pub struct G3RsToolchainConfigMsrvConsistencyInput {
    pub toolchain_rel_path: String,
    pub toolchain_toml: RustToolchainToml,
    pub cargo_rel_path: String,
    pub cargo_toml: CargoToml,
}

pub fn check_channel_and_components(
    input: &G3RsToolchainConfigChannelComponentsInput,
) -> Vec<G3CheckResult>;

pub fn check_msrv_consistency(
    input: &G3RsToolchainConfigMsrvConsistencyInput,
) -> Vec<G3CheckResult>;
```

Rules in package: RS-TOOLCHAIN-CONFIG-01, RS-TOOLCHAIN-CONFIG-02
Rules in app: RS-TOOLCHAIN-01, RS-TOOLCHAIN-04

### Garde
```rust
use cargo_toml_parser::CargoToml;
use clippy_toml_parser::ClippyToml;
use guardrail3_check_types::G3CheckResult;
use std::path::PathBuf;

pub struct G3RsGardeConfigDependencyCheckInput {
    pub cargo_rel_path: String,
    pub cargo: CargoToml,
}

pub struct G3RsGardeConfigClippyBanChecksInput {
    pub clippy_rel_path: String,
    pub clippy: ClippyToml,
}

pub fn check_dependency_present(
    input: &G3RsGardeConfigDependencyCheckInput,
) -> Vec<G3CheckResult>;

pub fn check_clippy_bans(
    input: &G3RsGardeConfigClippyBanChecksInput,
) -> Vec<G3CheckResult>;

pub struct G3RsAstFile {
    pub rel_path: String,
    pub abs_path: PathBuf,
}

pub struct G3RsGardeAstChecksInput {
    pub source_files: Vec<G3RsAstFile>,
    pub guardrail_toml: G3RsAstFile,
}

pub fn check(input: &G3RsGardeAstChecksInput) -> Vec<G3CheckResult>;
```

Rules in `g3rs-garde-config-checks`: RS-GARDE-CONFIG-01, RS-GARDE-CONFIG-02, RS-GARDE-CONFIG-03, RS-GARDE-CONFIG-04, RS-GARDE-CONFIG-05
Rules in `g3rs-garde-ast-checks`: RS-GARDE-AST-01, RS-GARDE-AST-02, RS-GARDE-AST-03, RS-GARDE-AST-04, RS-GARDE-AST-05, RS-GARDE-AST-06, RS-GARDE-AST-07, RS-GARDE-AST-08
Rules in app: RS-GARDE-10

Current bridge note:

- app still owns garde applicability gating from policy and source adoption
- app still owns missing / unparseable covering clippy handling for
  `RS-GARDE-CONFIG-02/03/04/06`
- app still owns malformed-input reporting through `RS-GARDE-10`
- `g3rs-garde-config-checks` owns the typed parsed-file path for root-policy checks
- `g3rs-garde-ast-checks` owns the governed Rust source-file path and required
  `guardrail3.toml` path for AST/source checks

### Deps
```rust
use cargo_toml_parser::CargoToml;
use guardrail3_check_types::G3CheckResult;
use guardrail3_domain_config::types::GuardrailConfig;

pub struct G3RsDepsConfigPolicyChecksInput {
    pub workspace_cargo_rel_path: String,
    pub workspace_cargo: CargoToml,
    pub crate_cargo_rel_path: String,
    pub crate_cargo: CargoToml,
    pub guardrail_rel_path: String,
    pub guardrail: GuardrailConfig,
}

pub struct G3RsDepsConfigDirectDependencyCapInput {
    pub workspace_cargo_rel_path: String,
    pub workspace_cargo: CargoToml,
    pub crate_cargo_rel_path: String,
    pub crate_cargo: CargoToml,
}

pub fn check_policy(input: &G3RsDepsConfigPolicyChecksInput) -> Vec<G3CheckResult>
pub fn check_direct_dependency_cap(input: &G3RsDepsConfigDirectDependencyCapInput) -> Vec<G3CheckResult>
```

Rules in package: RS-DEPS-CONFIG-01, 06, 07, 08, 12
Rules in app: RS-DEPS-01, 02, 03, 04, 09, 10, 11

Package boundary:

- receives full parsed files only
- each input represents one crate policy opportunity inside one workspace
- `workspace_cargo` exists because `RS-DEPS-CONFIG-01..07` may need workspace
  dependency resolution such as `workspace = true`
- current wired policy file is legacy `guardrail3.toml`
- package does not own tool presence, lockfile discovery, `.gitignore`
  ownership, or malformed-input fail-closed reporting

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
