## Summary

Prepared `eslint-plugin-astro-pipeline` for npm publication. The package now has release metadata, publish-time safety scripts, and a README that documents install, configuration, rule intent, and release workflow. The package also passes local test, pack, and publish dry-runs.

## Decisions made

- Kept the package `UNLICENSED` for now.
  - Reason: the final license has not been provided yet, and inventing one would be wrong.
  - This keeps the package explicit about its current legal state while leaving the publish workflow otherwise ready.

- Added `prepack` and `prepublishOnly`.
  - Reason: release safety should be automatic.
  - `prepack` now rebuilds `dist/**`.
  - `prepublishOnly` reruns the test suite before publish.

- Rewrote the README around the external consumer story.
  - Reason: the prior README was still internal and incomplete.
  - The new README explains:
    - install
    - exports
    - example ESLint wiring
    - rule intent
    - option meanings
    - local development
    - release workflow

- Kept README examples limited to this package's verified surface.
  - Reason: avoid speculating about third-party plugin config APIs that were not read in this session.

## Key files for context

- `.plans/2026-04-24-102020-astro-pipeline-plugin-release-prep.md`
- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`

## Verification

- `npm test`
- `npm pack --dry-run`
- `npm publish --dry-run --access public`

## Next steps

- Replace `UNLICENSED` with the intended license before the real publish.
- If you want a cleaner consumer install story, publish `0.1.0` after the license change and then update `ts/astro` docs and real-app setup to use the published package instead of local path or workspace installs.
