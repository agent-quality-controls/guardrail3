Goal
- Implement the delegated post-build npm package `g3ts-astro-robots-checks`.
- Validate only a generated `robots.txt` artifact from explicit config.

Approach
- Add package at `packages/ts/astro/robots/g3ts-astro-robots-checks`.
- Provide a TypeScript library API for validating a configured robots file path, canonical site, and approved sitemap URLs.
- Provide a CLI bin that accepts explicit `--robots`, `--site`, and one or more `--sitemap` options.
- Use `robots-parser` for robots syntax parsing and extract `Sitemap:` directives line-by-line only for sitemap comparison because the parser API does not expose sitemap directives.
- Add package-local tests for valid pass, missing file, parser failure, exact sitemap set, duplicate sitemap, HTTP sitemap, wrong host, and non-canonical bare/www variants.
- Add README covering install, run examples, and non-responsibilities.

Key Decisions
- Keep this package outside G3TS Rust checker packages because it is a delegated npm post-build artifact checker.
- Reject hidden source inference. The package reads only the configured robots file and config values.
- Compare sitemap URLs as an exact multiset-free set after URL parsing; duplicates fail before set comparison.
- Treat canonical host mismatch as a wrong-host error and identify bare/www counterpart mismatches as non-canonical variants.

Files To Modify
- `packages/ts/astro/robots/g3ts-astro-robots-checks/package.json`
- `packages/ts/astro/robots/g3ts-astro-robots-checks/tsconfig.json`
- `packages/ts/astro/robots/g3ts-astro-robots-checks/tsconfig.build.json`
- `packages/ts/astro/robots/g3ts-astro-robots-checks/src/index.ts`
- `packages/ts/astro/robots/g3ts-astro-robots-checks/src/cli.ts`
- `packages/ts/astro/robots/g3ts-astro-robots-checks/tests/*.test.ts`
- `packages/ts/astro/robots/g3ts-astro-robots-checks/README.md`
