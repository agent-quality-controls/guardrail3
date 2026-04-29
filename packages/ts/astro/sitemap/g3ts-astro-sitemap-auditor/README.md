# g3ts-astro-sitemap-auditor

Astro integration that audits generated sitemap XML after `astro build`.

This package validates generated sitemap XML from Astro output. It is not an ESLint plugin and not a sitemap generator.

## Install

```sh
npm add -D g3ts-astro-sitemap-auditor
```

## Astro Usage

```ts
import { defineConfig } from "astro/config";
import sitemap from "@astrojs/sitemap";
import g3tsSitemapAuditor from "g3ts-astro-sitemap-auditor";

export default defineConfig({
  site: "https://example.com",
  integrations: [
    sitemap(),
    g3tsSitemapAuditor({
      site: "https://example.com",
      trailingSlash: "always"
    })
  ]
});
```

## Library Usage

```ts
import { checkSitemap } from "g3ts-astro-sitemap-auditor";

const result = await checkSitemap({
  site: "https://example.com",
  trailingSlash: "always",
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
- Sitemap URLs follow the configured trailing-slash policy.
- Built HTML pages are present in the sitemap unless explicitly allowed.
- Sitemap page URLs map to built HTML pages unless explicitly allowed.

## Non-Responsibilities

- Does not infer routes from source files.
- Does not inspect Astro collections.
- Does not generate sitemap XML.
- Does not expose a CLI.
- Does not mutate final output.
- Does not validate page links.
- Does not validate `robots.txt` or `llms.txt`.
