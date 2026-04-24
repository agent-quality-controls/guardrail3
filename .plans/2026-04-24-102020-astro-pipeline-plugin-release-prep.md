# Goal
Make `packages/ts/eslint-plugin-astro-pipeline` release-ready for npm publication: correct metadata, safe publish scripts, and README documentation that explains installation, configuration, rules, and release workflow.

# Approach
1. Update `package.json` with release metadata and publish-time safety:
   - repository metadata
   - keywords
   - side-effects declaration
   - publish config
   - prepack / prepublishOnly scripts so the tarball cannot be published stale
2. Rewrite the README so an external Astro app can:
   - install the package
   - wire the plugin into ESLint
   - understand the rule surface
   - understand the required options
   - run the package locally
   - publish a new version
3. Verify the package with:
   - `npm test`
   - `npm pack --dry-run`
4. Run an adversarial review against the package surface and README to catch missing publish/setup details.

# Key Decisions
- Keep the package unlicensed until the user provides the intended license.
  - Do not invent a license.
  - If needed, mark the package metadata accordingly so publish status is explicit.
- Prefer `prepack` and `prepublishOnly` over relying on manual build discipline.
- Document one supported install path first:
  - published npm package
  - not workspace-local setup as the primary story

# Files To Modify
- `packages/ts/eslint-plugin-astro-pipeline/package.json`
- `packages/ts/eslint-plugin-astro-pipeline/README.md`
