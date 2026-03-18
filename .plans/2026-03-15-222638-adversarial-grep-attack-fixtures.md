# Create 10 adversarial grep-attack fixtures for rust-allow false positives

**Date:** 2026-03-15 22:26
**Task:** Create test fixtures where `#[allow(` appears in non-code contexts

## Goal
10 minimal Rust files in `tests/fixtures/grep-attacks/rust-allow/` that contain `#[allow(` patterns in strings, comments, doc comments, and macros. These are false positives for the current grep-based scanner. Each file must use string concatenation to avoid tripping guardrail3's own pre-commit hook.

## Approach
Create each file with the concatenation trick from allow_checks.rs: split `#[allow(` across string fragments so grep doesn't match it literally in the source, but the runtime string value contains the pattern.

## Files to Create
- `tests/fixtures/grep-attacks/rust-allow/` (directory)
- 10 .rs files per the spec in 01_adversarial_fixtures.md
