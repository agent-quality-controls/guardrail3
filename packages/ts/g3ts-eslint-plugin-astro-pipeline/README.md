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
- 16 custom rules:
- `astro-pipeline/mdx-component-imports-from-approved-map`
- `astro-pipeline/mdx-component-map-no-raw-ui-exports`
- `astro-pipeline/mdx-component-wrapper-requires-zod-parse`
- `astro-pipeline/mdx-imports-only-approved-components`
- `astro-pipeline/no-authored-content-fs-read`
- `astro-pipeline/no-authored-content-glob`
- `astro-pipeline/no-authored-content-imports`
- `astro-pipeline/no-content-data-modules-in-routes`
- `astro-pipeline/no-direct-astro-content-in-routes`
- `astro-pipeline/no-raw-mdx-images`
- `astro-pipeline/no-runtime-mdx-eval`
- `astro-pipeline/no-side-loader-imports`
- `astro-pipeline/no-velite-imports`
- `astro-pipeline/require-approved-content-adapter-in-routes`
- `astro-pipeline/require-approved-json-ld-helper-in-routes`
- `astro-pipeline/require-approved-metadata-helper-in-routes`

## Example

```js
import astroPipeline from "g3ts-eslint-plugin-astro-pipeline";

const astroPipelineOptions = {
  routeGlobs: ["src/pages/**/*.{astro,md,mdx,html}"],
  endpointGlobs: ["src/pages/**/*.{ts,js}"],
  contentDataModuleGlobs: ["src/**/*.data.{ts,tsx,js,jsx,mts,cts,mjs,cjs}"],
  approvedContentAdapterModules: ["src/content/landing-homepage.ts"],
  approvedJsonLdHelperModules: ["src/seo/json-ld.ts"],
  approvedMetadataHelperModules: ["src/seo/metadata.ts"],
  mdxContentGlobs: ["content/**/*.{md,mdx}"],
  approvedMdxComponentModules: ["src/mdx-components.tsx"],
  approvedMdxComponentNames: ["Callout", "FAQ", "ArticleImage"],
  allowedMdxComponentMapExports: ["mdxComponents"],
  approvedMdxImageComponents: ["ArticleImage"],
  mdxPropsParserName: ["parseMdxComponentProps"],
  rawUiModuleGlobs: ["@project/ui"],
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

`mdx-component-imports-from-approved-map` requires MDX component imports to come from approved component-map modules only.

`mdx-component-map-no-raw-ui-exports` blocks approved component-map modules from directly re-exporting raw UI components.

`mdx-component-wrapper-requires-zod-parse` requires every approved MDX component-map export to validate raw props through a local Zod schema and configured parser helper before rendering UI.

`mdx-imports-only-approved-components` requires MDX files to import only explicitly approved component names from approved component-map modules.

`no-authored-content-fs-read` blocks route and endpoint import closures from reading authored content files with `fs`.

`no-authored-content-glob` blocks route and endpoint import closures from discovering authored content with `import.meta.glob`.

`no-authored-content-imports` blocks route and endpoint import closures from importing authored content modules directly.

`no-content-data-modules-in-routes` blocks route and endpoint import closures from reaching ad hoc page-copy data modules.

`no-direct-astro-content-in-routes` blocks route and endpoint modules from importing `astro:content` directly.

`no-raw-mdx-images` blocks Markdown image syntax and raw `<img>` elements in MDX content so images use configured validated wrappers.

`no-runtime-mdx-eval` blocks runtime MDX bridges such as `new Function`, `@mdx-js/mdx` `evaluate`, and `@mdx-js/mdx` `run`.

`no-side-loader-imports` blocks routes from smuggling content access through unapproved helper modules.

`no-velite-imports` blocks route and endpoint import closures from reaching Velite packages, Velite config, or `.velite` outputs.

`require-approved-content-adapter-in-routes` requires public page routes to import an approved content adapter module.

`require-approved-json-ld-helper-in-routes` requires public page routes to render JSON-LD through an approved helper module.

`require-approved-metadata-helper-in-routes` requires public page routes to derive metadata through an approved helper module.

## Publish

The repo keeps the npm token in root `.env.local` as `NPM_TOKEN`. npm does not read that file by itself, so publish with a temporary user config:

```sh
set -a
source /Users/tartakovsky/Projects/websmasher/guardrail3/.env.local
set +a
TMP_NPMRC=$(mktemp)
printf '%s\n' 'registry=https://registry.npmjs.org/' "//registry.npmjs.org/:_authToken=${NPM_TOKEN}" > "$TMP_NPMRC"
npm --userconfig "$TMP_NPMRC" publish --access public
STATUS=$?
rm -f "$TMP_NPMRC"
exit $STATUS
```
