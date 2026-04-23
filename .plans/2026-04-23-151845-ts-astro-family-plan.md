# TS-ASTRO Family Plan

## Goal

Add a `ts/astro` family that enforces Astro-specific setup and Astro-specific pipeline-bypass prevention.

`ts/astro` should make it hard for agents to:
- misconfigure Astro itself
- skip Astro-owned validation hooks
- skip the required Astro lint/plugin surfaces
- bypass the approved Astro content pipeline from routes/endpoints/helpers

`ts/astro` should not re-implement:
- content-entry schema validation
- generic ESLint wiring
- generic package/script ownership
- sitemap/robots/metadata policy
- rendered-page SEO/a11y checks
- report snapshot/data-contract rules

## Core correction

The first draft overreached.

Specific flaw:
- it collapsed Astro setup, generic content policy, generic ESLint/package ownership, and SEO setup into one family
- that duplicated existing `ts/content`, `ts/seo`, `ts/eslint`, `ts/package`, and `ts/tsconfig` ownership
- it also used oversized bag inputs instead of minimal typed rule inputs

Second flaw in the next draft:
- it over-corrected by treating shared config surfaces as if only one family could own assertions on them
- that is not how the Rust families work
- shared parsers can feed many families
- `ts/astro` should own Astro-specific requirements on shared config surfaces
- `ts/eslint`, `ts/package`, and `ts/seo` still own their general setup and parsers

Third flaw in the next draft:
- it still talked as if sibling families owned parsing
- that is also wrong
- parsers live under `packages/parsers`
- families do not parse config files directly
- family ingestion is only allowed to call shared parser packages and normalize the parsed result into family facts

The corrected family is narrower:
- Astro framework/config surfaces
- Astro integration wiring
- Astro render-mode contract
- Astro-specific setup requirements on shared config surfaces
- Astro lint/plugin contract for source-level enforcement

## Family position

`ts/astro` becomes a new canonical TS family.

Follow-up docs to update when implementation starts:
- `.plans/todo/checks/ts/README.md`
- `.plans/todo/checks/ts/arch.md`
- `.plans/todo/checks/ts/content.md`
- `.plans/todo/checks/ts/seo.md`

High-level ownership after the split:
- `ts/astro`
  - Astro framework setup
  - Astro-specific requirements on shared config surfaces
  - Astro integration wiring
  - Astro lint/plugin contract for source-level enforcement
- `ts/content`
  - framework-agnostic content model and content-site API safety
- `ts/seo`
  - generic sitemap, robots, metadata, static-route ownership
- `ts/eslint`
  - generic ESLint setup and rule ownership
- `ts/package`
  - generic package.json setup and script ownership
- `ts/tsconfig`
  - TypeScript compiler policy

## Owned root

Owned root:
- TS package/app roots identified as Astro apps

Astro detection signals:
- required primary signal:
  - `astro.config.*`
  - or `astro` package dependency
- corroborating signals only:
  - `src/content.config.*`
  - `src/live.config.*`
  - Astro route roots

## Package layout

Create:
- `packages/ts/astro/g3ts-astro-types`
- `packages/ts/astro/g3ts-astro-ingestion`
- `packages/ts/astro/g3ts-astro-config-checks`
- `packages/ts/astro/g3ts-astro-file-tree-checks`

Do not create:
- `g3ts-astro-seo-checks`
- `g3ts-astro-package-checks`
- `g3ts-astro-eslint-checks`
- `g3ts-astro-report-checks`
- `g3ts-astro-source-checks` in v1

Those belong to other families.

## External validator/tool contract

`ts/astro` should enforce only real, concrete contracts.

## Parser model

Use the existing parser architecture exactly.

- shared parsers live under:
  - `packages/parsers`
- each parser is file-type-specific, not family-specific
- family ingestion crates are forbidden to parse config files themselves
- ingestion may only:
  - select files
  - call shared parser packages
  - normalize parsed output into family facts and rule inputs

Examples already in repo:
- [eslint-config-parser](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/eslint-config-parser)
- [package-json-parser](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/package-json-parser)
- [tsconfig-json-parser](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/parsers/tsconfig-json-parser)

