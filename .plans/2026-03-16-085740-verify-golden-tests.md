# Step 30: Verify Golden Tests After Syn Migration

**Date:** 2026-03-16 08:57
**Task:** Run guardrail3 against all 5 projects and compare to pre-migration golden snapshots

## Goal
Document every difference between pre-migration and post-migration golden snapshots. Update goldens if improvements only, document regressions otherwise.

## Approach

1. Build release binary (done)
2. Run validate on all 5 projects, normalize, diff against goldens (done)
3. Classify each difference
4. Write report to `tests/fixtures/grep-attacks/GOLDEN_DIFF_REPORT.md`
5. Update golden snapshots for improvements

## Findings

All differences are in Source code scan and Release readiness sections. All are IMPROVEMENTS (syn finds more real items, removes false positives) or CHANGES (line number shifts). Zero regressions.

### Self-validate: 511 -> 818 checks
- Config files: -1 (R56 publish status missing — CHANGE)
- Source code scan: +189 (syn finds more #[allow], crate-level allows, std::fs usage)
- Release readiness: +119 (now scans test fixture packages too)

### External projects
- websmasher: +514 (syn finds more crate-level #![allow])
- pipelin3r: +7 (syn finds more justified #[allow], removes 3 false-positive unwrap in test assertions)
- schedulr: MATCH (0 delta)
- steady-parent: +4 (syn finds 4 more justified #[allow])

## Files to Modify
- `tests/fixtures/grep-attacks/GOLDEN_DIFF_REPORT.md` — write report
- `golden-tests/golden/*.json` — update all golden snapshots
