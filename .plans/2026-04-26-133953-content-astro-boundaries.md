# Goal

Define the correct split between Astro-specific content guardrails and framework-independent content guardrails for agent-managed landing, blog, docs, and MDX sites.

# Core Rule

If removing Astro changes the rule, the rule belongs to `TS-ASTRO`, not `TS-CONTENT`.

`TS-CONTENT` is not allowed to require:

- Astro collections
- `astro:content`
- `src/content.config.ts`
- `.astro` route files
- Astro integrations
- Astro route adapters
- Astro MDX integration
- Astro output mode
- Astro package pins

Those are `TS-ASTRO`.

# Audit Resolution

This plan was adversarially reviewed after the first draft.

Accepted corrections:

- `TS-CONTENT` must consume normalized content facts. It must not enforce framework validator configuration directly.
- `.plans/todo/checks/ts/content.md` is legacy background only. It leaks Velite, Contentlayer, generated artifact freshness, API route safety, and image component choices into content. Those are not final `TS-CONTENT` rules.
- `public/llms.txt` is not Astro-owned as a product rule. It belongs to a future `TS-SEO` or public-output family. Astro may expose public-file facts, but Astro should not own the product requirement long term.
- `TS-ASTRO-FILETREE-04` is currently too route-global. In strict content mode it must apply to content routes after route classification, not blindly to every Markdown/MDX route.
- Current plugin option checks are too route-global. Strict mode must derive plugin options from normalized route classes.
- `require-approved-content-adapter-in-routes` is not enough by itself because an unused adapter import can satisfy it. Strict mode needs a stronger provider-reference rule.
- The ESLint plugin import closure utility must fail closed on unresolved, unreadable, parse-error, and unsupported alias edges in strict content scopes.
- Contentlayer must be banned alongside Velite in Astro strict local content apps.
- `astro-seo` is banned, not required. The older Astro delegation plan section requiring `astro-seo` is obsolete.
- Generated legacy framework state must be explicitly rejected in Astro strict apps. This includes `.next/**`, `.velite/**`, `.contentlayer/**`, and undeclared generated content outputs.
- Runtime MDX execution must be impossible in strict Astro content apps. G3TS should require the delegated ESLint rules; it should not parse MDX or JavaScript execution patterns itself.
- Blog route contracts, metadata provenance, and JSON-LD helper contracts are Astro rules when they mention Astro routes, Astro content adapters, or Astro layouts. Rendered SEO quality remains future `TS-SEO` or Nuasite.

# Family Boundary

## `TS-ASTRO`

Owns framework setup and framework-specific content pipeline enforcement.

Already implemented or partly implemented:

- Astro package and integration stack.
- Static output.
- `@astrojs/react`, `@astrojs/mdx`, `@astrojs/check`.
- `@astrojs/sitemap`, `astro-robots`.
- `@nuasite/checks` wiring and fail-closed build checks.
- `schema-dts` for JSON-LD typing.
- `g3ts-astro-nuasite-checks` structured data presence check.
- `src/content.config.ts` existence.
- No Velite/Contentlayer style bypass in Astro apps.
- No Next generated state or Next runtime package in Astro landing/content apps.
- No direct content filesystem reads/imports/globs in routes through `g3ts-eslint-plugin-astro-pipeline`.
- No direct `astro:content` usage in routes.
- Routes must use approved content adapter modules.
- MDX ESLint lane is active.
- Inline public-copy lint is active on Astro/TS/TSX source lanes.
- `astro-seo` and related brittle SEO packages are forbidden through Syncpack.
- Existing route/global checks still need strict route-class refinement where noted below.

Should own next:

- Astro route classes for strict local content apps.
- Content route globs, chrome route globs, utility route globs, generated route globs, report shell route globs, endpoint globs.
- Approved Astro content adapter globs.
- Approved Astro MDX component registry path.
- Required blog/content route contracts when the selected profile declares a blog content domain.
- Astro metadata and JSON-LD helper wiring from typed content data.
- Astro-specific waiver matching for Astro family and Astro plugin findings.
- Effective plugin options for route classes and adapter globs.
- Strict Astro content profile in `guardrail3-rs.toml`.
- Any rule that mentions Astro collections, `.astro`, `astro:content`, or `@astrojs/*`.

## Final `TS-ASTRO` Rule Inventory To Add

