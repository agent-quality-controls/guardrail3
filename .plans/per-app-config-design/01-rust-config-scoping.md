# Per-App Config Architecture: Rust Tool File Scoping

**Date:** 2026-03-18
**Status:** Design document (no code changes)

> Historical design note. The Clippy lookup discussion in this document is stale.
> For real upstream Clippy config resolution, use [`.plans/by_file/tools/edge-cases/clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_file/tools/edge-cases/clippy.md).
> For the live guardrail family contract, use [`.plans/todo/checks/rs/clippy.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/clippy.md).

## Problem Statement

guardrail3 generates config files (clippy.toml, deny.toml, rustfmt.toml, etc.) for Rust projects. Today it works well for single-workspace projects. But real monorepos have **multiple independent Rust applications** that need **different configs** -- different clippy bans, different deny.toml exceptions, different profiles. The current system applies one global set of overrides (`.guardrail3/overrides/`) to all apps uniformly, which is wrong.

This document specifies exactly how each config file type should be scoped, generated, overridden, validated, and reported in a multi-app monorepo.

---

## Reference: The steady-parent Monorepo

```
steady-parent/
├── Cargo.toml                        # Virtual workspace: members=["packages/*"], exclude=["apps/*"]
├── packages/
│   ├── low-expectations/Cargo.toml   # Rust library (workspace member)
│   └── seo-site-files/Cargo.toml     # Rust library (workspace member)
├── apps/
│   ├── validator-rust/               # Nested workspace (excluded from root workspace)
│   │   ├── Cargo.toml               # [workspace] members=["crates/*"]
│   │   ├── clippy.toml
│   │   ├── deny.toml
│   │   ├── rustfmt.toml
│   │   └── crates/
│   │       ├── domain/Cargo.toml
│   │       ├── ports/outbound/Cargo.toml
│   │       ├── app/Cargo.toml
│   │       ├── adapters/outbound/Cargo.toml
│   │       └── adapters/inbound/api/Cargo.toml
│   └── substack-publisher/           # Standalone crate (NOT a workspace)
│       ├── Cargo.toml               # [package] only, no [workspace]
│       ├── clippy.toml
│       ├── deny.toml
│       └── rustfmt.toml
├── rust-toolchain.toml               # Repo-wide
├── guardrail3.toml
└── .guardrail3/
    └── overrides/
        ├── clippy-methods.toml       # Global overrides
        ├── clippy-types.toml
        ├── deny-bans.toml
        ├── deny-skip.toml
        └── deny-feature-bans.toml
```

This structure contains **three distinct Rust scopes**:

1. **Root workspace** (`steady-parent/Cargo.toml`) -- contains `packages/*` as members, excludes `apps/*`
2. **Nested workspace** (`apps/validator-rust/`) -- its own `[workspace]` with `crates/*` as members
3. **Standalone crate** (`apps/substack-publisher/`) -- single `[package]`, no workspace

Each scope is an independent compilation unit with its own tooling context.

---

## File-by-File Analysis

### 1. clippy.toml

#### Tool Resolution Semantics

clippy reads `clippy.toml` (or `.clippy.toml`) from `CARGO_MANIFEST_DIR` -- the directory containing the `Cargo.toml` of the crate being compiled. **There is no walk-up.** If no `clippy.toml` exists in `CARGO_MANIFEST_DIR`, clippy uses built-in defaults.

**Critical subtlety:** For workspace builds (`cargo clippy --workspace`), clippy compiles each crate independently. Each crate's `CARGO_MANIFEST_DIR` is its own `Cargo.toml` directory, NOT the workspace root. So `clippy.toml` at the workspace root is **only read when compiling crates whose `Cargo.toml` is at the workspace root** (virtual workspaces have no such crate).

**Per-crate clippy.toml is technically possible** (each crate can have its own), but clippy issue #7353 documents that this is unreliable and not officially supported for `disallowed_methods`/`disallowed_types` in workspace builds. In practice, `cargo clippy --workspace` from the workspace root picks up the workspace root's `clippy.toml` for all member crates. This is the behavior guardrail3 relies on.

#### Scoping Rules

