# g3ts-astro-nuasite-checks

G3TS-owned custom checks for `@nuasite/checks` in Astro content sites.

This package is not an ESLint plugin and not an Astro integration. Apps pass its exports into `@nuasite/checks`.

## Install

```sh
pnpm add -D g3ts-astro-nuasite-checks
```

## Usage

```ts
import checks from "@nuasite/checks";
import { structuredDataPresentCheck } from "g3ts-astro-nuasite-checks";

export default {
  integrations: [
    checks({
      mode: "full",
      failOnError: true,
      failOnWarning: true,
      reportJson: true,
      ai: false,
      customChecks: [structuredDataPresentCheck]
    })
  ]
};
```

## Exports

- `structuredDataPresentCheck`: fails a rendered page when `ctx.pageData.jsonLd.length === 0`.
