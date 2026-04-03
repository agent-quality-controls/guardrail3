# Extract families into independent packages

## Goal

Each rule family becomes an independent package under `packages/` with no
knowledge of the app, other families, or the filesystem. The app is the
orchestrator — it discovers the project, runs legality, builds typed inputs,
and calls each family's `check()` function.

## Architecture

```
packages/
  guardrail3-check-types/         ← shared data types + shared .rs parser
  guardrail3-family-clippy/       ← ClippyInput + check()
  guardrail3-family-deny/         ← DenyInput + check()
  guardrail3-family-cargo/        ← CargoInput + check()
  guardrail3-family-code/         ← CodeInput + check()
  guardrail3-family-garde/        ← GardeInput + check()
  guardrail3-family-arch/         ← ArchInput + check()
  guardrail3-family-hexarch/      ← HexarchInput + check()
  guardrail3-family-fmt/          ← FmtInput + check()
  guardrail3-family-toolchain/    ← ToolchainInput + check()
  guardrail3-family-deps/         ← DepsInput + check()
  guardrail3-family-test/         ← TestInput + check()

apps/guardrail3/                  ← orchestrator
  crates/app/rs/
    structure/                    ← ProjectTree → RustStructureFacts
    legality/                     ← structure → RustLegalityFacts
    input_builder/                ← legality + tree → per-family inputs (NEW)
    runtime/                      ← calls family packages, collects results
```

## Design principles

1. **Families are pure logic.** No filesystem, no traits, no callbacks. Input
   is owned structs. Output is `Vec<CheckResult>`. Nothing else.

2. **The app pre-extracts cross-domain fields.** When a family needs data from
   another family's file, the app extracts the specific field and passes it as
   a typed value — not the raw file. Fmt gets `toolchain_channel: Option<String>`,
   not `toolchain_configs: Vec<ConfigFile>`.

3. **Input types enforce boundaries.** ClippyInput has no .rs files. CodeInput
   has no deny.toml. The type system prevents cross-family file access.

4. **Families depend only on guardrail3-check-types.** No dependency on the app,
   on legality, on FamilyView, on FamilyMapper, or on each other.

5. **Topology is not a family.** It reports legality findings. It stays in the
   app as a report phase after legality.

6. **The app parses shared data once.** Workspace members, profiles, Cargo.toml
   manifests — parsed once by the app, passed as structured data in RootInfo.
   Families never re-parse workspace membership or guardrail3.toml profiles.

7. **Family-owned configs stay as raw content.** clippy.toml goes to clippy as
   a string — clippy parses it internally. deny.toml goes to deny as a string.
   These are the family's own files; it owns the parsing.

## Audit findings (2026-04-03)

### Duplication that the extraction eliminates

1. **Workspace discovery (4x)** — clippy, deny, deps, hexarch all independently
   parse [workspace].members with glob expansion. Moves to app → RootInfo.

2. **Profile resolution (3x)** — clippy, deny, deps all parse guardrail3.toml
   for profile maps. Moves to app → RootInfo.profile.

3. **parse_rust_file() (3x)** — code, garde, test all duplicate identical syn
   parsing with BOM stripping. Moves to check-types as shared utility.

4. **Package name extraction (4x)** — cargo, deps, arch, hexarch all read
   [package].name. Moves to app → MemberInfo.package_name.

### Cross-domain reads resolved by pre-extraction

| Family | Currently reads | Extracted to input field |
|--------|----------------|------------------------|
| fmt | rust-toolchain.toml (channel) | `FmtInput.toolchain_channel: Option<String>` |
| fmt | Cargo.toml (edition) | `FmtInput.cargo_edition: Option<String>` |
| garde | clippy.toml (method bans) | `GardeInput.clippy_method_bans: Vec<String>` |
| clippy | .cargo/config.toml (CLIPPY_CONF_DIR) | `ClippyInput.cargo_env_overrides: Vec<CargoEnvOverride>` |
| toolchain | Cargo.toml (rust-version) | `ToolchainInput.cargo_rust_version: Option<String>` per root |
| all config families | guardrail3.toml (profile) | `RootInfo.profile` |
| all config families | Cargo.toml (workspace members) | `RootInfo.workspace_members` |

### No rule overlap found

Despite parsing duplication, no two families check the same thing:
- cargo: lint policy, edition, resolver
- deps: allowlists, lockfiles, tool installation
- arch: facades, module structure, crate complexity, feature gates
- hexarch: layer boundaries, dependency direction, directory layout
- code: lint attributes, unsafe, exceptions, API shape
- garde: validation derives, field constraints, deserialize impls
- test: test structure, assertions, mutation config

## Shared types (guardrail3-check-types)

