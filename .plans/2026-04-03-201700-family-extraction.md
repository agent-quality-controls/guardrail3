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
   not the raw rust-toolchain.toml content.

3. **Input types enforce boundaries.** ClippyInput has no .rs files. CodeInput
   has no deny.toml. The type system prevents cross-family file access.

4. **Families depend only on guardrail3-check-types** (and standard parser
   crates like `toml`). No dependency on the app, on legality, on FamilyView,
   on FamilyMapper, or on each other.

5. **Topology is not a family.** It reports legality findings. It stays in the
   app as a report phase after legality.

6. **Shared files use standard parsers.** Cargo.toml parsed once by the app
   using the `cargo_toml` crate → families receive `cargo_toml::Manifest`.
   rust-toolchain.toml parsed once using `rust-toolchain-file` crate. No
   family re-parses shared files.

7. **Family-owned configs stay as raw content.** clippy.toml, deny.toml,
   rustfmt.toml, nextest.toml, mutants.toml — no standard parser exists for
   these. The family is the only consumer and understands the schema. It
   receives raw content string and parses with the `toml` crate internally.

8. **guardrail3.toml parsed once by the app.** Families receive a
   `GuardrailPolicy` struct with profile, escape hatches, and per-family
   config sections. No family reads guardrail3.toml directly.

## Parsing strategy

### Shared files — parsed by the app, typed structs to families

| File | Parser | Families that receive it |
|------|--------|------------------------|
| Cargo.toml | `cargo_toml::Manifest` | All (except topology) |
| rust-toolchain.toml | `rust-toolchain-file` | toolchain, fmt (channel only) |
| guardrail3.toml | Our `GuardrailPolicy` | All that need profile/escape hatches |

### Family-owned files — raw content to the family

| File | Family | Why raw |
|------|--------|---------|
| clippy.toml, .clippy.toml | clippy | No standard parser; clippy owns the schema |
| deny.toml, .deny.toml, .cargo/deny.toml | deny | No standard parser; deny owns the schema |
| rustfmt.toml, .rustfmt.toml | fmt | Existing parsers unmaintained; fmt owns the schema |
| .cargo/mutants.toml | test | No standard parser |
| .config/nextest.toml | test | No standard parser |

### Cross-domain fields — pre-extracted by the app

| Family needs | From file | Receives |
|-------------|-----------|----------|
| fmt: toolchain channel | rust-toolchain.toml | `channel: Option<String>` |
| fmt: cargo edition | Cargo.toml | `Manifest.package.edition` (via cargo_toml) |
| toolchain: cargo rust-version | Cargo.toml | `Manifest.package.rust_version` (via cargo_toml) |
| garde: clippy method bans | clippy.toml | `clippy_disallowed_methods: Vec<String>` |
| garde: garde dep present | Cargo.toml | `Manifest.dependencies` (via cargo_toml) |
| clippy: cargo env overrides | .cargo/config.toml | `CargoEnvOverride` struct |
| code: unsafe_code lint level | Cargo.toml | `Manifest.lints` (via cargo_toml) |

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
// Re-export standard parsers — families depend on check-types, not directly
pub use cargo_toml::Manifest as CargoManifest;

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

// === Parsed manifests (shared files, parsed once by app) ===

/// Cargo.toml parsed with the cargo_toml crate. Families read typed fields.
pub struct ManifestFile {
    pub rel_path: String,
    pub manifest: CargoManifest,
}

// === Raw file content (family-owned configs) ===

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

pub struct GuardrailPolicy {
    pub profile: String,
    pub escape_hatches: Vec<EscapeHatch>,
}

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

// === Shared .rs parser (deduplicated from code/garde/test) ===

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
    pub manifests: Vec<ManifestFile>,                // parsed Cargo.toml (workspace detection)
    pub clippy_configs: Vec<ConfigFile>,              // clippy.toml, .clippy.toml (raw, family-owned)
    pub cargo_env_overrides: Vec<CargoEnvOverride>,   // pre-extracted from .cargo/config.toml
    pub policy: GuardrailPolicy,
}