Concrete specimen:
- [g3ts-eslint-ingestion runtime](/Users/tartakovsky/Projects/websmasher/guardrail3/packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/run.rs)
  - selects the active ESLint config
  - calls the shared `eslint_config_parser`
  - converts parser output into family snapshots
  - does not parse ESLint itself

### Astro-owned validator

- `astro check`
- must be invoked explicitly
- no "or equivalent" language

### Rendered-page validator

V1 contract:
- `@nuasite/checks`
- must be installed
- must be wired through `astro.config.*`

Do not mention a generic post-build CLI contract in v1.
There is no real package for that yet.

### Astro integrations

Use explicit integration contracts only:
- `@astrojs/mdx`
- `@astrojs/react`
- `@nuasite/checks`

Do not use vague "approved integration" wording.

### Source-rule validator

Preferred contract:
- repo-owned Astro ESLint plugin

Reason:
- these are source-level policy rules
- they fit lint better than guardrails
- user intent is that guardrails enforce validator presence, not duplicate validator behavior

### Delegated families

`ts/astro` depends on facts from:
- shared parser packages plus family facts from:
- `packages/parsers/eslint-config-parser`
- `packages/parsers/package-json-parser`
- `packages/parsers/tsconfig-json-parser`
- `ts/package`
  - package-level policy facts when needed
- `ts/seo`
  - semantic requirement facts
  - sitemap/robots/metadata ownership
- `ts/content`
  - content-model/schema/loader policy where framework-agnostic
- `ts/tsconfig`
  - compiler strictness

## What Astro itself already owns

Astro owns:
- content collection entry validation
- collection reference validation
- collection typing
- entry parse failures

`ts/astro` should not duplicate those.

## App profile

Use a small Astro-specific profile only for routing Astro rules.

```rust
pub enum G3TsAstroAppKind {
    Static,
    Hybrid,
    Server,
}

pub enum G3TsAstroContentMode {
    None,
    BuildCollections,
    LiveCollections,
}

pub struct G3TsAstroAppProfile {
    pub render_mode: G3TsAstroRenderMode,
    pub content_mode: G3TsAstroContentMode,
}
```

Do not put:
- `uses_i18n`
- `requires_sitemap`
- `requires_render_validator`

Those are cross-family policy facts, not Astro-local profile facts.

## Minimal typed inputs

Do not use bag-of-files contracts.

### File-tree inputs

```rust
pub struct G3TsAstroAppRootInput {
    pub app_root_rel_path: String,
    pub astro_config_rel_path: Option<String>,
    pub content_config_rel_path: Option<String>,
    pub live_config_rel_path: Option<String>,
}

pub struct G3TsAstroRouteMarkdownPageInput {
    pub rel_path: String,
}

pub struct G3TsAstroCrossRootSideLoaderInput {
    pub loader_rel_path: String,
    pub target_rel_path: String,
}
```

### Config inputs

```rust
pub struct G3TsAstroConfigSurfaceInput {
    pub rel_path: String,
    pub render_mode: G3TsAstroRenderMode,
    pub has_site: bool,
    pub has_adapter: bool,
    pub integrations: Vec<String>,
}

pub struct G3TsAstroContentConfigSurfaceInput {
    pub rel_path: String,
    pub content_mode: G3TsAstroContentMode,
    pub uses_schema: bool,
    pub uses_loader_schema: bool,
    pub loader_modules: Vec<String>,
    pub has_inline_custom_loader: bool,
}

pub struct G3TsAstroIntegrationContractInput {
    pub package_json_rel_path: String,
    pub dependencies: Vec<String>,
    pub integrations: Vec<String>,
    pub script_names: Vec<String>,
    pub script_bodies: Vec<(String, String)>,
    pub requires_render_validator: bool,
    pub uses_react_in_astro: bool,
}
```

### Source inputs

