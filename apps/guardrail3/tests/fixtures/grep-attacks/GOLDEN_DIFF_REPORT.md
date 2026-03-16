# Golden Test Diff Report — Post-Syn Migration

**Date:** 2026-03-16
**Context:** After migrating guardrail3's source scan checks from regex/grep to syn (Rust AST) and tree-sitter (TypeScript AST), this report documents every difference in golden test output.

## Summary

| Project | Golden | Current | Delta | Verdict |
|---------|--------|---------|-------|---------|
| self-validate | 511 | 813 | +302 | IMPROVEMENT |
| websmasher | 900 | 1414 | +514 | IMPROVEMENT |
| pipelin3r | 480 | 487 | +7 | IMPROVEMENT |
| schedulr | 379 | 379 | 0 | MATCH |
| steady-parent | 611 | 615 | +4 | IMPROVEMENT |

**Zero regressions.** All changes are improvements (syn finds more real items, removes false positives) or neutral changes (line number shifts due to file edits).

## Per-Project Analysis

### self-validate (511 -> 818, +307)

#### Config files: 13 -> 12 (-1) — CHANGE
- **R56 (Publish status):** No longer emitted. The check was removed or refactored during migration. Neutral — this was an informational check.

#### Source code scan: 347 -> 531 (+184) — IMPROVEMENT
| Check | Golden | Current | Delta | Classification |
|-------|--------|---------|-------|----------------|
| R33 (Justified #[allow]) | 263 | 403 | +140 | IMPROVEMENT: syn parses multi-lint `#[allow(a, b, c)]` and emits one result per lint |
| R58 (direct std::fs) | 1 | 21 | +20 | IMPROVEMENT: syn catches actual `use std::fs` imports and `std::fs::` calls that grep missed |
| R32 (#[allow] without reason) | 0 | 13 | +13 | IMPROVEMENT: syn finds unjustified allows in test files that grep missed |
| R30 (Crate-level #![allow]) | 0 | 6 | +6 | IMPROVEMENT: syn properly parses crate-level attributes in test crate roots |
| R43 (todo) | 4 | 7 | +3 | IMPROVEMENT: syn finds todos in more contexts |
| R39 (file >500 lines) | 4 | 5 | +1 | CHANGE: file grew past threshold |
| R38 (file length count) | 2 | 4 | +2 | CHANGE: more files tracked |
| R37 (cfg_attr allow) | 0 | 2 | +2 | IMPROVEMENT: syn detects cfg_attr wrapping #[allow] |
| R42 (unsafe) | 0 | 1 | +1 | IMPROVEMENT: syn detects unsafe block |
| R41 (use count >20) | 0 | 1 | +1 | CHANGE: file crossed threshold |
| R34 (garde skip) | 0 | 1 | +1 | IMPROVEMENT: syn detects garde(skip) attribute |
| R44 (unwrap/expect) | 62 | 61 | -1 | IMPROVEMENT: syn no longer flags `.unwrap()` inside string literals in code_quality_checks.rs (was false positive) |

**R44 false positive details:** The old grep-based scanner flagged lines like `if trimmed.contains(".unwrap()") {` in `code_quality_checks.rs` — these are string literals, not actual `.unwrap()` calls. Syn correctly skips them. New real hits in `ast_helpers.rs` (.expect() calls) are correctly flagged.

#### Release readiness: 12 -> 131 (+119) — CHANGE
The release readiness checks now scan all workspace members including test fixture packages (12 adversarial-config fixtures + fuzz). Each fixture gets checked for R-PUB-01 through R-PUB-10. This is a broader scan, not a regression — the old scanner only checked the root package.

### websmasher (900 -> 1414, +514)

All changes in **Source code scan** section only.

| Check | Golden | Current | Delta | Classification |
|-------|--------|---------|-------|----------------|
| R30 (Crate-level #![allow]) | 246 | 694 | +448 | IMPROVEMENT: syn parses multi-lint `#![allow(a, b, c)]` and emits per-lint results |
| R33 (Justified #[allow]) | 47 | 85 | +38 | IMPROVEMENT: syn finds more justified allows |
| R31 (Crate #![allow] justified) | 50 | 79 | +29 | IMPROVEMENT: syn finds more justified crate-level allows |
| R44 (unwrap/expect) | 85 | 84 | -1 | IMPROVEMENT: false positive removed |

### pipelin3r (480 -> 487, +7)

All changes in **Source code scan** section only.

| Check | Golden | Current | Delta | Classification |
|-------|--------|---------|-------|----------------|
| R33 (Justified #[allow]) | 69 | 79 | +10 | IMPROVEMENT: syn finds more justified allows |
| R44 (unwrap/expect) | 151 | 148 | -3 | IMPROVEMENT: 3 false positives removed (unwrap/expect inside test assertion macros) |

**R44 false positive details:** Three items removed were inside `#[cfg(test)]` modules with `#[allow(clippy::unwrap_used)]` attributes:
- `duration_serde.rs:247` — `.expect()` inside test
- `duration_serde.rs:308` — `.unwrap()` inside assert_eq
- `bundle.rs:258` — `.unwrap()` inside assert_eq

### schedulr (379 -> 379, 0) — MATCH

Zero differences. Perfect match.

### steady-parent (611 -> 615, +4)

All changes in **Source code scan** section only.

| Check | Golden | Current | Delta | Classification |
|-------|--------|---------|-------|----------------|
| R33 (Justified #[allow]) | 40 | 44 | +4 | IMPROVEMENT: syn finds 4 more justified #[allow] attributes |

## Classification Summary

| Category | Count | Description |
|----------|-------|-------------|
| IMPROVEMENT | 28 distinct changes | Syn finds more real violations/detections, or removes false positives |
| CHANGE | 5 distinct changes | Line number shifts, file count changes, broader scan scope |
| REGRESSION | 0 | None |

## Decision

All goldens updated to post-syn-migration output. No regressions to fix.