pub struct CargoEnvOverride {
    pub config_rel_path: String,
    pub env_key: String,
    pub env_value: String,
}
```
Clippy parses its own clippy.toml. Reads `Manifest.workspace` for coverage.
App pre-extracts CLIPPY_CONF_DIR from .cargo/config.toml.

### DenyInput
```rust
pub struct DenyInput {
    pub roots: Vec<RootInfo>,
    pub manifests: Vec<ManifestFile>,         // parsed Cargo.toml (workspace members)
    pub deny_configs: Vec<ConfigFile>,        // deny.toml, .deny.toml, .cargo/deny.toml (raw)
    pub policy: GuardrailPolicy,
}
```
Deny parses its own deny.toml files. Reads `Manifest.workspace.members`
for coverage. Handles config precedence internally.

### CargoInput
```rust
pub struct CargoInput {
    pub roots: Vec<RootInfo>,
    pub manifests: Vec<ManifestFile>,         // parsed Cargo.toml — cargo reads ALL fields
    pub policy: GuardrailPolicy,
}
```
Cargo gets pre-parsed Manifests. Reads lints, edition, resolver, members,
features — everything. No raw TOML parsing needed.

### FmtInput
```rust
pub struct FmtInput {
    pub roots: Vec<RootInfo>,
    pub rustfmt_configs: Vec<ConfigFile>,     // rustfmt.toml, .rustfmt.toml (raw, family-owned)
    pub cargo_edition: Option<String>,        // pre-extracted from Manifest.package.edition
    pub toolchain_channel: Option<String>,    // pre-extracted from rust-toolchain.toml
    pub policy: GuardrailPolicy,
}
```
Fmt parses its own rustfmt.toml. Never touches Cargo.toml or
rust-toolchain.toml — app extracts edition and channel as strings.

### ToolchainInput
```rust
pub struct ToolchainInput {
    pub roots: Vec<ToolchainRootInput>,
}

pub struct ToolchainRootInput {
    pub root: RootInfo,
    pub toolchain_config: Option<ConfigFile>,  // rust-toolchain.toml (raw, family-owned)
    pub legacy_toolchain_exists: bool,         // rust-toolchain (no .toml) file presence
    pub cargo_rust_version: Option<String>,    // pre-extracted from Manifest.package.rust_version
}
```
Toolchain parses its own rust-toolchain.toml. App pre-extracts
rust-version from parsed Cargo.toml Manifest.

### CodeInput
```rust
pub struct CodeInput {
    pub roots: Vec<RootInfo>,
    pub source_files: Vec<SourceFile>,        // all .rs files (family-owned for syn parsing)
    pub manifests: Vec<ManifestFile>,         // parsed Cargo.toml (reads workspace.lints.rust)
    pub policy: GuardrailPolicy,
}
```
Code reads `Manifest.lints` for unsafe_code level. Owns .rs parsing.

### GardeInput
```rust
pub struct GardeInput {
    pub roots: Vec<GardeRootInput>,
    pub source_files: Vec<SourceFile>,        // all .rs files (family-owned for syn parsing)
    pub policy: GuardrailPolicy,
}

