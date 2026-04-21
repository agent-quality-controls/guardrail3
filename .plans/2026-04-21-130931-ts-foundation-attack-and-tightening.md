## Goal

Attack the first `g3ts` foundation set and tighten any real gaps before moving on:

- `ts/eslint`
- `ts/tsconfig`
- `ts/package`
- `ts/npmrc`

End state:

- family boundaries still match the intended parser -> ingestion -> config-checks split
- tests prove the important behavior instead of only happy paths
- `g3ts` still validates real target roots cleanly

## Approach

1. Re-read the family plans for the intended ownership and wave-1 scope.
2. Inspect the live package shapes and tests for all four families.
3. Attack from:
   - boundary leakage
   - missing negative coverage
   - applicability mistakes
   - parser/family seam drift
4. Fix any real bugs or weak tests at the architecturally correct layer.
5. Re-run package tests, `g3rs validate`, and `g3ts validate` on the live `websmasher` targets.
6. Commit the tightening pass with a worklog.

## Key Decisions

- Attack the existing foundation set before adding more TS families.
  - Why: adding more families on top of weak seams compounds the damage.

- Keep scope on the current TS foundation packages and `apps/guardrail3-ts`.
  - Why: the worktree has a large unrelated dirty Rust slice that should not be touched here.

## Files To Modify

- `packages/parsers/npmrc-parser/**`
- `packages/ts/eslint/**`
- `packages/ts/tsconfig/**`
- `packages/ts/package/**`
- `packages/ts/npmrc/**`
- `apps/guardrail3-ts/**`
- `.worklogs/**`
