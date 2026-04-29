# g3ts-astro-llms-auditor

Astro integration that audits generated `llms.txt` after `astro build`.

This package validates a generated `llms.txt` artifact. It does not generate files.

## Install

```sh
pnpm add -D g3ts-astro-llms-auditor
```

## Astro Usage

```ts
import { defineConfig } from "astro/config";
import g3tsLlmsAuditor from "g3ts-astro-llms-auditor";
import g3tsLlmsGenerator from "g3ts-astro-llms-generator";

export default defineConfig({
  site: "https://example.com",
  integrations: [
    g3tsLlmsGenerator({
      title: "Example",
      site: "https://example.com",
      sections: [
        {
          heading: "Docs",
          links: [{ title: "Docs", href: "/docs/" }]
        }
      ]
    }),
    g3tsLlmsAuditor({
      site: "https://example.com",
      requiredSections: ["Docs"],
      requiredRoutePatterns: ["/docs/"],
      allowedExternalUrls: [],
      allowedNonPageUrls: [],
      ignoredHtmlFiles: []
    })
  ]
});
```

## Library

```ts
import { checkLlmsTxt } from "g3ts-astro-llms-auditor";

const result = await checkLlmsTxt({
  outputDir: "dist",
  site: "https://example.com",
  requiredSections: ["Docs", "Policies"],
  requiredRoutePatterns: ["/docs/**", "/policies/**"],
  allowedExternalUrls: [],
  allowedNonPageUrls: [],
  ignoredHtmlFiles: []
});

if (!result.ok) {
  process.exitCode = 1;
}
```

## Checks

- the configured file exists when the auditor runs
- the file parses as `llms.txt` with `parse-llms-txt`
- every configured required section exists
- every configured required route pattern is represented by a link
- external links are explicitly allowed
- internal links map to built HTML pages unless explicitly allowed

## Non-responsibilities

- does not generate `llms.txt`
- does not expose a CLI
- does not mutate build output
- does not inspect Astro source files
- does not inspect Astro content collections
- does not validate sitemap XML
- does not validate `robots.txt`
- does not infer required sections or links from hidden source conventions
