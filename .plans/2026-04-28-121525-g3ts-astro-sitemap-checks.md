# g3ts-astro-sitemap-checks

## Goal

Create the delegated post-build TypeScript package `g3ts-astro-sitemap-checks` for validating generated sitemap XML only.

## Approach

- Add package at `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks`.
- Use an explicit library API that accepts `site` plus either `outputDir` or `sitemapFiles`, with optional `indexFilename`.
- Use a real XML parser to parse sitemap XML.
- Follow sitemap indexes recursively and validate all discovered URL loc values.
- Add a CLI bin that maps explicit flags into the same library API and exits non-zero on findings.
- Add package-local Node test runner coverage for valid pass, malformed XML, sitemap index recursion, HTTPS host exactness, HTTP URLs, foreign hosts, bare/www mixing, duplicate loc values, and slash/no-slash pairs.
- Add README with install/run examples and non-responsibilities.

## Key Decisions

- Use a package-local npm setup matching the existing TypeScript checker package style instead of introducing repo-wide workspace config.
- Keep the package post-build only: no Astro source inspection, no collection checks, no generation, no mutation, no robots/llms validation.
- Treat `site` as the canonical exact HTTPS origin. Any loc must have protocol `https:` and the exact configured host.

## Files To Modify

- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks/package.json`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks/tsconfig.json`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks/tsconfig.build.json`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks/src/index.ts`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks/src/cli.ts`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks/tests/*.test.ts`
- `packages/ts/astro/sitemap/g3ts-astro-sitemap-checks/README.md`
