# Step 30: Verify Golden Tests Against All 5 Projects

## Goal
Run guardrail3 against all 5 projects and compare to pre-migration golden snapshots. Document every difference.

## Task (1 agent)

1. Build release: `cargo build --release`
2. For each project (self, websmasher, pipelin3r, schedulr, steady-parent):
   - Run validation: `./target/release/guardrail3 validate {path} --format json | sh golden-tests/normalize.sh {path}`
   - Compare to golden: `diff golden-tests/golden/{name}.json <(above)`
   - If diff: classify each difference as:
     - `IMPROVEMENT`: false positive removed (string/comment was incorrectly flagged)
     - `REGRESSION`: real violation now missed
     - `CHANGE`: different line number or message wording (neutral)
3. Write report: `tests/fixtures/grep-attacks/GOLDEN_DIFF_REPORT.md`
4. If ANY regression: STOP. Fix before proceeding.
5. If only improvements: update golden snapshots with new output.

## Verification
```bash
sh golden-tests/compare.sh  # must pass (after updating goldens for improvements)
```

## On Failure
Regressions mean a syn/tree-sitter check is less capable than the grep version. Read the specific check, find what pattern it misses, fix the AST walker.
