# Plan: ProjectTree — Parse Once, Check Many

> Historical note: this is background architecture context, not the current `RS-ARCH` source of truth. For live `arch` behavior, use:
> - `apps/guardrail3/crates/app/rs/families/arch/README.md`
> - `apps/guardrail3/crates/app/rs/README.md`
> - `.plans/todo/checks/rs/arch.md`

**Date:** 2026-03-21
**Prerequisite for:** RS-ARCH-01 rules 07-11, checker architecture refactor

## Problem

Current checks interleave filesystem access with validation logic. Each check calls `fs.list_dir()`, `fs.metadata()`, `fs.read_file()` inline. This creates three problems:

1. **Cross-file checks are impossible.** Workspace rule 07 needs both `[workspace].members` from `Cargo.toml` AND the full set of discovered crate paths. The crate paths are only known after the tree walk completes. Currently there's no way to pass that information — each check fires independently during the walk.

2. **Duplicate filesystem access.** Multiple checks enumerate the same directories. The walker and the checks are entangled.

3. **Checks are hard to test.** Every test copies a golden fixture to a temp dir, mutates real files, runs the full check pipeline. With a pre-parsed model, tests construct the tree in code — no filesystem, no temp dirs.

## Solution

A `ProjectTree` captures the full state of the project before any checks run:

1. **Full directory structure** — every dir and file that exists, everywhere in the project
2. **Cached content of every config file** — all TOMLs, JSONs, YAMLs, config files. Everything we check that isn't source code.
3. **No source file content** — `.rs`, `.ts`, `.tsx` files appear in the structure (we know they exist and where) but their content is NOT cached. Source scan checks stream those on demand.

## Representation

### In-memory: flat maps with relative path keys

```rust
use std::collections::BTreeMap;
use std::path::PathBuf;

/// The full project tree. Built once by the walker, consumed by all checkers.
/// Serializable to JSON for debugging (`guardrail3 dump-tree`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTree {
    /// Absolute path to project root
    pub root: PathBuf,

    /// Directory structure: every dir visited → its children.
    /// Keyed by relative path from root. "" = root directory itself.
    pub structure: BTreeMap<String, DirEntry>,

    /// Cached config file contents, keyed by relative path from root.
    /// Contains every config file we check. Does NOT contain source code.
    pub content: BTreeMap<String, String>,
}

/// A single directory's immediate children.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
    /// Child directory names (just names, not paths)
    pub dirs: Vec<String>,
    /// Child file names (just names, not paths)
    pub files: Vec<String>,
}
```

Relative paths as keys — not absolute. `"apps/devctl/crates/app"` not `"/Users/foo/project/apps/devctl/crates/app"`. This makes the tree portable, testable, and serializable without leaking machine paths.

### Querying the tree

```rust
impl ProjectTree {
    /// Check if a directory exists
    fn dir_exists(&self, rel: &str) -> bool {
        self.structure.contains_key(rel)
    }

    /// Get children of a directory
    fn dir_contents(&self, rel: &str) -> Option<&DirEntry> {
        self.structure.get(rel)
    }

    /// Get cached config file content
    fn file_content(&self, rel: &str) -> Option<&str> {
        self.content.get(rel).map(String::as_str)
    }

    /// Absolute path for a relative path
    fn abs_path(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }
}
```

### Serialization format: JSON

`ProjectTree` derives `Serialize`/`Deserialize`. The canonical serialization is JSON.

`guardrail3 dump-tree [path]` outputs:

```json
{
  "root": "/Users/dev/myproject",
  "structure": {
    "": { "dirs": ["apps", "packages", ".github"], "files": ["Cargo.toml", "clippy.toml", "deny.toml"] },
    "apps": { "dirs": ["devctl", "backend", "worker", "admin", "landing"], "files": [] },
    "apps/devctl": { "dirs": ["crates"], "files": ["Cargo.toml"] },
    "apps/devctl/crates": { "dirs": ["app", "domain", "adapters", "ports"], "files": [] },
    "apps/devctl/crates/app": { "dirs": ["core"], "files": [] },
    "apps/devctl/crates/app/core": { "dirs": ["src"], "files": ["Cargo.toml"] },
    "apps/devctl/crates/app/core/src": { "dirs": [], "files": ["lib.rs"] },
    "packages": { "dirs": ["shared-types", "ui-kit"], "files": [] },
    "packages/shared-types": { "dirs": ["src"], "files": ["Cargo.toml"] }
  },
  "content": {
    "Cargo.toml": "[workspace]\nmembers = [\"packages/shared-types\"]\nresolver = \"2\"",
    "clippy.toml": "max-struct-bools = 3\n...",
    "deny.toml": "[graph]\ntargets = []\n...",
    "apps/devctl/Cargo.toml": "[workspace]\nmembers = [\n    \"crates/domain/types\",\n    ...\n]\nresolver = \"2\"",
    "apps/devctl/crates/app/core/Cargo.toml": "[package]\nname = \"devctl-app-core\"\n...",
    "packages/shared-types/Cargo.toml": "[package]\nname = \"shared-types\"\n..."
  }
}
```

Properties:
- **Relative paths everywhere** — keys are always relative to root
- **Sorted** — BTreeMap gives deterministic ordering, diffs are meaningful
- **Human-readable** — you can grep the JSON to find "where is clippy.toml", "what apps exist"
- **Portable** — same project tree regardless of where it's checked out
- **Diffable** — two runs produce the same JSON if the project hasn't changed

### Why flat maps, not nested tree

A nested tree (`{ "apps": { "devctl": { "crates": { ... } } } }`) mirrors the filesystem but:
- O(n) path traversal to reach a deep node
- Harder to query "does apps/devctl/crates/app exist?" — you parse the path and walk
- More complex data structure (recursive types)

Flat maps with path keys:
- O(log n) lookup by path
- "Does X exist?" is `tree.dirs.contains_key("apps/devctl/crates/app")`
- "What's in X?" is `tree.dirs["apps/devctl/crates/app"]`
- Navigation: look up dir → get child names → join path → look up child
- Trivially serializable

## The Walker

Single function: `pub fn walk_project(fs: &dyn FileSystem, root: &Path) -> ProjectTree`

### What it does

1. **Recursively enumerate directories** starting from `root`. For each directory visited, record its `DirEntry` in `tree.structure` (child dir names + child file names).

2. **For each file encountered**, check if its name matches the config file list. If yes, read content and store in `tree.content`.

3. **Skip irrelevant subtrees** for performance:
   - `node_modules/`, `target/`, `.git/`, `dist/`, `build/`, `.next/`

### Config files to cache (the complete list)

Every file checked by any rule that isn't source code:

| File | Checked by |
|------|-----------|
| `Cargo.toml` (all of them) | cargo, arch, release, garde, test |
| `Cargo.lock` | deps (banned crate check) |
| `clippy.toml` / `.clippy.toml` | clippy |
| `deny.toml` | deny |
| `rustfmt.toml` | fmt |
| `rust-toolchain.toml` | toolchain |
| `guardrail3.toml` | config |
| `release-plz.toml` | release |
| `cliff.toml` | release |
| `.cargo/mutants.toml` | test |
| `package.json` (all of them) | ts config, deps |
| `.npmrc` | ts config |
| `tsconfig.json` / `tsconfig.base.json` | ts config |
| `eslint.config.mjs` | ts config |
| `cspell.json` | ts tools |
| `stryker.config.json` / `stryker.config.mjs` | ts test |
| `vitest.config.ts` / `jest.config.*` | ts test |
| `.github/workflows/*.yml` / `*.yaml` | release |
| `.git/hooks/pre-commit` | hooks |
| `CLAUDE.md` | deps |
| `.gitkeep` | arch (existence only, content is empty) |

### What the walker does NOT do

