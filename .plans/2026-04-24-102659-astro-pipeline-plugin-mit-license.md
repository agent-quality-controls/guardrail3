# Goal
Finalize the `eslint-plugin-astro-pipeline` package license as MIT so the package can be published with correct legal metadata.

# Approach
1. Update the package metadata from `UNLICENSED` to `MIT`.
2. Add an MIT `LICENSE` file in the package root so the published tarball carries the full license text.
3. Remove the outdated README note that says the license is still pending.
4. Verify with `npm pack --dry-run` so the tarball includes the license file and updated metadata.

# Key Decisions
- Put the `LICENSE` file in the package root.
  - Reason: npm includes standard license files in the published tarball.
- Keep scope limited to the plugin package.
  - No repo-wide license changes.

# Files To Modify
- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`
- `packages/ts/eslint-plugin-astro-pipeline/LICENSE`