```rust
pub enum G3TsAstroModuleRole {
    RouteEntry,
    EndpointEntry,
    ContentAdapter,
    MdxRuntime,
    ContentSideLoader,
    RouteRegistry,
}

pub struct G3TsAstroModuleInput {
    pub rel_path: String,
    pub role: G3TsAstroModuleRole,
    pub content: String,
    pub imports: Vec<String>,
}

pub struct G3TsAstroImportEdgeInput {
    pub importer_rel_path: String,
    pub importer_role: G3TsAstroModuleRole,
    pub imported_rel_path: String,
    pub imported_role: G3TsAstroModuleRole,
}
```

This gives the source lane enough structure to distinguish:
- route entry
- endpoint
- allowed content adapter
- forbidden side loader
- MDX runtime bridge
- route registry helper

These facts are still useful even if source-level rule enforcement moves to ESLint:
- `ts/astro` can require the plugin contract
- the plugin can mirror the same conceptual module roles
- a later fallback `g3ts-astro-source-checks` package can reuse the same facts if one source rule proves not implementable in lint

## Ingestion responsibilities

`g3ts-astro-ingestion` owns:
- Astro app detection
- `astro.config.*` discovery and targeted extraction
- `src/content.config.*` discovery and targeted extraction
- `src/live.config.*` discovery and targeted extraction
- calling shared parser packages for shared file types
  - ESLint config
  - package.json
  - tsconfig
- render-mode classification
- content-mode classification
- integration extraction from `astro.config.*`
- package script extraction from `package.json`
- discovery of:
  - route entries
  - endpoint entries
  - content adapter modules
  - content side-loader modules
  - MDX runtime modules
  - route registry modules
- import-edge extraction between those modules
- cross-root side-loader detection

`g3ts-astro-ingestion` does not own:
- parsing config files itself
- final rendered HTML validation
- content-entry schema correctness

## Family boundaries

### `ts/astro` owns

- `astro.config.*` presence and parseable Astro-specific config facts
- `src/content.config.*` / `src/live.config.*` presence and mode classification
- Astro-specific requirements on `eslint.config.*`
  - Astro plugin package presence
  - Astro plugin wiring
  - Astro pipeline plugin/package presence and wiring
- Astro-specific requirements on `package.json`
  - `astro` package presence
  - `astro check` invocation
  - Astro integration package presence
- Astro integration package-and-config coupling
- render mode / adapter / prerender contract
- Astro-specific SEO/checking integrations when Astro app policy requires them
  - `@nuasite/checks`
  - `@astrojs/sitemap`
- route/endpoint/helper bypasses around Astro content access
- route-side raw MDX/runtime-eval bridges
- route-side direct collection query usage when adapter modules should own access
- route-side `import.meta.glob*()` authored-content bypasses
- route-side cross-root content/spec loader bypasses
- Astro-specific source-rule plugin contract

### `ts/content` continues to own

- generic content roots and content-site contracts
- content schema/model policy
- content-site endpoint/action safety
- framework-agnostic content loader policy where applicable

### `ts/seo` continues to own

- sitemap requirement
- robots requirement
- metadata/canonical/structured data ownership
- static-route and indexable-route completeness

### `ts/eslint` continues to own

- generic ESLint policy
- exact lint rule/severity contracts

### `ts/package` continues to own

- generic `lint` / `build` script existence

### `ts/tsconfig` continues to own

- strict compiler settings

## Rule inventory

## File-tree lane

### TS-ASTRO-FILETREE-01
- require `astro.config.*`

### TS-ASTRO-FILETREE-02
- require `src/content.config.*` when `content_mode = BuildCollections`

### TS-ASTRO-FILETREE-03
- require `src/live.config.*` when `content_mode = LiveCollections`

### TS-ASTRO-FILETREE-04
- forbid authored `.md` / `.mdx` route pages under `src/pages/**` for collection-backed Astro content apps
- explicit whitelist support is not in the v1 input model and should not be implied until it is actually modeled

### TS-ASTRO-FILETREE-05
- forbid cross-root side loaders that read authored/spec content from sibling packages or arbitrary non-app roots unless the root is an approved generated-artifact root
- v1 note:
  live discovery of these loaders is deferred until there is a real parser-owned source fact or a plugin-owned source rule surface
  raw string scanning in Astro ingestion is explicitly forbidden for this slice

