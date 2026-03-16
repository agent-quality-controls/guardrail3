# Step 99: Convergence Loop

## Goal
Repeat verification + adversarial review + fix until stable.

## Loop

```
ITERATION = 1
while true:
    1. Run golden tests (step 30)
       - If regressions: fix them (step 33), restart loop

    2. Run adversarial fixtures (step 31)
       - If GREP_BUG fixtures still fail: migration incomplete, fix (step 33), restart loop

    3. Run adversarial review (step 32)
       - If new issues found: fix them (step 33), restart loop

    4. Run full test suite
       cargo test
       cargo test --test cli_tests
       cargo test --test adversarial_fixtures
       cargo test --test adversarial_config_tests
       cargo test --test adversarial_grep_attacks
       cargo test --test property_tests
       - If any fail: fix (step 33), restart loop

    5. Run mutation testing (if time permits)
       cargo mutants -j 4 --timeout 30
       - If kill rate dropped below previous baseline: investigate, restart loop

    6. All pass → CONVERGED
       ITERATION += 1
       if ITERATION > 5:
           WARN: "5 iterations without convergence — needs human review"
           STOP

    break  # converged
```

## Convergence Criteria (ALL must be true)

1. Golden tests: 0 regressions (improvements documented)
2. Adversarial fixtures: 0 remaining GREP_BUG markers
3. Adversarial review: 0 new CRITICAL or HIGH findings
4. All tests: 218+ pass
5. No crashes on any input (property tests + fuzz targets confirm)

## Final Output

When converged, write:
- `MIGRATION_REPORT.md` — summary of what changed, improvements, any known limitations
- Updated golden snapshots
- Updated adversarial fixture expectations
- Commit with message: "Complete grep→AST migration: N false positives eliminated, 0 regressions"

## Expected Iterations
- Iteration 1: fix most issues, ~5-10 golden diffs (false positive removals)
- Iteration 2: fix edge cases found by adversarial review, ~2-3 issues
- Iteration 3: clean — convergence
