# Goal

Split the current flat `ts/astro` family into real Astro child families before adding more content rules.

The end state is not separate generic families. These remain Astro-owned because the checks rely on Astro packages, Astro config, Astro content collections, Astro routes, Astro integrations, Astro ESLint lanes, or Astro rendered-output tooling.

# Problem

The current Astro family is already too broad. `G3TsAstroConfigChecksInput` and related types mix unrelated fact surfaces:

- package scripts
- Syncpack policy
- Astro config
- Astro integrations
- ESLint effective config
- content route scopes
- approved content adapters
- MDX component maps
- metadata helpers
- JSON-LD helpers
- Nuasite rendered-check config
- file-tree state

This is manageable only while the family is small. Adding collection schema and i18n rules into this flat graph would make the ingestion package an Astro dumping ground.

# Target Child Families

## `ts/astro/setup`

Purpose:

- prove the app is an Astro static content app with the required baseline toolchain
- prove Astro-specific package pins/bans are delegated to Syncpack
- prove Astro config has required baseline settings and integrations that are not owned by narrower child families

Initial migrated rules:

- `TS-ASTRO-CONFIG-01` -> `g3ts-astro-setup/astro-package-present`
- `TS-ASTRO-CONFIG-02` -> `g3ts-astro-setup/astro-check-present`
- `TS-ASTRO-CONFIG-03` -> `g3ts-astro-setup/astro-eslint-plugin-package-present`
- `TS-ASTRO-CONFIG-05` -> `g3ts-astro-setup/astro-eslint-plugin-wired`
- `TS-ASTRO-CONFIG-09` -> `g3ts-astro-setup/syncpack-stack-pins`
- `TS-ASTRO-CONFIG-10` -> `g3ts-astro-setup/syncpack-forbidden-deps`
- `TS-ASTRO-CONFIG-11` -> `g3ts-astro-setup/site-url`
- `TS-ASTRO-CONFIG-12` -> `g3ts-astro-setup/static-output`
- `TS-ASTRO-CONFIG-21` -> `g3ts-astro-setup/required-integrations`

Initial file-tree rules:

- `TS-ASTRO-FILETREE-01` -> `g3ts-astro-setup/astro-config-exists`
- `TS-ASTRO-FILETREE-03` -> `TS-ASTRO-SETUP-FILETREE-03`
- `TS-ASTRO-FILETREE-11` -> `TS-ASTRO-SETUP-FILETREE-11`
- `TS-ASTRO-FILETREE-12` -> `TS-ASTRO-SETUP-FILETREE-12`

Ingestion facts:

- app root
- `package.json` parsed surface
- package script command facts from the shared package-script parser
- `.syncpackrc` parsed surface from the shared Syncpack parser
- `astro.config.*` parsed surface from the shared Astro config parser
- baseline file-tree facts

Explicit non-ownership:

- no content collection schema facts
- no MDX component-map facts
- no SEO helper usage facts
- no i18n locale facts

## `ts/astro/content`

Purpose:

- prove public Astro content routes are fed by Astro content collections through approved adapters
- prove Astro content collections and adapter boundaries are real
- close the hardcoded-content loopholes without enforcing one exact file layout

Initial migrated rules:

- `TS-ASTRO-CONFIG-18` -> `g3ts-astro-content/content-adapter-rule`
- `TS-ASTRO-CONFIG-23` -> `g3ts-astro-content/strict-content-policy`
- `TS-ASTRO-CONFIG-24` -> `g3ts-astro-content/strict-policy-paths`
- `TS-ASTRO-CONFIG-25` -> `g3ts-astro-content/route-scope-overlap`
- `TS-ASTRO-CONFIG-26` -> `g3ts-astro-content/policy-eslint-coverage`
- `TS-ASTRO-CONFIG-27` -> `g3ts-astro-content/content-adapter-exists`
- `TS-ASTRO-CONFIG-28` -> `g3ts-astro-content/content-adapter-astro-content`

Initial file-tree rules:

