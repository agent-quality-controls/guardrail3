# Implement per-crate profile + dependency allowlist checks

**Date:** 2026-03-16 14:15
**Task:** Add R-DEPS-01 and R-DEPS-02 checks per plan at .plans/2026-03-16-141024-per-crate-allowlists.md

## Goal
Each crate in a workspace can have `profile` and `allowed_deps` in guardrail3.toml. R-DEPS-01 flags unauthorized deps, R-DEPS-02 warns library crates without allowlists.

## Approach

### Step 1: Update CrateConfig (domain/config/types.rs)
Add `profile: Option<String>` and `allowed_deps: Option<Vec<String>>` fields.

### Step 2: Create dependency_allowlist.rs (app/rs/validate/)
- `check_dependency_allowlist`: parse Cargo.toml, check [dependencies] keys against allowlist, skip workspace path deps
- `check_library_has_allowlist`: if profile=library and no allowed_deps, warn

### Step 3: Wire into orchestrator (app/rs/validate/mod.rs)
- Load full guardrail config (not just profile) to access crate configs
- Call both check functions in the architecture section

### Step 4: Update generate command (commands/generate.rs)
- Per-crate clippy.toml: use crate's own profile if set, falling back to workspace profile

### Step 5: Tests in dependency_allowlist.rs
5 tests covering both checks.

### Step 6: Register module in mod.rs

## Files to Modify
- `src/domain/config/types.rs` — add fields to CrateConfig
- `src/app/rs/validate/dependency_allowlist.rs` — NEW: R-DEPS-01, R-DEPS-02
- `src/app/rs/validate/mod.rs` — wire checks + register module
- `src/commands/generate.rs` — per-crate profile in clippy generation
