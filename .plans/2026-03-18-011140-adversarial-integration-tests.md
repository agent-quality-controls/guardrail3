# Write adversarial integration tests for ws discovery and type detection

**Date:** 2026-03-18 01:11
**Task:** Create two new integration test files for guardrail3

## Goal
Two new test files that exercise edge cases in workspace discovery (`rs generate --dry-run`) and TS app type auto-detection (`ts init --dry-run`).

## Approach

### File 1: adversarial_ws_discovery.rs
6 tests exercising Rust workspace discovery via `rs generate --dry-run`:
- Empty workspace members
- Glob members matching nothing
- Multiple nested workspaces
- Workspace exclude
- Single crate (no workspace)
- Virtual workspace with packages only

### File 2: adversarial_type_detection.rs
7 tests exercising TS app type auto-detection via `ts init --dry-run` and `ts generate`:
- velite in devDependencies
- content/ directory detection
- hex arch detection
- no signals default
- content vs service signal priority
- library type generates no stylelint
- mixed types generate correct eslint

## Files to Modify
- `apps/guardrail3/tests/adversarial_ws_discovery.rs` -- new file
- `apps/guardrail3/tests/adversarial_type_detection.rs` -- new file
