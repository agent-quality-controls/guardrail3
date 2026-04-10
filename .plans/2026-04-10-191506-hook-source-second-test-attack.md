# Goal

Run another adversarial pass on the extracted hook source lanes, add tests for uncovered edge cases, and fix only the failures those tests prove.

# Approach

1. Re-read the hook plans and the extracted source/ingestion packages.
2. Identify untested edge cases in:
   - `hooks-shared` source checks and source ingestion
   - `hooks-rs` source checks and source ingestion
3. Add tests first for each confirmed gap.
4. Run the affected workspaces and let the new tests fail.
5. Fix the implementation at the right boundary.
6. Re-run tests and one more local attack pass.

# Key decisions

## Tests first

Do not fix spec gaps from inspection alone.
Each new edge case must be pinned with a failing test before code changes.

## Boundary-first fixes

If a bug is about:
- file selection or repo classification -> fix ingestion
- command parsing or command shape -> fix shared parser or rule support
- rule applicability -> fix the rule input contract

## Scope

Only the extracted hook source lanes in `packages/` are in scope here.

# Files to modify

- `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks/**`
- `packages/rs/hooks-shared/g3rs-hooks-shared-ingestion/**`
- `packages/rs/hooks-rs/g3rs-hooks-rs-source-checks/**`
- `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion/**`
- `.worklogs/*`
