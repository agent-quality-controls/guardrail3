# Per-Crate Profiles + Dependency Allowlists

**Date:** 2026-03-16 14:10
**Task:** Add per-crate profile assignment and dependency allowlists for workspace monorepos

## Goal
Each crate in a workspace can be independently configured as `service` or `library` with an explicit dependency allowlist. Libraries cannot use ANY dependency not in their allowlist.

## Config Schema

```toml
[rust]
workspace_root = "."

[rust.crates.my-api]
profile = "service"
# Services: full access, no allowlist required (but can have one)

[rust.crates.my-domain]
profile = "library"
allowed_deps = ["serde", "thiserror", "chrono"]
# Everything not listed → R-DEPS-01 error

[rust.crates.my-sdk]
profile = "library"
allowed_deps = ["serde", "serde_json", "reqwest", "tokio", "thiserror"]
# SDK needs HTTP — explicitly allowed
```

## New Checks

### R-DEPS-01: Unauthorized dependency
For each crate with `allowed_deps`:
1. Parse the crate's Cargo.toml
2. Read `[dependencies]` keys
3. For each dep not in `allowed_deps` → Error
4. Skip `[dev-dependencies]` (test deps are fine)
5. Skip `[build-dependencies]` (build tools are fine)
6. Skip workspace dependencies that are in the same workspace (internal deps checked by R51)

### R-DEPS-02: Library crate without allowlist
For each crate with `profile = "library"` that does NOT have `allowed_deps` → Warn
"Library crate has no dependency allowlist. Add allowed_deps to enforce least privilege."

### Per-crate profile enforcement
When `profile = "library"` is set on a crate:
- clippy.toml for that crate gets I/O bans (same as workspace library profile)
- Global state bans applied (LazyLock, OnceLock)
- R58 std::fs check applies with stricter rules

When `profile = "service"` is set:
- Normal service-level clippy.toml
- composition-root layer allowed (global state OK)

## Implementation

### Config changes (domain/config/types.rs)
```rust
pub struct CrateConfig {
    pub layer: Option<String>,        // existing: "pure" | "composition-root"
    pub profile: Option<String>,      // NEW: "service" | "library"
    pub allowed_deps: Option<Vec<String>>,  // NEW: dependency allowlist
}
```

### New check module (app/rs/validate/dependency_allowlist.rs)
```rust
pub fn check_dependency_allowlist(
    cargo_path: &Path,
    allowed: &[String],
    fs: &dyn FileSystem,
    results: &mut Vec<CheckResult>,
)
```

### Generate changes (commands/generate.rs)
Per-crate clippy.toml generation should use crate profile (not just layer).

## Files to Create/Modify
- domain/config/types.rs — add profile + allowed_deps to CrateConfig
- app/rs/validate/dependency_allowlist.rs — NEW: R-DEPS-01, R-DEPS-02 checks
- app/rs/validate/mod.rs — wire new checks
- commands/generate.rs — use per-crate profile for clippy generation
- tests — add unit + integration tests

## Depends On
- Hex arch refactor (task 55) — need &dyn FileSystem in validation signatures