## Config lane

### TS-ASTRO-CONFIG-01
- require `astro` package

### TS-ASTRO-CONFIG-02
- require a real `astro check` invocation in the app's script surface

### TS-ASTRO-CONFIG-03
- require `eslint-plugin-astro` package

### TS-ASTRO-CONFIG-04
- require `@nuasite/checks` package when Astro app policy requires rendered-page validation

### TS-ASTRO-CONFIG-05
- require Astro ESLint plugin wiring in `eslint.config.*`

### TS-ASTRO-CONFIG-06
- require repo-approved Astro pipeline ESLint plugin package when Astro app policy requires source-pipeline linting

### TS-ASTRO-CONFIG-07
- require repo-approved Astro pipeline ESLint plugin wiring in `eslint.config.*`

### TS-ASTRO-CONFIG-08
- require `@nuasite/checks` wiring in `astro.config.*`

### TS-ASTRO-CONFIG-09
- require `@astrojs/sitemap` package when `ts/seo` marks sitemap as required for the Astro app

### TS-ASTRO-CONFIG-10
- require `@astrojs/sitemap` integration wiring in `astro.config.*` when `ts/seo` marks sitemap as required

### TS-ASTRO-CONFIG-11
- require `@astrojs/mdx` package when MDX entries exist

### TS-ASTRO-CONFIG-12
- require `@astrojs/mdx` integration wiring in `astro.config.*`

### TS-ASTRO-CONFIG-13
- require `@astrojs/react` package when Astro/MDX renders React components

### TS-ASTRO-CONFIG-14
- require `@astrojs/react` integration wiring in `astro.config.*`

### TS-ASTRO-CONFIG-15
- require loader modules in `src/content.config.*` / `src/live.config.*` to come from:
  - `astro/loaders`
  - or repo-approved loader modules

### TS-ASTRO-CONFIG-16
- forbid inline custom loader definitions in content config unless repo policy explicitly allows them

### TS-ASTRO-CONFIG-17
- require adapter package/config coupling for `render_mode = Server | Hybrid`

### TS-ASTRO-CONFIG-18
- forbid server/hybrid config in profiles that require static Astro output

### TS-ASTRO-CONFIG-19
- ban `LiveCollections` in v1 unless repo policy explicitly enables them

## Source-rule contract

Do not implement source rules in `g3ts` first.

Implement them first in the repo-owned Astro ESLint plugin.

`ts/astro` then enforces:
- plugin package presence
- plugin wiring
- required rule enablement

Initial Astro ESLint rule inventory:
- forbid `fs.readFile*()` on authored content/spec content in route/endpoint import closures
- forbid local `JSON.parse()` / YAML parse / ad hoc MDX parse on authored content/spec content
- forbid `import.meta.glob*()` over authored content outside approved content adapter modules
- require routes/endpoints to access collections only through approved adapter modules
- ban direct route imports from `astro:content` in collection-backed apps
- forbid content side-loaders one import away from routes/endpoints
- forbid runtime-eval MDX renderers
- forbid authored-MDX wrappers whose input surface is `code: string`
- require MDX component imports to come only from approved MDX component surfaces
- forbid route/endpoint modules from using unsafe type assertions and index-based extraction over collection query results when the value originates from direct collection access or an approved adapter
- forbid route-registry duplication outside approved route registry helpers
- forbid direct fetch/database access from collection-backed public content routes unless the access goes through an approved loader/adapter module

Only add `g3ts-astro-source-checks` later if one of these rules proves infeasible in lint.

## Astro ESLint plugin spec

This needs to be detailed enough to delegate directly.

### Package

Create a repo-owned package:
- `packages/ts/eslint-plugin-astro-pipeline`

Purpose:
- enforce Astro source-level content-pipeline rules
- leave `g3ts` to enforce:
  - package presence
  - plugin wiring
  - required rule enablement

### Package shape

Expected files:
- `package.json`
- `src/index.ts`
- `src/rules/`
- `src/utils/`
- `src/configs/recommended.ts`
- `README.md`

