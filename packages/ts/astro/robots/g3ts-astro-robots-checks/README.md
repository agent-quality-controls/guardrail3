# g3ts-astro-robots-checks

Post-build `robots.txt` artifact checker for strict G3TS Astro sites.

This package validates a generated `robots.txt` file. It does not generate files, inspect Astro source, inspect content collections, validate sitemap XML contents, validate llms files, or mutate output.

## Install

```sh
pnpm add -D g3ts-astro-robots-checks
```

## CLI

```sh
g3ts-astro-robots-checks \
  --output-dir dist \
  --site https://example.com \
  --sitemap https://example.com/sitemap-index.xml
```

Pass multiple approved sitemap URLs by repeating `--sitemap`.

## Library

```ts
import { validateRobotsTxt } from "g3ts-astro-robots-checks";

const result = await validateRobotsTxt({
  robotsFilePath: "dist/robots.txt",
  site: "https://example.com",
  approvedSitemapUrls: ["https://example.com/sitemap-index.xml"]
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
- No output mutation.
- No Astro source inspection.
- No content collection inspection.
- No sitemap XML validation.
- No llms validation.