pub struct GardeRootInput {
    pub root: RootInfo,
    pub garde_dependency_present: bool,       // pre-extracted: "garde" in Manifest.dependencies
    pub clippy_disallowed_methods: Vec<String>, // pre-extracted from clippy.toml
    pub clippy_disallowed_types: Vec<String>,   // pre-extracted from clippy.toml
}
```
Garde never reads Cargo.toml or clippy.toml. App pre-extracts
dependency presence and clippy ban lists. Garde owns .rs AST parsing.

### ArchInput
```rust
pub struct ArchInput {
    pub roots: Vec<RootInfo>,
    pub manifests: Vec<ManifestFile>,         // parsed Cargo.toml (features, metadata, deps)
    pub dir_tree: DirTree,                    // full directory structure
    pub facade_files: Vec<SourceFile>,        // lib.rs + mod.rs content (family-owned for syn)
    pub module_files: Vec<SourceFile>,        // .rs files under src/ (for mod declarations)
}
```
Arch reads `Manifest.features`, `Manifest.dependencies.len()`,
`Manifest.package.metadata`. Owns .rs facade/module parsing.

### HexarchInput
```rust
pub struct HexarchInput {
    pub roots: Vec<RootInfo>,
    pub manifests: Vec<ManifestFile>,         // parsed Cargo.toml (deps, patches, members)
    pub dir_tree: DirTree,                    // directory structure
    pub entrypoint_files: Vec<SourceFile>,    // lib.rs/main.rs only (for trait/fn counting)
    pub policy: GuardrailPolicy,
}
```
Hexarch reads `Manifest.dependencies`, `Manifest.patch`, members.
Owns lib.rs/main.rs trait counting. Gets dir tree from app.

### DepsInput
```rust
pub struct DepsInput {
    pub roots: Vec<RootInfo>,
    pub manifests: Vec<ManifestFile>,         // parsed Cargo.toml (all dependency sections)
    pub lockfile_status: Vec<LockfileStatus>,
    pub tools: Vec<ToolStatus>,
    pub policy: GuardrailPolicy,
}

pub struct LockfileStatus {
    pub root_rel_dir: String,
    pub cargo_lock_exists: bool,
    pub cargo_lock_gitignored: bool,          // pre-extracted from .gitignore by app
}
```
Deps reads `Manifest.dependencies` etc. for allowlist checks. App
pre-extracts Cargo.lock existence and .gitignore scanning.

### TestInput
```rust
pub struct TestInput {
    pub roots: Vec<RootInfo>,
    pub source_files: Vec<SourceFile>,        // all .rs files (family-owned for syn parsing)
    pub manifests: Vec<ManifestFile>,         // parsed Cargo.toml (tokio dep, package names)
    pub mutants_configs: Vec<ConfigFile>,     // .cargo/mutants.toml (raw, family-owned)
    pub nextest_configs: Vec<ConfigFile>,     // .config/nextest.toml (raw, family-owned)
    pub dir_tree: DirTree,                    // for test file categorization
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

**Shared files: parsed once with standard crates.** Cargo.toml parsed by
app with `cargo_toml` crate → families receive `ManifestFile` (typed).
rust-toolchain.toml parsed with `rust-toolchain-file` crate. No family
re-parses shared files.

**Family-owned files: raw content.** clippy.toml, deny.toml, rustfmt.toml,
nextest.toml, mutants.toml → family receives raw string, parses with `toml`
crate internally. No standard parser exists for these; the family owns the
schema.

**Cross-domain fields: pre-extracted.** When a family needs a specific value
from another family's file (not the whole file), the app extracts it as a
typed value. Examples: fmt gets `toolchain_channel: Option<String>`, garde
gets `clippy_disallowed_methods: Vec<String>`.

**Workspace iteration stays in the app.** The app iterates workspace roots
and calls the family per-workspace or once for project-wide.

**Facts stay inside the family.** The internal facts/inputs/rules pipeline
doesn't change. Only the entry point changes.

**Tests move with the family.** Each package has its own tests. test_support
and assertions fold into the package.

## Rule ownership audit (2026-04-03)

All rules stay in their current families. The "misplaced" rules identified
in the audit are actually in the correct family by domain knowledge — they
just read the wrong file. The extraction resolves this by pre-extracting
cross-domain fields.

**Confirmed dead rules to delete:** RS-CODE-26, RS-CODE-27 (moved to ARCH).

**Rule to move:** RS-CODE-32 (test expect message quality) → TEST family.

**Tool installation checks** (DEPS-01/02/03/04, TEST-11): keep in current
families for now. Consider a shared "tools" pre-check phase later if more
tool checks are added.

## What this enables

- Families are independently versioned, tested, developed
- Type system prevents cross-family file access
- No more "family reads a file it shouldn't" bugs
- Workspace discovery, profile resolution, shared parsing happen once
- Families become reusable outside guardrail3