Rule files:
- `src/rules/no-authored-content-fs-read.ts`
- `src/rules/no-authored-content-parse.ts`
- `src/rules/no-authored-content-glob.ts`
- `src/rules/no-direct-astro-content-in-routes.ts`
- `src/rules/no-side-loader-imports.ts`
- `src/rules/no-runtime-mdx-eval.ts`
- `src/rules/mdx-components-only-from-approved-surface.ts`
- `src/rules/no-unsafe-content-shaping.ts`
- `src/rules/no-duplicate-route-registry.ts`
- `src/rules/no-live-data-in-collection-routes.ts`

Shared utilities:
- `src/utils/module-role.ts`
- `src/utils/import-closure.ts`
- `src/utils/path-policy.ts`
- `src/utils/content-source.ts`
- `src/utils/ast-helpers.ts`

### Plugin config contract

Expose one recommended config:
- `plugin.configs.recommended`

Recommended config enables all Astro pipeline rules at `error`.

### Plugin options

The plugin must take explicit options.

Config shape:

```ts
type AstroPipelineOptions = {
  routeGlobs: string[]
  endpointGlobs: string[]
  adapterModuleGlobs: string[]
  mdxRuntimeModuleGlobs: string[]
  routeRegistryModuleGlobs: string[]
  approvedContentAdapterModules: string[]
  approvedLoaderModules: string[]
  approvedMdxComponentModules: string[]
  approvedGeneratedArtifactRoots: string[]
  authoredContentGlobs: string[]
  specContentGlobs: string[]
}
```

Do not hardcode repo paths in rules.
All repo-specific roots/modules must come through options.

### Rule IDs

Use stable ESLint rule names:
- `astro-pipeline/no-authored-content-fs-read`
- `astro-pipeline/no-authored-content-parse`
- `astro-pipeline/no-authored-content-glob`
- `astro-pipeline/no-direct-astro-content-in-routes`
- `astro-pipeline/no-side-loader-imports`
- `astro-pipeline/no-runtime-mdx-eval`
- `astro-pipeline/mdx-components-only-from-approved-surface`
- `astro-pipeline/no-unsafe-content-shaping`
- `astro-pipeline/no-duplicate-route-registry`
- `astro-pipeline/no-live-data-in-collection-routes`

### Rule details

#### `no-authored-content-fs-read`

Fire when:
- a route/endpoint module or any imported helper in its server-side import closure calls:
  - `fs.readFile`
  - `fs.readFileSync`
  - `promises.readFile`
  - equivalent imported aliases
- and the target path resolves to authored/spec content

Do not fire when:
- the call is inside an approved loader or approved generated-artifact reader

#### `no-authored-content-parse`

Fire when:
- route/endpoint import closure parses authored/spec content with:
  - `JSON.parse`
  - YAML parsers
  - ad hoc frontmatter parsing
  - ad hoc MD/MDX parsing

#### `no-authored-content-glob`

Fire when:
- route/endpoint import closure uses:
  - `import.meta.glob`
  - `import.meta.globEager`
- over authored/spec content roots

Do not fire when:
- inside approved content adapter modules

#### `no-direct-astro-content-in-routes`

Fire when:
- a route/endpoint imports `astro:content`
- or imports collection query helpers directly

Do not fire when:
- the import is inside an approved content adapter module

#### `no-side-loader-imports`

Fire when:
- a route/endpoint imports a module outside the approved adapter/registry/runtime surfaces
- and that module reads authored/spec content or sibling-package content mirrors

This is the "one import away" bypass rule.

#### `no-runtime-mdx-eval`

Fire when:
- code uses runtime-eval MDX rendering patterns such as:
  - `new Function(...)`
  - wrapper components that accept compiled `code: string`
  - dynamic execution of compiled MDX bodies

Do not fire when:
- standard Astro/MDX compile-time rendering is used

#### `mdx-components-only-from-approved-surface`

Fire when:
- MDX runtime/component modules import components for authored MDX from non-approved modules
- or deep app-internal paths are used instead of the approved MDX component surface

#### `no-unsafe-content-shaping`