```rust
// === Output ===

pub struct CheckResult {
    pub id: String,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub inventory: bool,
}

pub enum Severity { Error, Warn, Info }

// === Project structure (built by app from legality) ===

pub struct RootInfo {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub classification: RootClassification,
    pub is_workspace: bool,
    pub workspace_members: Vec<MemberInfo>,
    pub profile: Option<String>,  // resolved by app from guardrail3.toml
}

pub struct MemberInfo {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub package_name: Option<String>,
}

pub enum RootClassification { App, Package, Auxiliary, Other }

// === File content (family-owned configs) ===

pub struct ConfigFile {
    pub rel_path: String,
    pub content: String,
}

pub struct SourceFile {
    pub rel_path: String,
    pub content: String,
    pub is_test_context: bool,
}

// === Directory structure ===

pub struct DirEntry {
    pub dirs: Vec<String>,
    pub files: Vec<String>,
}

pub struct DirTree {
    pub entries: BTreeMap<String, DirEntry>,
}

// === Policy (extracted from guardrail3.toml by the app) ===

pub struct EscapeHatch {
    pub rule_id: String,
    pub rel_path: String,
    pub reason: Option<String>,
}

// === Tool availability ===

pub struct ToolStatus {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
}

// === Shared parser (deduplicated from 3 families) ===

/// Parse a .rs file with syn, stripping BOM. Returns None on parse failure.
pub fn parse_rust_file(content: &str) -> Option<syn::File> {
    let clean = content.strip_prefix('\u{feff}').unwrap_or(content);
    syn::parse_file(clean).ok()
}
```

## Per-family input types

### ClippyInput
```rust
pub struct ClippyInput {
    pub roots: Vec<RootInfo>,
    pub clippy_configs: Vec<ConfigFile>,          // clippy.toml, .clippy.toml (family-owned)
    pub cargo_env_overrides: Vec<CargoEnvOverride>, // .cargo/config.toml env entries (pre-extracted)
    pub escape_hatches: Vec<EscapeHatch>,
}

pub struct CargoEnvOverride {
    pub config_rel_path: String,
    pub env_key: String,
    pub env_value: String,
}
```
App extracts: workspace members → RootInfo, profile → RootInfo.profile,
CLIPPY_CONF_DIR → CargoEnvOverride. Clippy parses its own clippy.toml.

### DenyInput
```rust
pub struct DenyInput {
    pub roots: Vec<RootInfo>,
    pub deny_configs: Vec<ConfigFile>,  // deny.toml, .deny.toml, .cargo/deny.toml (family-owned)
    pub escape_hatches: Vec<EscapeHatch>,
}
```
App extracts: workspace members → RootInfo, profile → RootInfo.profile.
Deny parses its own deny.toml files, handles precedence internally.

### CargoInput
```rust
pub struct CargoInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,  // all Cargo.toml files (family-owned)
    pub escape_hatches: Vec<EscapeHatch>,
}
```
Cargo owns Cargo.toml — gets raw content, parses everything internally.

### FmtInput
```rust
pub struct FmtInput {
    pub roots: Vec<RootInfo>,
    pub rustfmt_configs: Vec<ConfigFile>,          // rustfmt.toml, .rustfmt.toml (family-owned)
    pub cargo_edition: Option<String>,             // pre-extracted from Cargo.toml
    pub toolchain_channel: Option<String>,         // pre-extracted from rust-toolchain.toml
    pub escape_hatches: Vec<EscapeHatch>,
}
```
App extracts: edition from Cargo.toml, channel from rust-toolchain.toml.
Fmt never touches those files directly.

### ToolchainInput
```rust
pub struct ToolchainInput {
    pub roots: Vec<ToolchainRootInput>,
}

pub struct ToolchainRootInput {
    pub root: RootInfo,
    pub toolchain_config: Option<ConfigFile>,       // rust-toolchain.toml (family-owned)
    pub legacy_toolchain_exists: bool,              // rust-toolchain (no .toml) presence
    pub cargo_rust_version: Option<String>,         // pre-extracted from Cargo.toml
}
```
App extracts: rust-version from Cargo.toml. Toolchain owns rust-toolchain.toml.

### CodeInput
```rust
pub struct CodeInput {
    pub roots: Vec<RootInfo>,
    pub source_files: Vec<SourceFile>,              // all .rs files (family-owned for parsing)
    pub unsafe_code_lint_level: Option<String>,     // pre-extracted from Cargo.toml [workspace.lints.rust]
    pub escape_hatches: Vec<EscapeHatch>,
}
```
App extracts: unsafe_code lint level from Cargo.toml. Code owns .rs parsing.

### GardeInput
```rust
pub struct GardeInput {
    pub roots: Vec<GardeRootInput>,
    pub source_files: Vec<SourceFile>,              // all .rs files
    pub escape_hatches: Vec<EscapeHatch>,
}

pub struct GardeRootInput {
    pub root: RootInfo,
    pub garde_dependency_present: bool,             // pre-extracted from Cargo.toml
    pub clippy_disallowed_methods: Vec<String>,     // pre-extracted from clippy.toml
    pub clippy_disallowed_types: Vec<String>,       // pre-extracted from clippy.toml
}
```
App extracts: garde dep presence from Cargo.toml, method/type bans from
clippy.toml. Garde never reads those files. Owns .rs parsing.

