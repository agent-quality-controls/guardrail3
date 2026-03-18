# Step 02: Capture "Before" Results Against All Fixtures

## Goal
Run current grep-based guardrail3 against every adversarial fixture. Save the results. After migration, compare to prove improvements.

## Task (1 agent)

1. For each fixture in `tests/fixtures/grep-attacks/`:
   - Run `guardrail3 rs validate {fixture_project} --format json`
   - Save output to `tests/fixtures/grep-attacks/{category}/RESULTS_BEFORE.json`

2. Create a summary: `tests/fixtures/grep-attacks/BEFORE_SUMMARY.md`
   - For each fixture: check IDs fired, severities, expected vs actual
   - Count: total false positives, total correct detections, total missed

## Verification

```bash
# Summary file exists and has content
test -s tests/fixtures/grep-attacks/BEFORE_SUMMARY.md
```

## On Failure
If guardrail3 crashes on any fixture, that's a bug. Report it but continue with other fixtures.
