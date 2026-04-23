## Summary

Closed the remaining real correctness gaps in the active `ts/astro` slice. The ESLint plugin now covers the missing Astro-content, fs, and runtime-MDX alias paths, and the Astro family no longer carries dead render-validator or cross-root filetree placeholder contracts.

## Decisions made

- Kept source-policy in `eslint-plugin-astro-pipeline`.
  - Reason: the remaining bypasses were all AST and import-closure problems, not guardrail problems.
- Removed dead `ts/astro` contract state instead of reviving placeholder rules.
  - Removed the unused render-validator package contract and the dead cross-root filetree placeholder because the current Astro slice does not enforce them.
- Removed parsed `astro.config.*` state from the active Astro family contract.
  - Reason: no live Astro-family rule consumes parsed config facts in the current slice, so carrying them through ingestion was dead baggage.
- Fixed alias-graph bugs at the shared helper layer when possible.
  - `createRequire(import.meta.url)`, `node:module` `require` destructuring, type-only closure edges, and re-exported default `fs` objects all belonged in shared rule utilities, not as one-off rule patches.

## Key files for context

- `.plans/2026-04-23-205111-ts-astro-plugin-convergence.md`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/ast-helpers.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/import-closure.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/module-exports.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-fs-read.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-direct-astro-content-in-routes.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-runtime-mdx-eval.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-side-loader-imports.ts`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`

## Next steps

- Wire the plugin into a real Astro app fixture once there is an app target worth validating end-to-end.
- Decide whether the standalone `astro-config-parser` package should stay parked for future Astro config rules or be removed until a live consumer exists.
- If the Astro family grows again, keep new config facts out of ingestion until a live rule actually consumes them.
