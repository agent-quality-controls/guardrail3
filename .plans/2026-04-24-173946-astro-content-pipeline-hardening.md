# Astro Content Pipeline Hardening Plan

## Status

This plan supersedes the first draft after adversarial review.

The first draft had real architecture defects:

- It treated every Astro route as content-backed.
- It made literal-size checks universal.
- It put waiver matching into config checks.
- It grouped too many plugin-effectiveness assertions into one G3TS rule.
- It allowed string matching for shell/script semantics.
- It did not define stable waiver selectors.
- It did not prove MDX ESLint execution.
- It did not prove packed npm artifacts before publish.

This version fixes those points.

## Goal

Make strict Astro content apps stable under agent management.

The target apps are:

- landing sites
- blogs
- docs sites
- static report shells that load static content
- other local-file Astro content apps

The target does not include arbitrary Astro apps by default. Non-content routes and non-local content pipelines must be declared through route classes or policy profiles, not discovered by guesswork.

## Explicit Scope Exception

`AGENTS.md` says the active roadmap is Rust-only unless explicitly asked.

This plan is an explicit TypeScript/G3TS exception because the current user request is specifically about Astro/G3TS content pipeline guardrails.

## Core Principle

Hard rules are correct only after the app surface is classified.

The plan enforces:

- hard source-boundary rules on `content` routes
- hard setup/wiring rules on Astro content apps
- hard bypass visibility rules everywhere the Astro pipeline is required
- guardrail waivers for real exceptions

The plan does not enforce content-route rules on every Astro file globally.

## Route Classes

Every Astro route belongs to one class.

Classes:

- `content`: page content must come from Astro collections through approved adapters.
- `chrome`: app shell, layouts, homepage chrome, or static scaffolding with no authored content.
- `utility`: `404`, redirects, robots, health pages, status pages.
- `generated`: generated source or generated route output.
- `report_shell`: static report host/shell pages whose report payload is an immutable generated artifact.
- `endpoint`: JSON, XML, RSS, sitemap, API-like endpoints.

Default inference:

- `src/pages/**/*.astro` is `content` unless matched by another configured class.
- `src/pages/404.astro` is `utility`.
- endpoint files are `endpoint`.
- files under approved generated roots are `generated`.

Policy override:

- Route classes are configured in the selected guardrail policy file.
- Explicit route class globs must be disjoint.
- Default inference applies only after explicit class globs.
- Explicit-vs-explicit overlap is an error.
- Default-vs-explicit overlap is not an error; explicit class wins.
- There is no "more specific by policy order" behavior.
- Content-source rules apply only to `content` routes unless a rule explicitly says otherwise.

Planned policy shape in `guardrail3-rs.toml`:

```toml
[ts.astro]
profile = "strict-local-content"
authored_content_globs = ["src/content/**"]
content_route_globs = []
chrome_route_globs = []
utility_route_globs = ["src/pages/404.astro"]
generated_route_globs = []
report_shell_route_globs = []
endpoint_globs = ["src/pages/**/*.{ts,js,mts,mjs}"]
content_data_module_globs = ["src/**/*.data.{ts,tsx,js,jsx,mts,cts,mjs,cjs}"]
query_adapter_globs = ["src/lib/content/adapters/**/*.{ts,tsx}"]
adapter_barrel_globs = ["src/lib/content/adapters/index.ts"]
adapter_helper_globs = ["src/lib/content/adapters/_helpers/**/*.{ts,tsx}"]
route_registry_globs = []
content_component_globs = ["src/components/content/**/*.{tsx,astro}"]
content_config_globs = ["src/content.config.*"]
mdx_content_globs = ["src/content/**/*.mdx"]
approved_mdx_component_globs = ["src/lib/content/mdx-components.{ts,tsx}"]
approved_generated_artifact_globs = ["src/generated/**"]
astro_content_type_import_globs = ["src/lib/content/types/**/*.{ts,tsx}"]
```

`profile = "strict-local-content"` means:

- local authored content lives under `src/content/**`
- `src/content.config.*` is required
- Velite and Contentlayer are forbidden
- public content routes must reach approved providers
- ESLint bypasses are audited

Remote CMS support is deferred. A remote-loader app must use a different profile later.

`content_route_globs = []` means content routes are inferred by default after explicit non-content classes are subtracted. If `content_route_globs` is non-empty, it is treated as explicit and must not overlap any other explicit route class. Plugin-facing content route globs are the normalized class result, not the raw policy list.

## Exception Model

Accepted exception channel:

- selected guardrail policy file
- `[[waivers]]`

Rejected independent exception channels:

- inline `eslint-disable`
- broad ESLint config ignores
- package scripts that skip required lanes
- untracked generated output directories

Waiver fields:

```toml
[[waivers]]
rule = "TS-ASTRO-SOURCE-01"
file = "apps/landing/src/pages/index.astro"
selector = "eslint-disable-next-line:astro-pipeline/no-static-content-data-in-content-components:line=12"
reason = "Temporary migration waiver until landing copy moves into src/content/pages/home.json."
```

Matching:

- `rule` must match the G3TS rule ID or plugin rule ID being waived.
- `file` is normalized relative to the selected policy root.
- resolved waiver file must be inside the selected app root.
- `selector` must match the canonical finding selector.
- stale waivers are errors.
- duplicate waivers for the same finding are errors.
- broad selectors such as `*`, directory paths, or `astro-pipeline/*` are errors unless the finding is under approved generated artifact globs.

Waiver application stage:

