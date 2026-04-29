# g3ts-astro-robots-auditor

Astro integration that audits generated `robots.txt` after `astro build`.

This package validates a generated `robots.txt` file. It does not generate files, inspect Astro source, inspect content collections, validate sitemap XML contents, validate llms files, or mutate output.

## Install

```sh
pnpm add -D g3ts-astro-robots-auditor
```

## Astro Usage

```ts
import { defineConfig } from "astro/config";
import robotsTxt from "astro-robots";
import g3tsRobotsAuditor from "g3ts-astro-robots-auditor";

export default defineConfig({
  site: "https://example.com",
  integrations: [
    robotsTxt(),
    g3tsRobotsAuditor({
      site: "https://example.com",
      sitemapUrls: ["https://example.com/sitemap-index.xml"]
    })
  ]
});
```

## Library

```ts
import { validateRobotsTxt } from "g3ts-astro-robots-auditor";

const result = await validateRobotsTxt({
  outputDir: "dist",
  site: "https://example.com",
  sitemapUrls: ["https://example.com/sitemap-index.xml"]
});

if (!result.ok) {
  throw new Error(result.issues.map((issue) => issue.message).join("\n"));
}
```

## Checks

- `robots.txt` exists.
- `robots.txt` parses with `robots-parser`.
- `Sitemap:` directives exactly match the approved URL list.
- Sitemap URLs must use HTTPS.
- Sitemap URL hosts must exactly match the canonical `site` host.
- Sitemap URLs must not use the non-canonical bare or `www` variant.

## Non-Responsibilities

- No robots generation.
- No CLI.
- No output mutation.
- No Astro source inspection.
- No content collection inspection.
- No sitemap XML validation.
- No llms validation.
