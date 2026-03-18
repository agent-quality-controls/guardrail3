# Audit and fix all check result messages to be agent-actionable

**Date:** 2026-03-16 19:45
**Task:** Rewrite all CheckResult message fields to be imperative, specific, and self-contained for AI agent consumption.

## Goal
Every CheckResult message tells an agent exactly what to DO, not just what's wrong. Messages start with what's wrong (one sentence) then follow with exactly how to fix it.

## Approach
Systematically update message fields in all Rust check files. Only change `title` and `message` string content. Do NOT change check IDs, severities, or logic.

## Files to Modify
- config_files.rs — R1, R2, R3, R8, R21, R24
- allow_checks.rs — R30-R37
- code_quality_checks.rs — R43, R44, R49, R58
- garde_checks.rs — R-GARDE-01 through R-GARDE-05
- hex_arch_checks.rs — R-ARCH-01 through R-ARCH-04
- cargo_lints.rs — R26-R29
- dependency_scan.rs — R45-R50
- deny_bans.rs — R12-R13, R17-R18
- deny_audit.rs — R8-R11
- deny_licenses.rs — R14-R16
- deny_inventory.rs — R19-R20
- structure_checks.rs — R38-R42, R53
- clippy_coverage.rs — R4-R7
- test_checks.rs — R-TEST-01 through R-TEST-09
- test_quality_checks.rs — R-TEST-05 through R-TEST-08
- toolchain_check.rs — R25
- rustfmt_check.rs — R22-R23
- workspace_metadata.rs — R55-R57
- dependency_allowlist.rs — R-DEPS-01, R-DEPS-02
- release_*.rs — R-PUB-*, R-REL-*, R-BIN-*
