# g3ts-astro-llms-generator

Narrow Astro integration that writes `llms.txt` during `astro build`.

This package is not a checker. It only generates one file from explicit config.

## Install

```sh
pnpm add -D g3ts-astro-llms-generator zod
```

## Usage

```ts
import { defineConfig } from "astro/config";
import g3tsLlmsGenerator from "g3ts-astro-llms-generator";

export default defineConfig({
  integrations: [
    g3tsLlmsGenerator({
      title: "Site title",
      site: "https://example.com",
      sections: [
        {
          heading: "Docs",
          links: [{ title: "Home", href: "/" }]
        }
      ]
    })
  ]
});
```

## Output

The integration writes `llms.txt` to Astro's build output directory.

```txt
# Site title

> https://example.com/

## Docs

- [Home](https://example.com/)
```

Sections and links are sorted deterministically. Relative links are resolved against `site`. Hash fragments are removed from generated links.

## Non-Responsibilities

- Does not generate sitemap XML.
- Does not generate `robots.txt`.
- Does not inject JSON-LD.
- Does not patch or generate headers.
- Does not validate generated output.
- Does not read content adapters or infer content from folder names.