These are the next Astro rules. They are intentionally Astro-specific.

### `TS-ASTRO-POLICY-01` - selected Astro policy exists

Owner: G3TS Astro config checks.

Rule:

- For every detected Astro app root, select nearest `guardrail3-rs.toml` at app root or ancestor.
- Require `[ts.astro]`.
- Require `profile = "strict-local-content"` for landing/blog/docs/public local-content apps.
- Missing, unreadable, or parse-error policy file fails closed.

Why Astro:

- The profile config names Astro route classes, Astro source lanes, and Astro content adapter paths.

Delegate:

- Parsing belongs in a shared TOML/guardrail policy parser.
- G3TS Astro owns semantic checks over parsed facts.

### `TS-ASTRO-POLICY-02` - strict local content profile has required globs

Owner: G3TS Astro config checks.

Required `[ts.astro]` keys for `strict-local-content`:

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
- `content_component_globs`
- `content_config_globs`
- `mdx_content_globs`
- `approved_mdx_component_globs`
- `approved_generated_artifact_globs`
- `contentlayer_config_globs`
- `contentlayer_generated_globs`
- `forbidden_generated_state_globs`
- `build_output_globs`
- `blog_index_route_globs`
- `blog_article_route_globs`
- `metadata_helper_globs`
- `json_ld_helper_globs`

Rule:

- Keys must exist.
- Empty is allowed only for route class overrides where default inference is explicitly defined.
- `blog_index_route_globs` and `blog_article_route_globs` may be empty only when the selected policy has no blog content domain.
- Globs must be app-relative and must not escape the app root.
- Generated globs must not overlap authored content globs.
- `forbidden_generated_state_globs` must include `.next/**`, `.velite/**`, and `.contentlayer/**` for strict Astro landing/content apps.
- `build_output_globs` must identify build output such as `dist/**` if it is present in the app root. Build output is allowed only as generated output, never as authored content or source.

Why Astro:

- These globs describe Astro route/source surfaces and plugin enforcement inputs.

Delegate:

- Glob parsing/matching should use a shared glob parser/helper.

### `TS-ASTRO-ROUTE-01` - route class globs are disjoint

Owner: G3TS Astro config checks or file-tree checks.

Route classes:

- `content`
- `chrome`
- `utility`
- `generated`
- `report_shell`
- `endpoint`

Rule:

- Explicit route class globs must not overlap each other.
- Default content inference happens only after explicit non-content classes are subtracted.
- Explicit class wins over default inference.
- Explicit-vs-explicit overlap is an error.

Why Astro:

- `.astro` routes and Astro endpoint route files are framework surfaces.

Delegate:

- Use shared glob matching.
- Do not inspect rendered HTML.

### `TS-ASTRO-ROUTE-02` - content routes are normalized and passed to ESLint

Owner: G3TS Astro config checks.

Rule:

- G3TS computes normalized content route globs/files from route class policy.
- Effective ESLint config for Astro/TS/TSX probes must pass the same normalized content route coverage to `g3ts-eslint-plugin-astro-pipeline`.
- A route class mismatch between policy and plugin options fails.

Why Astro:

- The rule connects Astro route classification to Astro-specific ESLint source rules.

Delegate:

- Source AST enforcement stays in `g3ts-eslint-plugin-astro-pipeline`.
- G3TS only proves the plugin receives correct coverage.

### `TS-ASTRO-CONFIG-08` - Astro pipeline plugin version supports strict rules

Owner: G3TS Astro config checks.

Rule:

- `g3ts-eslint-plugin-astro-pipeline` must be pinned through Syncpack at a version that contains every strict rule required by the selected `[ts.astro]` profile.
- Effective ESLint config cannot claim strict profile coverage if the installed plugin package version is too old.
- This is separate from package presence because package presence alone does not prove rule availability.

Why Astro:

- The required rules are Astro pipeline rules.

Delegate:

- Package policy remains Syncpack.
- G3TS consumes parsed Syncpack facts and package facts; it must not hand-roll semver range evaluation.

### `TS-ASTRO-ADAPTER-01` - approved content adapter globs are configured

Owner: G3TS Astro config checks.

Rule:

- `query_adapter_globs` must be non-empty for `strict-local-content`.
- `adapter_barrel_globs` must be non-empty.
- `adapter_helper_globs` may be empty only if no helpers exist.
- Effective `astro-pipeline/require-approved-content-adapter-in-routes` options must include these adapter globs or equivalent normalized module coverage.

