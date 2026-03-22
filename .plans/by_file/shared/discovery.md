# Project Discovery — Crawler Model

## Approach

One walk of the filesystem using the `ignore` crate (ripgrep's directory walker). Respects .gitignore natively. Collects every file we care about. Then infers the project structure from what was found.

No hardcoded path assumptions. No `apps/*/` special cases. No fallback chains.

## Phase 1: Crawl

```rust
use ignore::WalkBuilder;

fn crawl(root: &Path) -> CrawlResult {
    let mut result = CrawlResult::default();

    let walker = WalkBuilder::new(root)
        .hidden(false)       // need .guardrail3/, .githooks/, .stylelintrc.*
        .git_ignore(true)    // respect .gitignore → skips node_modules/, target/, dist/
        .git_global(true)
        .git_exclude(true)
        .max_depth(None)     // no depth limit — hex-in-hex can be deep
        .build();

    for entry in walker.flatten() {
        if !entry.file_type().map_or(false, |ft| ft.is_file()) {
            continue;
        }
        let path = entry.path();
        let name = entry.file_name().to_str().unwrap_or("");

        match name {
            // Rust
            "Cargo.toml" => result.cargo_tomls.push(path.to_owned()),
            "Cargo.lock" => result.cargo_locks.push(path.to_owned()),
            "clippy.toml" | ".clippy.toml" => result.clippy_tomls.push(path.to_owned()),
            "deny.toml" | ".deny.toml" => result.deny_tomls.push(path.to_owned()),
            "rustfmt.toml" | ".rustfmt.toml" => result.rustfmt_tomls.push(path.to_owned()),
            "rust-toolchain.toml" => result.rust_toolchain = Some(path.to_owned()),

            // TypeScript / JS
            "package.json" => result.package_jsons.push(path.to_owned()),
            "pnpm-workspace.yaml" => result.pnpm_workspace = Some(path.to_owned()),
            "tsconfig.json" | "tsconfig.base.json" => result.tsconfigs.push(path.to_owned()),

            // ESLint (multiple valid names)
            s if s.starts_with("eslint.config.") => result.eslint_configs.push(path.to_owned()),

            // Stylelint (multiple valid names)
            s if s.starts_with(".stylelintrc") || s.starts_with("stylelint.config.") => {
                result.stylelint_configs.push(path.to_owned());
            }

            // Other configs
            "cspell.json" | ".cspell.json" => result.cspell_configs.push(path.to_owned()),
            s if s.starts_with("cspell.config.") => result.cspell_configs.push(path.to_owned()),
            ".jscpd.json" => result.jscpd_configs.push(path.to_owned()),
            ".npmrc" => result.npmrcs.push(path.to_owned()),
            "release-plz.toml" | ".release-plz.toml" => result.release_plz = Some(path.to_owned()),
            "cliff.toml" => result.cliff_toml = Some(path.to_owned()),

            // guardrail3 own config
            "guardrail3.toml" => result.guardrail3_toml = Some(path.to_owned()),

            _ => {}
        }
    }
    result
}
```

This finds EVERYTHING in one pass. .gitignore handles the filtering — no need to hardcode `node_modules`, `target`, `dist`. If the user gitignores `legacy/`, it's skipped. If they don't, it's found (and we can check if it should be ignored via guardrail3.toml config).

## Phase 2: Build Rust structure

From the collected `cargo_tomls`, build the workspace graph:

```
1. Parse each Cargo.toml — classify as:
   - Workspace: has [workspace] section
   - Package: has [package] section, no [workspace]
   - Both: has [workspace] AND [package] (workspace root that's also a crate)

2. For each Workspace, resolve members:
   - Read [workspace].members globs
   - Resolve against filesystem
   - Apply [workspace].exclude
   - Result: set of absolute member paths

3. Collect all member paths across all workspaces into a global set

4. For each Package Cargo.toml:
   - If its directory is in the global member set → it's a workspace member, skip
   - If not → it's a standalone crate (its own compilation scope)

5. Build RustWorkspace list:
   - Each [workspace] Cargo.toml → RustWorkspace with resolved members
   - Each standalone [package] → RustWorkspace with is_standalone=true, one member
```

### Resolving workspace membership

The tricky part: a Cargo.toml at `apps/validator-rust/crates/domain/Cargo.toml` is a `[package]`. Is it standalone? No — it's a member of the `apps/validator-rust/` workspace. We know because the workspace's `members = ["crates/*"]` resolves to include it.

We must resolve workspace members BEFORE classifying standalone crates. Process workspaces first, collect all member paths, then check each package against the member set.

### Handling hex-in-hex

Outer workspace: `apps/my-app/Cargo.toml` with `members = ["crates/*"]` and `exclude = ["crates/adapters/complex"]`.

Inner workspace: `apps/my-app/crates/adapters/complex/Cargo.toml` with `[workspace] members = ["crates/*"]`.

The crawler finds both Cargo.toml files. Phase 2:
- Outer workspace resolves members, excludes `crates/adapters/complex`
- Inner workspace resolves its own members
- Inner workspace's member crates are NOT in outer workspace's member set (excluded)
- Inner workspace treated as independent compilation scope

If the outer workspace does NOT exclude the inner one, the inner's Cargo.toml is in the outer's member list. But it has `[workspace]` — Cargo actually forbids this (a workspace member can't itself be a workspace). We should detect and warn about this invalid structure.

## Phase 3: Build TS structure

From the collected `package_jsons` and optional `pnpm_workspace`:

```
1. If pnpm-workspace.yaml exists:
   - Parse packages array (e.g., ["apps/*", "packages/*"])
   - These are the DECLARED workspace packages
   - Match each found package.json against these patterns
   - Classify by pattern: apps/* → app, packages/* → package, other → tool/misc

2. If no pnpm-workspace.yaml:
   - Check root package.json for "workspaces" field (npm/yarn workspaces)
   - Same classification logic

3. For each TS app/package, note which config files exist nearby:
   - Has tsconfig.json?
   - Has eslint.config.*?
   - Has own .stylelintrc.*?
   - Has own .jscpd.json?
```

### Classification without path assumptions

Instead of hardcoding `apps/* = app, packages/* = package`, use the pnpm workspace patterns. If patterns are `["apps/*", "packages/*", "tools/*"]`, then `tools/freebie-renderer` is discovered and classified.

For projects without pnpm-workspace.yaml, fall back to checking the root package.json `workspaces` field. If that doesn't exist either, scan for package.json files and classify by directory name convention (best-effort).

## Phase 4: Map config files to scopes

Now we know where every config file lives AND what the project structure is. Map each config to its scope:

```
For each clippy.toml found:
  - Find which RustWorkspace root it's in (or closest ancestor workspace)
  - If it's AT a workspace root → expected, this is the workspace's clippy config
  - If it's INSIDE a workspace member (not at root) → WARN: shadows workspace config
  - If it's not in any workspace → orphan, warn

For each deny.toml found:
  - Same logic as clippy.toml

For each eslint.config.* found:
  - If at project root → root ESLint config
  - If in a TS app directory → per-app ESLint config
  - If elsewhere → unexpected, note it

For each tsconfig*.json found:
  - tsconfig.base.json at root → the shared base
  - tsconfig.json in TS app → per-app config (validate extends base or has strict flags)
  - tsconfig.json in TS package → per-package config
  - tsconfig.json at root → root project references (optional)

For each package.json found:
  - At root → root workspace config
  - In TS app/package → per-app/package config
```

## Phase 5: Produce ProjectMap

The final output of discovery:

```rust
struct ProjectMap {
    root: PathBuf,

    // Rust
    rust_scopes: Vec<RustScope>,

    // TypeScript
    ts_scopes: Vec<TsScope>,

    // Root-level configs (shared across all scopes)
    root_configs: RootConfigs,

    // Raw crawl data (for orphan detection, shadow warnings)
    all_config_files: AllConfigFiles,
}

struct RustScope {
    root: PathBuf,                      // workspace or standalone crate root
    kind: RustScopeKind,                // Workspace | StandaloneCrate
    members: Vec<RustMember>,           // crates in this scope
    configs: RustScopeConfigs,          // which config files exist AT this scope root
}

enum RustScopeKind {
    Workspace { members_globs: Vec<String>, excludes: Vec<String> },
    StandaloneCrate,
}

struct RustMember {
    name: String,
    dir: PathBuf,                       // relative to project root
    has_own_clippy_toml: bool,          // shadow detection!
}

struct RustScopeConfigs {
    clippy_toml: Option<PathBuf>,
    deny_toml: Option<PathBuf>,
    rustfmt_toml: Option<PathBuf>,
    cargo_toml: PathBuf,               // always exists (it's what defined the scope)
}

struct TsScope {
    path: PathBuf,                      // relative to project root
    kind: TsScopeKind,                  // App | Package | Tool
    configs: TsScopeConfigs,
}

enum TsScopeKind {
    App,
    Package,
    Tool,
}

struct TsScopeConfigs {
    package_json: PathBuf,
    tsconfig: Option<PathBuf>,
    eslint_config: Option<PathBuf>,     // per-app ESLint if exists
    stylelint_config: Option<PathBuf>,  // per-scope if exists
    jscpd_config: Option<PathBuf>,      // per-scope if exists
}

struct RootConfigs {
    guardrail3_toml: Option<PathBuf>,
    package_json: Option<PathBuf>,
    pnpm_workspace: Option<PathBuf>,
    eslint_config: Option<PathBuf>,     // root eslint.config.mjs
    stylelint_config: Option<PathBuf>,
    tsconfig_base: Option<PathBuf>,
    npmrc: Option<PathBuf>,
    jscpd_config: Option<PathBuf>,
    cspell_config: Option<PathBuf>,
    rust_toolchain: Option<PathBuf>,
    release_plz: Option<PathBuf>,
    cliff_toml: Option<PathBuf>,
    githooks_dir: Option<PathBuf>,
}

struct AllConfigFiles {
    clippy_tomls: Vec<PathBuf>,         // ALL found, for shadow detection
    deny_tomls: Vec<PathBuf>,
    eslint_configs: Vec<PathBuf>,
    tsconfigs: Vec<PathBuf>,
    // etc.
}
```

## How this changes everything

### Discovery → generate

Currently: `generate_rust_files` calls `detect_project` and `resolve_app_paths` to map config names to paths.

With crawler: `generate` receives a `ProjectMap`. For each `RustScope`, it knows exactly where clippy.toml should go (the scope root). It knows which config files already exist there. No name-to-path resolution needed — the path IS the scope identity.

### Discovery → validate

Currently: `rs validate` uses `primary_workspace_root()` and checks configs there. Misses other workspaces.

With crawler: `validate` iterates `project_map.rust_scopes`. Each scope gets validated independently. No "primary workspace" concept.

### Discovery → reports

Currently: flat list of check results.

With crawler: results can be grouped by scope:

```
=== apps/validator-rust (Rust workspace, 5 crates) ===
  clippy.toml: 2 missing baseline bans
  deny.toml: OK
  [workspace.lints]: OK
  Architecture: OK

=== apps/substack-publisher (Rust standalone crate) ===
  clippy.toml: NOT FOUND — would create
  deny.toml: 1 missing baseline ban

=== Root workspace (packages/low-expectations, packages/seo-site-files) ===
  clippy.toml: NOT FOUND — would create (library profile)
  deny.toml: NOT FOUND — would create

=== apps/landing (TS content app) ===
  eslint.config.mjs: per-app config exists, missing engine import
  tsconfig.json: extends base, OK

=== apps/admin (TS service app) ===
  eslint.config.mjs: no per-app config (uses root)
  tsconfig.json: standalone, all strict flags present
```

The user sees their project as THEY organized it, not as a flat list of check IDs.

### Discovery → init

Currently: `rs init` discovers workspaces and generates guardrail3.toml config.

With crawler: `init` gets the `ProjectMap` and generates config entries for each scope. It knows everything that exists — no separate discovery step needed.

### Discovery → shadow detection

The crawler finds ALL clippy.toml files. Phase 4 maps them to scopes. Any clippy.toml that's inside a workspace member (not at the scope root) is a shadow — immediately flagged. No need for a separate scan.

## What about guardrail3.toml?

guardrail3.toml is the USER's configuration — which checks to enable, app types, overrides. Discovery is what EXISTS on disk. They interact:

```
ProjectMap (what exists) + guardrail3.toml (what user configured) → ActionPlan (what to do)
```

For `generate`: the ProjectMap tells us WHERE files go. guardrail3.toml tells us WHAT profile/checks to use.

For `validate`: the ProjectMap tells us WHAT to check. guardrail3.toml tells us which checks are enabled.

For `init`: the ProjectMap is the INPUT. guardrail3.toml is the OUTPUT.

## Steady-parent expected ProjectMap

```
root: /Users/tartakovsky/Projects/steady-parent

rust_scopes:
  [0] root: "."
      kind: Workspace { members: ["packages/*"], excludes: ["apps/validator-rust", "apps/substack-publisher"] }
      members: [
        { name: "low-expectations", dir: "packages/low-expectations", has_own_clippy_toml: false },
        { name: "seo-site-files", dir: "packages/seo-site-files", has_own_clippy_toml: false },
      ]
      configs: { clippy_toml: None, deny_toml: None, rustfmt_toml: None }

  [1] root: "apps/validator-rust"
      kind: Workspace { members: ["crates/*"], excludes: [] }
      members: [
        { name: "domain", dir: "apps/validator-rust/crates/domain", has_own_clippy_toml: false },
        { name: "ports", dir: "apps/validator-rust/crates/ports/outbound", has_own_clippy_toml: false },
        { name: "app", dir: "apps/validator-rust/crates/app", has_own_clippy_toml: false },
        { name: "adapters", dir: "apps/validator-rust/crates/adapters/outbound", has_own_clippy_toml: false },
        { name: "api", dir: "apps/validator-rust/crates/adapters/inbound/api", has_own_clippy_toml: false },
      ]
      configs: { clippy_toml: Some("apps/validator-rust/clippy.toml"), deny_toml: Some("apps/validator-rust/deny.toml"), rustfmt_toml: Some("apps/validator-rust/rustfmt.toml") }

  [2] root: "apps/substack-publisher"
      kind: StandaloneCrate
      members: [
        { name: "substack-publisher", dir: "apps/substack-publisher", has_own_clippy_toml: false },
      ]
      configs: { clippy_toml: Some("apps/substack-publisher/clippy.toml"), deny_toml: Some("apps/substack-publisher/deny.toml"), rustfmt_toml: Some("apps/substack-publisher/rustfmt.toml") }

ts_scopes:
  [0] path: "apps/landing", kind: App
      configs: { package_json, tsconfig: Some, eslint_config: Some("apps/landing/eslint.config.mjs"), ... }
  [1] path: "apps/admin", kind: App
      configs: { package_json, tsconfig: Some, eslint_config: None, ... }
  [2..10] path: "packages/*", kind: Package
      configs: { package_json, tsconfig: Some (most), ... }

root_configs:
  guardrail3_toml: Some("guardrail3.toml")
  package_json: Some("package.json")
  pnpm_workspace: Some("pnpm-workspace.yaml")
  eslint_config: Some("eslint.config.mjs")
  stylelint_config: Some(".stylelintrc.mjs")
  tsconfig_base: Some("tsconfig.base.json")
  npmrc: Some(".npmrc")
  jscpd_config: Some(".jscpd.json")
  cspell_config: None
  rust_toolchain: None
  release_plz: None
  cliff_toml: None

all_config_files:
  clippy_tomls: ["apps/validator-rust/clippy.toml", "apps/substack-publisher/clippy.toml"]
  deny_tomls: ["apps/validator-rust/deny.toml", "apps/substack-publisher/deny.toml"]
  eslint_configs: ["eslint.config.mjs", "apps/landing/eslint.config.mjs"]
  tsconfigs: ["tsconfig.base.json", "apps/landing/tsconfig.json", "apps/admin/tsconfig.json", ... 19 total]
  jscpd_configs: [".jscpd.json", "apps/validator-rust/.jscpd.json"]
```

## Dependencies

- `ignore` crate (replaces or supplements `walkdir`)
- `toml` crate (already have — for parsing Cargo.toml)
- `serde_yaml` or similar (for parsing pnpm-workspace.yaml — could also just do simple line parsing since the format is simple)
