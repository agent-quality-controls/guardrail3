# Create 10 Adversarial Rust Structural Fixtures

**Date:** 2026-03-15 22:26
**Task:** Create Agent 3's fixtures from the adversarial fixtures plan

## Goal
Create `tests/fixtures/grep-attacks/rust-structural/` with 10 Rust fixture files testing structural edge cases (false positives for R58/R38/R40).

## Approach
Create each fixture file directly. For line-count and use-count fixtures, generate the exact number of effective lines programmatically. No tests — just fixtures.

## Files to Create
- `tests/fixtures/grep-attacks/rust-structural/string_use_std_fs.rs`
- `tests/fixtures/grep-attacks/rust-structural/comment_use_std_fs.rs`
- `tests/fixtures/grep-attacks/rust-structural/use_in_doc_comment.rs`
- `tests/fixtures/grep-attacks/rust-structural/reexport_fs.rs`
- `tests/fixtures/grep-attacks/rust-structural/cfg_gated_use.rs`
- `tests/fixtures/grep-attacks/rust-structural/exactly_500_lines.rs`
- `tests/fixtures/grep-attacks/rust-structural/exactly_501_lines.rs`
- `tests/fixtures/grep-attacks/rust-structural/exactly_20_uses.rs`
- `tests/fixtures/grep-attacks/rust-structural/exactly_21_uses.rs`
- `tests/fixtures/grep-attacks/rust-structural/blank_lines_only.rs`
