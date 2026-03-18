# Hex Arch Enforcement — Structural Validation of Dependency Flow

**Date:** 2026-03-16 16:03
**Task:** Replace heuristic R51 with proper hex arch enforcement that validates actual dependency flow

## Problem
R51 currently guesses crate layer from directory names ("domain", "ports", etc.) and has hardcoded banned dependency lists. It doesn't:
- Use guardrail3.toml per-crate config
- Validate the hex arch structure exists
- Check actual dependency flow against declared layers
- Understand apps/ vs packages/ convention

## New Checks

### R-ARCH-01: Service must have hex arch structure
For each crate with profile = "service" in guardrail3.toml:
- Check that crates/domain/ exists (or domain/ submodule)
- Check that crates/ports/ exists
- Check that crates/app/ exists
- Check that crates/adapters/ exists
Severity: Warn (structural guidance, not blocking)

### R-ARCH-02: Dependency flow violation
For each workspace member, determine its LAYER from:
1. guardrail3.toml [rust.crates.NAME.layer] if set
2. Path-based detection: path contains "/domain" → domain, "/ports" → ports,
   "/app" → app, "/adapters" → adapters
3. If neither → skip (not a hex arch crate)

Then validate dependency flow by reading the crate's Cargo.toml [dependencies]:
- domain layer: CANNOT depend on ports, app, or adapter crates
- ports layer: CAN depend on domain. CANNOT depend on app or adapter crates
- app layer: CAN depend on domain and ports. CANNOT depend on adapter crates
- adapters layer: CAN depend on everything (implements ports, wires deps)

"Adapter crate" = any crate whose path contains "/adapters/" or has layer = "adapters"
"Ports crate" = path contains "/ports/" or layer = "ports"
etc.

Detection: check if a dependency is a workspace path dep, then check which layer
that path resolves to.

### R-ARCH-03: Library in packages/ depends on service internal crate
Libraries in packages/ should NEVER depend on a crate inside apps/*/crates/.
They can depend on other packages/ crates.

## Implementation

### New file: src/app/rs/validate/hex_arch_checks.rs

```rust
pub fn check_hex_arch_structure(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    crate_configs: &BTreeMap<String, CrateConfig>,
    results: &mut Vec<CheckResult>,
)

pub fn check_dependency_flow(
    fs: &dyn FileSystem,
    workspace_root: &Path,
    project: &ProjectInfo,
    crate_configs: &BTreeMap<String, CrateConfig>,
    results: &mut Vec<CheckResult>,
)
```

### Wire into mod.rs orchestrator
Replace R51 heuristic with new checks. Keep R52 (dependency graph inventory) as-is.

### Update R51
Either replace R51 with R-ARCH-02 or keep R51 as a legacy fallback for projects
without guardrail3.toml crate config.

## Files
- NEW: src/app/rs/validate/hex_arch_checks.rs
- MODIFY: src/app/rs/validate/mod.rs (wire new checks)
- MODIFY: src/app/rs/validate/dependency_direction.rs (simplify or deprecate)
