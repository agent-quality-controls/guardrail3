# g3ts-astro-llms-checks

Post-build `llms.txt` checks for strict Astro content sites.

This package is a CLI and library for validating a generated `llms.txt` artifact. It is not an Astro integration and it does not generate files.

## Install

```sh
pnpm add -D g3ts-astro-llms-checks
```

## CLI

```sh
g3ts-astro-llms-checks \
  --output-dir dist \
  --required-section "Docs" \
  --required-section "Policies" \
  --required-link "https://example.com/docs/" \
  --required-link "https://example.com/policies/privacy/"
```

The CLI prints JSON. It exits `0` when every check passes and `1` when any finding is emitted.

## Library

```ts
import { checkLlmsTxt } from "g3ts-astro-llms-checks";

const result = await checkLlmsTxt({
  llmsPath: "dist/llms.txt",
  requiredSections: ["Docs", "Policies"],
  requiredLinks: [
    "https://example.com/docs/",
    "https://example.com/policies/privacy/"
  ]
});

if (!result.ok) {
  process.exitCode = 1;
}
```

## Checks

- the configured file exists when the checker runs
- the file parses as `llms.txt` with `parse-llms-txt`
- every configured required section exists
- every configured required link URL is present in any section

## Non-responsibilities

- does not generate `llms.txt`
- does not mutate build output
- does not inspect Astro source files
- does not inspect Astro content collections
- does not validate sitemap XML
- does not validate `robots.txt`
- does not infer required sections or links from hidden source conventions