- Source/config/filetree checks never consume waivers and never suppress their own findings.
- Plugin lint findings are ingested into G3TS as typed pre-policy `AstroPluginFinding` facts before policy checks if plugin waivers are supported in this slice.
- `TS-ASTRO-POLICY-01` consumes pre-policy Astro findings plus parsed waivers.
- Policy matching emits inventory for matched findings and errors for stale/missing/duplicate/broad waivers.
- Runner output still includes the original finding and an inventory item showing it is waived, unless the final report layer later adds explicit suppression support.
- "All Astro family findings" means config, filetree, source, and ingested plugin findings before policy checks. It excludes policy-check findings to avoid cycles.
- If plugin finding ingestion is not implemented in this slice, policy waivers apply only to G3TS findings and the implementation must not claim plugin waivers are supported.

Selected policy file:

- For each Astro app root, select the nearest `guardrail3-rs.toml` at the app root or an ancestor.
- Record `policy_rel_path`, `policy_root_rel_path`, and `app_root_rel_path`.
- A waiver in a root policy can target one app only by using the app-relative path under the policy root.
- Invalid or unreadable selected policy files fail closed.

## Ownership Boundaries

### ESLint Plugin

`eslint-plugin-astro-pipeline` owns source semantics:

- Astro/TS/TSX/MDX AST rules
- import closure traversal
- content adapter/source boundary checks
- static data module checks
- MDX component import checks

The plugin may read source files for import closure traversal. It must not depend on G3TS internals.

### Shared Parsers

Packages under `packages/parsers` own parsing:

- ESLint effective config
- package.json dependency specs
- guardrail policy files
- ESLint directive comments
- package script command tokens

Astro ingestion consumes parser outputs. It must not parse config files directly.

### G3TS Astro Family

`packages/ts/astro` owns:

- Astro-specific package policy
- Astro-specific filetree policy
- route class normalization
- ESLint plugin install/version/effective rule checks
- policy/waiver matching for Astro findings
- fail-closed handling when required parser facts are missing

### Future Renderer Validator

Rendered HTML checks are deferred:

- title
- meta description
- canonical
- hreflang
- Open Graph/Twitter
- JSON-LD
- internal links
- generated report artifact invariants

These belong in a future Astro integration or post-build validator, not in this source-policy slice.

## Plugin Option Semantics

All new option names ending in `Globs` are app-relative file globs.

The plugin resolves imports to files before matching globs.

Supported import resolution for this slice:

- relative paths
- `@/` to `src/`
- `~/` to `src/`
- `src/`

Custom TS path aliases are not supported in strict Astro content apps until resolver support is implemented.

G3TS must fail strict Astro content apps that configure other local path aliases in `tsconfig.json`, or the plugin must be extended to resolve them before those aliases are allowed.

Compatibility:

- Existing options such as `routeGlobs`, `approvedContentAdapterModules`, and `approvedGeneratedArtifactRoots` remain for old rules for one release.
- New strict rules use `contentRouteGlobs`, `queryAdapterGlobs`, `adapterBarrelGlobs`, `adapterHelperGlobs`, `approvedGeneratedArtifactGlobs`, and similar `*Globs` names.
- G3TS normalizes both old and new option names into typed facts.

## ESLint Plugin Rules To Add

### `astro-pipeline/require-content-provider-in-content-routes`

Purpose:

- A `content` route must import and reference an approved content provider.

Scope:

- files matching `contentRouteGlobs`

Provider surfaces:

- `queryAdapterGlobs`
- `adapterBarrelGlobs`
- `routeRegistryGlobs`

Fail when:

- route imports no approved provider
- route has only an unused provider import
- route uses only side-effect import from a provider
- route imports through unsupported alias
- route imports a provider path that matches no existing file

Pass when:

- route imports an approved provider and references an imported binding
- route imports an approved registry and references the registry binding
- route is not a `content` route

Implementation notes:

- This is not a dataflow proof.
- It is a structural contract: content routes must visibly depend on the content provider surface.
- Import closure traversal is used only after static resolution succeeds.
- Side-effect imports do not count.
- Imported provider bindings must be referenced in the route module.

Landing failure this should catch:

- `src/pages/index.astro` imports `../app/page` and never imports an approved content provider.

### `astro-pipeline/no-astro-content-value-imports-outside-query-adapters`

Purpose:

- Runtime/value access to `astro:content` must stay in query adapters and content config.

Scope:

- all linted Astro/TS/TSX/MDX source files

Allowed:

- files matching `queryAdapterGlobs`
- files matching `contentConfigGlobs`

Type-only imports:

- allowed only in files matching `astroContentTypeImportGlobs`
- otherwise flagged separately with an explanatory message

Fail when:

- non-allowed file value-imports `astro:content`
- non-allowed file re-exports value bindings from `astro:content`
- non-allowed file dynamically imports or requires `astro:content`

Pass when:

- query adapter imports `getCollection`, `getEntry`, `getEntries`, or `render`
- content config imports Astro content helpers
- configured type surface imports only types

Compatibility:

- Keep `no-direct-astro-content-in-routes` for old configs for one release.
- New G3TS strict checks require this global rule.

### `astro-pipeline/query-adapter-must-query-astro-content`

Purpose:

- Query adapters must actually use Astro collection helpers.

Scope:

- files matching `queryAdapterGlobs`

Fail when:

- file imports no value helper from `astro:content`
- file only imports types from `astro:content`
- imported Astro helper binding is never referenced by an exported function or exported const initializer
- adapter imports only approved generated artifacts without a generated-artifact provenance rule enabled

Pass when:

- exported adapter function references `getCollection`, `getEntry`, `getEntries`, or `render`
- exported adapter const references those helpers

Accepted export/reference forms:

- `export async function load() { return getCollection(...) }`
- `export const load = async () => getCollection(...)`
- `async function load() { return getCollection(...) }; export { load }`
- `export default async function load() { ... }`
- one-hop local helper calls count only when the exported function calls a local helper and that helper references the imported Astro content binding.
- imported Astro content helper aliases count when the binding resolves to `astro:content`.

