# Checker Architecture — Per-rule files, ProjectTree input

**Date:** 2026-03-21 (updated 2026-03-21)

## Principles

1. **One rule, one file.** Every check is a single `.rs` file, independently testable, greppable by ID.
2. **One rule group, one folder.** Rules that check the same file type live in one directory with a `mod.rs` orchestrator.
3. **`ProjectTree` is the repository snapshot.** The walker builds it once. It is the only shared project-wide discovery object. (Replaces the old `CrawlResult` + `fs: &dyn FileSystem` pattern.)
4. **There is an explicit middle layer between `ProjectTree` and rules.** Family orchestrators do two jobs:
   - read the `ProjectTree` / streamed source files
   - construct minimal typed inputs for each rule
5. **Rules are pure functions over typed inputs.** Input → errors. No I/O, no tree-walking, no path discovery, no hidden state.
6. **Source files are the exception.** Source scan rules (`.rs`, `.ts`) get file content streamed by the orchestrator via `fs` — not cached in the tree. The tree gives discovery (which files exist), the orchestrator reads content on demand.

## Pipeline

The architecture is a 4-stage pipeline:

```text
Project walker
  -> ProjectTree
  -> family orchestrator
  -> typed rule inputs
  -> pure rule functions
```

This middle layer is required. Rules should never receive the full tree when a smaller input struct will do.

### Layer responsibilities

**1. Walker**
- Traverses the repo once
- Applies ignore / git rules
- Builds `ProjectTree`
- Caches config-file content only

**2. ProjectTree**
- Repository snapshot, not business logic
- Knows what files/dirs/config blobs exist
- Does not know rule semantics

**3. Family orchestrator**
- Owns all extraction and normalization logic for a checker family
- Converts `ProjectTree` into typed, family-specific inputs
- Handles parse-once work shared by many rules
- Streams source files when needed
- Calls per-rule functions

**4. Rule**
- Receives minimal typed input
- Emits results
- No filesystem access
- No `ProjectTree` access
- No parsing of unrelated files

## The missing middle layer: typed rule inputs

The key abstraction is not just `ProjectTree`, but **family-specific input structs** constructed by the orchestrator.

Examples:

```rust
pub struct ClippyTomlInput<'a> {
    pub rel_path: &'a str,
    pub parsed: &'a toml::Value,
    pub profile: Option<&'a str>,
}

pub struct HookScriptInput<'a> {
    pub rel_path: &'a str,
    pub raw: &'a str,
    pub executable_lines: &'a [ExecutableLine<'a>],
    pub is_modular: bool,
}

pub struct RustSourceFileInput<'a> {
    pub rel_path: &'a str,
    pub content: &'a str,
    pub ast: &'a syn::File,
    pub is_test: bool,
    pub profile: Option<&'a str>,
}
```

This keeps rules small and prevents every rule from re-implementing its own discovery/parsing logic.

## Naming

**Rule IDs:** `RS-CLIPPY-04`, `TS-ESLINT-12`, `HOOK-RS-03`, `DEPLOY-TS-01`

**File names:** `rs_clippy_04_method_bans.rs` — prefix matches the ID, 1-2 word suffix describes the rule.

**Folder names:** `checks/rs/clippy/`, `checks/ts/eslint/`, `checks/hooks/shared/`

## Folder structure (example, not exhaustive)

The definitive rule-by-rule list is in `.plans/todo/checks/`. This is just the structural pattern.