Why Astro:

- This enforces the Astro route-to-Astro-content adapter boundary.

Delegate:

- Import traversal and AST source checks stay in ESLint plugin.

### `TS-ASTRO-ADAPTER-02` - adapter modules may use `astro:content`

Owner: `g3ts-eslint-plugin-astro-pipeline`.

Rule:

- Content routes must not import `astro:content` directly.
- Approved adapter modules may import `astro:content`.
- Adapter helper modules may not be imported directly by routes unless also matched by approved adapter globs.

Why Astro:

- `astro:content` is Astro-specific.

Delegate:

- ESLint plugin owns source semantics.
- G3TS owns plugin installation/config/effective options.

### `TS-ASTRO-ADAPTER-03` - content routes reference approved providers

Owner: `g3ts-eslint-plugin-astro-pipeline` plus G3TS effective rule checks.

Rule:

- Every normalized `content` route must reference an approved content provider in the route data path.
- An unused import from an approved adapter module does not satisfy the rule.
- A provider reference can be:
  - awaited/called in frontmatter for `.astro` routes
  - called/referenced in route `load`/handler-style source where applicable
  - passed to a typed content component as data props
- The rule must reject routes that render only local literals, static `.data.ts` objects, direct content file imports, or unused adapter imports.
- The rule must fail closed when import closure traversal cannot prove where the provider comes from.

Why Astro:

- It proves Astro content routes actually depend on the approved Astro collection adapter path.

Delegate:

- Source/reference/data-flow semantics belong in the ESLint plugin.
- G3TS must require `astro-pipeline/require-content-provider-in-content-routes` or its final rule name at `error` on normalized content route probes.

### `TS-ASTRO-CONTENT-01` - Astro content config is selected by policy

Owner: G3TS Astro file-tree checks.

Rule:

- For `strict-local-content`, every glob in `content_config_globs` must resolve to exactly one selected Astro content config unless policy explicitly allows multiple configs.
- Default expected config is `src/content.config.ts`.
- Missing config fails.
- Multiple matching configs fail unless policy permits them.

Why Astro:

- Astro content collections are configured through Astro content config.

Delegate:

- Config parsing can use Astro/TS parser facts later.

### `TS-ASTRO-CONTENT-03` - Astro content config defines direct schemas

Owner: `g3ts-eslint-plugin-astro-pipeline` plus G3TS effective rule checks.

Rule:

- Astro content collections in `src/content.config.*` must define direct schema contracts.
- Schemas must be Zod-backed through Astro's content schema surface, normally `z` from `astro:content`.
- Collection entries that use unconstrained blobs, missing schema declarations, or schema values imported from unapproved side modules fail.
- The rule should be limited to Astro content config files selected by policy.

Why Astro:

- Astro collection schema shape is Astro-specific.

Delegate:

- AST/source semantics belong in the ESLint plugin.
- G3TS only verifies that the rule is active over selected content config probes.

### `TS-ASTRO-CONTENT-02` - authored content lives under approved Astro authored content globs

Owner: G3TS Astro file-tree checks plus ESLint plugin.

Rule:

- Authored local content files for strict Astro apps must live under `authored_content_globs`.
- Files matching authored content extensions outside approved globs fail unless they are generated or waived.
- Route/source files must not import authored content files directly.

Why Astro:

- This is about the local Astro app content layout and route import boundary.

Delegate:

- File placement is G3TS file-tree.
- Import/source semantics are ESLint plugin.

### `TS-ASTRO-MDX-01` - MDX content is under approved content globs

Owner: G3TS Astro file-tree checks.

Rule:

- `.mdx` authored content must match `mdx_content_globs`.
- `.md` or `.mdx` under `src/pages` is forbidden only when the route is classified as `content`.
- Utility, generated, and report-shell route classes can allow route Markdown/MDX only when explicitly configured.
- Current `TS-ASTRO-FILETREE-04` should be retained and connected to policy globs.

Why Astro:

- Astro supports Markdown/MDX as routes; strict content apps require collection-backed MDX instead.

Delegate:

- MDX syntax/linting to `eslint-plugin-mdx` and remark.

### `TS-ASTRO-MDX-02` - MDX component registry is approved

