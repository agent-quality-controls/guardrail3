# Checker Architecture — Per-rule files, ProjectTree input

**Date:** 2026-03-21 (updated 2026-03-21)

## Principles

1. **One rule, one file.** Every check is a single `.rs` file, independently testable, greppable by ID.
2. **One rule group, one folder.** Rules that check the same file type live in one directory with a `mod.rs` orchestrator.
3. **`ProjectTree` is the input.** The walker builds it once. Orchestrators read from it. Rules never touch the filesystem. (Replaces the old `CrawlResult` + `fs: &dyn FileSystem` pattern.)
4. **Rules are pure functions.** Input → errors. No I/O, no state, no side effects. The orchestrator extracts what each rule needs from the tree and passes it directly.
5. **Source files are the exception.** Source scan rules (`.rs`, `.ts`) get file content streamed by the orchestrator via `fs` — not cached in the tree. The tree gives discovery (which files exist), the orchestrator reads content on demand.

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

Each `mod.rs` is an orchestrator. It receives `&ProjectTree`, extracts what its rules need, and feeds them. Rules never see the tree.

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
    rs_clippy_04_method_bans::check(&parsed, results);
    rs_clippy_05_type_bans::check(&parsed, results);
    // ...
}
```

```rust
// checks/rs/clippy/rs_clippy_04_method_bans.rs — one rule

const ID: &str = "RS-CLIPPY-04";

pub fn check(clippy_toml: &toml::Value, results: &mut Vec<CheckResult>) {
    // pure function: parsed TOML in, errors out
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
            // Feed each rule
            rs_source_09_file_length::check(&rel, &content, results);
            rs_source_13_unsafe::check(&rel, &content, results);
            // ...
        }
    }
}
```

## Testing

**Rule tests:** construct the minimal input (a `toml::Value`, a `&str`, a struct) and call the rule function. No tree, no filesystem, microseconds.

**Orchestrator tests:** construct a `ProjectTree` in code, call the orchestrator, verify results. No filesystem.

**Integration tests:** walk a real fixture (golden or adversarial), run the full pipeline, verify end-to-end. These catch wiring bugs.

**Walker tests:** already implemented — lossless verification against `git ls-files` + `walkdir` + `git check-ignore` on real projects.

## Migration

1. Create `checks/` skeleton alongside existing `rs/validate/`
2. Migrate one checker at a time (smallest first: fmt, toolchain)
3. Old orchestrator delegates to new checker, verifies same output
4. Once all checkers migrated, remove old `rs/validate/` files
5. Parallelize: once all checkers are pure, swap `iter` for `par_iter`

## What changes

- **Rule IDs:** `RS-CLIPPY-04` instead of `R4`
- **Input:** `&ProjectTree` instead of `&dyn FileSystem` + `CrawlResult`
- **File organization:** one file per rule, one folder per group
- **Purity:** rules are pure functions, orchestrator owns all I/O

## What stays

- `CheckResult` / `Report` / `Severity` — same output types
- CLI interface — same commands, same flags
- Test infrastructure — `copy_golden`, `assert_file_field`, etc.
