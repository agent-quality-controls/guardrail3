# eslint-plugin-astro-pipeline

Astro source-policy rules for keeping route code on the approved content pipeline.

This package is for Astro apps that want lint failures when routes or endpoint closures:

- read authored content files directly
- glob authored content directly
- import authored content files directly
- source page copy from ad hoc data modules
- import `astro:content` directly from routes
- evaluate MDX at runtime
- pull side-loader helper modules into route closures
- pull Velite package or `.velite` outputs into route closures
- render authored public copy from source literals instead of Astro content entries

## Status

- Package is published on npm.

## Compatibility

- Node: `>=20`
- ESLint: `^9`
- TypeScript: `>=4.8.4 <6.1.0`

## Install

```sh
pnpm add -D eslint-plugin-astro-pipeline
```

You still need the normal Astro lint stack in the app:

```sh
pnpm add -D eslint eslint-plugin-astro astro-eslint-parser typescript
```

This package only documents its own plugin surface. Keep your existing Astro parser and Astro ESLint setup, then add `astro-pipeline` on top.

`eslint-plugin-astro-pipeline` depends on `eslint-plugin-i18next` and `eslint-mdx`. Do not install those directly in Astro apps. The custom Astro plugin owns the content-pipeline policy and config; maintained parser/lint packages own source and MDX AST handling under that plugin boundary.

## What it exports

- default ESLint plugin export
- `configs.recommended`
- `configs["strict-content"]`
- 8 rules:
  - `astro-pipeline/no-authored-content-fs-read`
  - `astro-pipeline/no-authored-content-glob`
  - `astro-pipeline/no-authored-content-imports`
  - `astro-pipeline/no-content-data-modules-in-routes`
  - `astro-pipeline/no-direct-astro-content-in-routes`
  - `astro-pipeline/no-runtime-mdx-eval`
  - `astro-pipeline/no-side-loader-imports`
  - `astro-pipeline/no-velite-imports`
- delegated public-copy rule in the strict content config:
  - `i18next/no-literal-string`

## Example config

```js
import astroPipeline from "eslint-plugin-astro-pipeline";

const astroPipelineOptions = {
  routeGlobs: ["src/pages/**/*.{astro,ts,tsx,js,jsx}"],
  endpointGlobs: ["src/pages/**/*.json.ts", "src/pages/**/*.xml.ts"],
  contentDataModuleGlobs: [
    "src/**/*.data.{ts,tsx,js,jsx,mts,cts,mjs,cjs}"
  ],
  adapterModuleGlobs: ["src/lib/content/adapters/**/*.{ts,tsx,js,jsx}"],
  mdxRuntimeModuleGlobs: ["src/lib/mdx/**/*.{ts,tsx,js,jsx}"],
  routeRegistryModuleGlobs: [],
  approvedContentAdapterModules: [
    "src/lib/content/adapters/**/*.{ts,tsx,js,jsx}"
  ],
  approvedLoaderModules: ["src/lib/content/loaders/**/*.{ts,tsx,js,jsx}"],
  approvedMdxComponentModules: [],
  approvedGeneratedArtifactRoots: ["src/generated/**"],
  authoredContentGlobs: ["src/content/**"],
  specContentGlobs: ["specs/**"]
};

export default [
  {
    files: ["src/**/*.{astro,ts,tsx,js,jsx,mjs,cjs,mts,cts,mdx}"],
    plugins: {
      "astro-pipeline": astroPipeline
    },
    rules: {
      ...astroPipeline.configs.recommended.rules,
      "astro-pipeline/no-authored-content-fs-read": [
        "error",
        astroPipelineOptions
      ],
      "astro-pipeline/no-authored-content-glob": [
        "error",
        astroPipelineOptions
      ],
      "astro-pipeline/no-authored-content-imports": [
        "error",
        astroPipelineOptions
      ],
      "astro-pipeline/no-content-data-modules-in-routes": [
        "error",
        astroPipelineOptions
      ],
      "astro-pipeline/no-direct-astro-content-in-routes": [
        "error",
        astroPipelineOptions
      ],
      "astro-pipeline/no-runtime-mdx-eval": [
        "error",
        astroPipelineOptions
      ],
      "astro-pipeline/no-side-loader-imports": [
        "error",
        astroPipelineOptions
      ],
      "astro-pipeline/no-velite-imports": [
        "error",
        astroPipelineOptions
      ]
    }
  }
];
```

## Strict content config

Use `configs["strict-content"]` on public Astro source lanes to reject hardcoded page copy in routes, UI, and source data objects:

```js
import astroPipeline from "eslint-plugin-astro-pipeline";

export default [
  {
    files: ["src/**/*.{astro,ts,tsx,js,jsx,mjs,cjs,mts,cts}"],
    plugins: {
      "astro-pipeline": astroPipeline
    },
    rules: {
      ...astroPipeline.configs.recommended.rules
    }
  },
  {
    files: [
      "src/pages/**/*.{astro,ts,tsx,js,jsx,mjs,cjs,mts,cts}",
      "src/ui/**/*.{astro,ts,tsx,js,jsx,mjs,cjs,mts,cts}",
      "src/components/**/*.{astro,ts,tsx,js,jsx,mjs,cjs,mts,cts}",
      "src/content/**/*.{ts,tsx,js,jsx,mjs,cjs,mts,cts}"
    ],
    ...astroPipeline.configs["strict-content"]
  }
];
```

The strict content config delegates to `i18next/no-literal-string` with `mode: "all"` so both JSX/Astro text and source object literals are checked. It allows structural strings such as classes, IDs, URLs, asset paths, import paths, TS literal types, enum-like uppercase tokens, and decorative `alt=""`.

It intentionally does not allow public-copy attributes such as `alt`, `aria-label`, `title`, or `placeholder`. If those contain words, they are user-facing content and should come from an Astro content entry.

## Rule intent

### `no-authored-content-fs-read`

Flags route or endpoint import closures that read authored or spec content with `fs`, `node:fs`, or `node:fs/promises`.

Use this to force content reads into loader or adapter modules.

### `no-authored-content-glob`

Flags route or endpoint import closures that discover authored or spec content with `import.meta.glob` or imported aliases of that glob surface.

Use this to force content discovery into loader or adapter modules.

### `no-authored-content-imports`

Flags route or endpoint import closures that import authored or spec content modules directly.

Use this to keep public routes off raw JSON, Markdown, MDX, and similar content-file imports, and on loader or adapter surfaces instead.

### `no-direct-astro-content-in-routes`

Flags route or endpoint modules that import `astro:content` directly.

Use this to keep collection queries behind app-owned content adapters.

### `no-content-data-modules-in-routes`

Flags route or endpoint import closures that reach configured page-copy data modules such as `homepage-v2.data.ts`.

Use this to keep landing and public page copy out of ad hoc `*.data.*` modules and on the typed content pipeline.

### `no-runtime-mdx-eval`

Flags runtime MDX bridges such as:

- `new Function(...)`
- aliased `Function(...)`
- `@mdx-js/mdx` `evaluate`
- `@mdx-js/mdx` `run`

Use this to force MDX through a generated or precompiled artifact path.

### `no-side-loader-imports`

Flags one-hop route helper imports that pull unapproved content-loading helpers into a route closure.

Use this to stop routes from smuggling content access through cross-root helpers or helper modules that import `astro:content`.

### `no-velite-imports`

Flags route or endpoint import closures that reach the `velite` package, `velite.config.*`, or `.velite` generated artifacts.

Use this to keep Astro apps off parallel Velite content pipelines.

### `i18next/no-literal-string` through `configs["strict-content"]`

Flags hardcoded authored copy in public source files:

- JSX and Astro text nodes
- string literals in public-copy attributes such as `alt`, `aria-label`, `title`, and `placeholder`
- object or array literals used as source data
- frontmatter or module-scope string literals that would bypass Astro content collections

Use this to keep landing pages, blog shells, and public UI rendering from typed Astro content entries instead of from source literals.

## Options

Every rule takes the same option object.

| Option | Meaning |
| --- | --- |
| `routeGlobs` | Files treated as route modules |
| `endpointGlobs` | Files treated as endpoint modules |
| `contentDataModuleGlobs` | Data-module globs that must not appear in route import closures |
| `adapterModuleGlobs` | General adapter modules for Astro route consumption |
| `mdxRuntimeModuleGlobs` | Allowed MDX runtime helper modules |
| `routeRegistryModuleGlobs` | Route registry modules that should not count as side-loaders |
| `approvedContentAdapterModules` | Exact allowed modules for route-side content adapter imports |
| `approvedLoaderModules` | Exact allowed modules for raw content loading |
| `approvedMdxComponentModules` | Allowed MDX component import surfaces |
| `approvedGeneratedArtifactRoots` | Generated-code roots allowed to bypass raw-content bans |
| `authoredContentGlobs` | Authored content roots such as `src/content/**` |
| `specContentGlobs` | Spec or content-like roots such as `specs/**` |

## Local development

```sh
npm install
npm test
```

`npm test` rebuilds `dist/**` before running the test suite.

## Release workflow

1. Update the version in `package.json`.
2. Run `npm test`.
3. Run `npm pack --dry-run`.
4. Publish:

```sh
npm publish --access public
```

`prepack` rebuilds `dist/**` and `prepublishOnly` reruns the test suite, so the package should fail closed if the release artifact is stale or broken.