- `TS-ASTRO-FILETREE-02` -> `g3ts-astro-content/content-config-exists`
- `TS-ASTRO-FILETREE-04` -> `g3ts-astro-content/no-route-markdown-pages`
- `TS-ASTRO-FILETREE-05` -> `g3ts-astro-content/no-velite-config`
- `TS-ASTRO-FILETREE-06` -> `g3ts-astro-content/no-velite-output`

New rules after mechanical split:

- `TS-ASTRO-CONTENT-CONFIG-33`: required collections exist in `src/content.config.ts`.
- `TS-ASTRO-CONTENT-CONFIG-34`: required collections use `defineCollection` with `loader` and non-empty `schema`.
- `TS-ASTRO-CONTENT-CONFIG-35`: collection schemas import `z` from `astro/zod`.
- `TS-ASTRO-CONTENT-CONFIG-36`: required schema fields exist per collection.
- `TS-ASTRO-CONTENT-CONFIG-37`: approved adapters call `getCollection` or `getEntry` for required collections.
- `TS-ASTRO-CONTENT-CONFIG-38`: dynamic content routes derive `getStaticPaths()` from approved adapters.
- `TS-ASTRO-CONTENT-CONFIG-39`: content route render data is derived from approved adapter output, not just a no-op adapter import.

Policy shape:

```toml
[ts.astro.routes]
content = ["src/pages/**/*.astro"]
non_content = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]

[ts.astro.content]
root = "src/content"
adapters = ["src/lib/content"]
required_collections = ["landing", "blog"]

[ts.astro.content.collection_fields]
landing = ["title", "description", "sections"]
blog = ["title", "description", "status", "publishedAt"]
```

This is a capability contract, not file taxonomy. The app chooses adapter paths and collection names. G3TS verifies the declared surfaces are real and used.

Ingestion facts:

- app root
- `src/content.config.ts` parsed facts
- strict content policy facts
- route scope facts
- route file facts for content/non-content/endpoint matching
- approved content adapter source facts
- ESLint effective config facts only for content-route pipeline rules
- Velite/Contentlayer file-tree facts

Explicit non-ownership:

- no Astro SEO helpers
- no generic Markdown link checking
- no Tailwind/style policy
- no generic i18n policy

## `ts/astro/mdx`

Purpose:

- prove Astro MDX support and MDX source constraints are active
- prove MDX imports React components only through approved component-map surfaces
- delegate Markdown/MDX syntax linting to `eslint-plugin-mdx` and Remark

Initial migrated rules:

- `TS-ASTRO-CONFIG-20` -> `g3ts-astro-mdx/mdx-lane`
- `TS-ASTRO-CONFIG-30` -> `g3ts-astro-mdx/mdx-component-map-rule`

Setup dependency:

- `@astrojs/mdx` integration can remain in setup as a required baseline integration, or be emitted as a shared setup fact consumed by MDX.
- Do not duplicate package checks across setup and MDX.

Policy shape:

```toml
[ts.astro.mdx]
component_maps = ["src/mdx-components.tsx"]
```

Future delegated rules:

- require `remark-lint` through `mdx/remark` if we decide to enforce prose/Markdown conventions
- link checking is explicitly out of scope for now

Ingestion facts:

- MDX content probe facts from ESLint effective config
- approved component-map source facts
- MDX content file facts if source checks become necessary

Explicit non-ownership:

- no content collection validation
- no SEO helpers
- no link checking now

## `ts/astro/seo`

Purpose:

- prove Astro-specific SEO and rendered-output wiring is present
- keep this Astro-owned because Nuasite integration, sitemap integration, robots integration, and Astro route/layout helper usage are Astro-specific

Initial migrated rules:

- `TS-ASTRO-CONFIG-13` -> `g3ts-astro-seo/nuasite-checks`
- `TS-ASTRO-CONFIG-14` -> `g3ts-astro-seo/sitemap-integration`
- `TS-ASTRO-CONFIG-15` -> `g3ts-astro-seo/robots-integration`
- `TS-ASTRO-CONFIG-16` -> `g3ts-astro-seo/llms-txt`
- `TS-ASTRO-CONFIG-17` -> `g3ts-astro-seo/seo-packages`
- `TS-ASTRO-CONFIG-22` -> `g3ts-astro-seo/structured-data-check`
- `TS-ASTRO-CONFIG-29` -> split:
  - MDX component-map presence moves to `TS-ASTRO-MDX-CONFIG-*`
  - metadata/json-LD helper presence moves to `TS-ASTRO-SEO-CONFIG-*`
- `TS-ASTRO-CONFIG-31` -> `g3ts-astro-seo/metadata-helper-rule`
- `TS-ASTRO-CONFIG-32` -> `g3ts-astro-seo/json-ld-helper-rule`

Policy shape:

```toml
[ts.astro.seo]
metadata_helpers = ["src/lib/seo/metadata.ts"]
json_ld_helpers = ["src/lib/seo/json-ld.ts"]
```

Ingestion facts:

- Astro config integrations relevant to sitemap, robots, Nuasite
- package presence for `schema-dts`, `@nuasite/checks`, `g3ts-astro-nuasite-checks`
- helper source facts for metadata and JSON-LD
- ESLint effective config facts for SEO helper route rules
- `llms.txt` file/route fact

Explicit non-ownership:

- no generic SEO family now
- no Next SEO now
- no content collection schema facts

## `ts/astro/i18n`

Purpose:

- future child family for Astro-specific locale routing and localized content enforcement

Do not implement in this split.

Future ingestion facts:

- Astro config `i18n.locales`
- Astro config `i18n.defaultLocale`
- route locale params/static-path facts
- adapter localized collection query facts
- layout `lang` and alternate/canonical helper usage

# Parser Config Migration

Current config:

```toml
[ts.astro]
profile = "strict-local-content"
content_routes = ["src/pages/**/*.astro"]
non_content_routes = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]
content_root = "src/content"
content_adapter = "src/lib/content"
mdx_component_maps = ["src/mdx-components.tsx"]
metadata_helpers = ["src/lib/seo/metadata.ts"]
json_ld_helpers = ["src/lib/seo/json-ld.ts"]
forbidden_state = [".next/**", ".velite/**", ".contentlayer/**"]
```

Target config:

```toml
[ts.astro]
profile = "strict-static-content"

[ts.astro.routes]
content = ["src/pages/**/*.astro"]
non_content = ["src/pages/404.astro"]
endpoints = ["src/pages/**/*.ts"]

[ts.astro.content]
root = "src/content"
adapters = ["src/lib/content"]
required_collections = ["landing", "blog"]

[ts.astro.content.collection_fields]
landing = ["title", "description", "sections"]
blog = ["title", "description", "status", "publishedAt"]

[ts.astro.mdx]
component_maps = ["src/mdx-components.tsx"]

[ts.astro.seo]
metadata_helpers = ["src/lib/seo/metadata.ts"]
json_ld_helpers = ["src/lib/seo/json-ld.ts"]

[ts.astro.state]
forbidden = [".next/**", ".velite/**", ".contentlayer/**"]
```

Migration rule:

- No backward compatibility.
- Current app configs must be updated to nested shape.
- Parser should reject or ignore old fields only if existing unknown-field policy already allows it. The Astro semantic rule must require the new nested fields.

# Package Layout

Target package roots:

```text
packages/ts/astro/shared/g3ts-astro-shared-types
packages/ts/astro/shared/g3ts-astro-shared-ingestion-support

packages/ts/astro/setup/g3ts-astro-setup-types
packages/ts/astro/setup/g3ts-astro-setup-ingestion
packages/ts/astro/setup/g3ts-astro-setup-config-checks
packages/ts/astro/setup/g3ts-astro-setup-file-tree-checks

packages/ts/astro/content/g3ts-astro-content-types
packages/ts/astro/content/g3ts-astro-content-ingestion
packages/ts/astro/content/g3ts-astro-content-config-checks
packages/ts/astro/content/g3ts-astro-content-file-tree-checks

packages/ts/astro/mdx/g3ts-astro-mdx-types
packages/ts/astro/mdx/g3ts-astro-mdx-ingestion
packages/ts/astro/mdx/g3ts-astro-mdx-config-checks

packages/ts/astro/seo/g3ts-astro-seo-types
packages/ts/astro/seo/g3ts-astro-seo-ingestion
packages/ts/astro/seo/g3ts-astro-seo-config-checks
```