Owner: G3TS Astro config checks and ESLint plugin.

Rule:

- `approved_mdx_component_globs` must identify the only modules allowed to provide MDX components for content rendering.
- MDX content may use only components exported by approved registry modules.
- Routes/components must not define ad hoc MDX component maps inline.

Why Astro:

- This is about Astro MDX rendering wiring and avoiding ad hoc runtime component injection.

Delegate:

- Component usage/source semantics should be ESLint plugin or `eslint-plugin-mdx`/remark where feasible.
- G3TS enforces package/plugin/config coverage.

### `TS-ASTRO-MDX-03` - runtime MDX execution is impossible

Owner: G3TS Astro config checks plus delegated ESLint rules.

Rule:

- Strict Astro content apps must have delegated source rules active that reject runtime MDX compilation or execution.
- Rejected patterns include `eval`, `new Function`, dynamic MDX compiler imports, runtime MDX evaluator imports, and source lanes that compile MDX outside Astro build/content collection flow.
- G3TS must prove the delegated rules are active on normalized Astro source lanes.
- G3TS must not implement JavaScript or MDX execution-pattern detection itself.

Why Astro:

- This protects Astro's static content pipeline from being bypassed by runtime MDX execution.

Delegate:

- Source detection belongs in ESLint core rules, TypeScript ESLint rules, and `g3ts-eslint-plugin-astro-pipeline/no-runtime-mdx-eval`.
- G3TS only verifies rule presence, effective severity, source lane coverage, and bypass policy.

### `TS-ASTRO-COPY-01` - public copy rule covers Astro source lanes

Owner: G3TS Astro config checks.

Rule:

- `i18next/no-literal-string` must be active on normalized public Astro/TS/TSX source lanes.
- Config must use strict options already required by `TS-ASTRO-CONFIG-19`.
- Future policy must derive source lane globs from route classes instead of hand-coded app paths.

Why Astro:

- The lane selection is Astro-specific. The literal-copy rule is delegated to an ESLint plugin.

Delegate:

- Literal detection stays in `eslint-plugin-i18next`.

### `TS-ASTRO-PARALLEL-01` - Contentlayer is absent from Astro strict apps

Owner: G3TS Astro config/file-tree checks and Astro ESLint plugin.

Rule:

- Syncpack must ban `contentlayer`, `next-contentlayer`, and other selected Contentlayer package names for strict Astro apps.
- File tree must reject `contentlayer.config.*`, `.contentlayer/**`, and generated Contentlayer output globs in Astro app roots.
- Source rules must reject imports from Contentlayer packages or generated Contentlayer modules in Astro route/import closures.

Why Astro:

- Contentlayer is a parallel content pipeline that bypasses Astro collections in Astro strict apps.

Delegate:

- Package policy through Syncpack.
- File-tree in G3TS.
- Source imports through ESLint plugin.

### `TS-ASTRO-FILETREE-11` - legacy generated framework state is absent

Owner: G3TS Astro file-tree checks.

Rule:

- Strict Astro landing/content app roots must not contain `.next/**`, `.contentlayer/**`, `contentlayer.config.*`, or other non-Velite paths listed in `forbidden_generated_state_globs`.
- `.velite/**` remains covered by existing `TS-ASTRO-FILETREE-06` until Velite checks are consolidated.
- `dist/**` is allowed only when matched by `build_output_globs`; it must not be treated as authored content, generated content data, or source.
- Generated output directories must be ignored or classified as build output. They must not satisfy content, route, adapter, or MDX requirements.

Why Astro:

- `.next`, `.velite`, and `.contentlayer` prove a parallel framework/content pipeline exists or existed inside an app that is supposed to be strict Astro.

Delegate:

- File-tree ownership stays in G3TS.
- Package bans stay in Syncpack.
- Source imports from those systems stay in ESLint plugin rules.

### `TS-ASTRO-BLOG-01` - declared blog domains have Astro route contracts

Owner: G3TS Astro config/file-tree checks plus ESLint plugin effective option checks.

Rule:

- When the selected `[ts.astro]` profile declares a blog content domain, `blog_index_route_globs` and `blog_article_route_globs` must resolve to real Astro route files.
- The default public contract is a blog index route equivalent to `/blog` and an article route equivalent to `/blog/[slug]`.
- Equivalent route names are allowed only when explicitly declared in policy.
- Blog route files must be classified as `content` routes and must satisfy `TS-ASTRO-ADAPTER-03`.
- Blog routes must not import `.md`, `.mdx`, JSON, or static `.data.ts` content directly.

