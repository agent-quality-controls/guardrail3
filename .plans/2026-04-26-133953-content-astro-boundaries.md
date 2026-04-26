# Goal

Define the split between Astro-specific content guardrails and framework-independent content guardrails for agent-managed landing, blog, docs, and MDX sites.

# Core Boundary

If removing Astro changes the rule, the rule belongs to `TS-ASTRO`, not `TS-CONTENT`.

`TS-CONTENT` must not require:

- Astro collections
- `astro:content`
- `src/content.config.ts`
- `.astro` routes
- Astro integrations
- Astro content adapters
- Astro MDX integration
- Astro output mode
- Astro package pins

# Final `TS-ASTRO` Direction

Astro owns framework setup and Astro-specific content pipeline enforcement.

Already implemented or partly implemented:

- Astro package and integration stack.
- Static output.
- `@astrojs/react`, `@astrojs/mdx`, `@astrojs/check`.
- `@astrojs/sitemap`, `astro-robots`.
- `@nuasite/checks` with fail-closed build checks.
- `schema-dts` for JSON-LD typing.
- `g3ts-astro-nuasite-checks` structured data presence check.
- `src/content.config.ts` existence.
- No Velite/Contentlayer bypass in Astro apps.
- No Next generated state or Next runtime package in Astro landing/content apps.
- No direct content filesystem reads/imports/globs in routes through `g3ts-eslint-plugin-astro-pipeline`.
- No direct `astro:content` usage in routes.
- Routes must use approved content adapter modules.
- MDX ESLint lane is active.
- Inline public-copy lint is active on Astro/TS/TSX source lanes.
- `astro-seo` and related brittle SEO packages are forbidden through Syncpack.

# Minimal Astro Policy

Use enforcement scope, not product taxonomy.

```toml
[ts.astro]
profile = "strict-local-content"

content_routes = ["src/pages/**/*.astro"]
non_content_routes = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]

content_root = "src/content"
content_adapter = "src/lib/content"

forbidden_state = [".next/**", ".velite/**", ".contentlayer/**"]
```

Removed and unsupported fields:

- `authored_content_globs`
- `content_route_globs`
- `chrome_route_globs`
- `utility_route_globs`
- `generated_route_globs`
- `report_shell_route_globs`
- `endpoint_globs`
- `content_data_module_globs`
- `query_adapter_globs`
- `adapter_barrel_globs`
- `adapter_helper_globs`
- `route_registry_globs`
- `content_component_globs`
- `content_config_globs`
- `mdx_content_globs`
- `approved_mdx_component_globs`
- `approved_generated_artifact_globs`
- `astro_content_type_import_globs`
- `contentlayer_config_globs`
- `contentlayer_generated_globs`
- `forbidden_generated_state_globs`
- `build_output_globs`
- `blog_index_route_globs`
- `blog_article_route_globs`
- `metadata_helper_globs`
- `json_ld_helper_globs`

# `TS-ASTRO` Rules To Build

## `TS-ASTRO-CONFIG-23` - strict content policy exists

Owner: G3TS Astro config checks.

Implemented in this slice.

Rule:

- For every detected Astro app root, read app-local `guardrail3-ts.toml`.
- Parse through shared `guardrail3-rs-toml-parser`.
- Require `[ts.astro]`.
- Require `profile = "strict-local-content"`.
- Require non-empty `content_routes`.
- Require non-empty `content_root`.
- Require non-empty `content_adapter`.
- Require `forbidden_state` to include `.next/**`, `.velite/**`, and `.contentlayer/**`.
- Missing, unreadable, parse-error, or incomplete policy fails closed.
- Old `*_globs` route-class fields are not supported.

Delegate:

- TOML parsing belongs to `guardrail3-rs-toml-parser`.
- G3TS Astro owns semantic checks over parsed facts.

## `TS-ASTRO-CONFIG-24` - strict policy globs are structurally valid

Owner: G3TS Astro config checks.

Implemented in this slice.

Rule:

- Every `content_routes`, `non_content_routes`, `endpoints`, and `forbidden_state` entry must be app-relative.
- No entry may be absolute.
- No entry may contain `..`.
- Empty string entries are invalid.
- `content_root` and `content_adapter` must be app-relative directories.
- `content_root` and `content_adapter` must not overlap.

Delegate:

- Use a shared path/glob validator if one exists.
- If no parser exists, implement this as a tiny shared path-policy helper, not inside each rule.

## `TS-ASTRO-CONFIG-25` - content and non-content route scopes are disjoint

Owner: G3TS Astro config checks.

Implemented in this slice.

Rule:

- A route file must not match both `content_routes` and `non_content_routes`.
- Endpoint files are checked separately through `endpoints`; they are not route pages.
- This rule evaluates discovered files against configured globs, not glob-vs-glob theoretical overlap.

Delegate:

- Use `globset` for matching discovered files.
- Do not invent route classes beyond content route, non-content route, endpoint.

## `TS-ASTRO-CONFIG-26` - ESLint plugin route coverage matches Astro policy

Owner: G3TS Astro config checks.

Implemented in this slice.

Rule:

- Effective ESLint config for Astro, TS, and TSX probes must enable `g3ts-eslint-plugin-astro-pipeline`.
- Route-scoped plugin options must cover every discovered file matched by `content_routes`.
- Route-scoped plugin options must not require content adapter imports for files matched by `non_content_routes`.
- Endpoint options must cover every discovered file matched by `endpoints`.
- A mismatch between `[ts.astro]` policy and effective ESLint options fails.

Delegate:

- Source AST rules stay in `g3ts-eslint-plugin-astro-pipeline`.
- G3TS only proves the plugin is installed, active, and scoped to the same files as the Astro policy.

## `TS-ASTRO-CONFIG-27` - content adapter path exists and is exported

Owner: G3TS Astro config or file-tree checks.

Implemented in this slice.

Rule:

- `content_adapter` must resolve to a real directory or module under the app root.
- At least one adapter module must exist under `content_adapter`.
- Content routes must import only approved adapter modules according to the delegated ESLint plugin.

Delegate:

- File existence belongs to G3TS.
- Import semantics stay in `g3ts-eslint-plugin-astro-pipeline`.

## `TS-ASTRO-FILETREE-12` - forbidden generated state is absent

Owner: G3TS Astro file-tree checks.

Implemented in this slice.

Rule:

- Every configured `forbidden_state` pattern must be checked against included and ignored workspace entries.
- `.next/**`, `.velite/**`, and `.contentlayer/**` are always forbidden in strict Astro content apps.
- Matched files or directories fail.

Delegate:

- G3TS owns file-tree facts.
- Syncpack owns package bans.

# `TS-CONTENT` Direction

`TS-CONTENT` must be framework-independent.

It may own normalized facts and rules such as:

- slug uniqueness
- duplicate canonical URLs
- future publish-date policy
- draft publication policy
- broken internal links
- image existence
- missing title/description in normalized content records

It must not own Astro-specific configuration or file paths.

# `TS-SEO` Direction

Rendered SEO quality should move out of Astro long term.

`TS-SEO` may own:

- rendered title existence
- rendered meta description existence
- canonical link existence
- valid JSON-LD presence
- sitemap/robots/llms public-output policy

Current Astro implementation can continue to require Nuasite wiring because Nuasite is wired through Astro build integration today.

# `TS-CSS` / Style Family Direction

Tailwind and design-token rules are not Astro-owned.

Style family may own:

- arbitrary Tailwind value bans
- required token utilities
- class ordering
- framework-independent CSS policy

# Implementation Order

1. Done: `TS-ASTRO-CONFIG-24`.
2. Done: `TS-ASTRO-CONFIG-25`.
3. Done: `TS-ASTRO-CONFIG-26`.
4. Done: `TS-ASTRO-CONFIG-27`.
5. Done: `TS-ASTRO-FILETREE-12`.
6. Then design framework-independent `TS-CONTENT` from normalized content facts only.