Fire when:
- route/endpoint modules use content-derived values with:
  - `as`
  - `as unknown as`
  - unsafe member/index extraction
- after direct collection access or adapter return values

Do not fire when:
- shaping happens inside approved adapter modules

#### `no-duplicate-route-registry`

Fire when:
- route inventory helpers duplicate singleton/static page registry logic outside approved route-registry modules
- applies to:
  - `getStaticPaths`
  - equivalent route registry helpers

#### `no-live-data-in-collection-routes`

Fire when:
- collection-backed public content routes fetch DB/API/live product data directly

Do not fire when:
- the access goes through approved loader/adapter modules explicitly allowed by policy

### Parsing model

Use ESLint AST plus import-resolution utilities.

Do not try to do full project crawling inside each rule.

The plugin should share:
- import-resolution utility
- path-policy matcher
- route/endpoint/adapter role classifier

### Delegation split

Delegate plugin work separately from `g3ts`.

One work item for:
- plugin scaffold
- one work item per rule
- one work item for recommended config
- one work item for test harness and fixtures

### Test pattern

Each plugin rule needs:
- positive cases
- negative cases
- one-import-away bypass cases
- alias/import indirection cases
- path-policy edge cases

### What `g3ts` enforces about the plugin

`ts/astro` config checks should enforce:
- plugin package present in `package.json`
- plugin wired in `eslint.config.*`
- `recommended` config or exact required rules enabled
- rules set to `error`
- Astro apps do not disable the required Astro pipeline rules

## Explicit non-goals

Do not add to `ts/astro`:
- sitemap/robots rules
- metadata helper rules
- canonical/hreflang rules
- JSON-LD rules
- ESLint plugin presence/rule severity checks
- generic `lint` / `build` script presence
- TS strictness rules
- report artifact immutability rules
- report snapshot runtime contract rules
- source-level policy checks that can live in the Astro ESLint plugin

Those belong elsewhere.

## Implementation order

1. adopt `ts/astro` into canonical family docs
2. scaffold packages
3. file-tree lane
  - Astro root
  - content/live config
  - route-MDX bypass
  - cross-root side-loader detection
4. config lane
  - `astro check`
  - `@nuasite/checks`
  - `@astrojs/mdx`
  - `@astrojs/react`
  - Astro ESLint plugin package/wiring
  - render-mode / adapter / live-collection contract
5. Astro ESLint plugin implementation
  - `fs` / parse / `import.meta.glob*()` bypasses
  - route-to-helper import closure
  - direct `astro:content` ban
  - runtime-eval MDX ban
  - adapter-only normalization rules

## First implementation slice

Start with:
- `TS-ASTRO-FILETREE-01`
- `TS-ASTRO-FILETREE-02`
- `TS-ASTRO-FILETREE-04`
- `TS-ASTRO-FILETREE-05`
- `TS-ASTRO-CONFIG-02`
- `TS-ASTRO-CONFIG-03`
- `TS-ASTRO-CONFIG-04`
- `TS-ASTRO-CONFIG-06`
- `TS-ASTRO-CONFIG-07`

These are concrete, high-value, and directly tied to the real landing-app failure modes.

## Deferred candidate rules

Keep these out of the first implementation slice until ingestion exposes the required typed facts:
- collection schema-source enforcement stays in `ts/content`
- static-route ownership stays in `ts/seo`
- source rules stay in the Astro ESLint plugin unless lint proves insufficient

## Open decisions

- whether `@nuasite/checks` stays the long-term validator or becomes the initial contract only
- exact repo-approved loader module registry shape
- exact approved content-adapter module registry shape
- exact approved MDX component surface
- whether static-route ownership for Astro stays in `ts/seo` or gets one Astro-specific fact dependency hook

Current recommendation:
- keep `ts/seo` as the owner
- let `ts/astro` enforce Astro-specific package/integration requirements that satisfy `ts/seo` requirements on Astro apps
- do not model report-specific modes in base `ts/astro`
- if report-specific Astro setup appears later, add a sibling policy or app contract keyed off `render_mode = Static`, not a `ReportRenderer` app kind
