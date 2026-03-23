# Crawler-Validate Gap Analysis

## 1. Top-Level Validate Command (`commands/validate.rs`)

**Flow:**
1. Canonicalizes path, loads `guardrail3.toml` via `load_config(fs, path.join("guardrail3.toml"))`
2. Calls `discover::detect_project(fs, abs_path)` to get `ProjectInfo` (has_rust, has_typescript, workspaces, package_json_path)
3. Resolves scoped files (--staged, --dirty, --commits, --files)
4. Calls `rs::validate::run(fs, path, project, scoped_files, categories, thorough, tc)` if has_rust
5. Calls `ts::validate::run(fs, path, scoped_files, categories, config)` if has_typescript
6. Calls `hooks::validate::run(...)` always

**Does NOT call the crawler.** Uses `discover::detect_project` instead.

**Hardcoded paths:** `guardrail3.toml` at project root (line 234).

---

## 2. Rust Validate Orchestrator (`app/rs/validate/mod.rs`)

**Flow:**
1. Gets `workspace_root` from `project.primary_workspace_root()` (first workspace in ProjectInfo)
2. Loads `guardrail3.toml` again (redundant — top-level already loaded it)
3. Runs code checks, architecture checks, garde checks, test checks, release checks

**Config file discovery — all hardcoded to `workspace_root.join(...)`:**
- `config_files::check(fs, workspace_root)` — checks for `clippy.toml`, `rustfmt.toml`, `rust-toolchain.toml` at workspace root
- `config_files::check_per_crate_clippy(fs, workspace_root, member_dirs)` — checks `clippy.toml` in each member dir
- `clippy_coverage::check(fs, workspace_root)` — reads `workspace_root.join("clippy.toml")`
- `deny_audit::check(fs, workspace_root)` — reads `workspace_root.join("deny.toml")`
- `cargo_lints::check(fs, workspace_root)` — reads `workspace_root.join("Cargo.toml")`

**Functions that would need to change:**
- `config_files::check()` — currently constructs paths like `workspace_root.join("clippy.toml")`. Should receive `CrawlResult.clippy_tomls` instead.
- `config_files::check_per_crate_clippy()` — iterates member_dirs and joins `clippy.toml`. Could use `CrawlResult.clippy_tomls` filtered by workspace.
- `clippy_coverage::check()` — hardcodes `workspace_root.join("clippy.toml")`.
- `deny_audit::check()` — hardcodes `workspace_root.join("deny.toml")`.
- `cargo_lints::check()` — hardcodes `workspace_root.join("Cargo.toml")`.

**What the crawler already provides that RS validate discovers manually:**
| Validate discovers manually | Crawler field |
|---|---|
| `workspace_root.join("clippy.toml")` | `crawl.clippy_tomls` |
| `crate_dir.join("clippy.toml")` per member | `crawl.clippy_tomls` (all of them) |
| `workspace_root.join("deny.toml")` | `crawl.deny_tomls` |
| `workspace_root.join("rustfmt.toml")` | `crawl.rustfmt_tomls` |
| `workspace_root.join("rust-toolchain.toml")` | `crawl.rust_toolchains` |
| `workspace_root.join("Cargo.toml")` | `crawl.cargo_tomls` |
| `workspace_root.join("guardrail3.toml")` | `crawl.guardrail3_tomls` |

---

## 3. TS Validate Orchestrator (`app/ts/validate/mod.rs`)

**Flow:**
1. Runs config_files::check (eslint, tsconfig, npmrc, package.json, jscpd, velite)
2. Reads `eslint.config.mjs` at root for plugin checks
3. Runs source scan, architecture checks, test checks

**Config file discovery — all hardcoded to `path.join(...)`:**
- `eslint_check::check_eslint_config(fs, path)` — `path.join("eslint.config.mjs")`
- `tsconfig_check::check_tsconfig(fs, path)` — `path.join("tsconfig.json")`
- `npmrc_check::check_npmrc(fs, path)` — `path.join(".npmrc")`
- `package_check::check_package_json(fs, path)` — `path.join("package.json")`
- `jscpd_check::check_jscpd(fs, path)` — `path.join(".jscpd.json")`
- `jscpd_check::check_velite_config(path)` — `path.join("velite.config.ts")` / `.mjs`
- `path.join("eslint.config.mjs")` for plugin checks (lines 45, 93)