Out of scope:

- proving every returned field is derived from collection data
- proving semantic correctness of mappings

### `astro-pipeline/no-static-content-data-in-content-route-closures`

Purpose:

- Content route import closures cannot pull authored page copy from static data modules.

Scope:

- files matching `contentRouteGlobs`
- their resolved import closures

Fail when route closure reaches:

- files matching `contentDataModuleGlobs`
- modules exporting module-scope object or array literals with content-like string fields
- raw authored content files outside approved generated artifacts

Content-like fields:

- `title`
- `heading`
- `body`
- `copy`
- `description`
- `items`
- `sections`
- `blocks`
- `cta`
- `label` when nested under content arrays/objects

Pass when:

- static UI config contains no content-like fields
- route imports content provider and components only
- short enum/discriminant fields such as `type`, `tone`, `variant`, `align`

This is a structural static-data rule, not a universal large-literal rule.

### `astro-pipeline/no-static-content-data-in-content-components`

Purpose:

- Configured content components are prop-driven and must not own page copy modules.

Scope:

- files matching `contentComponentGlobs`
- their resolved local import closures

Fail when:

- component defines or imports module-scope content-like object/array literals
- component imports files matching `contentDataModuleGlobs`
- component closure reaches static page-copy data modules

Pass when:

- component renders text from props
- component owns small UI chrome strings
- component owns accessibility labels below the configured small-label threshold
- component is outside `contentComponentGlobs`

Literal threshold checks:

- optional profile feature, not universal.
- If enabled, thresholds must be configured under `[ts.astro.literal_policy]`.
- Default strict profile does not fail arbitrary JSX text solely by word count.
- Without literal policy, this rule flags only structural static data modules and exported/module-scope content-like objects.
- It must not flag local JSX text or inline UI chrome by default.

Landing failure this should catch:

- `homepage-v2.data.ts` exports static content arrays.
- landing components import those arrays.

### `astro-pipeline/no-static-content-data-in-query-adapters`

Purpose:

- Query adapters transform content entries; they do not author static page blocks.

Scope:

- files matching `queryAdapterGlobs`

Fail when:

- adapter exports module-scope content-like object/array literals
- adapter returns a content-like static object/array literal without referencing a collection entry or Astro content helper in the same exported function

Pass when:

- adapter maps collection entry fields to view models
- adapter defines short collection names
- adapter defines short error messages
- adapter defines discriminated union mappers

Out of scope:

- full provenance/dataflow

Candidate narrowing for static-content-data rules:

- A violation candidate must match at least one of:
  - file matches `contentDataModuleGlobs`
  - exported top-level const/object/array name matches `*CONTENT*`, `*SECTIONS*`, `*BLOCKS*`, `*PAGE*`, `*COPY*`, or configured content data names
  - object/array contains at least two content-like keys and at least one long authored string field
- Single `label`, `items`, `description`, nav/menu config, enum/discriminant config, variant maps, and UI chrome objects are false positives unless combined with the stronger criteria above.
- Add explicit false-positive tests for nav links, menus, variants, aria labels, empty-state strings, and test fixtures outside content scopes.

### `astro-pipeline/mdx-imports-from-approved-component-globs`

Purpose:

- Authored MDX can import only approved MDX component surfaces.

Scope:

- files matching `mdxContentGlobs`

Options:

- `approvedMdxComponentGlobs`

Semantics:

- Empty `approvedMdxComponentGlobs` means no MDX component imports are allowed.
- Non-empty allowlist permits imports resolved to matching files.

Fail when:

- MDX imports local components outside `approvedMdxComponentGlobs`
- MDX imports arbitrary helpers
- MDX import cannot be resolved

Pass when:

- MDX imports approved registry
- MDX has no component imports

Required proof:

- test with real `.mdx` filename and real MDX parser config.
- G3TS must add an `MdxContent` probe and require this rule effective on discovered MDX content files.

### `astro-pipeline/require-direct-content-collection-schemas`

Purpose:

- `src/content.config.*` must declare schemas in a simple, agent-auditable shape.

Scope:

- files matching `contentConfigGlobs`

Accepted style:

```ts
const pages = defineCollection({
  schema: pageSchema,
});

export const collections = { pages };
```

Fail when:

- `defineCollection()` argument is not a direct object literal
- direct object literal has no own `schema` key
- `schema` is `undefined`
- config exports no `collections`
- collection is declared through spread or imported factory
- `defineCollection` does not resolve to an import from `astro:content`
- `collections` is exported through an unsupported shape

Reason:

- This is intentionally strict for agent-managed projects. Indirection is not worth the audit cost.

Pass when:

- schema is inline
- schema is imported
- schema is a local identifier
- schema is a function schema
- `defineCollection` is imported directly or through a local import alias from `astro:content`
- valid export shapes:
  - `export const collections = { pages, blog }`
  - `const collections = { pages, blog }; export { collections }`

## ESLint Plugin Utility Work

Files:

- `packages/ts/eslint-plugin-astro-pipeline/src/utils/options.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/import-closure.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/module-role.ts`
- `packages/ts/eslint-plugin-astro-pipeline/src/utils/path-policy.ts`
- new `src/utils/static-content-data.ts`
- new `src/utils/mdx-imports.ts`

Required changes:

- Extend option schema beyond string arrays:
  - string arrays
  - optional booleans
  - optional numeric thresholds for opt-in literal policy