Why Astro:

- This is an Astro route-shape and Astro content-adapter contract.

Delegate:

- Route discovery and class checks stay in G3TS.
- Source import/provider checks stay in the ESLint plugin.

### `TS-ASTRO-META-01` - public route metadata comes from typed content data

Owner: `g3ts-eslint-plugin-astro-pipeline` plus G3TS effective rule checks.

Rule:

- Public content routes must pass title, description, canonical path, and indexability metadata from approved typed content data or adapter output.
- Route and layout files must not contain hardcoded public SEO title or description literals.
- Shared layout components must require metadata props instead of defining default public SEO copy.
- The rule must reject local metadata literals, route-local static metadata objects, and fallback layout titles/descriptions that can mask missing content metadata.

Why Astro:

- It binds Astro routes/layouts to Astro content adapter data.

Delegate:

- Literal/source provenance detection belongs in the ESLint plugin and `i18next/no-literal-string`.
- Rendered title/meta correctness belongs in Nuasite and future `TS-SEO`.
- G3TS verifies package presence, rule severity, options, source lane coverage, and bypass policy.

### `TS-ASTRO-JSONLD-01` - JSON-LD helper is typed and not string-built

Owner: G3TS Astro config checks plus ESLint plugin and TypeScript.

Rule:

- `json_ld_helper_globs` must resolve to approved helper/component modules for strict public content apps.
- Approved helpers must construct JSON-LD as typed `schema-dts` objects and render it as `application/ld+json`.
- String-built JSON-LD blobs, raw template-string JSON-LD, and untyped `Record<string, unknown>` public structured data are rejected.
- Public content routes must use the approved helper or pass structured data through approved typed layout props.

Why Astro:

- The helper is wired through Astro public routes/layouts and must consume Astro content data.

Delegate:

- Type checking belongs to TypeScript.
- Source-shape detection belongs in ESLint plugin rules.
- Rendered JSON-LD presence and parseability belongs in `g3ts-astro-nuasite-checks`, Nuasite, and future `TS-SEO`.
- G3TS verifies `schema-dts` package policy, helper globs, delegated rule activation, and rendered-check script wiring.

### `TS-ASTRO-BYPASS-01` - Astro ESLint bypasses are visible

Owner: G3TS Astro policy checks.

Rule:

- Inline `eslint-disable` for `astro-pipeline/*`, `i18next/no-literal-string`, `mdx/remark`, or `astro/*` in Astro app source must produce G3TS findings.
- A finding can be allowed only through `guardrail3-rs.toml` waiver with exact file and selector.
- Broad disables fail.
- Stale waivers fail.

Why Astro:

- This controls bypasses of Astro-owned delegated validators.

Delegate:

- Parsing ESLint directive comments belongs in shared parser.
- G3TS Astro owns waiver matching.

### `TS-ASTRO-BYPASS-02` - ESLint ignores cannot skip Astro content lanes

Owner: G3TS Astro config checks.

Rule:

- Effective ESLint probes for representative Astro, TS, TSX, and MDX content files must not be ignored.
- Broad `ignores` patterns that cover content routes, adapters, authored content, or MDX content fail unless generated globs explain them.

Why Astro:

- The required probes and route/content lanes are Astro-specific.

Delegate:

- Effective config evaluation stays in shared ESLint parser/runtime.

### `TS-ASTRO-BYPASS-03` - import closure failures fail closed

Owner: `g3ts-eslint-plugin-astro-pipeline`.

Rule:

- In strict content scopes, import closure traversal must not silently drop unresolved imports, unreadable files, parse errors, unsupported extensions, or unsupported aliases.
- A strict rule that depends on import closure traversal must report an error when traversal cannot prove the closure is safe.

Why Astro:

- The closure is used to enforce Astro route-to-content boundaries.

Delegate:

- ESLint plugin owns import graph traversal.
- G3TS verifies strict rules are active.

### `TS-ASTRO-GENERATED-01` - generated artifacts are declared and isolated

Owner: G3TS Astro file-tree checks.

Rule:

