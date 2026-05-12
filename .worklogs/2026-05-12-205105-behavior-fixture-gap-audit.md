# Behavior fixture gap audit

## Summary

Ran four adversarial fixture audits against the current behavior replay fixture stack.

Recorded the missing coverage plan in `.plans/2026-05-12-204524-behavior-fixture-gap-audit.md`.

## Decisions

- Kept the audit as a plan file because no fixture implementation changed in this step.
- Split the report by audit agent scope: CLI/shared/topology/hooks, config/tooling, source/architecture/policy, and G3TS.
- Preserved the layered fixture rule: add compound fixtures by unlock layer instead of one fixture per old test.
- Recorded runner requirements before test deletion: multiple commands, `validate-repo`, `--family`, `--staged`, runner modes, executable bits, Git config, and baseline comparison.

## Key Files

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md`
- `.plans/2026-05-12-204524-behavior-fixture-gap-audit.md`
- `behavior/fixtures/g3rs`
- `scripts/behavior/verify-fixtures.py`

## Verification

- `git diff --check`

## Next Steps

- Extend fixture metadata and verifier for multiple commands per fixture.
- Add command-mode fixture support for `validate-repo`, `--family`, and `--staged`.
- Implement Agent A G3RS fixtures first because they prove command modes and repo-level behavior.
