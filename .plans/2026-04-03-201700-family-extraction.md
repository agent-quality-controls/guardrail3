# Extract families into independent packages

## Goal

Each rule family becomes an independent package under `packages/` with no
knowledge of the app, other families, or the filesystem. The app is the
orchestrator — it discovers the project, runs legality, builds typed inputs,
and calls each family's `check()` function.

## Architecture

```
packages/
  guardrail3-check-types/         ← shared data types
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

2. **The app builds inputs.** It runs legality, discovers files, parses nothing
   (passes raw content strings). Families parse what they need internally.

3. **Input types enforce boundaries.** ClippyInput has no .rs files. CodeInput
   has no deny.toml. The type system prevents cross-family file access.

4. **Families depend only on guardrail3-check-types.** No dependency on the app,
   on legality, on FamilyView, on FamilyMapper, or on each other.

5. **Topology is not a family.** It reports legality findings. It stays in the
   app as a report phase after legality, or becomes a thin adapter over legality
   output.

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

// === Project structure ===

pub struct RootInfo {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub classification: RootClassification,
    pub is_workspace: bool,
    pub workspace_members: Vec<MemberInfo>,
    pub profile: Option<String>,  // "service", "library", etc.
}

pub struct MemberInfo {
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub package_name: Option<String>,
}

pub enum RootClassification {
    App,
    Package,
    Auxiliary,
    Other,
}

// === File content ===

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

// === Config ===

/// Parsed guardrail3.toml — the parts families need.
/// NOT the full internal GuardrailConfig (that stays in the app).
pub struct GuardrailPolicy {
    pub profile: String,
    pub escape_hatches: Vec<EscapeHatch>,
    // Per-family config sections extracted by the app
}

pub struct EscapeHatch {
    pub rule_id: String,
    pub rel_path: String,
    pub reason: Option<String>,
}

// === Tool availability (for deps/test/hooks) ===

pub struct ToolStatus {
    pub name: String,
    pub installed: bool,
    pub version: Option<String>,
}
```

## Per-family input types

### ClippyInput
```rust
pub struct ClippyInput {
    pub roots: Vec<RootInfo>,
    pub clippy_configs: Vec<ConfigFile>,     // clippy.toml, .clippy.toml
    pub cargo_configs: Vec<ConfigFile>,      // .cargo/config.toml (CLIPPY_CONF_DIR)
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml (workspace detection)
    pub policy: GuardrailPolicy,
}
```

### DenyInput
```rust
pub struct DenyInput {
    pub roots: Vec<RootInfo>,
    pub deny_configs: Vec<ConfigFile>,       // deny.toml, .deny.toml, .cargo/deny.toml
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml (workspace members)
    pub policy: GuardrailPolicy,
}
```

### CargoInput
```rust
pub struct CargoInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // all Cargo.toml files
    pub policy: GuardrailPolicy,
}
```

### CodeInput
```rust
pub struct CodeInput {
    pub roots: Vec<RootInfo>,
    pub source_files: Vec<SourceFile>,       // all .rs files
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml (lint settings)
    pub policy: GuardrailPolicy,
}
```

### GardeInput
```rust
pub struct GardeInput {
    pub roots: Vec<RootInfo>,
    pub source_files: Vec<SourceFile>,       // all .rs files
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml (garde dep detection)
    pub clippy_configs: Vec<ConfigFile>,     // clippy.toml (method bans)
    pub policy: GuardrailPolicy,
}
```

### ArchInput
```rust
pub struct ArchInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // all Cargo.toml files
    pub dir_tree: DirTree,                   // full directory structure
    pub rs_file_rels: Vec<String>,           // paths to all .rs files
    pub facade_files: Vec<SourceFile>,       // lib.rs + mod.rs content only
}
```

### HexarchInput
```rust
pub struct HexarchInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // all Cargo.toml files
    pub dir_tree: DirTree,                   // directory structure
    pub policy: GuardrailPolicy,
}
```

### FmtInput
```rust
pub struct FmtInput {
    pub roots: Vec<RootInfo>,
    pub rustfmt_configs: Vec<ConfigFile>,    // rustfmt.toml, .rustfmt.toml
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml (edition)
    pub toolchain_configs: Vec<ConfigFile>,  // rust-toolchain.toml (channel)
    pub policy: GuardrailPolicy,
}
```