**TS app discovery is ALSO manual:** `ts_arch_checks::discover_ts_apps(fs, root)` uses `fs.list_dir(root.join("apps"))` and checks each subdir for `package.json` or `.ts/.tsx` files. This is independent of both the crawler and `discover::detect_project`.

**Functions that would need to change:**
- `eslint_check::check_eslint_config()` — should receive eslint config path from crawl
- `tsconfig_check::check_tsconfig()` — should receive tsconfig path from crawl
- `npmrc_check::check_npmrc()` — should receive npmrc path from crawl
- `package_check::check_package_json()` — should receive package.json path from crawl
- `jscpd_check::check_jscpd()` / `check_velite_config()` — should use crawl data
- `ts_arch_checks::discover_ts_apps()` — could potentially use crawl's `package_jsons` filtered to `apps/*/`

**What the crawler already provides that TS validate discovers manually:**
| Validate discovers manually | Crawler field |
|---|---|
| `path.join("eslint.config.mjs")` | `crawl.eslint_configs` |
| `path.join("tsconfig.json")` | `crawl.tsconfigs` |
| `path.join(".npmrc")` | `crawl.npmrcs` |
| `path.join("package.json")` | `crawl.package_jsons` |
| `path.join(".jscpd.json")` | `crawl.jscpd_configs` |
| velite.config.* existence | `crawl.velite_configs` |
| next.config.* existence | `crawl.next_configs` |
| .stylelintrc.* existence | `crawl.stylelint_configs` |
| `discover_ts_apps()` listing `apps/*/` | Derivable from `crawl.package_jsons` |

---

## 4. Project Discovery (`app/discover.rs`)

**What it does:**
- `detect_project(fs, path)` returns `ProjectInfo { has_rust, has_typescript, workspaces, package_json_path }`
- Rust detection: checks `path/Cargo.toml`, then `path/crates/Cargo.toml`, then `path/apps/backend/Cargo.toml`. Parses `[workspace].members` with glob expansion. Also discovers nested workspaces in `apps/*/` with their own `[workspace]`.
- TS detection: checks `path/package.json` (with TS signals: tsconfig.json or typescript in deps), then `path/apps/*/package.json`.

**Hardcoded path assumptions:**
- Workspace fallback search order: `.`, `crates/`, `apps/backend/`
- Nested workspace discovery only in `apps/*/`
- TS detection checks root then `apps/*/`

**Relationship to crawler:** `discover.rs` and `crawl.rs` are completely independent. The crawler doesn't produce `ProjectInfo`, and `discover.rs` doesn't use `CrawlResult`. They duplicate work:
- Both find `Cargo.toml` files
- Both find `package.json` files
- Both scan `apps/*/` directories

**What would need to change:** `detect_project` could be reimplemented on top of `CrawlResult.cargo_tomls` and `CrawlResult.package_jsons` instead of doing its own filesystem walks. The workspace member parsing still needs to read Cargo.toml content, but the file-exists checks could use crawl data.

---

## 5. Crawler (`app/crawl.rs`)

**What it produces:** `CrawlResult` with 25+ categorized file lists plus 3 source-directory sets.

**How it works:** Single `ignore::WalkBuilder` walk respecting .gitignore. Classifies every file by filename match. Sorts all vectors for determinism.

**Key data the crawler has that validate doesn't use:**
- `dirs_with_rs` / `dirs_with_ts` / `dirs_with_css` — source directory coverage (only used by coverage maps today)
- `pre_commit_hooks` — hooks validate currently does its own discovery
- `guardrail3_tomls` / `guardrail3_overrides` — validate loads config by hardcoded path
- `license_files`, `claude_mds`, `cargo_mutants_tomls` — used by workspace_metadata checks but discovered manually
- `release_plz_tomls`, `cliff_tomls` — release checks discover these manually
- `github_workflows` — not used by validate at all today

