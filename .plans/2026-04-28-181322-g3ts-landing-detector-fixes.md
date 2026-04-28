# Goal

Fix G3TS false negatives against the `landing-seo-artifact-guardrails` app branch:

- effective ESLint config has eslint-comments rules active, but G3TS misses `require-description` and some restricted-disable contracts
- `validate` script runs `astro build` before artifact checks, but unrelated shell syntax in `start` makes G3TS lose script facts

# Approach

- Add parser/check tests that reproduce the exact effective-config shapes from `eslint --print-config`.
- Fix ESLint suppression ingestion so plugin identity and rule option extraction match ESLint flat-config output, not only idealized internal test fixtures.
- Add parser/check tests for package scripts containing `${PORT:-3001}` in an unrelated `start` script.
- Fix package-script ingestion so one script parse blocker does not erase facts from other scripts.
- Reinstall local G3TS and verify the landing branch no longer reports the false positives.

# Files to modify

- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/eslint.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/eslint_suppression.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/package.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion/src/package.rs`
- relevant sidecar tests under the touched packages

# Decisions

- Do not weaken rules. The app must still fail when delegated enforcement is actually missing.
- Do not parse ESLint source config. Use the existing effective-config surface and make extraction match real ESLint output.
- Do not treat `start` script parse blockers as blockers for `validate`, `lint`, `lint:packages`, `typecheck`, or `build` facts.