- Normalize app-relative file globs.
- Resolve import specifiers to concrete file paths before matching.
- Return explicit unresolved/unsupported import states where a rule needs fail-closed behavior.
- Change `collectImportClosure()` to return typed edge states instead of silently dropping unresolved edges:
  - `resolved`
  - `external`
  - `unsupported_alias`
  - `missing_file`
  - `unreadable`
  - `parse_error`
- Closure-based strict rules must define behavior for every edge state:
  - unsupported local aliases fail in strict content scopes
  - missing/unreadable/parse-error local imports fail in strict content scopes
  - external package imports are ignored unless they are known forbidden content packages
- Add tests for Windows path separators.
- Add tests for aliases supported by the plugin.
- Add tests for unsupported aliases.

## G3TS Parser Work

### `eslint-config-parser`

Files:

- `packages/parsers/eslint-config-parser/crates/types/src/document.rs`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser.rs`
- `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/parsers/eslint-config-parser/crates/assertions/src/parser.rs`

Add facts:

```rust
pub struct EslintEffectiveConfigProbe {
    pub probe: EslintProbeKind,
    pub rel_path: String,
    pub ignored: bool,
    pub plugins: Vec<String>,
    pub rules: BTreeMap<String, EslintRuleSetting>,
    pub project_service: Option<bool>,
    pub linter_options_no_inline_config: Option<bool>,
    pub linter_options_report_unused_disable_directives: Option<EslintReportUnusedSetting>,
    pub linter_options_report_unused_inline_configs: Option<EslintReportUnusedSetting>,
}

pub enum EslintReportUnusedSetting {
    Off,
    Warn,
    Error,
}

pub enum EslintProbeKind {
    AstroSource,
    TsSource,
    TsxSource,
    MdxContent,
    AstroContentConfig,
    TsTest,
    JsSource,
    ConfigFile,
}
```

Important:

- Do not drop ignored probes.
- Keep `ignored` in typed facts.
- Astro ingestion decides whether ignored is allowed.

Tests:

- no inline config true
- report unused disable directives error
- report unused inline configs error
- missing linter options
- ignored probe retained
- MDX probe retained
- malformed helper payload fails closed

### `package-json-parser`

Files:

- `packages/parsers/package-json-parser/crates/types/src/document.rs`
- `packages/parsers/package-json-parser/crates/runtime/src/parser.rs`

Add dependency spec facts:

```rust
pub struct PackageDependencySpec {
    pub name: String,
    pub raw_spec: String,
    pub section: PackageDependencySection,
    pub parsed: PackageDependencySpecParseState,
}

pub enum PackageDependencySpecParseState {
    Exact { version: SemverVersion },
    Range { minimum: Option<SemverVersion>, allows_below_minimum_unknown: bool },
    Workspace { raw: String },
    File { raw: String },
    Link { raw: String },
    Catalog { raw: String },
    Unsupported { raw: String, reason: String },
}
```

Keep existing fields:

- `dependencies`
- `dev_dependencies`

Add:

- `dependency_specs`
- or `dependencies_by_name` and `dev_dependencies_by_name` plus helper parser output

Tests:

- exact minimum
- below minimum
- caret/range
- prerelease
- workspace
- file
- link
- catalog
- dependency and devDependency
- invalid non-string version

### `guardrail3-rs-toml-parser`

Files:

- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- runtime parser tests

Add typed optional TS Astro policy:

```rust
pub struct Guardrail3RsToml {
    ...
    pub ts: Option<TsPolicyConfig>,
}

pub struct TsPolicyConfig {
    pub astro: Option<TsAstroPolicyConfig>,
}

pub struct TsAstroPolicyConfig {
    pub profile: Option<String>,
    pub authored_content_globs: Vec<String>,
    pub content_route_globs: Vec<String>,
    pub chrome_route_globs: Vec<String>,
    pub utility_route_globs: Vec<String>,
    pub generated_route_globs: Vec<String>,
    pub report_shell_route_globs: Vec<String>,
    pub endpoint_globs: Vec<String>,
    pub content_data_module_globs: Vec<String>,
    pub query_adapter_globs: Vec<String>,
    pub adapter_barrel_globs: Vec<String>,
    pub adapter_helper_globs: Vec<String>,
    pub route_registry_globs: Vec<String>,
    pub content_component_globs: Vec<String>,
    pub content_config_globs: Vec<String>,
    pub mdx_content_globs: Vec<String>,
    pub approved_mdx_component_globs: Vec<String>,
    pub approved_generated_artifact_globs: Vec<String>,
    pub astro_content_type_import_globs: Vec<String>,
}
```

Existing `waivers` stay top-level.

Tests:

- policy parses
- absent `[ts.astro]` parses as `None` or empty typed fields
- waiver fields parse
- unknown fields remain in `extra`

Strict-local defaulting belongs to Astro ingestion/policy normalization tests, not parser tests.

### New `eslint-directive-parser`

Location:

- `packages/parsers/eslint-directive-parser`

Purpose:

- Parse comments from JS/TS/TSX/Astro/MDX source into typed ESLint directive facts.

Facts:

```rust
pub struct EslintDirectiveFileState {
    pub rel_path: String,
    pub state: EslintDirectiveParseState,
}

pub enum EslintDirectiveParseState {
    Parsed { findings: Vec<EslintDirectiveFinding> },
    Unsupported { reason: String },
    ParseError { reason: String },
    Ambiguous { reason: String },
}

pub struct EslintDirectiveFinding {
    pub rel_path: String,
    pub directive_kind: EslintDirectiveKind,
    pub disabled_rules: EslintDisabledRuleSet,
    pub line: u32,
    pub target_line: Option<u32>,
    pub canonical_selector: String,
}

pub enum EslintDirectiveKind {
    Disable,
    DisableLine,
    DisableNextLine,
    Enable,
}

