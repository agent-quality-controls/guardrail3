# Restructure crates/ into hex arch layout

**Date:** 2026-03-20 11:06
**Scope:** All files under apps/guardrail3/crates/, tests, lib.rs, main.rs

## Summary
Moved all code into proper hex arch directory structure to fix 14 R-ARCH-01 violations. Added new `hex_arch_structure.rs` check that auto-detects service apps and enforces the full structural template. All 306 tests pass, 0 R-ARCH-01 violations remain.

## Decisions Made

### New hex_arch_structure.rs replaces old R-ARCH-01
- **Chose:** Auto-detect service apps from `apps/*/Cargo.toml`, enforce full template (not just domain+adapters)
- **Why:** Old check only looked for 2 of 4 layers and required guardrail3.toml config. New check enforces all dirs, container rules, .gitkeep placeholders, src/ ban, loose file detection
- **Old check removed:** `check_hex_arch_structure` deleted from `hex_arch_checks.rs`

### No backward-compat re-exports
- **Chose:** Update all import paths to use new locations directly
- **Why:** User explicitly rejected re-exports — "don't cope out with backwards compat"
- **Impact:** Every `crate::app::crawl` → `crate::app::core::crawl`, `crate::commands` → `crate::adapters::inbound::cli`, etc.

### Cargo.toml stubs not workspace members
- **Chose:** Add `version = "0.0.0"` Cargo.toml stubs that satisfy R-ARCH-01 but aren't workspace members yet
- **Why:** Structural compliance first, actual crate splitting is a separate phase

## File moves
- `commands/`, `cli.rs`, `help_gen.rs` → `adapters/inbound/cli/`
- `report/` → `adapters/outbound/report/`
- `adapters/outbound/{fs.rs,tool_runner.rs}` → `adapters/outbound/{fs/,tool-runner/}`
- `ports/outbound.rs` → `ports/outbound/traits/`
- `domain/report.rs` → `domain/report/`
- `app/{crawl,discover,gitignore,project_map}.rs` → `app/core/`
- Removed `mod.rs` from structural/container dirs (adapters/, ports/, domain/, app/, adapters/outbound/)

## Key Files for Context
- `crates/app/rs/validate/hex_arch_structure.rs` — new R-ARCH-01 structural enforcement
- `crates/lib.rs` — module tree with `#[path]` for tool-runner (hyphen) and traits dirs
- `crates/main.rs` — updated imports

## Next Steps
1. Run guardrails again — R-PUB warnings on stub Cargo.toml crates are expected
2. Decide whether to split into actual workspace crates or keep single-crate with correct structure
3. Address remaining non-arch violations (R26, R55, R-TEST-08)