Do not create `astro/i18n` packages until there are rules to implement.

# Shared Boundaries

Allowed shared support:

- app-root selection helpers
- Astro config parser output mapping helpers
- package surface mapping helpers
- ESLint effective-config mapping helpers
- policy path validation helpers

Forbidden shared support:

- one mega `AstroFacts` struct
- one ingestion function that returns all child-family facts
- one config-check input consumed by all child families
- child-family rules reaching into another child-family input

# Migration Strategy

## Slice 1 - Plan and Parser Shape

- Write this plan.
- Update `guardrail3-rs-toml-parser` to parse the target nested `[ts.astro.*]` shape.
- Keep the old flat `TsAstroPolicyConfig` fields only if needed to make tests compile temporarily, but semantic checks must move to the new nested fields before the slice ends.
- Add parser tests proving:
  - nested route/content/mdx/seo/state fields parse
  - `collection_fields` parses as map of string arrays
  - missing nested sections default empty

## Slice 2 - Extract Shared Astro Types

- Add `packages/ts/astro/shared/g3ts-astro-shared-types`.
- Move only cross-child passive types there:
  - output mode
  - static value snapshots
  - integration snapshots
  - package script command facts
  - package/syncpack/astro-config surface states if multiple child families need them
- Do not move child-specific inputs into shared.

## Slice 3 - Setup Child Family

- Create setup packages.
- Move setup-owned rules and tests first.
- Add a setup ingestion entry point that returns only setup input.
- Wire family runner to execute setup child checks when `--family astro` is requested.
- Keep old flat rules disabled or removed in the same commit to avoid duplicate findings.

## Slice 4 - Content Child Family

- Create content packages.
- Move content-owned rules and tests.
- Add content ingestion entry point that returns only content input.
- Move current route policy and adapter source facts here.
- Wire runner.

## Slice 5 - MDX Child Family

- Create MDX packages.
- Move MDX-owned rules and tests.
- Move MDX component-map policy and ESLint MDX lane facts here.
- Wire runner.

## Slice 6 - SEO Child Family

- Create SEO packages.
- Move SEO-owned rules and tests.
- Split old helper-surface rule 29 into MDX and SEO child rules.
- Move metadata/json-LD helper facts here.
- Wire runner.

## Slice 7 - Delete Flat Astro Packages

- Remove or turn the old flat packages into compatibility-free empty facades only if Cargo workspace structure requires temporary package roots.
- Remove old flat IDs from expected outputs and tests.
- Update docs/worklogs.

## Slice 8 - New Content Rules

Only after split is complete:

- implement collection existence
- implement collection loader/schema checks
- implement schema `astro/zod` import checks
- implement required collection field checks
- implement adapter collection query checks
- implement route `getStaticPaths()` adapter derivation checks
- implement route render data adapter derivation checks

# Verification Per Slice

Every slice must run:

- `cargo test --workspace` in every touched Rust package
- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path` on every touched Rust package that has Rust guardrail config
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

For plugin changes:

- `npm test --prefix packages/ts/g3ts-eslint-plugin-astro-pipeline`
- publish if the app-facing package version changes and landing needs it

# Adversarial Review Gates

After each migration slice:

- send an adversarial reviewer against this plan and the code
- reviewer must check:
  - no child family ingests unrelated facts
  - no generic family received Astro-specific ownership
  - no new exact file layout taxonomy was introduced
  - no old flat rule remains active if the child rule replaced it
  - landing-facing error messages still tell agents what to install/configure

# First Implementation Choice

Start with Slice 1 only.

Reason:

- parser/config shape is the root seam
- it is lower risk than moving all packages first
- it lets landing agents converge on the nested capability config before new content rules are added

