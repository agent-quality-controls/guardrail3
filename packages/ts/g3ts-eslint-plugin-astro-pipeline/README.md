# g3ts-eslint-plugin-astro-pipeline

G3TS-owned ESLint rules for Astro content-pipeline enforcement.

The npm package name is `g3ts-eslint-plugin-astro-pipeline`. Apps register it in ESLint under namespace `astro-pipeline`, so rule IDs stay `astro-pipeline/*`.

## Install

```sh
pnpm add -D g3ts-eslint-plugin-astro-pipeline
```

The app owns third-party delegated plugins directly. This package does not depend on or configure `eslint-plugin-i18next` or `eslint-plugin-mdx`.

## Exports

- default ESLint plugin export
- `configs.recommended`
- 9 custom rules:
- `astro-pipeline/no-authored-content-fs-read`
- `astro-pipeline/no-authored-content-glob`
- `astro-pipeline/no-authored-content-imports`
- `astro-pipeline/no-content-data-modules-in-routes`
- `astro-pipeline/no-direct-astro-content-in-routes`
- `astro-pipeline/no-runtime-mdx-eval`
- `astro-pipeline/no-side-loader-imports`
- `astro-pipeline/no-velite-imports`
- `astro-pipeline/require-approved-content-adapter-in-routes`

## Example

```js
import astroPipeline from "g3ts-eslint-plugin-astro-pipeline";

const astroPipelineOptions = {
  routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"],
  endpointGlobs: ["src/pages/**/*.{ts,js}"],
  contentDataModuleGlobs: ["src/**/*.data.{ts,tsx,js,jsx,mts,cts,mjs,cjs}"],
  approvedContentAdapterModules: ["src/content/landing-homepage.ts"],
  authoredContentGlobs: ["src/content/**"],
  specContentGlobs: ["specs/**"]
};

export default [
  {
    files: ["src/**/*.{astro,ts,tsx,js,jsx,mjs,cjs,mts,cts}"],
    plugins: {
      "astro-pipeline": astroPipeline
    },
    rules: Object.fromEntries(
      Object.keys(astroPipeline.rules).map((ruleName) => [
        `astro-pipeline/${ruleName}`,
        ["error", astroPipelineOptions]
      ])
    )
  }
];
```

## Rule Intent

`no-authored-content-fs-read` blocks route and endpoint import closures from reading authored content files with `fs`.

`no-authored-content-glob` blocks route and endpoint import closures from discovering authored content with `import.meta.glob`.

`no-authored-content-imports` blocks route and endpoint import closures from importing authored content modules directly.

`no-content-data-modules-in-routes` blocks route and endpoint import closures from reaching ad hoc page-copy data modules.

`no-direct-astro-content-in-routes` blocks route and endpoint modules from importing `astro:content` directly.

`no-runtime-mdx-eval` blocks runtime MDX bridges such as `new Function`, `@mdx-js/mdx` `evaluate`, and `@mdx-js/mdx` `run`.

`no-side-loader-imports` blocks routes from smuggling content access through unapproved helper modules.

`no-velite-imports` blocks route and endpoint import closures from reaching Velite packages, Velite config, or `.velite` outputs.

`require-approved-content-adapter-in-routes` requires public page routes to import an approved content adapter module.