- Parse file content (TOML, JSON, YAML parsing is the checker's job)
- Interpret structure (which dirs are "apps" vs "packages" — checker's job)
- Know about hex arch, workspaces, or any rules (dumb tree builder)

## Typed Views (extract when needed, not upfront)

No pre-designed view structs. Orchestrators navigate the raw `ProjectTree` directly and feed checks their inputs. If repeated tree query patterns emerge across multiple checks, extract a shared struct at that point.

The arch orchestrator will likely need some shared navigation logic (finding containers, classifying leaves, recursing hex-in-hex). But the shape of that logic — and whether it warrants a typed struct or just helper functions — becomes clear when writing the actual checks, not before.

## Orchestrator Pattern

**Individual rules never see the tree.** Each rule is a pure function that receives exactly the input it needs — a parsed TOML value, a typed struct, a file path + content. The orchestrator is what sees the tree, extracts what's needed, and feeds it to rules.

Three layers:
1. **Walker** — builds `ProjectTree` from filesystem (the only thing that touches `fs` for config files)
2. **Orchestrator** (one per checker group) — reads from `ProjectTree`, extracts/parses inputs, calls rules
3. **Rules** — pure functions: input → errors. No tree, no fs, no discovery.

### Example: clippy checker

```rust
// Orchestrator (mod.rs) — sees the tree, feeds rules
pub fn check(tree: &ProjectTree, results: &mut Vec<CheckResult>) {
    // Extract what clippy rules need from the tree
    let raw = tree.content.get("clippy.toml");

    // Rule 01 just needs to know if the file exists
    rule_01::check(raw.is_some(), results);

    // Rules 02-07 need the parsed TOML
    if let Some(content) = raw {
        let parsed: toml::Value = match toml::from_str(content) {
            Ok(v) => v,
            Err(_) => { /* push parse error, return */ }
        };
        rule_02::check(&parsed, results);   // max-struct-bools
        rule_03::check(&parsed, results);   // max-fn-params-bools
        rule_04::check(&parsed, results);   // method bans
        // ...
    }
}

// Rule (rule_02_struct_bools.rs) — never sees tree or fs
pub fn check(clippy_toml: &toml::Value, results: &mut Vec<CheckResult>) {
    let threshold = clippy_toml.get("max-struct-bools")
        .and_then(|v| v.as_integer());
    if threshold != Some(3) {
        results.push(/* ... */);
    }
}
```

### Example: cargo checker (discovers multiple Cargo.toml files)

```rust
// Orchestrator — discovers Cargo.toml files from content map, parses, feeds to rules
pub fn check(tree: &ProjectTree, results: &mut Vec<CheckResult>) {
    // Root workspace lints
    if let Some(raw) = tree.content.get("Cargo.toml") {
        let parsed: toml::Value = toml::from_str(raw)?;
        rule_01::check(&parsed, results);   // lint completeness
        rule_02::check(&parsed, results);   // lint levels
    }

    // Per-crate lint inheritance — discover all crate Cargo.toml files
    for (path, raw) in &tree.content {
        if path == "Cargo.toml" { continue; }       // skip root
        if !path.ends_with("/Cargo.toml") { continue; }
        let parsed: toml::Value = toml::from_str(raw)?;
        rule_04::check(path, &parsed, results);     // lints.workspace = true
    }
}

// Rule (rule_04_inheritance.rs) — receives one Cargo.toml, checks one thing
pub fn check(path: &str, cargo_toml: &toml::Value, results: &mut Vec<CheckResult>) {
    let has_workspace_lints = cargo_toml
        .get("lints").and_then(|l| l.get("workspace"))
        .and_then(|w| w.as_bool()) == Some(true);
    if !has_workspace_lints {
        results.push(/* ... */);
    }
}
```

### Example: arch checker (navigates tree structure, feeds rules)

```rust
// Orchestrator — navigates tree, extracts what each rule needs
pub fn check(tree: &ProjectTree, results: &mut Vec<CheckResult>) {
    // Discover Rust apps: dirs under apps/ that have Cargo.toml
    let apps_entry = tree.dir_contents("apps");
    for app_name in apps_entry.map(|e| &e.dirs).unwrap_or(&vec![]) {
        let app_rel = format!("apps/{app_name}");
        if tree.file_content(&format!("{app_rel}/Cargo.toml")).is_none() {
            continue; // not a Rust app
        }

        // Rule 12: src/ banned
        let has_src = tree.dir_exists(&format!("{app_rel}/src"));
        check_12::check(app_name, has_src, results);

        // Rule 01: crates/ exists
        let crates_rel = format!("{app_rel}/crates");
        if !tree.dir_exists(&crates_rel) {
            check_01::check(app_name, results);
            continue;
        }

        // Rules 02-06: navigate containers, feed each rule its input
        let crates_entry = tree.dir_contents(&crates_rel).unwrap();
        check_02::check(app_name, &crates_entry.dirs, results);
        // ... walk containers, call check_05, check_06 per container
    }

    // Rule 11: root workspace must not include apps
    let root_cargo = tree.file_content("Cargo.toml");
    check_11::check(root_cargo, &app_names, results);
}

// Rule (check_12_src_banned.rs) — receives a bool
pub fn check(app_name: &str, has_src: bool, results: &mut Vec<CheckResult>) {
    if has_src {
        results.push(/* "Service `{app_name}` has src/ directory" */);
    }
}
```

### Example: source scan (exception — orchestrator uses fs for content)

```rust
// Orchestrator — uses tree for discovery, fs for reading source content
pub fn check(tree: &ProjectTree, fs: &dyn FileSystem, results: &mut Vec<CheckResult>) {
    for (rel_dir, entry) in &tree.structure {
        for file_name in &entry.files {
            if !file_name.ends_with(".rs") { continue; }
            let rel_path = format!("{rel_dir}/{file_name}");
            let abs_path = tree.abs_path(&rel_path);
            let Some(content) = fs.read_file(&abs_path) else { continue };
            // Feed each rule the file path + content
            rule_09::check(&rel_path, &content, results);    // file length
            rule_13::check(&rel_path, &content, results);    // unsafe
            rule_14::check(&rel_path, &content, results);    // todo
        }
    }
}

// Rule (rule_09_file_length.rs) — receives path + content, nothing else
pub fn check(path: &str, content: &str, results: &mut Vec<CheckResult>) {
    let lines = content.lines().count();
    if lines > 500 {
        results.push(/* ... */);
    }
}
```

### Pattern summary

| Checker group | Orchestrator sees | Rules receive |
|---|---|---|
| clippy | `tree.content["clippy.toml"]` | `&toml::Value` |
| deny | `tree.content["deny.toml"]` | `&toml::Value` |
| fmt | `tree.content["rustfmt.toml"]` | `&toml::Value` |
| toolchain | `tree.content["rust-toolchain.toml"]` | `&toml::Value` |
| cargo | all `*/Cargo.toml` from content map | `(path, &toml::Value)` per file |
| arch | tree navigation (structure map) | `(&str, &[String])`, `(&str, bool)` etc. |
| deps | `tree.content["Cargo.lock"]` + tool checks | `&str` (lock content), `bool` (tool exists) |
| source | tree for discovery, `fs` for content | `(&str, &str)` (path + content) per file |
| release | `*/Cargo.toml` + workflow YAMLs from content | per-file parsed content |
| garde | `Cargo.toml` + `clippy.toml` + source files | mix of parsed TOML + source content |

## File Layout

```
crates/domain/
├── project_tree.rs             # ProjectTree, DirEntry (the raw model)

crates/app/
├── walker.rs                   # walk_project() — builds ProjectTree from FileSystem

crates/app/rs/validate/arch/
├── rs_arch_01/
│   ├── mod.rs                  # orchestrator — navigates tree, feeds checks
│   ├── check_01..check_12      # pure functions, receive extracted inputs
```

`ProjectTree` in domain — it's the domain model of "what a project looks like."
Walker in app — it uses the `FileSystem` port.
Orchestrators alongside their checks — each checker group navigates the tree itself.

## Testing

### Walker tests (integration, golden fixture)
```rust
fn walker_captures_full_structure() {
    let tmp = copy_golden("tests/fixtures/r_arch_01/golden");
    let tree = walk_project(&RealFileSystem, tmp.path());
    // Structure captured
    assert!(tree.dir_exists("apps/devctl/crates/app"));
    // Config file content cached
    assert!(tree.file_content("apps/devctl/Cargo.toml").unwrap().contains("[workspace]"));
    // Source files appear in structure but NOT in content cache
    assert!(tree.structure["apps/devctl/crates/app/core/src"].files.contains(&"lib.rs".to_string()));
    assert!(!tree.content.contains_key("apps/devctl/crates/app/core/src/lib.rs"));
}
```

### Orchestrator + check tests (unit, construct tree in code)
```rust
fn arch_check_finds_missing_crates() {
    let tree = ProjectTree {
        root: PathBuf::from("/project"),
        structure: btreemap! {
            "".into() => DirEntry { dirs: vec!["apps".into()], files: vec!["Cargo.toml".into()] },
            "apps".into() => DirEntry { dirs: vec!["devctl".into()], files: vec![] },
            "apps/devctl".into() => DirEntry { dirs: vec![], files: vec!["Cargo.toml".into()] },
            // Note: no "apps/devctl/crates" entry — crates/ doesn't exist
        },
        content: btreemap! {
            "apps/devctl/Cargo.toml".into() => "[workspace]\nmembers = []\nresolver = \"2\"".into(),
        },
    };
    let mut results = Vec::new();
    rs_arch_01::check(&tree, &mut results);
    assert_eq!(results.len(), 1);
    assert!(results[0].title.contains("missing crates/"));
}
```

No temp dirs, no filesystem, microsecond tests.

### Dump-tree integration test
```rust
fn dump_tree_is_valid_json() {
    let tree = walk_project(&RealFileSystem, project_root);
    let json = serde_json::to_string_pretty(&tree).unwrap();
    let roundtrip: ProjectTree = serde_json::from_str(&json).unwrap();
    assert_eq!(tree.structure.len(), roundtrip.structure.len());
}
```

## Migration Strategy

1. **Add `ProjectTree` + walker** — new domain type + app walker, no existing code changes
2. **Add `guardrail3 dump-tree`** — CLI command, useful immediately for debugging
3. **Implement rules 07, 08, 10, 11** — new checks with orchestrator navigating tree
4. **Migrate existing arch checks** — from `(fs, ...)` to `(tree, ...)`, one at a time
5. **Migrate other checkers** — clippy, deny, fmt, cargo, etc. write orchestrators
6. **Extract shared structs** — if repeated patterns emerge across checks, factor out typed views
7. **Convert tests** — bulk of adversarial tests become tree construction tests

Steps 1-3 don't touch existing code. Steps 4-7 are incremental.

## Open Questions

1. **Walk depth**: Walk everything under project root (minus skip list)? Or only known paths? Walking everything is safest and simplest. Skip list handles the expensive dirs (node_modules, target, .git).

2. **Large file handling**: `Cargo.lock` can be large (hundreds of KB). Cache it anyway — it's checked by deps rules and there's only one. Workflow YAMLs are small.

3. **Incremental walks**: For `--staged`/`--dirty` modes, we still need the full tree for structural checks. But we could skip caching files that haven't changed. Future optimization, not needed now.

4. **Should .gitkeep content be cached?** It's always empty. We could just record its existence in the dir's file list and not cache content. The `has_gitkeep` check becomes `entry.files.contains(".gitkeep")` instead of `tree.file_content(path).is_some()`. Simpler.
