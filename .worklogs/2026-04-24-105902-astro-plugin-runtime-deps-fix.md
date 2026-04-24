## Summary

Fixed the published `eslint-plugin-astro-pipeline` package surface so runtime imports are actually declared for consumers. Added a release-surface test that checks built `dist/**` imports and their non-optional runtime peer dependencies against the package manifest before publish, then published the fixed package as `0.1.1`.

## Decisions made

- Fixed the npm package contract instead of changing `g3ts`.
  - Reason: the install/load failure was a published package bug, not a guardrail-detection bug.
- Declared `@typescript-eslint/parser`, `astro-eslint-parser`, and `eslint-mdx` as package dependencies.
  - Reason: built runtime code imports them directly.
- Declared `typescript` as a peer dependency.
  - Reason: the runtime parser stack requires it transitively via `@typescript-eslint/parser`, and consumer apps should provide the TypeScript version used for parsing their source.
- Added a release-surface test over built output plus runtime peers.
  - Reason: the first version of the test only caught direct imports and missed the transitive `typescript` runtime requirement.
- Did not change `ts/astro` for `TS-ASTRO-CONFIG-05` / `TS-ASTRO-CONFIG-07`.
  - Reason: the current real landing app at `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing` does not reproduce those findings.
  - Current state of that app:
    - `package.json` shows a Next app with `next`, not `astro`
    - no `astro.config.*`
    - `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory` returns `No findings.`
  - Conclusion: the pasted `CONFIG-05/07` report was stale state, different target, or different branch, not a current repro in this app.

## Key files for context

- `.plans/2026-04-24-104927-astro-plugin-runtime-deps-fix.md`
- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/package-lock.json`
- `packages/ts/eslint-plugin-astro-pipeline/tests/package-runtime-dependencies.test.ts`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`

## Verification

- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
  - result: `No findings.`
- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `npm pack --dry-run` in `packages/ts/eslint-plugin-astro-pipeline`
- packed tarball smoke test in a temp project:
  - install tarball plus `eslint` and `typescript`
  - `node --input-type=module -e 'await import("eslint-plugin-astro-pipeline")'`
  - result: plugin loaded and exported all 5 rules
- published release:
  - `npm publish --access public /Users/tartakovsky/Projects/websmasher/guardrail3/packages/ts/eslint-plugin-astro-pipeline`
  - result: `+ eslint-plugin-astro-pipeline@0.1.1`
- registry verification:
  - `npm view eslint-plugin-astro-pipeline name version`
  - result:
    - `name = 'eslint-plugin-astro-pipeline'`
    - `version = '0.1.1'`
- install-from-registry smoke test in a temp project:
  - `npm add eslint-plugin-astro-pipeline@0.1.1 eslint@^9 typescript@^6`
  - `node --input-type=module -e 'await import("eslint-plugin-astro-pipeline")'`
  - result: plugin loaded and exported all 5 rules

## Next steps

- If a real `TS-ASTRO-CONFIG-05` / `07` repro appears, capture the exact path and branch first, then investigate that state separately.