```
crates/app/checks/
├── mod.rs                                  # top-level orchestrator
│
├── rs/
│   ├── mod.rs
│   ├── clippy/                             # 22 rules — reads clippy.toml
│   │   ├── mod.rs                          # orchestrator: gets clippy.toml from tree, parses, feeds rules
│   │   ├── rs_clippy_01_exists.rs
│   │   ├── rs_clippy_04_method_bans.rs
│   │   ├── rs_clippy_05_type_bans.rs
│   │   └── ...
│   ├── deny/                               # 20 rules — reads deny.toml
│   │   ├── mod.rs
│   │   ├── rs_deny_01_exists.rs
│   │   └── ...
│   ├── fmt/                                # 8 rules — reads rustfmt.toml
│   │   ├── mod.rs
│   │   └── ...
│   ├── toolchain/                          # 4 rules — reads rust-toolchain.toml
│   │   ├── mod.rs
│   │   └── ...
│   ├── cargo/                              # 9 rules — reads Cargo.toml files
│   │   ├── mod.rs
│   │   └── ...
│   ├── source/                             # 29 rules — reads *.rs (syn AST, streamed)
│   │   ├── mod.rs
│   │   └── ...
│   ├── hexarch/                            # 18 rules — reads ProjectTree structure
│   │   ├── mod.rs
│   │   └── ...
│   ├── deps/                               # 9 rules — tool checks + Cargo.lock
│   │   ├── mod.rs
│   │   └── ...
│   ├── garde/                              # 9 rules — reads Cargo.toml + clippy.toml + *.rs
│   │   ├── mod.rs
│   │   └── ...
│   ├── test/                               # 18 rules — reads Cargo.toml + configs + *.rs
│   │   ├── mod.rs
│   │   └── ...
│   └── release/                            # 26 rules — reads Cargo.toml + workflows + configs
│       ├── mod.rs
│       └── ...
│
├── ts/
│   ├── mod.rs
│   ├── eslint/                             # reads eslint.config.mjs (tree-sitter)
│   ├── tsconfig/                           # reads tsconfig.json
│   ├── npmrc/                              # reads .npmrc
│   ├── package/                            # reads package.json files
│   ├── jscpd/                              # reads .jscpd.json
│   ├── plugins/                            # reads eslint plugin configs
│   ├── stylelint/                          # reads stylelint configs
│   ├── tools/                              # reads various tool configs
│   ├── source/                             # reads *.ts/*.tsx (tree-sitter, streamed)
│   ├── test/                               # reads test configs
│   └── arch/                               # reads ProjectTree structure
│
├── hooks/
│   ├── mod.rs
│   ├── shared/                             # language-agnostic hook structure
│   ├── rs/                                 # Rust-specific hook steps
│   └── ts/                                 # TS-specific hook steps
│
└── deploy/
    ├── mod.rs
    └── ts/                                 # TS deployment checks
```

## Orchestrator pattern

Each `mod.rs` is an orchestrator. It receives `&ProjectTree`, extracts what its rules need, builds typed inputs, and feeds them. Rules never see the tree.

```rust
// checks/rs/clippy/mod.rs — orchestrator

pub fn check(tree: &ProjectTree, results: &mut Vec<CheckResult>) {
    // Get clippy.toml content from tree (already cached)
    let raw = tree.file_content("clippy.toml");

    // Rule 01 just needs existence
    rs_clippy_01_exists::check(raw.is_some(), results);

    // Rules 02+ need parsed TOML
    let Some(content) = raw else { return };
    let Ok(parsed) = toml::from_str::<toml::Value>(content) else {
        // push parse error
        return;
    };
    let input = ClippyTomlInput {
        rel_path: "clippy.toml",
        parsed: &parsed,
        profile: None,
    };
    rs_clippy_04_method_bans::check(&input, results);
    rs_clippy_05_type_bans::check(&input, results);
    // ...
}
```

```rust
// checks/rs/clippy/rs_clippy_04_method_bans.rs — one rule

const ID: &str = "RS-CLIPPY-04";

pub fn check(input: &ClippyTomlInput<'_>, results: &mut Vec<CheckResult>) {
    // pure function: typed family input in, errors out
}
```

For source scan (exception — streams file content):

```rust
// checks/rs/source/mod.rs — orchestrator

pub fn check(tree: &ProjectTree, fs: &dyn FileSystem, results: &mut Vec<CheckResult>) {
    // Discover .rs files from tree structure
    for (dir_rel, entry) in &tree.structure {
        for file_name in &entry.files {
            if !file_name.ends_with(".rs") { continue; }
            let rel = ProjectTree::join_rel(dir_rel, file_name);
            let abs = tree.abs_path(&rel);
            let Some(content) = fs.read_file(&abs) else { continue };
            let Ok(ast) = syn::parse_file(&content) else { continue };
            let input = RustSourceFileInput {
                rel_path: &rel,
                content: &content,
                ast: &ast,
                is_test: is_test_file(&rel),
                profile: None,
            };
            // Feed each rule
            rs_source_09_file_length::check(&input, results);
            rs_source_13_unsafe::check(&input, results);
            // ...
        }
    }
}
```

## Family-level extractor pattern