**What the crawler does NOT have that validate needs:**
- No `ProjectInfo` equivalent (workspace member lists, crate names, has_rust/has_typescript flags)
- No Cargo.toml content parsing (workspace members, lints, dependencies)
- No package.json content parsing (dependencies, scripts, overrides)
- No source file AST data (that's a different concern)

---

## 6. Coverage Engine (`commands/coverage/engine.rs`)

**How it uses the crawler:**
- The `CoverageTool` trait takes `&CrawlResult` in `config_files()` and `source_dirs()`.
- Each tool module (clippy, deny, eslint, tsconfig, etc.) implements `CoverageTool` and extracts the relevant file list from `CrawlResult`.
- `engine::build()` takes `(tool, root, crawl)` and does walk-up resolution to determine which config covers which source directory.

**Pattern to follow:** Coverage already demonstrates the correct pattern. The validate command should be wired similarly — receive `CrawlResult` at the top and pass relevant file lists down to each check module.

---

## Summary: The Gap

### Current state
- **Crawler** does a single efficient walk, classifies all files, used only by `coverage` and `map` commands.
- **Validate** does NOT use the crawler at all. Every check module independently constructs paths via `workspace_root.join("filename")`.
- **Discover** (`detect_project`) does its own filesystem walks, independent of both crawler and validate.
- **TS app discovery** (`discover_ts_apps`) is yet another independent filesystem walk.

### What wiring would look like

1. **Call `crawl(root)` once in `commands/validate.rs::run()`** and pass `&CrawlResult` down.

2. **Replace hardcoded path construction in each check module:**
   - RS config_files: use `crawl.clippy_tomls`, `crawl.rustfmt_tomls`, `crawl.rust_toolchains`
   - RS deny_audit: use `crawl.deny_tomls`
   - RS clippy_coverage: use `crawl.clippy_tomls`
   - RS cargo_lints: use `crawl.cargo_tomls`
   - TS eslint_check: use `crawl.eslint_configs`
   - TS tsconfig_check: use `crawl.tsconfigs`
   - TS npmrc_check: use `crawl.npmrcs`
   - TS package_check: use `crawl.package_jsons`
   - TS jscpd_check: use `crawl.jscpd_configs`
   - Hooks: use `crawl.pre_commit_hooks`

3. **Consider reimplementing `detect_project` on top of CrawlResult.** The crawler already finds all Cargo.toml and package.json files. ProjectInfo's workspace member parsing still needs to read file contents, but the initial existence checks are redundant.

4. **Consider replacing `discover_ts_apps` with crawl-derived data.** TS apps = directories in `apps/*/` that have entries in `crawl.package_jsons`.

### Semantic difference to handle

Validate checks ask "does file X exist at location Y?" (single expected location). The crawler answers "where are all files of type X?" (multiple possible locations). The wiring needs a resolution step: given all clippy.tomls found by crawl, which one applies to this workspace root? The coverage engine already solves this with walk-up resolution — validate can use the same pattern or just filter by path prefix.

### Impact on check semantics

Some checks are "file must exist at workspace root" (R1: clippy.toml). With crawler data, this becomes "is there a clippy.toml whose parent == workspace_root in crawl.clippy_tomls?" Same semantic, different implementation. The check modules don't need to change their logic — just how they get the file path.

Other checks are "file must exist somewhere" (R2: per-crate clippy.toml). These benefit more from the crawler — instead of iterating member_dirs and joining, filter crawl.clippy_tomls by membership.

### Files that would change

| File | What changes |
|---|---|
| `commands/validate.rs` | Add `crawl(abs_path)` call, pass `&CrawlResult` to rs/ts/hooks validate |
| `app/rs/validate/mod.rs` | Accept `&CrawlResult`, pass relevant fields to check modules |
| `app/rs/validate/config_files.rs` | Accept clippy/rustfmt/toolchain paths from crawl instead of constructing |
| `app/rs/validate/clippy_coverage.rs` | Accept clippy.toml path from crawl |
| `app/rs/validate/deny_audit.rs` | Accept deny.toml path from crawl |
| `app/rs/validate/cargo_lints.rs` | Accept Cargo.toml path from crawl |
| `app/ts/validate/mod.rs` | Accept `&CrawlResult`, pass relevant fields to check modules |
| `app/ts/validate/eslint_check.rs` | Accept eslint config path from crawl |
| `app/ts/validate/tsconfig_check.rs` | Accept tsconfig path from crawl |
| `app/ts/validate/npmrc_check.rs` | Accept npmrc path from crawl |
| `app/ts/validate/package_check.rs` | Accept package.json path from crawl |
| `app/ts/validate/jscpd_check.rs` | Accept jscpd/velite config paths from crawl |
| `app/ts/validate/ts_arch_checks.rs` | Could replace `discover_ts_apps` with crawl-derived data |
| `app/discover.rs` | Could be reimplemented on crawl data (optional, lower priority) |
| `app/hooks/validate.rs` | Accept `&CrawlResult` for pre-commit hook paths |
