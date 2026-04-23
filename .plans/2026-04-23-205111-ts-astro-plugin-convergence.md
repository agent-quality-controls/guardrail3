## Goal

Close the remaining real correctness gaps in the `eslint-plugin-astro-pipeline` slice and re-check the `ts/astro` family against the Astro family plan and the intended validator architecture.

This pass should either:
- fix the remaining plugin gaps with red-first tests, or
- prove a gap belongs outside the current slice and remove or narrow the incorrect contract.

## Approach

1. Review the current plugin utilities and rules that still have known gaps:
   - `src/utils/import-closure.ts`
   - `src/utils/ast-helpers.ts`
   - `src/rules/no-authored-content-fs-read.ts`
   - `src/rules/no-authored-content-glob.ts`
   - `src/rules/no-direct-astro-content-in-routes.ts`
   - `src/rules/no-runtime-mdx-eval.ts`
   - `src/rules/no-side-loader-imports.ts`
2. Add failing tests for the still-open bypass classes if they are in scope:
   - cross-module alias re-exports for fs/glob/`Function`
   - direct route-side `require("astro:content")`
   - cross-root content mirrors that bypass ESM `astro:content`
3. Fix the root utility logic instead of patching each rule separately if the failure is closure or alias-graph related.
4. Run adversarial review in parallel:
   - plugin/tests against the Astro family plan
   - Astro family config/filetree contracts against common-sense ownership and validator architecture
5. Re-run the plugin and Astro-family test suites and only then decide whether the slice is converged.

## Key decisions

- Keep source-policy in the ESLint plugin.
  - Reason: these are AST- and import-graph checks, not guardrail checks.
- Keep `ts/astro` focused on enforcing Astro-specific validator/setup contracts.
  - Reason: guardrails should enforce validator presence and wiring, not duplicate the validator logic.
- Fix shared alias/import-graph handling at the utility layer when possible.
  - Reason: the current open gaps are mostly the same class expressed through different rules.

## Files to modify

- `packages/ts/eslint-plugin-astro-pipeline/src/utils/ast-helpers.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/import-closure.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-fs-read.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-glob.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-direct-astro-content-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-runtime-mdx-eval.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-side-loader-imports.ts`
- `packages/ts/eslint-plugin-astro-pipeline/tests/*.test.ts`
- `packages/ts/astro/g3ts-astro-config-checks/**` only if attack review proves a live contract mismatch
- `packages/ts/astro/g3ts-astro-file-tree-checks/**` only if attack review proves a live contract mismatch
