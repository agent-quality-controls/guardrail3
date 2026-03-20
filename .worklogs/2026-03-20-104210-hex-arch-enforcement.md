# Rewrite hex arch structural enforcement + rename src/ to crates/

**Date:** 2026-03-20 10:42
**Scope:** hex_arch_structure.rs (new), hex_arch_checks.rs, mod.rs, test_hex_arch_checks.rs, app layout

## Summary
Rewrote R-ARCH-01 to enforce strict hex arch directory template. Auto-detects service apps from `apps/*/Cargo.toml` — no guardrail3.toml config needed. Renamed guardrail3's own `src/` to `crates/` as first step toward hex arch compliance.

## Context & Problem
The old R-ARCH-01 only checked 2 of 4 layers (domain, adapters), depended on guardrail3.toml `profile = "service"` config, and used `Severity::Warn`. It was too weak to enforce the hex arch template. Our own app had no `crates/` directory at all and the check didn't catch it.

## Decisions Made

### New R-ARCH-01: strict structural enforcement
- **Chose:** Auto-detect from `apps/*/Cargo.toml`, no config dependency
- **Why:** Structure is the rule — position in the tree determines everything
- **Alternatives:** Keep config-based detection — rejected because it requires manual setup and can drift

### `.gitkeep` as placeholder
- **Chose:** Container folders must have `.gitkeep` or at least one crate subdir
- **Why:** User may not need all layers yet (e.g., no inbound ports), but the structure must exist
- **Empty without `.gitkeep`:** Error — forces explicit acknowledgment

### `src/` banned at app root
- **Chose:** Flag `apps/{name}/src/` as error — code must be in `crates/`
- **Why:** `src/` means flat single-crate layout, not hex arch

## Structural rules enforced
1. `crates/` must exist with exactly `{adapters, app, domain, ports}`
2. `adapters/` and `ports/` must each contain exactly `{inbound, outbound}`
3. Container folders: must have `.gitkeep` or crate subdirs (not empty)
4. Every subdir in a container must have `Cargo.toml`
5. Only `.gitkeep` files allowed in structural and container dirs
6. No `src/` directory at app root

## Key Files for Context
- `apps/guardrail3/crates/app/rs/validate/hex_arch_structure.rs` — new structural checks
- `apps/guardrail3/crates/app/rs/validate/hex_arch_checks.rs` — R-ARCH-02/03/04 (unchanged)
- `.plans/2026-03-20-094539-hex-arch-enforcement.md` — original plan

## Next Steps
1. Plan the full hex arch refactoring of guardrail3 (what code goes where in `crates/`)
2. Split single crate into proper hex arch crates with Cargo.toml each
3. Update workspace members, imports, dependencies
