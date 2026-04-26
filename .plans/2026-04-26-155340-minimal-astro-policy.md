# Goal

Replace the overgrown Astro strict-content policy with the smallest useful contract.
Do not keep backward compatibility for the unused `*_globs` route taxonomy.

# Problem

The previous Astro/content plan invented route classes and a large list of policy fields.
Most fields were not used by implemented rules and would force agents to maintain fragile path inventories.
That is bad guardrail design: it adds configuration surface without increasing enforcement.

# Decision

Use enforcement scope, not product taxonomy.

The Astro policy only needs to answer:

- which page routes are content-enforced
- which page routes are explicitly not content-enforced
- which files are endpoints
- where authored content lives
- where approved content adapter code lives
- which generated/legacy states are forbidden

# Minimal Policy

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

# Removed Fields

Remove these from the typed parser schema and from future plans:

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

# Implementation

- Update `guardrail3-rs-toml-parser` to expose only the minimal `TsAstroPolicyConfig`.
- Update parser assertions and tests to use the minimal field names.
- Update `.plans/2026-04-26-133953-content-astro-boundaries.md` so the current source of truth no longer tells agents to build the discarded taxonomy.
- Do not add compatibility aliases.
- Do not build new rules against removed fields.

# Next Build Step

After parser cleanup, add Astro policy ingestion facts:

- selected policy path
- strict profile enabled
- content routes
- non-content routes
- endpoints
- content root
- content adapter
- forbidden state

Then add the first rule:

- strict policy exists and has non-empty `content_routes`, `content_root`, and `content_adapter`.

# Progress

- Done: `guardrail3-rs-toml-parser` exposes only the minimal `[ts.astro]` fields.
- Done: parser tests prove the minimal field mapping and wrong new-field types.
- Done: Astro ingestion reads app-local `guardrail3-ts.toml` through the shared parser.
- Done: Astro config checks include `TS-ASTRO-CONFIG-23` for the strict content policy.
- Not done: route overlap checks.
- Not done: wiring ESLint plugin option checks to policy-derived content route scopes.