Every checker family should have a small extractor boundary before rule calls.

Examples:
- `checks/rs/clippy/mod.rs`
  - reads + parses `clippy.toml`
  - builds `ClippyTomlInput`
- `checks/rs/source/mod.rs`
  - discovers `.rs` files from the tree
  - streams file content
  - parses `syn::File`
  - builds `RustSourceFileInput`
- `checks/hooks/shared/mod.rs`
  - reads hook file(s)
  - classifies executable vs comment lines
  - builds `HookScriptInput`
- `checks/rs/hexarch/mod.rs`
  - reads tree structure + relevant `Cargo.toml`s
  - builds layer/workspace facts
  - feeds structural rules

This is where shared parsing and normalization lives. If two rules need the same parsed object, the orchestrator parses once.

## Testing

**Rule tests:** construct the minimal typed input struct and call the rule function. No tree, no filesystem, microseconds.

**Orchestrator tests:** construct a `ProjectTree` in code, call the orchestrator, verify extraction + wiring. No filesystem unless the family is a streamed-source family.

**Integration tests:** walk a real fixture (golden or adversarial), run the full pipeline, verify end-to-end. These catch wiring bugs.

**Walker tests:** already implemented — lossless verification against `git ls-files` + `walkdir` + `git check-ignore` on real projects.

## Migration

1. Create `checks/` skeleton alongside existing `rs/validate/`
2. Migrate one checker at a time (smallest first: fmt, toolchain)
3. Old orchestrator delegates to new checker, verifies same output
4. Once all checkers migrated, remove old `rs/validate/` files
5. Parallelize: once all checkers are pure, swap `iter` for `par_iter`

## Family-by-family implementation order

Build families in ascending complexity:

1. `rs/fmt`
2. `rs/toolchain`
3. `rs/clippy`
4. `rs/deny`
5. `rs/cargo`
6. `rs/source`
7. `rs/hexarch`
8. `rs/deps`
9. `rs/garde`
10. `rs/test`
11. `rs/release`

For each family:

1. define `facts.rs`
2. define `inputs.rs`
3. write `mod.rs` orchestrator
4. migrate the simplest 1-2 rules first
5. add orchestrator tests for extraction/fan-out
6. migrate remaining rules

This ensures the hard part is solved once per family: extraction and atomic input design.

## First family: `rs/fmt`

`rs/fmt` is the template for single-config-file families.

### Files

```text
checks/rs/fmt/
├── mod.rs
├── facts.rs
├── inputs.rs
├── rs_fmt_01_exists.rs
├── rs_fmt_02_settings.rs
├── rs_fmt_03_extra_settings.rs
├── rs_fmt_04_nightly_on_stable.rs
├── rs_fmt_05_per_crate_override.rs
├── rs_fmt_06_edition_mismatch.rs
├── rs_fmt_07_ignore_escape_hatch.rs
└── rs_fmt_08_dual_file_conflict.rs
```

### Facts

```rust
pub struct RustfmtFacts {
    pub root_config_rel: Option<String>,
    pub root_config_kind: Option<RustfmtConfigKind>, // rustfmt.toml vs .rustfmt.toml
    pub root_parsed: Option<toml::Value>,
    pub extra_config_rels: Vec<String>,              // non-root rustfmt configs
    pub dual_file_conflicts: Vec<String>,            // dirs that contain both variants
    pub workspace_edition: Option<String>,
    pub toolchain_channel: Option<String>,
}
```

This is already normalized enough that rules never need to discover config files themselves.

### Inputs

```rust
pub struct RustfmtRootInput<'a> {
    pub facts: &'a RustfmtFacts,
}

pub struct RustfmtExtraConfigInput<'a> {
    pub rel_path: &'a str,
}

pub struct RustfmtDualConflictInput<'a> {
    pub dir_rel: &'a str,
}
```

### Fan-out strategy

- `RS-FMT-01..04`, `06`, `07` run once on `RustfmtRootInput`
- `RS-FMT-05` runs once per `RustfmtExtraConfigInput`
- `RS-FMT-08` runs once per `RustfmtDualConflictInput`

No rule gets a list unless the rule itself is an inventory/set rule.

### Orchestrator responsibilities

