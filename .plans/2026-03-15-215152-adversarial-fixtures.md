# Adversarial test fixtures for R30-R58 source scan checks

**Date:** 2026-03-15 21:51
**Task:** Create adversarial Rust source files that should trigger specific guardrail3 violations, then run guardrail3 against them and verify detection.

## Goal
Comprehensive adversarial fixture suite that proves R30-R58 checks catch violations, including edge cases that might evade pattern-based detection.

## Approach

### Fixture strategy
Each fixture is a minimal .rs file with exactly one violation type. The integration test creates a temp directory with a valid Cargo.toml, copies each fixture, runs `guardrail3 rs validate --format json`, and asserts the expected check ID appears.

### Key implementation insights from reading the code
1. **R30**: Detects `#![allow(` by string concatenation (`["#!", "[allow("].concat()`). Multi-line crate allows with empty lint on first line are SKIPPED.
2. **R32**: Checks `has_comment = trimmed.contains("//")` — any `//` on the same line counts as having a reason, even `//` inside a string.
3. **R37**: Looks for `#[cfg_attr(` + `allow(` on same line. R37 is Info severity, not Error.
4. **R42**: Checks specific patterns: `unsafe {`, `unsafe{`, `unsafe fn `, `unsafe impl `, `unsafe trait `.
5. **R44**: `.unwrap()` is only checked on non-test files.
6. **R58**: Checks `trimmed.starts_with("use std::fs")` — only catches imports, not `std::fs::` inline calls (except separately).

### Potential bugs to probe
- R32: `#[allow(clippy::foo)]` where the `//` is inside the lint name — false negative?
- R42: `unsafe` followed by newline then `{` — does it get caught? (No, pattern is single-line)
- R58: `use std::fs::read_to_string;` — starts with "use std::fs" so should be caught
- Multi-line allow: `#[allow(\n  clippy::unwrap_used\n)]` — the checker skips empty lint names and appends "..." for multi-line

## Files to Create
- `tests/fixtures/adversarial/*.rs` — 16 fixture files
- `tests/adversarial_fixtures.rs` — integration test

## Risks
- Fixtures must avoid tripping guardrail3's own pre-commit on THIS repo (use string concat for allow patterns)
- The test runs the binary against a temp dir, so fixtures need a valid Cargo.toml context