pub enum EslintDisabledRuleSet {
    AllRules,
    Rules(Vec<String>),
}
```

Parser requirements:

- skip strings
- skip template literal bodies unless they contain comments in expression code
- support block and line comments
- support comma-separated rule IDs
- support disable descriptions after `--`
- support Astro frontmatter comments
- support MDX comments if parser can expose them; otherwise scan raw MDX with documented limitations and fail closed on ambiguous directives

Tests:

- all directive kinds
- all-rules disable
- specific `astro-pipeline/*`
- unrelated rule
- description suffix
- malformed directive
- directives inside strings not counted
- per-file unsupported state
- per-file ambiguous MDX/comment state

### New `package-script-command-parser`

Location:

- `packages/parsers/package-script-command-parser`

Purpose:

- Tokenize package script commands enough to find ESLint invocations and ignore flags.

Facts:

```rust
pub struct PackageScriptParseFact {
    pub script_name: String,
    pub state: PackageScriptParseState,
}

pub enum PackageScriptParseState {
    Parsed { commands: Vec<PackageScriptCommand> },
    NoEslintInvocation,
    Unsupported { reason: String },
    ParseError { reason: String },
}

pub struct PackageScriptCommand {
    pub invocation: String,
    pub executable: String,
    pub args: Vec<String>,
}

pub struct EslintInvocation {
    pub script_name: String,
    pub args: Vec<String>,
    pub ignore_patterns: Vec<String>,
    pub ignore_path: Option<String>,
    pub config_path: Option<String>,
}
```

Must handle:

- `--ignore-pattern src/pages/**`
- `--ignore-pattern=src/pages/**`
- quoted args
- repeated flags
- `pnpm eslint`
- `pnpm exec eslint`
- `eslint .`
- simple `&&` and `||` command separators

Do not use raw substring checks.

Fail-closed behavior:

- unsupported or parse-error states are retained in ingestion.
- `TS-ASTRO-CONFIG-18` fails on unsupported or parse-error states for scripts whose name or raw command appears lint-related.
- tests must cover wrappers, unsupported separators, variables, and malformed quotes.

## G3TS Astro Types And Ingestion

Files:

- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`

New typed facts:

```rust
pub struct G3TsAstroPolicySurface {
    pub app_root_rel_path: String,
    pub policy_rel_path: Option<String>,
    pub policy_root_rel_path: Option<String>,
    pub parse_state: G3TsAstroPolicyParseState,
    pub route_classes: Vec<G3TsAstroRouteClassFact>,
    pub waivers: Vec<G3TsAstroWaiverFact>,
}

pub struct G3TsAstroRouteClassFact {
    pub rel_path: String,
    pub class: G3TsAstroRouteClass,
    pub matched_glob: String,
}

pub struct G3TsAstroPipelineRuleEffectivenessFact {
    pub app_root_rel_path: String,
    pub eslint_config_rel_path: String,
    pub probe_kind: G3TsAstroProbeKind,
    pub probe_rel_path: String,
    pub probe_ignored: bool,
    pub plugin_name: String,
    pub rule_name: String,
    pub severity: G3TsAstroRuleSeverity,
    pub options: G3TsAstroPipelineRuleOptionsState,
}

pub enum G3TsAstroPipelineRuleOptionsState {
    Parsed(G3TsAstroPipelineRuleOptions),
    Missing,
    Invalid { reason: String },
}

pub struct G3TsAstroPipelineRuleOptions {
    pub content_route_globs: Vec<String>,
    pub endpoint_globs: Vec<String>,
    pub content_data_module_globs: Vec<String>,
    pub query_adapter_globs: Vec<String>,
    pub adapter_barrel_globs: Vec<String>,
    pub adapter_helper_globs: Vec<String>,
    pub route_registry_globs: Vec<String>,
    pub content_component_globs: Vec<String>,
    pub content_config_globs: Vec<String>,
    pub mdx_content_globs: Vec<String>,
    pub approved_mdx_component_globs: Vec<String>,
    pub approved_generated_artifact_globs: Vec<String>,
    pub astro_content_type_import_globs: Vec<String>,
}

pub struct G3TsAstroContentSurfaceFact {
    pub app_root_rel_path: String,
    pub content_config_rel_path: Option<String>,
    pub src_content_files: Vec<String>,
    pub root_content_files: Vec<String>,
    pub mdx_content_files: Vec<String>,
}

pub struct G3TsAstroForbiddenArtifactFact {
    pub app_root_rel_path: String,
    pub rel_path: String,
    pub artifact_kind: G3TsAstroForbiddenArtifactKind,
    pub ignore_state: G3WorkspaceIgnoreState,
}

pub struct G3TsAstroEslintDirectiveFact {
    pub app_root_rel_path: String,
    pub rel_path: String,
    pub directive_kind: String,
    pub disabled_rule: G3TsAstroDisabledRule,
    pub canonical_selector: String,
    pub line: u32,
}
```

Ingestion must:

- parse selected guardrail policy through `guardrail3-rs-toml-parser`
- normalize route classes
- fail on overlapping route classes
- retain ignored ESLint probes
- inspect policy-sensitive ignored file entries
- collect root `content/**`
- collect `src/content/**`
- collect MDX content files
- collect `.contentlayer/**`
- collect `contentlayer.config.*`
- collect `.eslintignore` at app root and selected ancestors
- normalize plugin rule options once
- parse ESLint directives through shared parser
- parse package script invocations through shared parser
- retain directive parser per-file parse states
- retain package script per-script parse states
- ingest ESLint JSON/plugin findings into typed pre-policy Astro findings if policy waivers are expected to match plugin findings
- never inspect raw config JSON/TOML inside check rules

Policy-sensitive ignored entries:

- `.velite/**`
- `.contentlayer/**`
- `content/**`
- `contentlayer.config.*`
- `velite.config.*`
- `.eslintignore`

If the current workspace crawl cannot provide ignored entries, update the crawl layer first or make this a blocker.

## G3TS Astro Config Rules

Each rule must be one production file and one sidecar test module directory.

Do not create another grouped `TS-ASTRO-CONFIG-10` that validates all strict rules.

### Existing Rules To Keep

- `TS-ASTRO-CONFIG-01`
- `TS-ASTRO-CONFIG-02`
- `TS-ASTRO-CONFIG-03`
- `TS-ASTRO-CONFIG-04`
- `TS-ASTRO-CONFIG-05`
- `TS-ASTRO-CONFIG-06`
- `TS-ASTRO-CONFIG-07`

### `TS-ASTRO-CONFIG-08` - pipeline plugin version sufficient

Purpose:

- required plugin version contains the rules G3TS requires

Input:

- typed package dependency spec facts

Pass when:

- resolved or minimum version is >= required version

Fail when:

- missing package
- exact below minimum
- range can resolve below minimum
- prerelease below minimum
- unsupported spec without lockfile/resolution proof

Workspace specs:

- `workspace:*` is allowed only when the workspace contains the local plugin package and package version is >= required.
- external app without workspace package must use npm/file/link spec that resolves to a verifiable version.

### `TS-ASTRO-CONFIG-09` - ESLint inline config cannot hide pipeline rules

Purpose:

- required Astro pipeline rules cannot be disabled invisibly

Pass preferred:

- `linterOptions.noInlineConfig === true`

Pass fallback:

- `reportUnusedDisableDirectives === "error"`
- `reportUnusedInlineConfigs === "error"`
- `TS-ASTRO-SOURCE-01` scanner is wired in the runner

Fail:

- inline config allowed and source scanner missing
- unused disable directives not error
- probe ignored

### `TS-ASTRO-CONFIG-10` through `TS-ASTRO-CONFIG-17` - strict plugin rule effectiveness

One G3TS rule per plugin rule:

- `TS-ASTRO-CONFIG-10`: `require-content-provider-in-content-routes`
- `TS-ASTRO-CONFIG-11`: `no-astro-content-value-imports-outside-query-adapters`
- `TS-ASTRO-CONFIG-12`: `query-adapter-must-query-astro-content`
- `TS-ASTRO-CONFIG-13`: `no-static-content-data-in-content-route-closures`
- `TS-ASTRO-CONFIG-14`: `no-static-content-data-in-content-components`
- `TS-ASTRO-CONFIG-15`: `no-static-content-data-in-query-adapters`
- `TS-ASTRO-CONFIG-16`: `mdx-imports-from-approved-component-globs`
- `TS-ASTRO-CONFIG-17`: `require-direct-content-collection-schemas`

Each rule receives minimal facts:

- one app root
- one required rule name
- relevant probe facts
- normalized options for that rule
- discovered target paths that the options must cover

Each rule fails when:

- rule missing
- severity is not error
- plugin missing from probe
- probe ignored
- required option state missing or invalid
- required globs do not cover discovered target files
- target globs overmatch files in other route classes when the rule is route-class-specific

MDX rule special case:

- `approvedMdxComponentGlobs` may be empty.
- Empty means no MDX imports are allowed.
- Non-empty required only when policy allows MDX component imports.

### `TS-ASTRO-CONFIG-18` - lint scripts do not bypass required Astro lanes

Purpose:

- package scripts invoking ESLint must not use ignore flags that bypass required Astro surfaces

Input:

- typed `EslintInvocation` facts from `package-script-command-parser`

Fail when invocation contains:

- `--ignore-pattern` matching `src/pages/**`, `src/content/**`, configured content routes, configured content components, or content config
- `--ignore-path`
- alternate `--config` that is not the selected effective ESLint config
- unsupported or parse-error lint-related script state

Pass:

- normal `eslint .`
- `eslint --max-warnings 0 .`
- `pnpm exec eslint .`

### `TS-ASTRO-CONFIG-19` - no Contentlayer package

Purpose:

- match Velite package ban for another parallel content pipeline

Fail when dependencies/devDependencies include:

- `contentlayer`
- `next-contentlayer`
- `@contentlayer/*`

Pass:

- unrelated package names that contain `content` but are not Contentlayer.

## G3TS Astro Filetree Rules

### `TS-ASTRO-FILETREE-07` - strict local content roots

Purpose:

- in `strict-local-content`, authored content lives under configured `authored_content_globs`, default `src/content/**`

Fail in strict-local profile:

- root `content/**`
- `contents/**`
- `data/content/**`
- authored markdown/mdx/json/yaml content outside configured roots

Pass:

- `src/content/**`
- unrelated names such as `src/contentful.ts`
- non-strict profiles, once implemented

### `TS-ASTRO-FILETREE-08` - content config required for strict content routes

Purpose:

- strict local content routes require `src/content.config.*`

Fail when:

- app has at least one `content` route and no configured content config

Pass:

- content config exists
- app has only utility/chrome/report_shell routes

### `TS-ASTRO-FILETREE-09` - no explicit parallel content artifacts

Fail when app root contains:

- `.contentlayer/**`
- `contentlayer.config.*`
- `.velite/**`
- `velite.config.*`

Do not implement broad "generated content artifacts from other systems" detection.

### `TS-ASTRO-FILETREE-10` - no legacy ESLint ignore file for Astro app

Fail when selected app or selected ancestor contains:

- `.eslintignore`

Reason:

- flat config ignores must be visible to effective ESLint probes.

Pass:

- no `.eslintignore`
- unrelated file names

## G3TS Astro Source And Policy Rules

Create packages if missing:

```text
packages/ts/astro/g3ts-astro-source-checks
packages/ts/astro/g3ts-astro-policy-checks
```

### `TS-ASTRO-SOURCE-01` - inline ESLint disables cannot hide Astro pipeline rules

Input:

- typed ESLint directive file states
- app route/source classification
- approved generated artifact globs

Fail when a directive disables:

- all rules
- any `astro-pipeline/*` rule
- parser state for a linted Astro/TS/TSX/MDX source is `Unsupported`, `ParseError`, or `Ambiguous`

Pass:

- unrelated rule disables
- directives under approved generated artifact globs

Output:

- canonical selector for waiver matching
- source checks do not consume waivers
- source checks always emit live findings before policy matching

Do not detect "plugin disable" syntax. ESLint disables rule IDs or all rules, not plugin packages.

### `TS-ASTRO-POLICY-01` - Astro waivers match live findings exactly

Input:

- parsed policy facts
- all pre-policy Astro family findings
- each finding carries `waiver_policy: NotWaivable | WaiverOptional | WaiverRequired`

Fail when:

- waiver is stale
- waiver is duplicate
- waiver has weak reason
- waiver file resolves outside app root
- waiver selector is broad
- live finding with `waiver_policy = WaiverRequired` has no waiver
- waiver targets a `NotWaivable` finding

Info when:

- waiver matches exactly one live finding

Waiver policy classes:

- `NotWaivable`: malformed config, parser failure, missing required plugin package, missing required parser, route class overlap, stale waiver, duplicate waiver, broad waiver.
- `WaiverOptional`: inventory-style findings and generated-root exceptions.
- `WaiverRequired`: inline `eslint-disable` for `astro-pipeline/*`, broad inline `eslint-disable`, temporary route/content migration findings explicitly marked as waiveable by their rule.

Every check that emits a finding consumed by policy matching must set this field explicitly. Missing `waiver_policy` is a checker bug and must fail tests.

Canonical match key:

```text
policy_root_rel_path
app_root_rel_path
rule
normalized_file
canonical_selector
```

For ESLint disable findings, canonical selector:

```text
eslint-disable-next-line:<disabled-rule-or-all>:line=<target-line>
eslint-disable-line:<disabled-rule-or-all>:line=<line>
eslint-disable:<disabled-rule-or-all>:line=<line>
```

Plugin finding ingestion:

- Plugin rules must expose stable `messageId`, file path, and optional canonical selector in lint JSON output.
- G3TS policy checks may match plugin findings only after an ESLint JSON runner/import path exists.
- If plugin finding ingestion is not implemented in this slice, policy waivers apply only to G3TS findings and the plan must not claim plugin waivers are supported.

## Implementation Order

### Phase 1 - Shared Parsers

Deliverables:

- `eslint-config-parser` linter options and MDX probes
- `package-json-parser` dependency version/spec facts
- `guardrail3-rs-toml-parser` typed `[ts.astro]` policy
- new `eslint-directive-parser`
- new `package-script-command-parser`

Verification:

```sh
cargo test -q --manifest-path packages/parsers/eslint-config-parser/Cargo.toml --workspace
cargo test -q --manifest-path packages/parsers/package-json-parser/Cargo.toml --workspace
cargo test -q --manifest-path packages/parsers/guardrail3-rs-toml-parser/Cargo.toml --workspace
cargo test -q --manifest-path packages/parsers/eslint-directive-parser/Cargo.toml --workspace
cargo test -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace
```

### Phase 2 - ESLint Plugin Rules

Deliverables:

- add strict option schema
- add new rules
- update recommended config
- update README
- keep old rules

Tests:

- one test file per rule
- exact `messageId`
- exact error count
- exact filename where RuleTester exposes it
- valid and invalid cases from each rule's pass/fail section
- MDX parser integration for MDX rule
- unsupported alias tests
- unresolved approved adapter tests
- Windows path tests
- side-effect and unused import tests

Verification:

```sh
npm test --prefix packages/ts/eslint-plugin-astro-pipeline
npm pack --json --prefix packages/ts/eslint-plugin-astro-pipeline
```

Packed-package test before publish:

- create temp app
- install tarball from `npm pack --json`
- import plugin
- assert old and new rule names export
- assert recommended config sets required rules to error
- run ESLint against one valid and one invalid fixture

### Phase 3 - Astro Types And Ingestion

Deliverables:

- add typed facts
- normalize route classes
- normalize plugin options
- retain ignored probes
- discover policy-sensitive ignored files
- ingest waivers
- ingest directives
- ingest package script invocations

Tests:

- root `content/**`
- `src/content/**`
- MDX files
- `.contentlayer/**`
- `.eslintignore`
- waivers
- linterOptions
- ignored probes
- multiple apps
- policy root vs app root scoping
- ignored `node_modules`
- overlapping route classes
- explicit-vs-explicit route class overlap errors
- default-vs-explicit route class overlap passes with explicit class
- content route glob overmatches utility/chrome/report_shell route and fails strict rule effectiveness
- strict-local defaulting when `[ts.astro]` is absent and app is classified as Astro content app
- custom unsupported TS alias

Verification:

```sh
cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace
```

### Phase 4 - Astro Config Checks

Deliverables:

- `TS-ASTRO-CONFIG-08`
- `TS-ASTRO-CONFIG-09`
- `TS-ASTRO-CONFIG-10` through `TS-ASTRO-CONFIG-19`

Tests:

- one sidecar module directory per rule
- exact rule ID
- exact severity
- exact title
- exact path
- exact count
- golden pass
- missing fact fail-closed
- parse error fail-closed
- option coverage false positives

Semver tests:

- exact minimum
- below minimum
- caret/range resolving below minimum
- prerelease
- `workspace:*` internal
- `workspace:*` external
- `file:`
- `link:`
- missing dependency
- dependency and devDependency

Script tests:

- `--ignore-pattern src/pages/**`
- `--ignore-pattern=src/pages/**`
- quoted args
- repeated flags
- `--ignore-path`
- alternate `--config`
- normal `eslint .` pass

Verification:

```sh
cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace
```

### Phase 5 - Astro Filetree Checks

Deliverables:

- `TS-ASTRO-FILETREE-07`
- `TS-ASTRO-FILETREE-08`
- `TS-ASTRO-FILETREE-09`
- `TS-ASTRO-FILETREE-10`

File mapping:

- `ts_astro_filetree_07_strict_local_content_roots.rs`
- `ts_astro_filetree_08_content_config_for_content_routes.rs`
- `ts_astro_filetree_09_no_parallel_content_artifacts.rs`
- `ts_astro_filetree_10_no_legacy_eslint_ignore.rs`

Each file must have a matching rule-specific sidecar test directory:

- `ts_astro_filetree_07_strict_local_content_roots_tests/mod.rs`
- `ts_astro_filetree_08_content_config_for_content_routes_tests/mod.rs`
- `ts_astro_filetree_09_no_parallel_content_artifacts_tests/mod.rs`
- `ts_astro_filetree_10_no_legacy_eslint_ignore_tests/mod.rs`

Tests:

- root `content/**`
- `contents/**`
- `data/content/**`
- valid `src/content/**`
- unrelated `src/contentful.ts`
- `.contentlayer/**`
- `contentlayer.config.*`
- `.velite/**`
- `velite.config.*`
- ancestor `.eslintignore`
- app-local `.eslintignore`
- ignored forbidden entries still detected

Verification:

```sh
cargo test -q --manifest-path packages/ts/astro/g3ts-astro-file-tree-checks/Cargo.toml --workspace
```

### Phase 6 - Astro Source And Policy Checks

Deliverables:

- `g3ts-astro-source-checks`
- `g3ts-astro-policy-checks`
- `TS-ASTRO-SOURCE-01`
- `TS-ASTRO-POLICY-01`
- runner wiring in `apps/guardrail3-ts`

File mapping:

- `ts_astro_source_01_inline_eslint_disables.rs`
- `ts_astro_policy_01_waivers_match_live_findings.rs`

Each file must have a matching rule-specific sidecar test directory:

- `ts_astro_source_01_inline_eslint_disables_tests/mod.rs`
- `ts_astro_policy_01_waivers_match_live_findings_tests/mod.rs`

Tests:

- disable all rules
- disable one `astro-pipeline/*`
- unrelated disable
- matching waiver
- stale waiver
- duplicate waiver
- broad waiver
- weak reason
- waiver outside app root
- generated file exception

Verification:

```sh
cargo test -q --manifest-path packages/ts/astro/g3ts-astro-source-checks/Cargo.toml --workspace
cargo test -q --manifest-path packages/ts/astro/g3ts-astro-policy-checks/Cargo.toml --workspace
cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace
```

### Phase 7 - CLI And Real App Verification

Add CLI-level tests for `g3ts validate --family astro`:

- good minimal strict Astro fixture passes
- bad inline disable fails
- missing strict plugin rules fail
- root content fails
- route without provider fails
- good utility route without provider passes

Assertions:

- exit code
- rule IDs
- paths
- titles or stable message fragments

Run against landing:

```sh
g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory
```

Expected landing failure matrix after implementation:

| Rule | Expected path |
| --- | --- |
| `TS-ASTRO-CONFIG-04` | `package.json` |
| `TS-ASTRO-CONFIG-08` | `package.json` until upgraded |
| `TS-ASTRO-CONFIG-10..17` | `eslint.config.mjs` until strict rules wired |
| `TS-ASTRO-CONFIG-19` | `package.json` if Contentlayer is present |
| `TS-ASTRO-FILETREE-07` | `content/**` |
| `TS-ASTRO-FILETREE-09` | `.velite/**`, `velite.config.mjs` |
| `TS-ASTRO-FILETREE-10` | `.eslintignore` if present |
| plugin `require-content-provider-in-content-routes` | `src/pages/index.astro` |
| plugin `no-static-content-data-in-content-components` | `src/components/landing/homepage-v2.data.ts` or importing component |

### Phase 8 - Release

Release only after packed-package proof passes.

Commands:

```sh
npm test --prefix packages/ts/eslint-plugin-astro-pipeline
npm pack --json --prefix packages/ts/eslint-plugin-astro-pipeline
# install packed tarball into temp fixture and run export/config/lint checks
npm publish --access public --prefix packages/ts/eslint-plugin-astro-pipeline
cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force
```

## Exact Test Assertion Contract

Every red test must fail for the intended reason.

Required assertions where applicable:

- rule ID
- severity
- title or `messageId`
- file path
- count
- stable selector
- inventory/pass item for accepted exceptions
- absence of unrelated findings

Tests that only assert "some error happened" are not accepted.

## Done Criteria

- All parser tests pass.
- All plugin tests pass.
- Packed npm tarball works in a temp fixture.
- All Astro ingestion tests pass.
- All Astro config/filetree/source/policy tests pass.
- CLI tests prove new families are wired into `g3ts validate`.
- Real landing app produces the expected failure matrix.
- Adversarial review finds no concrete plan gap.

## Deferred

- rendered HTML validator
- Astro integration for post-render checks
- remote CMS profile
- full TypeScript resolver support for arbitrary aliases
- shared Astro content kit package
- automatic landing migration