### ToolchainInput
```rust
pub struct ToolchainInput {
    pub roots: Vec<RootInfo>,
    pub toolchain_configs: Vec<ConfigFile>,  // rust-toolchain.toml
    pub legacy_toolchain_files: Vec<ConfigFile>, // rust-toolchain (no .toml)
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml (rust-version)
}
```

### DepsInput
```rust
pub struct DepsInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // all Cargo.toml
    pub lockfiles: Vec<ConfigFile>,          // Cargo.lock
    pub gitignore_files: Vec<ConfigFile>,    // .gitignore
    pub tools: Vec<ToolStatus>,              // installed tool checks
    pub policy: GuardrailPolicy,
}
```

### TestInput
```rust
pub struct TestInput {
    pub roots: Vec<RootInfo>,
    pub cargo_manifests: Vec<ConfigFile>,    // Cargo.toml
    pub mutants_configs: Vec<ConfigFile>,    // .cargo/mutants.toml
    pub nextest_configs: Vec<ConfigFile>,    // .config/nextest.toml
    pub rs_file_rels: Vec<String>,           // .rs file paths (not content)
    pub dir_tree: DirTree,                   // for test file categorization
    pub tools: Vec<ToolStatus>,
}
```

## Topology (not a family)

Topology reports legality findings. It stays in the app:

```rust
// In the app, after legality:
pub fn report_topology(legality: &RustLegalityFacts) -> Vec<CheckResult>
```

It receives legality output directly — root classifications, zone overlaps,
misplaced roots, input failures. No filesystem access, no FamilyView.

## Migration plan

### Phase 1: Create check-types package
1. Create `packages/guardrail3-check-types/` with the shared types above
2. Move `CheckResult` and `Severity` from `domain/report` to check-types
3. Have `domain/report` re-export from check-types for backward compat

### Phase 2: Extract one family as proof of concept
Pick the simplest family — **toolchain** (fewest inputs, no .rs parsing,
no tool checking, 4 rules, 48 tests).

1. Create `packages/guardrail3-family-toolchain/`
2. Define `ToolchainInput` using check-types
3. Move facts + inputs + rules from `apps/.../families/toolchain/crates/runtime/`
4. Change facts::collect() to take `&ToolchainInput` instead of `&FamilyView`
5. Create `input_builder::build_toolchain_input()` in the app
6. Update runner to build input → call package → collect results
7. Verify all 48 tests pass

### Phase 3: Extract remaining families
Order by complexity (simplest first):
1. toolchain (phase 2) — 4 rules, config-only
2. fmt — 8 rules, config-only
3. cargo — 15 rules, Cargo.toml only
4. clippy — 25 rules, config + coverage
5. deny — 30 rules, config + coverage
6. deps — 12 rules, needs ToolStatus
7. test — 18 rules, needs ToolStatus + .rs paths
8. code — 36 rules, needs .rs content
9. garde — 14 rules, needs .rs AST + clippy config
10. arch — 9 rules, needs dir tree + facades
11. hexarch — 27 rules, needs dir tree + deps

### Phase 4: Move topology to legality report
Extract topology from the family system. Make it a report function that
takes legality output and produces CheckResults.

### Phase 5: Delete FamilyView
Once all families use typed inputs, FamilyView is dead code. Delete it
along with the mapper's route types. The input_builder replaces both.

## Key decisions

**Parsing stays in the family.** The app passes raw content strings. Each
family parses TOML/AST internally. This keeps families self-contained and
avoids a shared parsing layer that would need to know every field every
family needs.

**Workspace iteration stays in the app.** The app iterates workspace roots
and calls the family per-workspace (or once for project-wide families). The
family doesn't know about workspace iteration.

**Facts stay inside the family.** The internal facts/inputs/rules pipeline
doesn't change. Only the entry point changes: from `check(&FamilyView, &Route)`
to `check(&XInput)`.

**Tests move with the family.** Each package has its own tests. No more
cross-crate test_support/assertions dependencies — those fold into the
package's test module.

## What this enables

- Families can be versioned independently
- Families can be developed/tested without the full app
- New families can be added without touching the app's internals
- The type system enforces file access boundaries
- Families become reusable outside guardrail3 (e.g., in CI tools, IDE plugins)