`checks/rs/fmt/mod.rs`:
- find root rustfmt config via tree
- detect non-root configs
- detect dual-file conflicts
- parse root config once
- read workspace Cargo edition once
- read toolchain channel once
- build `RustfmtFacts`
- fan out the three input shapes above

### Tests

**Rule tests**
- construct `RustfmtRootInput` or `RustfmtExtraConfigInput` directly

**Orchestrator tests**
- root config only
- extra per-crate config discovered
- both `rustfmt.toml` and `.rustfmt.toml` in one dir
- root config + stable toolchain + nightly-only keys

## Second family: `rs/cargo`

`rs/cargo` is the template for parent/child and set-diff families.

### Files

```text
checks/rs/cargo/
├── mod.rs
├── facts.rs
├── inputs.rs
├── discover.rs
├── rs_cargo_01_workspace_lints.rs
├── rs_cargo_02_lint_levels.rs
├── rs_cargo_03_allow_inventory.rs
├── rs_cargo_04_lint_inheritance.rs
├── rs_cargo_05_workspace_metadata.rs
├── rs_cargo_06_no_weakened_overrides.rs
├── rs_cargo_07_priority_order.rs
├── rs_cargo_08_resolver.rs
└── rs_cargo_09_member_edition_drift.rs
```

### Facts

```rust
pub struct WorkspaceCargoFacts {
    pub rel_path: String,
    pub parsed: toml::Value,
    pub declared_members: BTreeSet<String>,
    pub workspace_edition: Option<String>,
    pub workspace_rust_version: Option<String>,
    pub resolver: Option<String>,
    pub profile: Option<String>,
}

pub struct MemberCargoFacts {
    pub rel_path: String,
    pub parsed: toml::Value,
    pub package_name: Option<String>,
    pub edition: Option<String>,
    pub lint_workspace_true: bool,
}

pub struct CargoFamilyFacts {
    pub workspace: WorkspaceCargoFacts,
    pub members: Vec<MemberCargoFacts>,
    pub discovered_member_rels: BTreeSet<String>,
}
```

### Inputs

```rust
pub struct WorkspaceCargoInput<'a> {
    pub workspace: &'a WorkspaceCargoFacts,
}

pub struct WorkspaceMemberInput<'a> {
    pub workspace: &'a WorkspaceCargoFacts,
    pub member: &'a MemberCargoFacts,
}

pub struct WorkspaceMembersSetInput<'a> {
    pub workspace: &'a WorkspaceCargoFacts,
    pub declared_members: &'a BTreeSet<String>,
    pub discovered_members: &'a BTreeSet<String>,
}
```

### Fan-out strategy

- `RS-CARGO-01`, `02`, `03`, `05`, `08` run once on `WorkspaceCargoInput`
- `RS-CARGO-04`, `06`, `09` run once per `WorkspaceMemberInput`
- any rule that compares membership sets runs once on `WorkspaceMembersSetInput`

This is the key parent/child pattern:
- the rule does **not** receive all children and crawl them
- the orchestrator binds exactly one `workspace + member` pair
- the rule only checks that pair

### Orchestrator responsibilities

`checks/rs/cargo/mod.rs` and `discover.rs`:
- identify the workspace Cargo.toml being checked
- parse workspace once
- resolve declared workspace members
- discover actual member `Cargo.toml` files from the tree
- parse each child once
- bind one `WorkspaceMemberInput` per discovered member
- build one `WorkspaceMembersSetInput` for set comparison rules

### Tests

**Rule tests**
- construct one `WorkspaceMemberInput` with minimal workspace/member TOML
- assert one pair produces one expected result

**Orchestrator tests**
- N member crates -> N `WorkspaceMemberInput`s
- excluded or non-member crates never get paired
- declared vs discovered set diff is correct
- missing member Cargo.toml handled in extractor/facts layer, not in every rule

## What changes

- **Rule IDs:** `RS-CLIPPY-04` instead of `R4`
- **Input:** `&ProjectTree` instead of `&dyn FileSystem` + `CrawlResult`
- **Middle layer:** family orchestrators build typed rule inputs from the tree
- **File organization:** one file per rule, one folder per group
- **Purity:** rules are pure functions, orchestrator owns all I/O

## What stays

- `CheckResult` / `Report` / `Severity` — same output types
- CLI interface — same commands, same flags
- Test infrastructure — `copy_golden`, `assert_file_field`, etc.