| Scope | Where clippy.toml goes | Why |
|---|---|---|
| Root workspace (packages/*) | `steady-parent/clippy.toml` | Workspace root config applies to all members (`packages/low-expectations`, `packages/seo-site-files`). `cargo clippy --workspace` from root uses this. |
| Nested workspace (apps/validator-rust/) | `apps/validator-rust/clippy.toml` | This is its own workspace. `cargo clippy --workspace` from `apps/validator-rust/` uses this. |
| Standalone crate (apps/substack-publisher/) | `apps/substack-publisher/clippy.toml` | Single crate, `CARGO_MANIFEST_DIR` is here. |

**Can there be multiple instances in a monorepo?** Yes. Each independent workspace/crate root gets one.

**What happens if clippy.toml exists at the wrong level?**
- `clippy.toml` at `packages/low-expectations/` -- clippy *may* read it when compiling that crate specifically, but behavior is undefined in workspace builds (per #7353). guardrail3 should WARN if it detects crate-level clippy.toml inside a workspace, because the user likely expects it to work but it won't reliably.
- `clippy.toml` at `apps/validator-rust/crates/domain/` -- same problem. clippy in workspace mode won't reliably use it.

#### Content Differences Per-App

**What MUST differ:**
- `disallowed-methods`: validator-rust bans `reqwest::Client::new` and `reqwest::Client::builder` (shared client pattern). substack-publisher might not have shared clients and may need direct reqwest.
- `disallowed-methods`: A CLI tool app would not ban `std::process::Command::new` or `std::process::exit`.
- `disallowed-types`: A pure library would ban `axum::*` types. A service would not.

**What SHOULD be the same:**
- `HashMap`/`HashSet` -> `BTreeMap`/`BTreeSet` ban (determinism is universal)
- `std::sync::Mutex` -> `parking_lot::Mutex` ban (always better)
- Thresholds (`too-many-lines-threshold`, `cognitive-complexity-threshold`) -- these reflect coding standards, not app-specific needs
- `std::env::set_var`/`remove_var` ban (always unsafe in multi-threaded)

**The split:**
- **Global baseline:** thresholds + universal bans (collections, sync, env mutation, serde validation)
- **Profile-driven:** service-specific bans (reqwest shared client, axum extractors, process::Command) vs library bans (all I/O banned)
- **Per-app customization:** additional bans or ban removals specific to one app

#### Generation

guardrail3 should generate `clippy.toml` at:
1. The root workspace root (`steady-parent/clippy.toml`) -- for `packages/*`
2. Each nested workspace root (`apps/validator-rust/clippy.toml`) -- for that workspace's crates
3. Each standalone crate root (`apps/substack-publisher/clippy.toml`)

**It should NOT generate at:**
- Individual crate directories inside a workspace (e.g., `apps/validator-rust/crates/domain/clippy.toml`) -- clippy won't reliably use it

#### What Currently Exists vs What's Needed

**Current behavior:** `generate_rust_files()` iterates `[rust.apps.*]` and generates `clippy.toml` at each app's resolved path. If no apps are configured, generates at `workspace_root`. The `[rust.packages]` config generates at root. BUT: all apps use the same `local.clippy_methods` and `local.clippy_types` from the global `.guardrail3/overrides/`. There is no per-app override mechanism.

**What's needed:** Per-app overrides. An app-specific override that adds or removes bans from the global baseline + global override.

---

### 2. deny.toml

#### Tool Resolution Semantics

`cargo-deny` reads `deny.toml` from the **current working directory**. It does NOT walk up. If you run `cargo deny check` from `apps/validator-rust/`, it reads `apps/validator-rust/deny.toml`. If you run from the repo root, it reads `steady-parent/deny.toml`.

`cargo-deny` has a `--config` flag to specify an explicit path, but the convention is cwd-based.

**Per-crate exceptions within deny.toml:** `cargo-deny` supports crate-specific skip entries and per-crate license exceptions within a single `deny.toml`. This means one `deny.toml` per workspace is sufficient even when different crates have different needs -- you just add `[[bans.deny]]` entries with `wrappers` or `skip` entries with `crate` names.

#### Scoping Rules

| Scope | Where deny.toml goes | Why |
|---|---|---|
| Root workspace | `steady-parent/deny.toml` | `cargo deny check` from root checks all root workspace members |
| Nested workspace | `apps/validator-rust/deny.toml` | `cargo deny check` from here checks this workspace only |
| Standalone crate | `apps/substack-publisher/deny.toml` | `cargo deny check` from here checks this crate |

**Can there be multiple?** Yes, one per scope.

#### Content Differences Per-App

**What MUST differ:**
- `[bans] skip` entries: Different apps have different transitive dependency conflicts. validator-rust might need `skip = [{crate = "windows-sys"}]` while substack-publisher doesn't.
- `[bans] deny` entries: A library package should ban `axum`, `tokio`, `reqwest`, `sqlx` (no I/O). A service should not.
- `[[bans.features]]` entries: Feature bans on tokio differ between service (ban `full`, allow specific features) and library (ban tokio entirely).
- `[licenses] allow`: Might differ if one app uses a crate with an unusual license.

**What SHOULD be the same:**
- `[bans] multiple-versions = "deny"` (always)
- `[licenses]` allow list (mostly the same baseline)
- `[advisories]` settings (universal security policy)
- `[sources]` restrictions (universal supply chain policy)
- The base set of crate bans (json, tls, http, logging, async, error, web, datetime, orm, serialization)

**The split:**
- **Global baseline:** graph settings, base deny list, licenses, advisories, sources
- **Profile-driven:** library profile adds I/O crate bans, removes tokio feature bans (bans tokio entirely)
- **Per-app customization:** extra skip entries, extra deny entries, license exceptions, feature ban adjustments

#### Generation

Same as clippy.toml -- one per scope:
1. Root workspace root
2. Each nested workspace root
3. Each standalone crate root

---

### 3. rustfmt.toml

#### Tool Resolution Semantics

`rustfmt` **walks up** from the file being formatted. It looks for `rustfmt.toml` or `.rustfmt.toml` starting from the file's directory, then parent, then grandparent, etc. The **nearest** config wins. There is **no merging** -- the nearest config completely shadows anything further up.

This walk-up behavior is critical for monorepos.

#### Scoping Rules

| Scope | Where rustfmt.toml goes | Why |
|---|---|---|
| Root workspace | `steady-parent/rustfmt.toml` | Applies to `packages/*` and any root-level files |
| Nested workspace | `apps/validator-rust/rustfmt.toml` | Shadows root for all files under this directory |
| Standalone crate | `apps/substack-publisher/rustfmt.toml` | Shadows root for all files under this directory |

**Does each crate inside a workspace need its own?** No. Walk-up means `apps/validator-rust/crates/domain/src/lib.rs` walks up to `apps/validator-rust/rustfmt.toml`. One per workspace/app root is sufficient.

**What happens with walk-up from packages/?** `packages/low-expectations/src/lib.rs` walks up to `packages/low-expectations/` (no rustfmt.toml), then to `steady-parent/` (finds rustfmt.toml). So the root-level rustfmt.toml covers all packages.

#### Content Differences Per-App

**Almost never.** Formatting should be uniform across the entire repo. The only conceivable difference is `edition` if different apps target different Rust editions, but this is strongly discouraged.

**The split:**
- **Global baseline:** all settings (edition, max_width, tab_spaces, etc.)
- **Per-app customization:** essentially nothing. If someone needs different formatting per app, something has gone wrong.

**Design decision: rustfmt.toml is always identical across all scopes.** guardrail3 generates the same content everywhere. Per-app overrides are not supported for rustfmt. If a user genuinely needs different formatting in one app, they can manually create a `rustfmt.toml` there and guardrail3 will warn on `check`/`diff` that the file doesn't match the expected content.

#### Generation

Generate at:
1. Root (for packages and any root-level source)
2. Each app root (to ensure walk-up from app files doesn't accidentally inherit some other config)

Even though the content is identical, we generate at each app root because:
- It prevents accidental inheritance from a `rustfmt.toml` the user might place at some intermediate directory
- It makes the monorepo self-contained -- each app has everything it needs to run `cargo fmt`

---

### 4. rust-toolchain.toml

#### Tool Resolution Semantics

`rustup` walks up from the current directory to find `rust-toolchain.toml` (or `rust-toolchain`). The nearest one wins. No merging.

#### Scoping Rules

**One at the repo root. Period.**

Rationale:
- All apps in a monorepo should use the same Rust toolchain. Different toolchains per app is a maintenance nightmare.
- CI/CD typically runs from the repo root and expects one toolchain.
- If an app needs a different toolchain (e.g., nightly for some feature), the correct approach is `cargo +nightly <command>` in that app's CI step, not a separate `rust-toolchain.toml`.

**What if rust-toolchain.toml exists inside an app?** guardrail3 should WARN. It means someone manually placed a toolchain file that will shadow the root one, potentially causing CI/local divergence.

#### Content Differences Per-App

None. The toolchain is repo-wide.

#### Generation

Generate at repo root only. Do not generate at app roots.

---

### 5. release-plz.toml

#### Tool Resolution Semantics

`release-plz` reads its config from the repo root (or wherever you run it from). One config per repo. It has per-package `[[package]]` sections within the single file.

#### Scoping Rules

**One at the repo root.**

Per-package customization happens inside the file via `[[package]]` sections:
```toml
[workspace]
changelog_config = "cliff.toml"

[[package]]
name = "low-expectations"
publish = true

[[package]]
name = "validator-rust"
publish = false  # internal app, not published
```

#### Content Differences Per-App

All differences are expressed as `[[package]]` sections within the single file, not as separate files.

**What differs per package:**
- `publish = true/false`
- `changelog_path` (if apps have separate changelogs)
- `release_always`

#### Generation

Generate one `release-plz.toml` at repo root. The generator should enumerate all publishable packages and create `[[package]]` sections for each.

**Future enhancement:** `guardrail3.toml` could have `[rust.apps.X.release]` config that controls what goes into the `[[package]]` section for app X.

---

### 6. cliff.toml

#### Tool Resolution Semantics

`git-cliff` reads `cliff.toml` from cwd or via `--config` flag. One per repo is standard.

#### Scoping Rules

**One at the repo root.**

If different packages need different changelog formats (unlikely), `release-plz.toml` can specify `changelog_config` per package pointing to different cliff configs. But this is rare.

#### Content Differences Per-App

None in practice. The changelog format should be uniform.

#### Generation

Generate one `cliff.toml` at repo root.

---

### 7. Cargo.toml [workspace.lints]

#### Tool Resolution Semantics

`[workspace.lints]` in a workspace's `Cargo.toml` defines lint levels. Crate members inherit them via `[lints] workspace = true` in their own `Cargo.toml`.

This is NOT a file guardrail3 generates -- it validates that the right lints are configured. But the scoping matters.

#### Scoping Rules

Each workspace has its own `[workspace.lints]`:
- `steady-parent/Cargo.toml` defines lints for `packages/*`
- `apps/validator-rust/Cargo.toml` defines lints for `crates/*`
- `apps/substack-publisher/Cargo.toml` has `[lints]` directly (no workspace)

#### Content Differences Per-App

**Should be the same.** Lint levels represent coding standards, not app-specific needs. The only difference might be `unused_crate_dependencies` being relaxed in test-heavy crates.

#### Validation

guardrail3 should validate `[workspace.lints]` (or `[lints]`) at each scope:
- Root workspace Cargo.toml
- Each nested workspace Cargo.toml
- Each standalone crate Cargo.toml

---

## Override Architecture

### Current State

```
.guardrail3/overrides/
├── clippy-methods.toml      # Applied to ALL clippy.toml generations
├── clippy-types.toml        # Applied to ALL
├── deny-bans.toml           # Applied to ALL
├── deny-skip.toml           # Applied to ALL
├── deny-feature-bans.toml   # Applied to ALL
```

This is flat. Every app gets the same overrides.

### Proposed Architecture

```
.guardrail3/
├── overrides/                          # Global overrides (apply to all apps)
│   ├── clippy-methods.toml
│   ├── clippy-types.toml
│   ├── deny-bans.toml
│   ├── deny-skip.toml
│   └── deny-feature-bans.toml
└── apps/                               # Per-app overrides
    ├── validator-rust/
    │   └── overrides/
    │       ├── clippy-methods.toml     # Additional bans for validator-rust
    │       ├── clippy-types.toml
    │       ├── deny-bans.toml
    │       ├── deny-skip.toml
    │       └── deny-feature-bans.toml
    └── substack-publisher/
        └── overrides/
            ├── clippy-methods.toml     # Different bans for substack-publisher
            └── deny-skip.toml
```

### Merge Precedence

```
Final config = Global baseline (from profile)
             + Global overrides (.guardrail3/overrides/)
             + Per-app overrides (.guardrail3/apps/{name}/overrides/)
```

All three layers are **additive** for bans/entries. A per-app override can ADD entries but currently cannot REMOVE entries from the global baseline or global overrides.

**Should per-app overrides support removals?** This is the hard question.

#### Case for removals

validator-rust needs `reqwest::Client::new` banned. substack-publisher does NOT -- it's a simple CLI that makes a few HTTP calls. The global baseline (service profile) includes the reqwest ban. substack-publisher needs to remove it.

Without removals, the user must either:
1. Remove the reqwest ban from the global baseline (weakens all apps)
2. Not use the service profile for substack-publisher (loses other service bans they want)
3. Use a per-app profile override (library vs service) -- but this is too coarse

#### Proposed: Per-app profile + per-app overrides + per-app exemptions

Three mechanisms:

1. **Per-app profile** (already exists via `[rust.apps.X.type]`): Controls the baseline. `type = "service"` vs `type = "library"` vs `type = "cli"` (new profile for CLI tools).

2. **Per-app additive overrides** (proposed above): Add bans/entries on top of the profile baseline + global overrides.

3. **Per-app exemptions** (new): A file that lists entries to REMOVE from the generated config for this app only.

```
.guardrail3/apps/substack-publisher/
├── overrides/
│   └── deny-skip.toml          # Additional skip entries
└── exemptions/
    └── clippy-methods.toml     # Methods to UN-ban for this app
```

Exemption file format (same TOML entry format, but meaning is "remove this"):
```toml
# Exempt reqwest::Client::new ban -- substack-publisher is a simple CLI, no shared client pattern
{ path = "reqwest::Client::new", reason = "Simple CLI tool -- no shared client needed" }
{ path = "reqwest::Client::builder", reason = "Simple CLI tool -- no shared client needed" }
```

**Merge with exemptions:**
```
Final config = (Global baseline + Global overrides + Per-app overrides) - Per-app exemptions
```

Each exemption MUST have a `reason` field. guardrail3 should report all active exemptions in `--verbose` output.

### Alternative Considered: Complete Override (No Merge)

Instead of additive merge, each app could have a COMPLETE override that replaces the entire generated config.

**Rejected because:**
- Defeats the purpose of guardrail3 -- users would have to maintain full configs manually
- Changes to the global baseline wouldn't propagate to apps with complete overrides
- Too easy to accidentally relax security-critical settings

### Alternative Considered: Override Files at the App Root

Place per-app overrides at `apps/validator-rust/.guardrail3/overrides/` instead of `.guardrail3/apps/validator-rust/overrides/`.

**Rejected because:**
- Scatters guardrail3 config across the repo
- Apps excluded from the root workspace might not have `.guardrail3/` in their .gitignore
- Harder to audit all overrides at once
- The `.guardrail3/` directory at repo root is the single source of truth for all guardrail3 configuration

---

## guardrail3.toml Config Schema

### Current

```toml
[rust.apps.guardrail3]
type = "service"
```

### Proposed

```toml
[rust]
workspace_root = "."          # Root workspace

[rust.packages]
type = "library"              # Profile for packages/* members

[rust.apps.validator-rust]
type = "service"              # Nested workspace: service profile
layer = "composition-root"    # Hex arch layer (for global-state ban logic)

[rust.apps.substack-publisher]
type = "service"              # Standalone crate: service profile
# Could also be type = "cli" in the future
```

The app name (`validator-rust`, `substack-publisher`) is matched to a discovered directory under `apps/`. This matching already works via `resolve_app_paths()`.

---

## Generation: Detailed Behavior

### `guardrail3 rs generate`

For the steady-parent example:

1. **Parse `guardrail3.toml`** -- get profile, apps, packages config
2. **Discover project structure** -- find root workspace, nested workspaces, standalone crates
3. **Load overrides:**
   - Global: `.guardrail3/overrides/clippy-methods.toml`, etc.
   - Per-app: `.guardrail3/apps/{name}/overrides/clippy-methods.toml`, etc.
   - Per-app exemptions: `.guardrail3/apps/{name}/exemptions/clippy-methods.toml`, etc.
4. **Generate for root workspace (packages):**
   - `steady-parent/clippy.toml` -- library profile + global overrides (no per-app overrides; packages use `[rust.packages]` config)
   - `steady-parent/deny.toml` -- library profile + global overrides
   - `steady-parent/rustfmt.toml` -- identical everywhere
5. **Generate for each app:**
   - `apps/validator-rust/clippy.toml` -- service profile + global overrides + validator-rust overrides - validator-rust exemptions
   - `apps/validator-rust/deny.toml` -- same layering
   - `apps/validator-rust/rustfmt.toml` -- identical
   - `apps/substack-publisher/clippy.toml` -- service profile + global overrides + substack-publisher overrides - substack-publisher exemptions
   - `apps/substack-publisher/deny.toml` -- same layering
   - `apps/substack-publisher/rustfmt.toml` -- identical
6. **Generate repo-wide files:**
   - `rust-toolchain.toml`
   - `release-plz.toml` (with `[[package]]` sections for each publishable package)
   - `cliff.toml`
   - `.githooks/pre-commit`

**Output:**
```
  wrote: clippy.toml (packages, profile: library)
  wrote: deny.toml (packages, profile: library)
  wrote: rustfmt.toml (packages)
  wrote: apps/validator-rust/clippy.toml (profile: service)
  wrote: apps/validator-rust/deny.toml (profile: service)
  wrote: apps/validator-rust/rustfmt.toml
  wrote: apps/substack-publisher/clippy.toml (profile: service, 2 exemptions)
  wrote: apps/substack-publisher/deny.toml (profile: service)
  wrote: apps/substack-publisher/rustfmt.toml
  wrote: rust-toolchain.toml
  wrote: release-plz.toml
  wrote: cliff.toml
  wrote: .githooks/pre-commit

Generated 13 files.
```

### `guardrail3 rs generate --app validator-rust`

Generate only for one app. Skips root workspace, other apps, and repo-wide files. Useful for quick iteration on per-app overrides.

---

## Diff / Dry-Run Behavior

### `guardrail3 diff`

Should group output by scope:

```
=== Packages (root workspace) ===
  clippy.toml: OK (matches generated)
  deny.toml: DIFFERS
    - missing ban: { name = "chrono" }
  rustfmt.toml: OK

=== validator-rust ===
  clippy.toml: OK
  deny.toml: OK
  rustfmt.toml: OK

=== substack-publisher ===
  clippy.toml: DIFFERS
    + extra method ban: { path = "custom::method" } (not in generated config)
    - missing method ban: { path = "std::fs::read_to_string" } (in generated but not in file)
  deny.toml: OK
  rustfmt.toml: OK

=== Repo-wide ===
  rust-toolchain.toml: OK
  release-plz.toml: DIFFERS (manual edits detected)
  cliff.toml: OK
```

**Showing override provenance:** When `--verbose` is used, diff should show WHERE each entry comes from:

```
=== substack-publisher: clippy.toml ===
  disallowed-methods:
    std::env::var               [baseline:service]
    std::env::var_os            [baseline:service]
    reqwest::Client::new        [baseline:service] EXEMPTED by .guardrail3/apps/substack-publisher/exemptions/clippy-methods.toml
    custom::special_method      [override:.guardrail3/apps/substack-publisher/overrides/clippy-methods.toml]
```

---

## Validate Behavior

### Current State

Validation runs per-workspace:
- Finds the workspace root
- Checks clippy.toml, deny.toml, rustfmt.toml, etc. at that root
- Checks Cargo.toml [workspace.lints]
- Runs source scan on all workspace members

### Required Changes

Validation must become per-scope:

1. **Discover all scopes** (root workspace, nested workspaces, standalone crates)
2. **For each scope**, run the full validation suite:
   - clippy.toml existence and completeness (R1-R7)
   - deny.toml structure and completeness (R8-R20)
   - rustfmt.toml existence and settings (R21-R23)
   - Cargo.toml [workspace.lints] or [lints] (R26-R29)
   - Source scan on that scope's files only
3. **For repo-wide files** (checked once):
   - rust-toolchain.toml (R24-R25)
   - release-plz.toml, cliff.toml

**Validation must account for per-app profiles and overrides.** When checking clippy.toml completeness, the expected bans depend on that app's profile + overrides + exemptions. The validator needs to compute the expected config for each app (using the same generation logic) and compare against the actual file.

**Output grouping:**
```
=== Packages (root workspace, profile: library) ===
  [PASS] R1: clippy.toml exists
  [PASS] R4: All expected method bans present (23/23)
  ...

=== validator-rust (profile: service) ===
  [PASS] R1: clippy.toml exists
  [PASS] R4: All expected method bans present (31/31)
  ...

=== substack-publisher (profile: service, 2 exemptions) ===
  [PASS] R1: clippy.toml exists
  [PASS] R4: All expected method bans present (29/31, 2 exempted)
  ...

=== Repo-wide ===
  [PASS] R24: rust-toolchain.toml exists
  [PASS] R25: Stable channel configured
  ...
```

---

## Edge Cases

### 1. App has no Cargo.toml (misconfigured)

**Scenario:** `guardrail3.toml` lists `[rust.apps.ghost-app]` but `apps/ghost-app/Cargo.toml` doesn't exist.

**Behavior:** `resolve_app_paths()` fails to find a matching discovered directory. guardrail3 should:
- WARN: "App 'ghost-app' declared in guardrail3.toml but no Cargo.toml found at apps/ghost-app/"
- Skip generation for that app
- During validate: report as an ERROR (app declared but not found)

### 2. App's workspace excludes some crates

**Scenario:** `apps/validator-rust/Cargo.toml` has `exclude = ["crates/experimental"]`.

**Behavior:** The excluded crate is not a workspace member. It doesn't get compiled by `cargo clippy --workspace` from that directory. guardrail3's source scan should also exclude it. The `discover_nested_workspaces()` function already handles `exclude` via `parse_workspace_excludes()`.

**However:** If the excluded crate has its own `clippy.toml`, that's a signal someone is running clippy on it independently. guardrail3 should NOT generate configs for it (it's excluded), but should WARN if it detects a `clippy.toml` in an excluded directory.

### 3. App depends on a package from the root workspace

**Scenario:** `apps/validator-rust/crates/domain` has `low-expectations = { path = "../../../packages/low-expectations" }`.

**Behavior:** This is normal Cargo path dependency. The dependency is compiled in the context of the consumer's workspace (`apps/validator-rust`). So the consumer's `clippy.toml` and `deny.toml` apply when building the dependency as part of that workspace.

**Implication for deny.toml:** The dependency's transitive deps are checked by the consumer's `deny.toml`. If `low-expectations` pulls in a crate that validator-rust's `deny.toml` bans, that's a deny error. This is correct behavior -- the consumer workspace's policy applies.

**Implication for clippy.toml:** The dependency's code is checked against the consumer's `clippy.toml`. If `low-expectations` uses `HashMap` and validator-rust bans it, clippy will flag it. This might be surprising but is correct -- the workspace owner decides the rules.

**guardrail3's role:** When validating `apps/validator-rust`, source scan should NOT scan `packages/low-expectations` source code (it's in a different workspace). But clippy/deny will enforce the rules transitively at compile time.

### 4. Two apps have the same crate name but different configs

**Scenario:** `apps/validator-rust/crates/domain` and `apps/other-app/crates/domain` both have `name = "domain"` in their `Cargo.toml`.

**Behavior:** This is fine. They're in separate workspaces, so there's no name collision. guardrail3 identifies apps by their directory path, not by crate name. The `[rust.apps.*]` keys are directory-based names, not crate names.

**Where it could be confusing:** In the validation output. guardrail3 should always report the full path context, not just the crate name:
```
=== validator-rust/crates/domain ===
```
Not:
```
=== domain ===
```

### 5. An app is both a workspace member AND has its own workspace

**Scenario:** Can a directory be both listed in the root workspace's `members` AND have its own `[workspace]` section?

**Answer:** No, this is impossible in Cargo. A crate's `Cargo.toml` can have EITHER `[package]` (making it a member/standalone crate) OR `[workspace]` (making it a virtual workspace root), or both (non-virtual workspace root). But if it has `[workspace]`, it IS a workspace root -- it cannot simultaneously be a member of another workspace. Cargo will error.

The correct pattern is the steady-parent one: root workspace `exclude`s the app directories that are their own workspaces.

**What if someone puts an app directory in BOTH `members` and `exclude`?** Cargo behavior is undefined/error. guardrail3 should detect this contradiction and report it as an error.

### 6. clippy.toml at crate level inside a workspace

**Scenario:** User places `apps/validator-rust/crates/domain/clippy.toml` expecting domain to have different bans.

**What actually happens:** In workspace builds (`cargo clippy --workspace`), clippy's behavior for per-crate `clippy.toml` is unreliable (issue #7353). The workspace root's `clippy.toml` typically wins for `disallowed_methods`/`disallowed_types`. For threshold settings (`too-many-lines-threshold`), per-crate config MAY work in some cases.

**guardrail3 behavior:**
- WARN: "clippy.toml found at apps/validator-rust/crates/domain/ -- per-crate clippy.toml is not reliably supported in workspace builds (see clippy#7353). Configure bans at the workspace root instead."
- Do NOT generate per-crate clippy.toml
- Do NOT validate per-crate clippy.toml (it's not a supported configuration)

### 7. Root workspace has no packages (virtual workspace with only apps)

**Scenario:**
```toml
[workspace]
members = []
exclude = ["apps/*"]
```

Root workspace has no members. All work happens in nested workspaces under `apps/`.

**Behavior:** guardrail3 should NOT generate clippy.toml/deny.toml at the repo root (no members = no code to check). Only generate at each app root.

**Detection:** If `[rust.packages]` is not in `guardrail3.toml` AND the root workspace has zero members (after excluding), skip root-level config generation.

### 8. Standalone crate (no workspace) with subcrates

**Scenario:** `apps/substack-publisher/` has `[package]` but no `[workspace]`, yet contains `apps/substack-publisher/subcrates/helper/` as a path dependency.

**Behavior:** `helper` is NOT a workspace member (there is no workspace). It's just a path dependency. clippy reads `clippy.toml` from `helper/` when compiling it, falling back to... nothing (there's no walk-up for clippy).

**Wait -- is there walk-up for clippy?** Let me be precise. clippy reads `clippy.toml` from `CARGO_MANIFEST_DIR`. For `helper`, that's `apps/substack-publisher/subcrates/helper/`. If no `clippy.toml` exists there, clippy uses defaults. **There is no walk-up to the parent crate's directory.**

**guardrail3 behavior:** Generate `clippy.toml` only at `apps/substack-publisher/`. For the subcrate, clippy won't find it. This is a known limitation of clippy's non-workspace crate handling.

**Mitigation:** If guardrail3 detects path dependencies in a non-workspace crate, WARN: "apps/substack-publisher has path dependency 'helper' but is not a workspace. clippy.toml from apps/substack-publisher/ won't apply to helper. Consider converting to a workspace."

### 9. App directory exists but is not in guardrail3.toml

**Scenario:** `apps/new-experiment/` has a `Cargo.toml` with `[workspace]`, but `guardrail3.toml` has no `[rust.apps.new-experiment]` entry.

**Behavior:**
- `generate`: Skip it. guardrail3 only generates for configured apps.
- `validate`: This is the interesting case. If `guardrail3 validate` discovers the workspace, should it validate it?
  - **Yes, but with default profile.** Validate should discover ALL workspaces and validate each. For unconfigured apps, use the root-level profile as default.
  - Report: "Note: apps/new-experiment is not configured in guardrail3.toml -- using default profile 'service'. Run `guardrail3 rs init` to add it."

### 10. Apps directory doesn't follow the convention

**Scenario:** Rust apps are at `services/api/` and `tools/cli/` instead of `apps/*`.

**Behavior:** `discover_nested_workspaces()` currently only looks under `apps/`. This is a limitation.

**Fix:** guardrail3.toml should support explicit workspace paths:
```toml
[rust]
workspaces = ["services/api", "tools/cli"]
```

This already exists in the schema (`RustConfig.workspaces: Option<Vec<String>>`). The generation code should use it when discovering app paths, falling back to `apps/*` convention when not specified.

### 11. Conflicting root workspace member and app

**Scenario:** Root workspace has `members = ["apps/validator-rust"]` (NOT excluded). And `apps/validator-rust/` has its own `[workspace]`.

**What happens:** Cargo error. You can't have a workspace member that is itself a workspace root. The member's `Cargo.toml` would need to be a `[package]` to be a workspace member.

**guardrail3 behavior:** Detect during discovery and ERROR: "apps/validator-rust is listed as a root workspace member but has its own [workspace] section. Either exclude it from the root workspace or remove its [workspace] section."

---

## Packages vs Apps

### The Distinction

In the steady-parent model:
- **Packages** (`packages/*`) are shared libraries, members of the root workspace, published to crates.io
- **Apps** (`apps/*`) are deployable services/tools, excluded from the root workspace, each with their own workspace or standalone Cargo.toml

### Config Implications

| Aspect | Packages | Apps |
|---|---|---|
| Profile default | `library` | `service` (or per-app config) |
| clippy.toml location | Repo root | App root |
| deny.toml location | Repo root | App root |
| I/O bans | Yes (library profile) | No (service profile) |
| Global-state bans | Yes (pure layer) | Depends on layer |
| Override source | `.guardrail3/overrides/` (global only) | Global + per-app |
| Source scan scope | Root workspace members | App workspace members |

### guardrail3.toml Schema for Packages

```toml
[rust.packages]
type = "library"    # Default profile for all packages

# Per-package overrides (if one package is special)
[rust.packages.low-expectations]
type = "library"
# allowed_deps = ["tokio"]  # If this library genuinely needs tokio
```

**Note:** The current `[rust.packages]` is a single `CrateConfig`, not a map of per-package configs. For the common case (all packages are libraries with identical settings), this is fine. If per-package differentiation is needed, it should be a `BTreeMap<String, CrateConfig>` like `[rust.apps]`.

---

## Summary: Where Each File Goes

| File | Root workspace | Nested workspace | Standalone crate | Repo root |
|---|---|---|---|---|
| clippy.toml | At root (for packages) | At workspace root | At crate root | Same as root workspace |
| deny.toml | At root (for packages) | At workspace root | At crate root | Same as root workspace |
| rustfmt.toml | At root (for packages) | At workspace root | At crate root | Same as root workspace |
| rust-toolchain.toml | -- | -- | -- | Yes (only here) |
| release-plz.toml | -- | -- | -- | Yes (only here) |
| cliff.toml | -- | -- | -- | Yes (only here) |
| Cargo.toml [workspace.lints] | Validated at root | Validated at workspace root | Validated at crate root | -- |

---

## Override Precedence Summary

```
Layer 1: Profile baseline
  └── Determined by [rust.apps.X.type] or [rust.packages.type]
  └── "service" | "library" | "cli" (future)
  └── Each profile has a fixed set of bans/settings

Layer 2: Global overrides
  └── .guardrail3/overrides/*.toml
  └── Applied to ALL scopes (packages and all apps)
  └── Additive only (adds entries to baseline)

Layer 3: Per-app overrides
  └── .guardrail3/apps/{name}/overrides/*.toml
  └── Applied to ONE specific app
  └── Additive only (adds entries on top of Layer 1 + Layer 2)

Layer 4: Per-app exemptions
  └── .guardrail3/apps/{name}/exemptions/*.toml
  └── Applied to ONE specific app
  └── Subtractive only (removes entries from Layer 1 + Layer 2 + Layer 3)
  └── Each exemption MUST have a reason
```

**Packages do not have per-package overrides or exemptions** (they use global overrides only). If per-package customization is needed, it's a signal that the package should be an app.

---

## Implementation Priority

1. **Per-app override loading** -- extend `load_local_overrides()` to accept an app name and load from `.guardrail3/apps/{name}/overrides/`
2. **Per-app exemption loading** -- new function to load exemptions and compute the final entry set
3. **Generate with per-app context** -- modify `generate_rust_files()` to pass per-app overrides/exemptions
4. **Validate per-scope** -- modify validation to iterate scopes and use per-scope expected configs
5. **Diff per-scope** -- modify diff to show per-scope comparisons with provenance
6. **CLI flag `--app`** -- filter generate/validate/diff to a single app
7. **Warning system** -- detect and warn about all edge cases listed above

---

## Open Questions

1. **Should packages support per-package overrides?** Current design says no. But what if `packages/special-math` needs `float_cmp` exempted from clippy? The workaround is to add `#[allow(clippy::float_cmp)] // reason: math library` per-function, which is actually better because it's visible in the source.

2. **Should there be a `cli` profile?** CLI tools don't need `reqwest::Client::new` banned (they're not long-running services), don't need `std::process::exit` banned (it's fine for CLIs), but still want most other guardrails. This is distinct from both `service` and `library`.

3. **How should `guardrail3 init` handle monorepos?** Currently `rs init` creates a single `guardrail3.toml` entry. For monorepos, it should discover all workspaces and standalone crates and generate `[rust.apps.*]` entries for each.

4. **Should guardrail3 manage `Cargo.toml [workspace.lints]`?** Currently it only validates, doesn't generate. For monorepos with multiple workspaces, each needs its own `[workspace.lints]`. Should guardrail3 generate/diff these too?

5. **How do pre-commit hooks work with multiple workspaces?** The hook runs from the repo root. It needs to run `cargo clippy` / `cargo deny` / `cargo test` for EACH workspace independently. The current hook assumes one workspace.
