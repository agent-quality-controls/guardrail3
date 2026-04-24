# eslint-plugin-astro-pipeline

Astro source-policy rules for keeping route code on the approved content pipeline.

This package is for Astro apps that want lint failures when routes or endpoint closures:

- read authored content files directly
- glob authored content directly
- import `astro:content` directly from routes
- evaluate MDX at runtime
- pull side-loader helper modules into route closures

## Status

- Package surface is ready for npm publication.

## Compatibility

- Node: `>=20`
- ESLint: `^9`

## Install

```sh
pnpm add -D eslint-plugin-astro-pipeline
```

You still need the normal Astro lint stack in the app:

```sh
pnpm add -D eslint eslint-plugin-astro astro-eslint-parser eslint-mdx
```

This package only documents its own plugin surface. Keep your existing Astro parser and Astro ESLint setup, then add `astro-pipeline` on top.

## What it exports

- default ESLint plugin export
- `configs.recommended`
- 5 rules:
  - `astro-pipeline/no-authored-content-fs-read`
  - `astro-pipeline/no-authored-content-glob`
  - `astro-pipeline/no-direct-astro-content-in-routes`
  - `astro-pipeline/no-runtime-mdx-eval`
  - `astro-pipeline/no-side-loader-imports`

## Example config

```js
import astroPipeline from "eslint-plugin-astro-pipeline";

const astroPipelineOptions = {
  routeGlobs: ["src/pages/**/*.{astro,ts,tsx,js,jsx}"],
  endpointGlobs: ["src/pages/**/*.json.ts", "src/pages/**/*.xml.ts"],
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
      ]
    }
  }
];
```

## Rule intent

### `no-authored-content-fs-read`

Flags route or endpoint import closures that read authored or spec content with `fs`, `node:fs`, or `node:fs/promises`.

Use this to force content reads into loader or adapter modules.

### `no-authored-content-glob`

Flags route or endpoint import closures that discover authored or spec content with `import.meta.glob` or imported aliases of that glob surface.

Use this to force content discovery into loader or adapter modules.

### `no-direct-astro-content-in-routes`

Flags route or endpoint modules that import `astro:content` directly.

Use this to keep collection queries behind app-owned content adapters.

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

## Options

Every rule takes the same option object.

| Option | Meaning |
| --- | --- |
| `routeGlobs` | Files treated as route modules |
| `endpointGlobs` | Files treated as endpoint modules |
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