### ArchInput
```rust
pub struct ArchInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // all Cargo.toml (family-owned — reads features, metadata)
    pub dir_tree: DirTree,                   // full directory structure
    pub facade_files: Vec<SourceFile>,       // lib.rs + mod.rs content (family-owned for syn parsing)
    pub module_files: Vec<SourceFile>,       // .rs files under src/ (for mod declaration scanning)
}
```
Arch owns Cargo.toml parsing (features, shared flag, deps count) and
.rs facade/module parsing. Gets directory tree from app.

### HexarchInput
```rust
pub struct HexarchInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // all Cargo.toml (family-owned — reads deps, patches)
    pub dir_tree: DirTree,                   // directory structure
    pub entrypoint_files: Vec<SourceFile>,   // lib.rs/main.rs only (for trait/fn counting)
    pub escape_hatches: Vec<EscapeHatch>,
}
```
Hexarch owns Cargo.toml dep parsing and lib.rs/main.rs trait counting.
Gets directory tree from app. Does NOT get full .rs files — only entrypoints.

### DepsInput
```rust
pub struct DepsInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // all Cargo.toml (family-owned — reads dep sections)
    pub lockfile_exists: Vec<LockfileStatus>,
    pub tools: Vec<ToolStatus>,
    pub escape_hatches: Vec<EscapeHatch>,
}

pub struct LockfileStatus {
    pub root_rel_dir: String,
    pub cargo_lock_exists: bool,
    pub cargo_lock_gitignored: bool,        // pre-extracted from .gitignore by app
}
```
App extracts: Cargo.lock existence, .gitignore scanning for Cargo.lock
patterns. Deps owns Cargo.toml dependency parsing.

### TestInput
```rust
pub struct TestInput {
    pub roots: Vec<RootInfo>,
    pub source_files: Vec<SourceFile>,       // all .rs files (for assertion/test analysis)
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml (tokio dep, package names)
    pub mutants_configs: Vec<ConfigFile>,    // .cargo/mutants.toml (family-owned)
    pub nextest_configs: Vec<ConfigFile>,    // .config/nextest.toml (family-owned)
    pub dir_tree: DirTree,                   // for test file categorization
    pub tools: Vec<ToolStatus>,
}
```

## Topology (not a family)

Topology reports legality findings. It stays in the app:

```rust
pub fn report_topology(legality: &RustLegalityFacts) -> Vec<CheckResult>
```

It receives legality output directly — root classifications, zone overlaps,
misplaced roots, input failures. No filesystem access, no FamilyView.

## Migration plan

### Phase 1: Create check-types package
1. Create `packages/guardrail3-check-types/` with shared types
2. Include `parse_rust_file()` (deduplicated from code/garde/test)
3. Move `CheckResult` and `Severity` from `domain/report` to check-types
4. Have `domain/report` re-export from check-types for backward compat

### Phase 2: Extract one family as proof of concept
Pick **toolchain** — simplest family (4 rules, no .rs parsing, no tools,
no guardrail3.toml, no cross-domain reads).

1. Create `packages/guardrail3-family-toolchain/`
2. Define `ToolchainInput` using check-types
3. Move facts + inputs + rules from app
4. Change facts::collect() to take `&ToolchainInput`
5. Create `input_builder::build_toolchain_input()` in the app
6. Update runner
7. Verify all 48 tests pass

### Phase 3: Extract remaining families
Order by complexity (simplest first):
1. toolchain (phase 2) — 4 rules, config-only, no cross-domain
2. fmt — 8 rules, needs 2 pre-extracted fields (edition, channel)
3. cargo — 15 rules, Cargo.toml only
4. clippy — 25 rules, needs CargoEnvOverride extraction
5. deny — 30 rules, config + coverage
6. deps — 12 rules, needs ToolStatus + LockfileStatus
7. test — 18 rules, needs ToolStatus + .rs content + dir tree
8. code — 36 rules, needs .rs content + 1 pre-extracted lint field
9. garde — 14 rules, needs .rs content + pre-extracted clippy bans
10. arch — 9 rules, needs dir tree + facade .rs content + Cargo.toml
11. hexarch — 27 rules, needs dir tree + deps + entrypoint .rs

### Phase 4: Move topology to legality report
Extract topology from the family system into a legality report function.

### Phase 5: Delete FamilyView + FamilyMapper
Once all families use typed inputs, FamilyView and FamilyMapper are dead.
The input_builder replaces both.

## Key decisions

**Family-owned files: raw content.** clippy.toml → clippy as string.
Cargo.toml → cargo as string (but also arch, deps, hexarch for different
fields). The family parses what it needs.

**Cross-domain fields: pre-extracted.** When a family needs one field from
another family's file, the app extracts it as a typed value. No raw file
access across domains.

**Workspace iteration stays in the app.** The app iterates workspace roots
and calls the family per-workspace or once for project-wide.

**Facts stay inside the family.** The internal facts/inputs/rules pipeline
doesn't change. Only the entry point changes.

**Tests move with the family.** Each package has its own tests. test_support
and assertions fold into the package.

## What this enables

- Families are independently versioned, tested, developed
- Type system prevents cross-family file access
- No more "family reads a file it shouldn't" bugs
- Workspace discovery, profile resolution, shared parsing happen once
- Families become reusable outside guardrail3
