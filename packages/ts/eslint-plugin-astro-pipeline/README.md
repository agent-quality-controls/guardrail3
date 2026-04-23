# eslint-plugin-astro-pipeline

Repo-owned Astro source-policy rules for guarding the approved content pipeline.

## Implemented rules

- `astro-pipeline/no-authored-content-fs-read`
- `astro-pipeline/no-authored-content-glob`
- `astro-pipeline/no-direct-astro-content-in-routes`
- `astro-pipeline/no-runtime-mdx-eval`

## Usage

```js
import astroPipeline from "eslint-plugin-astro-pipeline";

export default [
  {
    plugins: {
      "astro-pipeline": astroPipeline
    },
    rules: {
      ...astroPipeline.configs.recommended.rules,
      "astro-pipeline/no-authored-content-fs-read": [
        "error",
        {
          routeGlobs: ["src/pages/**/*.{ts,tsx,js,jsx,astro}"],
          endpointGlobs: ["src/pages/**/*.json.ts"],
          adapterModuleGlobs: ["src/lib/content/**/*.{ts,tsx,js,jsx}"],
          mdxRuntimeModuleGlobs: ["src/lib/mdx/**/*.{ts,tsx,js,jsx}"],
          routeRegistryModuleGlobs: [],
          approvedContentAdapterModules: ["src/lib/content/**/*.{ts,tsx,js,jsx}"],
          approvedLoaderModules: ["src/lib/content/**/*.{ts,tsx,js,jsx}"],
          approvedMdxComponentModules: [],
          approvedGeneratedArtifactRoots: ["src/generated/**"],
          authoredContentGlobs: ["src/content/**"],
          specContentGlobs: ["specs/**"]
        }
      ]
    }
  }
];
```

## Scripts

- `npm run build`
- `npm test`