- Generated artifacts must match `approved_generated_artifact_globs`.
- Generated artifacts must not live under authored content globs.
- Authored source must not edit/import generated artifacts except through approved report shell or generated route classes.

Why Astro:

- This describes how strict Astro apps host static generated/report artifacts.

Delegate:

- File-tree ownership in G3TS.
- Source import checks in ESLint plugin if needed.

### `TS-ASTRO-REPORT-01` - report shell routes are explicitly classified

Owner: G3TS Astro config/file-tree checks.

Rule:

- Routes that host immutable report artifacts must match `report_shell_route_globs`.
- Report shell routes are not treated as content routes.
- Report payload/artifact roots must be declared in generated artifact globs.

Why Astro:

- It describes Astro route classification for the static report use case.

Delegate:

- Artifact immutability policy may later move to a report/artifact family if it becomes framework-independent.

### `TS-ASTRO-FILETREE-10` - legacy `.eslintignore` cannot hide Astro probes

Owner: G3TS Astro file-tree/config checks.

Rule:

- Reject `.eslintignore` in the Astro app root or selected ancestors.
- Reject app-local ESLint ignore surfaces that skip content routes, adapters, authored content, content config, or MDX content probes unless they match approved generated globs.

Why Astro:

- Astro requires effective delegated source validators over specific Astro lanes.

Delegate:

- Effective config/probe logic stays in ESLint parser/runtime.

### `TS-ASTRO-SOURCE-01` - inline ESLint disables cannot hide Astro delegated rules

Owner: G3TS Astro source/policy checks.

Rule:

- Inline disables for `astro-pipeline/*`, `i18next/no-literal-string`, `mdx/remark`, and `astro/*` produce G3TS findings.
- Accepted exceptions must be represented as exact waivers in the selected guardrail policy file.
- Broad disables are errors.
- Stale waivers are errors.

Why Astro:

- These disables bypass Astro-owned delegated validators.

Delegate:

- ESLint directive comment parsing belongs in a shared parser.
- Astro owns matching directives to Astro delegated rules and waiver policy.

### `TS-ASTRO-ALIASES-01` - plugin-resolvable aliases only

Owner: G3TS Astro config checks, consuming shared tsconfig parser facts.

Rule:

- Strict Astro content apps may use only import aliases the Astro pipeline plugin can resolve.
- Allowed initially: relative imports, `@/` to `src/`, `~/` to `src/`, `src/`.
- Any other local TS path alias fails until the plugin resolver supports it.

Why Astro:

- This protects Astro plugin import-closure rules from silent bypass.

Delegate:

- `tsconfig` parsing stays in shared parser / TS config family.
- Astro owns this specific compatibility requirement.

## `TS-CONTENT`

Owns authored-content product invariants that survive framework replacement.

It should not know whether the implementation is Astro, Next, Vite, Vike, or a static generator.

Should own:

- Content product roots are explicitly declared or routed from shared app classification.
- Authored content entries have stable identity fields:
  - `slug`
  - `title`
  - `description`
  - `status`
  - `publishedAt`
  - `updatedAt` when required
  - `canonicalPath`
  - `tags` when the product uses tags
  - `authors` when the product uses authors
- Slugs are unique within a content collection or declared content domain.
- Canonical paths are unique across public content.
- Drafts are not public unless the app is explicitly in preview mode.
- Published content has valid dates.
- `updatedAt` is not earlier than `publishedAt`.
- Required landing-page blocks are present and non-empty.
- Required blog post frontmatter is present and non-empty.
- Required docs frontmatter is present and non-empty.
- Referenced content assets exist.
- Images have required alt/caption fields according to content type.
- Internal content links resolve.
- Internal content anchors resolve where feasible.
- External links follow the declared link policy.
- Rich text uses only approved portable component names or shortcodes.
- Unsafe HTML in authored content is rejected unless waived.
- Generated content artifacts are declared as generated and are not hand-authored.

Fact producers should delegate to existing tools where possible:

- Content schema validation can come from JSON Schema, Zod, or framework-owned schema output selected by the implementing framework family.
- Markdown/MDX AST facts can come from remark/rehype plugins where possible.
- Link facts can come from a dedicated link checker or remark plugin where possible.
- Spelling: `TS-SPELLING`, not `TS-CONTENT`.
- SEO tag rendering: `TS-SEO`/framework rendered-output validator, not `TS-CONTENT`.

