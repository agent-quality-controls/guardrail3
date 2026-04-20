## Goal

Expand `ts/eslint` from the first six checks into a real config-content family that forces a durable ESLint baseline through effective config evaluation.

End state:

- `g3ts-eslint-config-checks` enforces the core effective-config baseline, not just existence and parseability.
- The family still stays within config contents only.
- No package ownership, framework-specific UI policy, hexarch policy, or npm/package-manager policy is pulled into this pass.
- Parser, types, ingestion, and config-check roots remain clean under tests, formatting, and `g3rs validate`.

## Approach

1. Extend config-check support helpers for:
   - per-probe access
   - rule severity checks
   - numeric rule-threshold extraction
   - grouped missing-rule reporting
2. Add grouped config rules for:
   - baseline thresholds and restricted imports
   - core async/type rule baseline
   - extended type-safety rule baseline
   - extended hygiene rule baseline
   - unicorn rule baseline
   - regexp rule baseline
   - sonarjs rule baseline
   - TS test carve-out behavior
   - JS carve-out behavior
3. Extend runtime tests with:
   - a fully golden effective-config snapshot
   - missing-group failures
   - threshold mismatch failures
   - test/js carve-out failures
4. Re-run tests, formatting, and `g3rs validate` on parser and `ts/eslint` roots.

## Key Decisions

- Keep the expansion on effective config, not raw source imports.
  - Reason: the Node-backed parser gives us actual ESLint semantics, which is stronger and less bypassable than string checks.
- Keep React, jsx-a11y, Next, CSS, and package presence out of this pass.
  - Reason: those are either frontend/content-profile-specific or belong to other family ownership discussions.
- Use grouped rule IDs for baseline slices instead of one ID per ESLint rule.
  - Reason: the family should be enforceable without exploding the package into dozens of nearly identical files in this pass.

## Files To Modify

- `packages/ts/eslint/g3ts-eslint-config-checks/**`
