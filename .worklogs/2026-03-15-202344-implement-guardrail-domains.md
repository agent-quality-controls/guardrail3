# Implement 4 guardrail domains + pre-existing fixes

**Date:** 2026-03-15 20:23
**Scope:** CLI, orchestrators, 3 new validation domains, clippy modules, release config generation, self-validation fixes

## Summary

Implemented 41 new validation checks across 4 domains (--code, --architecture, --release, --tests) with domain filtering via CLI flags. Added Garde boundary validation clippy modules, release config generation/scaffolding, and fixed 3 pre-existing self-validation issues (LICENSE files, R58 test false positives, R50 anyhow skip).

## Context & Problem

guardrail3 had 134 checks across code and architecture. The plan called for expanding to 4 filterable domains with 41 new checks covering Garde boundary validation, release readiness, and test quality.

## Decisions Made

### Domain filtering architecture
- **Chose:** ValidateDomains struct passed through orchestrators, no flags = all domains
- **Why:** Simple, backward-compatible, composable (--code --release runs both)

### Check ID renumbering
- **Chose:** Own numbering scheme (R-PUB-01 to R-PUB-12) reconciling two conflicting plans
- **Why:** release_setup_validator.md and guardrail3-domains.md had different ID assignments for same checks
- **Alternatives:** Follow one plan exactly — rejected because both had gaps

### R58 test exclusion
- **Chose:** Skip all lines after #[cfg(test)] in a file
- **Why:** Convention in this codebase is test modules are always last. Simple heuristic that eliminates 99 false positives.

### R-PUB-09/R-REL-09 merge
- **Chose:** Merge into single per-crate R-PUB-09 gated by --thorough
- **Why:** Per-crate dry-run is more useful than repo-level, and the plans disagreed on scope

### File splits for 500-line limit
- **Chose:** Split by functional concern (metadata vs deps, repo vs binary, tools vs quality)
- **Why:** Better than splitting tests out — keeps related code together

## Architectural Notes

New file structure under src/rs/validate/:
- garde_checks.rs — R-GARDE-01 to R-GARDE-05
- release_checks.rs — orchestrator + crate discovery
- release_crate_checks.rs — R-PUB-01 to R-PUB-05, R-PUB-08
- release_crate_deps.rs — R-PUB-06/07, R-PUB-09/10/11
- release_repo_checks.rs — R-REL-01 to R-REL-08
- release_bin_checks.rs — R-BIN-01 to R-BIN-03
- test_checks.rs — R-TEST-01 to R-TEST-04 + orchestrator
- test_quality_checks.rs — R-TEST-05 to R-TEST-08

New file under src/ts/validate/:
- test_checks.rs — T-TEST-01 to T-TEST-05

New module: src/modules/release.rs (release-plz.toml + cliff.toml templates)

## Process Notes

Used task-driven execution with adversarial convergence loop:
- 52 initial tasks created from mechanical extraction of plan check IDs
- 6 background agents for parallel implementation
- 3 adversarial review passes
- Pass 1: found 3 MEDIUM gaps (missing release modules/generate/init) → fixed
- Pass 2: found 2 HIGH gaps (oversized files, stale clippy.toml) → fixed
- Pass 3: found 3 pre-existing issues (LICENSE, R58 noise, anyhow) → fixed
- Final: 0 errors in self-validation

## Key Files for Context

- `.plans/todo/` — original 5 planning documents (source of truth)
- `src/rs/validate/mod.rs` — ValidateDomains struct + domain routing
- `src/modules/clippy.rs` — Garde method/type ban modules
- `CLAUDE.md` — updated with new check inventory

## Next Steps

1. Update CLAUDE.md check inventory section with new R-GARDE, R-PUB, R-REL, R-BIN, R-TEST, T-TEST checks
2. Run guardrail3 validate on other projects (pipelin3r, low-expectations) to test release checks
3. Consider adding R-REL-09 as separate repo-level summary check if needed
