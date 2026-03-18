# Complete All Migration Gaps — Full Hardening Pass

**Date:** 2026-03-16 10:00
**Task:** Fix every gap found by adversarial verification agents across all migration steps

## Goal
Every migration step is fully complete: all tests exist, all checks migrated, file limits respected, convergence criteria met, MIGRATION_REPORT.md written.

## Gap Inventory (from 5 adversarial agents)

### Test Gaps — Rust side
1. R36 (EXCEPTION comments) — zero test coverage
2. R37 (cfg_attr allow) — no integration test in allow_checks.rs
3. R40 (use count >20) — no threshold detection test
4. R41 (use count 15-20) — no warning range test
5. R42 (unsafe) — no integration test in structure_checks.rs
6. R44 (unwrap/expect) — no integration test in code_quality_checks.rs
7. count_use_statements — missing string-literal false-positive test in ast_helpers.rs

### Test Gaps — TypeScript side
8. T23-T29 — zero unit tests in ts/validate/source_scan.rs
9. T30-T35 — zero unit tests in ts/validate/source_scan.rs
10. TS grep fallback paths — completely untested

### Structural Issues
11. ts/validate/source_scan.rs is 656 lines — must split under 500

### Remaining GREP_BUG
12. multiline_string.rs (R32) — line continuation fools scanner
13. cfg_gated_use.rs (R58) — cfg-gated std::fs flagged

### Plan Coverage Gaps — 6 checks not migrated
14. R-GARDE-05 — derive inventory scan → syn
15. R-TEST-04 — test existence → syn
16. R-TEST-05 — pub fn/test fn count → syn
17. R-TEST-07 — #[ignore] without reason → syn
18. T-TEST-04 — .skip() without reason → tree-sitter
19. T-TEST-05 — .only() in committed code → tree-sitter

### Step 02 Baseline Integrity
20. string_unwrap test contaminated with post-migration assertions
21. Missing RESULTS_BEFORE.json (0/5 categories)
22. Missing TypeScript adversarial tests (0/10 fixtures)

### Missing Deliverables
23. MIGRATION_REPORT.md (step 99 final output)

## Approach

### Phase 1: Fix all test gaps (tasks 1-5)
### Phase 2: Split source_scan.rs (task 6)
### Phase 3: Fix 2 GREP_BUGs (task 7)
### Phase 4: Migrate 6 uncovered checks (tasks 8-10)
### Phase 5: Step 02 baseline cleanup (task 11)
### Phase 6: Write MIGRATION_REPORT.md (task 12)
### Phase 7: Adversarial verification (2 agents)

## Files to Modify
- `src/rs/validate/allow_checks.rs` — add R36, R37 tests
- `src/rs/validate/structure_checks.rs` — add R40, R41, R42 tests
- `src/rs/validate/code_quality_checks.rs` — add R44 test
- `src/rs/validate/ast_helpers.rs` — add count_use_statements string test
- `src/ts/validate/source_scan.rs` — add T23-T35 tests, then split
- `src/rs/validate/allow_checks.rs` — fix multiline_string R32
- `src/rs/validate/code_quality_checks.rs` — fix cfg_gated_use R58
- `src/rs/validate/garde_checks.rs` — migrate R-GARDE-05 to syn
- `src/rs/validate/test_checks.rs` — migrate R-TEST-04 to syn
- `src/rs/validate/test_quality_checks.rs` — migrate R-TEST-05, R-TEST-07 to syn
- `src/ts/validate/test_checks.rs` — migrate T-TEST-04, T-TEST-05 to tree-sitter
- `tests/adversarial_grep_attacks.rs` — fix string_unwrap baseline