`TS-CONTENT` can require framework-neutral facts such as `schema_validated = true`, `entry_kind`, `asset_refs`, and `internal_links`.

`TS-CONTENT` cannot require a specific framework validator package or config file.

Must not own:

- Which framework reads content.
- Which framework validates content schemas.
- Which build tool emits pages.
- Which route file imports content.
- How Astro collections are configured.
- How MDX is wired into Astro.

## `TS-SEO`

Owns public-route SEO product invariants that can be checked independently of Astro once a framework family exposes route/render facts.

For Astro today, much SEO setup is enforced by `TS-ASTRO` because the enforcement is Astro-specific:

- Astro sitemap integration.
- Astro robots integration.
- Nuasite Astro integration.
- Astro static output.

Future `TS-SEO` should own framework-neutral checks:

- Every public page has title and description.
- Canonical URLs are unique and valid.
- Public indexability policy is explicit.
- Required JSON-LD types exist for declared page kinds.
- Sitemap contains expected public routes.
- Robots policy is explicit and consistent.
- `public/llms.txt` or equivalent LLM discovery artifact exists when the public-output profile requires it.

`TS-SEO` should consume facts from framework families or rendered-output validators. It should not parse Astro config directly.

## `TS-CSS` / Style Family

Owns style and design-system policy.

Not Astro:

- Tailwind arbitrary value bans.
- Tailwind class policy.
- Design token usage.
- Stylelint setup.
- Shared utility class reuse.
- Component visual duplication.

# Implementation Order

## Phase 1: Finish `TS-ASTRO` Strict Content Profile

Reason: current target is Astro landing/blog/MDX apps. The missing enforcement is still Astro-specific.

Deliverables:

- Add selected Astro policy parsing from `guardrail3-rs.toml`.
- Add `strict-local-content` profile facts.
- Add route class normalization.
- Add disjoint route class validation.
- Add plugin option enforcement for route class globs and approved adapter globs.
- Add Astro waiver policy skeleton, even if only G3TS findings are supported first.
- Extend `g3ts-eslint-plugin-astro-pipeline` only for source semantics G3TS should not parse.
- Keep all Astro collection and MDX wiring rules in `TS-ASTRO`.

## Phase 2: Build Minimal `TS-CONTENT`

Reason: only after framework-specific ingestion exposes content facts can content become framework-independent.

Deliverables:

- Define `ContentEntry` facts independent of Astro:
  - `root_rel_path`
  - `content_domain`
  - `entry_rel_path`
  - `entry_kind`
  - `slug`
  - `canonical_path`
  - `status`
  - `title_present`
  - `description_present`
  - `published_at`
  - `updated_at`
  - `asset_refs`
  - `internal_links`
  - `rich_component_refs`
- Define who produces those facts.
  - For Astro, `TS-ASTRO` ingestion can later expose normalized content facts.
  - For Next/Vite/Vike, those families can expose equivalent facts later.
- Implement pure content rules over those facts:
  - unique slugs
  - unique canonical paths
  - required metadata fields
  - draft/public policy
  - date consistency
  - asset refs exist
  - internal links resolve
  - rich component allowlist
- Do not parse Astro collection config inside `TS-CONTENT`.

## Phase 3: Build `TS-SEO`

Reason: SEO should consume route/render facts after Astro/Nuasite and content facts are stable.

Deliverables:

- Public page facts.
- Required metadata facts.
- Canonical URL facts.
- JSON-LD facts.
- Sitemap/robots facts from framework or rendered-output validator.
- Rules that are independent of Astro config.

## Phase 4: Build Style And Spelling

Reason: landing/content apps need style discipline and copy quality, but these are not Astro concerns.

Deliverables:

- `TS-CSS` for Tailwind/stylelint/design-token policy.
- `TS-SPELLING` for cspell or equivalent delegated spell checking.

# Immediate Next Work

Do not start `TS-CONTENT` by adding Astro collection rules.

Start with `TS-ASTRO` strict content profile:

1. Policy parser support for `[ts.astro]`.
2. Route class facts.
3. Route class disjointness checks.
4. Effective ESLint plugin option checks derived from normalized route classes.
5. Astro waiver skeleton.

Then build the minimal `TS-CONTENT` facts and rules only after there is a framework-neutral `ContentEntry` input shape.
