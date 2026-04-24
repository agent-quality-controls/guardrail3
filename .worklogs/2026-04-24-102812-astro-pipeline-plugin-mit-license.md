## Summary

Switched `eslint-plugin-astro-pipeline` from temporary `UNLICENSED` metadata to MIT and added the package-local MIT license text. The npm tarball now includes the `LICENSE` file and correct license metadata.

## Decisions made

- Put the MIT `LICENSE` file in the package root.
  - Reason: npm includes standard package-root license files in the published tarball.
- Kept the scope local to the plugin package.
  - No repo-wide license changes.
- Removed the stale README note that said the license was still pending.

## Key files for context

- `.plans/2026-04-24-102659-astro-pipeline-plugin-mit-license.md`
- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/LICENSE`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`

## Verification

- `npm pack --dry-run`
  - confirmed:
    - `license: MIT` in `package.json`
    - `LICENSE` included in the tarball

## Next steps

- The package is now legally and mechanically ready for a real npm publish.
- If you want, the next exact step is publishing `eslint-plugin-astro-pipeline@0.1.0`.
