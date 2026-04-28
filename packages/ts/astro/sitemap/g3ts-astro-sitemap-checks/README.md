# g3ts-astro-sitemap-checks

Post-build sitemap XML checks for Astro output.

This package validates generated sitemap XML. It is not an Astro integration, not an ESLint plugin, and not a sitemap generator.

## Install

```sh
npm add -D g3ts-astro-sitemap-checks
```

## CLI Usage

Validate the default sitemap index under an explicit output directory:

```sh
g3ts-astro-sitemap-checks --site https://example.com --output-dir dist
```

Use a custom sitemap index filename:

```sh
g3ts-astro-sitemap-checks --site https://example.com --output-dir dist --index-filename sitemap.xml
```

## Library Usage

```ts
import { checkSitemap } from "g3ts-astro-sitemap-checks";

const result = await checkSitemap({
  site: "https://example.com",
  outputDir: "dist"
});

if (!result.ok) {
  console.error(result.findings);
  process.exitCode = 1;
}
```

## Checks

- XML parses with a real XML parser.
- Sitemap index recursion is followed.
- Every `loc` uses the configured HTTPS host exactly.
- No `loc` uses `http`.
- No `loc` uses a foreign host.
- Bare and `www` host variants are not mixed.
- Duplicate `loc` values are rejected.
- Slash/no-slash pairs for the same path are rejected.

## Non-Responsibilities

- Does not infer routes from source files.
- Does not inspect Astro collections.
- Does not generate sitemap XML.
- Does not mutate final output.
- Does not validate page links.
- Does not validate `robots.txt` or `llms.txt`.
