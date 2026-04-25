# Astro Delegation Boundaries

## Goal

Define the exact Astro-family enforcement contract after the landing refactor exposed missing guardrails, and separate Astro-owned contracts from style, generic package, generic ESLint, and rendered-output validation.

The desired end state:

- Astro apps are forced onto Astro content collections.
- Public page source cannot hardcode authored copy.
- SEO, JSON-LD, sitemap, robots, accessibility, and rendered HTML checks are delegated to named maintained libraries listed in this plan.
- G3TS enforces installation and configuration of those delegated tools.
- Astro family does not absorb unrelated style-system policy.

## External Package Facts Checked On 2026-04-25

These facts were verified with `npm view`, not assumed:

- `astro` latest is `6.1.9`.
- `@astrojs/react` latest is `5.0.4` and peers on React `17`, `18`, or `19`.
- `@astrojs/mdx` latest is `5.0.4` and peers on `astro: ^6.0.0`.
- `@astrojs/check` latest is `0.9.8` and peers on `typescript: ^5.0.0`.
- `@astrojs/sitemap` latest is `3.7.2`.
- `astro-robots` latest is `2.3.1` and peers on `astro: >= 4.0.0`.
- `@nuasite/checks` latest is `0.31.0` and peers on `typescript: ^6.0.2`.
- `@nuasite/checks@0.18.0` peers on `typescript: ^5` and depends on `astro: ^6.0.2`.
- `@nuasite/checks@0.18.0` exports both default `checks` and named `checks`.
- `@nuasite/checks@0.18.0` exports type `Check`; its page context has `pageData.jsonLd`.
- `astro-seo` latest is `1.1.0`.
- `schema-dts` latest is `2.0.0`.
- `g3ts-astro-nuasite-checks` is not published on npm on 2026-04-25; implementation must create it before G3TS enforces it.
- `astro-seo-schema@6.0.0` peers on `schema-dts: ^1.1.0`, so we are not using it because the desired schema type package is `schema-dts@2.0.0`.
- `@codemint/astro-meta@3.1.4` peers on `astro: ^5.4.0`, so it is rejected for the Astro 6 stack.
- `astro-seo-meta@6.0.0` peers on `astro: ^6.0.0`, but it is rejected as the default because it is a weaker direct-meta helper rather than the chosen SEO component contract.
- `eslint-plugin-i18next` latest is `6.1.4`.
- `eslint-plugin-mdx` latest is `3.7.0` and depends on `eslint-mdx: ^3.7.0`.
- Bare `eslint-mdx` is not the app contract because it is the parser package used by `eslint-plugin-mdx`; it does not provide the `mdx` ESLint plugin namespace or the `mdx/remark` rule contract by itself.
- `g3ts-eslint-plugin-astro-pipeline` is not published on npm on 2026-04-25; implementation must rename and publish the existing G3TS-owned Astro pipeline ESLint plugin under that package name.

## Correction

Previous boundary mistake:

- I put "ban arbitrary Tailwind values" under Astro guardrails.
- That is wrong. Tailwind arbitrary-value policy is not Astro-specific.
- It belongs to a future style/CSS/UI family, not `ts-astro`.

Astro requires that the landing app is an Astro content site. It does not decide whether the style system allows `text-[3.5rem]`, `shadow-[...]`, or arbitrary gradients.

## Astro Family Owns

Astro-specific app setup:

- `astro` package presence.
- Static output is the default for landing, blog, docs, and public content sites.
- No Astro adapter is required for the default content-site mode.
- Astro server output is only allowed with an explicit mode/waiver and a concrete server-only requirement.
- Required Astro packages for the default public content-site mode:
- `astro@6.1.9`
- `@astrojs/react@5.0.4`
- `@astrojs/mdx@5.0.4`
- `@astrojs/check@0.9.8`
- `@astrojs/sitemap@3.7.2`
- `astro-robots@2.3.1`
- `@nuasite/checks@0.18.0`
- `astro-seo@1.1.0`
- `schema-dts@2.0.0`
- `g3ts-astro-nuasite-checks@0.1.0`
- `astro.config.*` existence and parseability through the shared Astro config parser.
- `output` is required to be set to `"static"` for default public content apps.
- `output: "server"` is rejected for default public content apps because it can prevent rendered HTML from being emitted for validators.
- Missing `output` is rejected even though Astro defaults to static, because the app contract must be explicit for agents.
- `site` in `astro.config.*` when sitemap, robots, canonical, or rendered SEO checks are required.
- Required Astro integrations in `astro.config.*` for default public content-site mode:
- `react()` imported from `@astrojs/react`.
- `mdx()` imported from `@astrojs/mdx`.
- `sitemap()` imported from `@astrojs/sitemap`.
- `robots()` imported from `astro-robots`.
- `checks()` imported from `@nuasite/checks`.
- Forbidden package for default public content-site mode:
- `@astrojs/node`, because the default content-site output is static and not server-rendered.

Astro content pipeline checks:

- `src/content.config.ts` exists for build collections.
- Velite and old generated Velite artifacts are absent from Astro apps.
- Routes do not read authored content directly.
- Routes do not glob authored content directly.
- Routes do not import authored content directly.
- Routes do not import `astro:content` directly.
- Routes import approved content adapter modules instead.
- Public source lanes have inline authored copy linting active.

Astro-owned G3TS rendered-output validator setup:

- This section is not a new G3TS validator and not an ESLint plugin.
- This section defines package/config/script facts that Astro-family G3TS config checks must enforce so Nuasite runs during `astro build`.
- The rendered HTML validator itself is `@nuasite/checks`.
- Use `@nuasite/checks@0.18.0` while the project TypeScript floor is `5.9.3`; it is the newest discovered version with peer `typescript: ^5`.
- Do not require `@nuasite/checks@0.18.1` or newer until the project moves to TypeScript 6 because those versions peer on `typescript: ^6.0.2`.
- G3TS enforces the package version through Syncpack, not by parsing package versions itself.
- G3TS enforces `checks()` wiring through `astro-config-parser` facts, not by regexing `astro.config.*`.
- G3TS enforces fail-closed `checks()` options through `astro-config-parser` facts.
- G3TS enforces rendered HTML emission by requiring `output: "static"` in `astro.config.*`.
- Server output without prerender makes this gate fake because Nuasite can check zero pages.

Astro-owned G3TS SEO-generation package setup:

- This section defines packages and Astro integrations that G3TS requires.
- These packages generate rendered output that `@nuasite/checks` validates after build.
- `astro-seo@1.1.0` for title, description, canonical, robots meta, Open Graph, and Twitter tags.
- `schema-dts@2.0.0` for typed JSON-LD objects.
- `@astrojs/sitemap@3.7.2` for sitemap generation.
- `astro-robots@2.3.1` for robots generation.
- Astro family does not validate rendered HTML itself when `@nuasite/checks` covers the condition.

## Astro Family Does Not Own

Style policy:

- Tailwind arbitrary values.
- Tailwind class ordering.
- Unknown utility classes.
- CSS token discipline.
- Design-token reuse.

Those belong to a style/CSS/UI family. That family can delegate to `eslint-plugin-tailwindcss/no-arbitrary-value`, Stylelint, Tailwind config checks, or design-system-specific rules.

Generic ESLint setup:

- Root ESLint presence.
- Generic ESLint config location.
- Generic parser wiring.
- Generic `eslint --max-warnings 0` script policy.

Astro family owns only these ESLint requirements inside Astro app config scope:

- `eslint-plugin-astro@1.7.0` is pinned through Syncpack.
- `eslint-plugin-astro` appears in effective ESLint config for the required `AstroSource` probe; missing or ignored `AstroSource` probe fails.
- `g3ts-eslint-plugin-astro-pipeline@0.1.5` is pinned through Syncpack.
- The app imports the npm package `g3ts-eslint-plugin-astro-pipeline` in `eslint.config.*`.
- The imported plugin is registered in ESLint as namespace `astro-pipeline`.
- ESLint effective config contains plugin namespace `astro-pipeline` for the required `AstroSource`, `TsSource`, and `TsxSource` probes; missing or ignored probes fail.
- All required `astro-pipeline/*` custom content rules are `error` on the required `AstroSource`, `TsSource`, and `TsxSource` probes.
- `eslint-plugin-i18next@6.1.4` is pinned through Syncpack.
- `i18next/no-literal-string` is `error` on the required public `AstroSource`, `TsSource`, and `TsxSource` probes with the exact strict public-copy options listed below.
- `eslint-plugin-mdx@3.7.0` is pinned through Syncpack.
- The MDX content probe has the `mdx` plugin active and `mdx/remark` at `error`.

Generic package policy:

- Package manager enforcement.
- General dependency hygiene.
- General script execution policy.
- Generic Syncpack setup.

Astro family only owns Astro-specific package pins and Astro-specific forbidden dependencies through Syncpack facts.

Generic TypeScript policy:

- TS strictness.
- Root tsconfig shape.
- Generic project references.

Astro family owns exactly these Astro-specific typecheck requirements:

- `@astrojs/check@0.9.8` is pinned through Syncpack.
- `package.json` has a script that safely invokes `astro check` according to `package-script-command-parser`.
- `astro check || true`, `astro check | tee`, unsupported shell syntax, or echoed text does not satisfy the contract.

Rendered HTML validation implementation:

- Astro family does not parse built HTML to validate SEO tags, JSON-LD, headings, sitemap, or robots.
- That work is delegated to `@nuasite/checks`.
- Rendered-output validator setup is enforced by these exact G3TS facts, with no rendered-HTML parser inside G3TS:
- `@nuasite/checks@0.18.0` is pinned through Syncpack.
- `astro.config.*` has `checks()` imported from `@nuasite/checks` in `integrations`.
- `checks()` has fail-closed options listed below.
- `astro.config.*` has `output: "static"`.
- `package.json` has a safe `astro build` script.
- `checks()` contains the required custom check `structuredDataPresentCheck` for JSON-LD presence.
- `structuredDataPresentCheck` is imported from `g3ts-astro-nuasite-checks`.
- G3TS fails when any of those facts is missing.
- G3TS does not open `dist/**/*.html`.
- G3TS does not reimplement Nuasite's title, description, canonical, Open Graph, Twitter, headings, image alt, robots, sitemap, accessibility, or performance checks.

## Delegation Decisions

Dependency floors and forbidden packages:

- Delegate to Syncpack.
- G3TS enforces the `.syncpackrc` contract and exact Astro policy groups.
- Do not reintroduce semver parsing into G3TS rules.
- Ingestion uses `syncpack-config-parser` to parse `.syncpackrc`.
- The Astro config checks pass only when the selected `.syncpackrc` has `source: ["package.json"]` relative to the Astro app root and a canonical prefix of `versionGroups`.
- Each required dependency must have one canonical pin group before any app-specific groups:
- `dependencies: ["<package>"]`
- `dependencyTypes: ["prod", "dev"]`
- `pinVersion: "<exact version>"`
- no `packages`
- no `specifierTypes`
- no `isIgnored`
- no `isBanned`
- Each forbidden dependency must have one canonical ban group before any app-specific groups:
- `dependencies: ["<package>"]`
- `dependencyTypes: ["prod", "dev", "optional", "peer"]`
- `isBanned: true`
- no `packages`
- no `specifierTypes`
- no `pinVersion`

Exact required Syncpack pins for default public Astro content apps:

```json
[
  ["astro", "6.1.9"],
  ["@astrojs/react", "5.0.4"],
  ["@astrojs/mdx", "5.0.4"],
  ["@astrojs/check", "0.9.8"],
  ["@astrojs/sitemap", "3.7.2"],
  ["astro-robots", "2.3.1"],
  ["@nuasite/checks", "0.18.0"],
  ["g3ts-astro-nuasite-checks", "0.1.0"],
  ["astro-seo", "1.1.0"],
  ["schema-dts", "2.0.0"],
  ["react", "19.2.5"],
  ["react-dom", "19.2.5"],
  ["@types/react", "19.2.14"],
  ["@types/react-dom", "19.2.3"],
  ["typescript", "5.9.3"],
  ["eslint-plugin-astro", "1.7.0"],
  ["g3ts-eslint-plugin-astro-pipeline", "0.1.5"],
  ["eslint-plugin-i18next", "6.1.4"],
  ["eslint-plugin-mdx", "3.7.0"]
]
```

Exact forbidden Syncpack dependencies for default public Astro content apps:

```json
[
  "next",
  "velite",
  "@astrojs/node",
  "eslint-plugin-astro-pipeline",
  "@codemint/astro-meta",
  "astro-seo-meta",
  "astro-seo-schema"
]
```

`g3ts-eslint-plugin-astro-pipeline@0.1.5` is the renamed G3TS-owned package for the existing Astro pipeline ESLint rules. It keeps the ESLint namespace `astro-pipeline` and rule IDs `astro-pipeline/*`. It removes transitive ownership of `eslint-plugin-i18next` from the package contract. If the implementation keeps using npm package `eslint-plugin-astro-pipeline@0.1.4`, this plan is not implemented.

## Parser Contracts Required By This Plan

`syncpack-config-parser`:

- Already owns JSON parsing of `.syncpackrc`.
- Must expose each `versionGroups` entry with these fields:
- `dependencies: Vec<String>`
- `dependency_types: Vec<String>`
- `pin_version: Option<String>`
- `is_banned: Option<bool>`
- `is_ignored: Option<bool>`
- `packages: Option<Vec<String>>`
- `specifier_types: Option<Vec<String>>`
- Astro ingestion consumes those facts and compares them to the exact required pin and ban groups in this plan.
- Astro config checks must not read dependency version strings from `package.json`.
- Astro config checks must not use a SemVer library.

`package-script-command-parser`:

- Already owns shell parsing for `package.json` scripts.
- Astro ingestion must ask it whether scripts safely execute `astro check`, `astro build`, and `syncpack lint`.
- Safe means the command is on an executable path, not inside `echo`, not behind fail-open `|| true`, not piped in an unsupported form, and not hidden inside unsupported shell syntax.
- Astro config checks must not use substring matching for scripts.

`eslint-config-parser`:

- Already owns effective ESLint config discovery through ESLint `calculateConfigForFile`.
- Astro ingestion must pass explicit probe targets to it.
- Astro config checks must read only these parser facts:
- `probe`
- `rel_path`
- `ignored`
- `plugins`
- `rules`
- `rules[rule_name].severity`
- `rules[rule_name].options`
- Astro config checks must not parse `eslint.config.*` directly.

`astro-config-parser`:

- Already exposes `site`, `output`, `integrations`, and `adapter`.
- This plan requires extending `AstroIntegrationSnapshot` so integrations can be checked without source regexes:

```rust
pub struct AstroIntegrationSnapshot {
    pub source_module: Option<String>,
    pub name: Option<String>,
    pub imported_name: Option<String>,
    pub call: Option<AstroCallSnapshot>,
}

pub struct AstroCallSnapshot {
    pub first_arg: Option<AstroStaticValue>,
}

pub enum AstroStaticValue {
    Bool(bool),
    Number(f64),
    String(String),
    Null,
    Array(Vec<AstroStaticValue>),
    Object(Vec<AstroStaticObjectProperty>),
    ImportedIdentifier {
        local_name: String,
        source_module: Option<String>,
        imported_name: Option<String>,
    },
}

pub struct AstroStaticObjectProperty {
    pub key: String,
    pub value: AstroStaticValue,
}
```

- The parser must fail closed for integration options that cannot reduce to the static value model above.
- Dynamic spreads inside `checks({...dynamic})` fail.
- Unresolved helper calls fail for rules that require `checks()` options; `integrations: [makeChecks()]` is a failing shape.
- Literal array spreads already supported by `integrations: [...base]` remain acceptable only when the spread resolves to an array literal.
- The parser must record import kind precisely enough to distinguish `import checks from "@nuasite/checks"` from `import { checks } from "@nuasite/checks"` and `import { checks as siteChecks } from "@nuasite/checks"`.
- The accepted `@nuasite/checks` integration call is:
- `source_module == Some("@nuasite/checks")`
- `imported_name == None` for default import, or `imported_name == Some("checks")` for named import
- `call.first_arg == Some(Object(...))`
- Inside that object, property `customChecks` must be an array containing `AstroStaticValue::ImportedIdentifier { local_name: "structuredDataPresentCheck", source_module: Some("g3ts-astro-nuasite-checks"), imported_name: Some("structuredDataPresentCheck") }`.

G3TS Astro pipeline ESLint plugin:

- Npm package name: `g3ts-eslint-plugin-astro-pipeline`.
- Package location in this repo after rename: `packages/ts/g3ts-eslint-plugin-astro-pipeline`.
- Package type: publishable TypeScript npm package.
- It is an ESLint plugin.
- ESLint namespace registered by apps: `astro-pipeline`.
- Rule IDs stay `astro-pipeline/*`.
- `g3ts-eslint-plugin-astro-pipeline@0.1.5` owns exactly these custom rules:
- `astro-pipeline/no-authored-content-fs-read`
- `astro-pipeline/no-authored-content-glob`
- `astro-pipeline/no-authored-content-imports`
- `astro-pipeline/no-content-data-modules-in-routes`
- `astro-pipeline/no-direct-astro-content-in-routes`
- `astro-pipeline/no-runtime-mdx-eval`
- `astro-pipeline/no-side-loader-imports`
- `astro-pipeline/no-velite-imports`
- `astro-pipeline/require-approved-content-adapter-in-routes`
- Every listed rule is `error` on the required `AstroSource`, `TsSource`, and `TsxSource` probes.
- Route-scoped rules must receive `routeGlobs` and `endpointGlobs` options.
- `routeGlobs` must match every actual page route selected by Astro ingestion.
- `endpointGlobs` must match every actual endpoint selected by Astro ingestion.
- Content-source rules must receive non-empty `authoredContentGlobs` or `specContentGlobs`.
- Content-data rule must receive non-empty `contentDataModuleGlobs`.
- `require-approved-content-adapter-in-routes` must receive non-empty `approvedContentAdapterModules`.
- Default `approvedContentAdapterModules` value for landing apps is `src/content/landing-homepage.ts`.
- G3TS checks rule effectiveness from ESLint effective config facts and option values; the ESLint plugin enforces source behavior.

Effective Astro/TS/TSX source-lane algorithm:

- Astro ingestion selects `AstroSource`, `TsSource`, and `TsxSource` probes before calling `eslint-config-parser`.
- `AstroSource` probe path:
- first included app-local `src/**/*.astro`
- else synthetic `src/__g3ts_probe__.astro`
- `TsSource` probe path:
- first included app-local `src/**/*.ts`
- else synthetic `src/index.ts`
- `TsxSource` probe path:
- first included app-local `src/**/*.tsx`
- else synthetic `src/__g3ts_probe__.tsx`
- "Included" means the project tree marks the file as not ignored by workspace ignore rules.
- Every public Astro content app must produce all three probe records: `AstroSource`, `TsSource`, and `TsxSource`.
- Missing probe record is a failure.
- Probe record with `ignored == true` is a failure.
- Current helper behavior where a missing lane returns `true` must be removed for Astro-family source lanes.
- For `eslint-plugin-astro`, pass is checked only on the `AstroSource` probe and means `plugins` contains `astro`.
- For `g3ts-eslint-plugin-astro-pipeline`, pass means the probe `plugins` contains ESLint namespace `astro-pipeline`.
- For every required `astro-pipeline/*` rule, pass means `rules[rule_name].severity == Error`.
- For rules with required options, pass also means the first options object contains the exact required keys and non-empty arrays listed in this plan.
- Package presence alone never satisfies source-lane effectiveness.

Route and endpoint option coverage algorithm:

- Astro ingestion must pass the checker the actual included route files selected from the app root.
- Actual page route set:
- included files under `src/pages/**` with extension `.astro`, `.md`, `.mdx`, or `.html`
- Actual endpoint route set:
- included files under `src/pages/**` with extension `.ts` or `.js`
- For each required rule option `routeGlobs`, compile the configured glob strings with `globset = "0.4"` in `packages/ts/astro/g3ts-astro-ingestion/crates/runtime`.
- Pass only when every actual page route path matches at least one configured `routeGlobs` pattern.
- For each required rule option `endpointGlobs`, compile the configured glob strings with `globset = "0.4"` in `packages/ts/astro/g3ts-astro-ingestion/crates/runtime`.
- Pass only when every actual endpoint path matches at least one configured `endpointGlobs` pattern.
- Empty actual endpoint set is allowed only when `endpointGlobs` is still present and non-empty; the option must not disappear just because the current app has no endpoints.
- Empty actual page route set is not allowed for a public content app.

Source copy in public UI/source:

- Delegated tool is `eslint-plugin-i18next/no-literal-string`.
- `eslint-plugin-i18next@6.1.4` is a direct app dev dependency, pinned by Syncpack with the canonical pin group above.
- It is not hidden inside `g3ts-eslint-plugin-astro-pipeline`.
- It catches JSX text, Astro template text, TS string literals, object literals, arrays, call literals, and public-copy attributes.
- G3TS determines effectiveness from `eslint-config-parser`, which calls ESLint `calculateConfigForFile` for real or synthetic probe files.
- Required source probes:
- `EslintProbeKind::AstroSource` for first included `src/**/*.astro`, or a synthetic `src/__g3ts_probe__.astro` when no Astro source file exists.
- `EslintProbeKind::TsSource` for first included `src/**/*.ts`, or synthetic `src/index.ts` when no TS source file exists.
- `EslintProbeKind::TsxSource` for first included `src/**/*.tsx`, or synthetic `src/__g3ts_probe__.tsx` when no TSX source file exists.
- A probe counts only when ESLint does not ignore it.
- Each required public source probe must have plugin `i18next`.
- Each required public source probe must have rule `i18next/no-literal-string` with severity `error`.
- Each required public source probe must have this exact first options object:

```js
{
  framework: "react",
  mode: "all",
  message:
    "Inline public copy must live in Astro content entries. Move this text into the content collection, validate it through the collection schema, and pass the typed value into source.",
  "should-validate-template": true,
  words: {
    include: [],
    exclude: ["[0-9!-/:-@[-`{-~]+", "[A-Z_-]+"]
  },
  "jsx-components": {
    include: [],
    exclude: []
  },
  "jsx-attributes": {
    include: [],
    exclude: [
      "as",
      "class",
      "className",
      "color",
      "data-.+",
      "height",
      "href",
      "id",
      "intent",
      "key",
      "name",
      "rel",
      "role",
      "size",
      "slot",
      "src",
      "style",
      "styleName",
      "target",
      "tone",
      "type",
      "variant",
      "width",
      "aria-hidden"
    ]
  },
  callees: {
    include: [],
    exclude: ["require", "clsx", "cn", "cx", "cva", "twMerge", "twJoin", "tv", "URL"]
  },
  "object-properties": {
    include: [],
    exclude: ["[A-Z_-]+"]
  },
  "class-properties": {
    include: [],
    exclude: ["displayName"]
  }
}
```

MDX source linting:

- MDX is part of the required Astro content-site stack for blogs and rich content.
- App installs `@astrojs/mdx` for Astro build/runtime support.
- App installs `eslint-plugin-mdx@3.7.0` for `.mdx` ESLint parser/plugin/config support.
- `eslint-plugin-mdx` exports the ESLint plugin namespace `mdx`, processors, configs, and rule `mdx/remark`.
- `eslint-mdx` is a dependency used by `eslint-plugin-mdx` for parsing/processing MDX.
- Installing bare `eslint-mdx` does not satisfy the contract because G3TS checks the effective ESLint config for plugin `mdx` and rule `mdx/remark`.
- App config has an explicit MDX ESLint lane for `**/*.mdx`.
- G3TS enforces the package version only through the Syncpack canonical pin group above.
- G3TS determines lane effectiveness from `eslint-config-parser`.
- Add `EslintProbeKind::MdxContent` to Astro ingestion probe targets.
- The MDX probe path is the first included app-local `content/**/*.mdx` or `src/content/**/*.mdx`; if none exists, use synthetic `content/__g3ts_probe__.mdx`.
- The MDX probe must not be ignored by ESLint.
- The MDX probe effective config must have plugin `mdx`.
- The MDX probe effective config must have `mdx/remark` at severity `error`.
- `g3ts-eslint-plugin-astro-pipeline` does not own `eslint-plugin-mdx` or `eslint-mdx`.

Nuasite rendered-output validator details:

- This is the validator executed by `astro build` after G3TS confirms `TS-ASTRO-CONFIG-13`.
- G3TS does not run these checks.
- G3TS only enforces package pin, Astro integration wiring, static output, and safe build script.
- Validator package: `@nuasite/checks@0.18.0` while the project is on TypeScript 5.9.3.
- It runs after `astro build` and scans rendered HTML pages.
- It already includes checks for title, description, canonical, JSON-LD parse validity, headings, Open Graph, Twitter card, image alt, duplicate meta tags, viewport, noindex, robots, sitemap, broken internal links, accessibility, performance, and GEO basics.
- Require `output: "static"` so this validator checks real pages.
- Require package script `build` to safely invoke `astro build` using `package-script-command-parser`; `astro build || true`, `astro build | tee`, or unsupported shell syntax does not satisfy the contract.
- Require `checks()` options to statically reduce through `astro-config-parser`.
- Required `checks()` options:
- `mode: "full"`
- `failOnError: true`
- `failOnWarning: true`
- `reportJson: true`
- `ai: false`
- `overrides` absent or `{}`.
- `seo` absent, `true`, or object; `seo: false` fails.
- `geo` absent, `true`, or object; `geo: false` fails.
- `performance` absent, `true`, or object; `performance: false` fails.
- `accessibility` absent, `true`, or object; `accessibility: false` fails.
- Do not upgrade past `0.18.0` until TypeScript 6 is adopted or the package supports TypeScript 5.9 again.

Sitemap:

- Delegate generation to official `@astrojs/sitemap`.
- G3TS enforces `@astrojs/sitemap@3.7.2` only through the Syncpack canonical pin group above.
- G3TS enforces non-empty absolute `site` through `astro-config-parser`.
- G3TS enforces `sitemap()` imported from `@astrojs/sitemap` appears in `integrations` through `astro-config-parser`.
- `@nuasite/checks` validates that rendered output contains a sitemap.

Robots:

- `astro-robots@2.3.1` is compatible with Astro 6 by peer metadata and by a throwaway build probe.
- G3TS enforces `astro-robots@2.3.1` only through the Syncpack canonical pin group above.
- G3TS enforces `robots()` imported from `astro-robots` appears in `integrations` through `astro-config-parser`.
- Explicit hand-authored `robots.txt` is not the default contract.
- `@nuasite/checks@0.18.0` validates that rendered output contains robots output.

LLMs discovery:

- Require `public/llms.txt` as a committed public file when the app is a public content site.
- G3TS checks this as a file-tree fact, not by parsing rendered output.
- `@nuasite/checks@0.18.0` can validate missing `llms.txt`.

SEO meta:

- Require `astro-seo@1.1.0`.
- A throwaway Astro 6.1.9 + TypeScript 5.9.3 probe passed `astro check` and `astro build` with `astro-seo@1.1.0`.
- `astro-seo` supports canonical, robots directives, Open Graph, Twitter, title templates, and layout props.
- Do not allow `titleDefault` or other defaults to hide missing content metadata.
- G3TS enforces package version only through the Syncpack canonical pin group above.
- G3TS does not inspect source for `<SEO>` usage; `@nuasite/checks@0.18.0` validates rendered title, description, canonical, Open Graph, and Twitter tags.
- `astro-seo-meta` is not part of the approved default stack because its API is weaker and all props are optional.
- Reject `@codemint/astro-meta` for the current Astro 6 stack because its peer dependency targets Astro 5.

JSON-LD:

- Require `schema-dts@2.0.0` directly for JSON-LD types.
- `schema-dts` owns TypeScript typing of schema objects.
- `astro-seo-schema@6.0.0` peers on `schema-dts: ^1.1.0`, while current `schema-dts` is `2.0.0`; do not require it if it forces old schema types.
- G3TS enforces package version only through the Syncpack canonical pin group above.
- Rendering uses an app-owned Astro component or layout snippet that accepts `WithContext<Thing>` from `schema-dts` and outputs `<script type="application/ld+json" is:inline set:html={JSON.stringify(schema)} />`.
- `@nuasite/checks` owns rendered JSON parse validation.
- `@nuasite/checks@0.18.0` does not enforce JSON-LD presence, only invalid JSON-LD.
- Add one package named `g3ts-astro-nuasite-checks@0.1.0`.
- Package location in this repo: `packages/ts/g3ts-astro-nuasite-checks`.
- Package type: publishable TypeScript npm package.
- It is not an ESLint plugin.
- It is not an Astro integration.
- It is not a G3TS checker crate.
- It exports reusable Nuasite custom checks for Astro content sites.
- `g3ts-astro-nuasite-checks@0.1.0` exports one custom Nuasite page check named `structuredDataPresentCheck`.
- `structuredDataPresentCheck.id` is `g3/structured-data-present`.
- `g3/structured-data-present` fails when `ctx.pageData.jsonLd.length === 0`.
- Apps pass it through `checks({ customChecks: [structuredDataPresentCheck] })`.
- G3TS enforces the package version only through the Syncpack canonical pin group above.
- G3TS enforces the custom check wiring through `astro-config-parser` by requiring the `customChecks` array to contain identifier `structuredDataPresentCheck` imported from `g3ts-astro-nuasite-checks`.
- Reason this is a package instead of app-local code: app-local custom checks would make each Astro app handwrite validator logic; the guardrail would then need to parse or trust arbitrary app code. A shared package makes the validator implementation identical across apps, and G3TS only enforces that it is installed and wired.

## ESLint Plugin Ownership

Correct ownership:

- Custom ESLint plugins contain custom rules only.
- Third-party ESLint plugins are direct app dev dependencies when their rules are required for the app.
- G3TS requires the exact plugin packages and verifies the effective ESLint config.
- G3TS does not accept package presence alone.
- No delegated ESLint validator is hidden as a transitive dependency of `g3ts-eslint-plugin-astro-pipeline`.
- No direct app dependency is banned just because a wrapper plugin could have owned it.

Required Astro content-site ESLint surfaces:

- `eslint-plugin-astro` owns Astro syntax and Astro-specific source correctness.
- `g3ts-eslint-plugin-astro-pipeline` owns only custom Astro content-pipeline rules.
- `eslint-plugin-i18next` owns delegated no-authored-copy literal detection through `i18next/no-literal-string`.
- `eslint-plugin-mdx` owns `.mdx` linting because it exports the `mdx` plugin namespace, processor/config surface, and `mdx/remark` rule.
- `eslint-mdx` is only a parser/processor dependency used by `eslint-plugin-mdx`; it is not the app-level ESLint plugin contract.

Required G3TS enforcement:

- Syncpack pins `eslint-plugin-astro`.
- Syncpack pins `g3ts-eslint-plugin-astro-pipeline`.
- Syncpack pins `eslint-plugin-i18next`.
- Syncpack pins `eslint-plugin-mdx`.
- G3TS verifies `eslint-plugin-astro` on the exact `AstroSource` probe using `plugins.contains("astro")`.
- G3TS verifies `g3ts-eslint-plugin-astro-pipeline` custom rules on the exact `AstroSource`, `TsSource`, and `TsxSource` probes using ESLint namespace fact `plugins.contains("astro-pipeline")` and `rules[rule_name].severity == Error`.
- G3TS verifies `i18next/no-literal-string` is effective on public Astro, TS, and TSX source lanes with strict options.
- G3TS verifies an MDX lane exists for `**/*.mdx` and uses the approved MDX parser/lint surface.

Migration from current released plugin:

- `eslint-plugin-astro-pipeline@0.1.4` currently exports `configs["strict-content"]` and depends on `eslint-plugin-i18next`.
- That package name and wrapper shape are deprecated by this plan.
- `g3ts-eslint-plugin-astro-pipeline@0.1.5` is the renamed replacement package.
- `g3ts-eslint-plugin-astro-pipeline@0.1.5` removes third-party plugin ownership from the package contract.
- Astro Syncpack policy bans old package name `eslint-plugin-astro-pipeline`.
- Astro Syncpack policy stops banning direct `eslint-plugin-i18next`.
- Astro Syncpack policy requires direct `eslint-plugin-i18next`.
- Astro Syncpack policy requires direct `eslint-plugin-mdx`.
- G3TS messages name the actual delegated plugin that provides the missing rule.

## New Astro Rules To Plan

Update existing rules:

- `TS-ASTRO-CONFIG-06` remains the package-present check for the Astro pipeline ESLint plugin, but the expected npm package is `g3ts-eslint-plugin-astro-pipeline` and the expected package version is `0.1.5` through Syncpack.
- `TS-ASTRO-CONFIG-07` stops checking `i18next/no-literal-string`; it checks only custom `astro-pipeline/*` rules.
- `TS-ASTRO-CONFIG-09` required Syncpack pins change to the exact list in "Exact required Syncpack pins".
- `TS-ASTRO-CONFIG-10` forbidden Syncpack deps change to the exact list in "Exact forbidden Syncpack dependencies".

`TS-ASTRO-CONFIG-11` - Astro config has site URL:

- Input from shared `astro-config-parser`.
- `astro-config-parser` must expose `site: Option<String>`.
- Pass when `site` exists and parses as an absolute `https://` URL.
- Fail when `site` is missing, not a string literal, not absolute, not HTTPS, or dynamically computed.
- This supports canonical URLs, sitemap generation, robots sitemap references, and rendered SEO validation.

`TS-ASTRO-CONFIG-12` - public content apps use static output:

- Input from shared `astro-config-parser`.
- `astro-config-parser` must expose `output: Option<AstroOutputMode>`.
- Pass only when `output == Some(AstroOutputMode::Static)`.
- Fail when `output` is missing.
- Fail when `output == Some(AstroOutputMode::Server)`.
- Server output requires a future explicit mode/waiver that is not part of the default public content-site contract.
- Rationale: in a server-output probe, `@nuasite/checks` ran but checked `0 pages` because no static HTML was emitted.

`TS-ASTRO-CONFIG-13` - rendered checks integration wired:

- Package presence: `package.json` dependencies or devDependencies contains `@nuasite/checks`.
- Package version policy: Syncpack has canonical pin group `@nuasite/checks -> 0.18.0`.
- Astro config wiring: `astro-config-parser` integrations contains a call imported from `@nuasite/checks`.
- Accepted import shapes:
- `import checks from "@nuasite/checks"; integrations: [checks({...})]`
- `import { checks } from "@nuasite/checks"; integrations: [checks({...})]`
- `import { checks as siteChecks } from "@nuasite/checks"; integrations: [siteChecks({...})]`
- Fail when `checks` is wrapped in a local helper, spread from a dynamic array, or called through an unresolved identifier.
- Required `checks()` options:

```js
{
  mode: "full",
  failOnError: true,
  failOnWarning: true,
  reportJson: true,
  ai: false,
  customChecks: [structuredDataPresentCheck]
}
```

- `structuredDataPresentCheck` must resolve to `import { structuredDataPresentCheck } from "g3ts-astro-nuasite-checks"`.
- `overrides` must be absent or an empty object.
- `seo: false`, `geo: false`, `performance: false`, and `accessibility: false` fail.
- Package script: `package-script-command-parser` must find a safe `astro build` invocation in `package.json` scripts.
- Static output: this rule depends on `TS-ASTRO-CONFIG-12` passing.

`TS-ASTRO-CONFIG-14` - sitemap integration wired:

- Package presence: `package.json` dependencies or devDependencies contains `@astrojs/sitemap`.
- Package version policy: Syncpack has canonical pin group `@astrojs/sitemap -> 3.7.2`.
- Astro config wiring: `astro-config-parser` integrations contains `sitemap()` imported from `@astrojs/sitemap`.
- Accepted import shape: `import sitemap from "@astrojs/sitemap"; integrations: [sitemap()]`.
- Fail when `sitemap()` is missing, imported from another package, wrapped in an unresolved helper, or spread from a dynamic array.
- `site` must pass `TS-ASTRO-CONFIG-11`.

`TS-ASTRO-CONFIG-15` - robots generation or explicit robots policy present:

- Package presence: `package.json` dependencies or devDependencies contains `astro-robots`.
- Package version policy: Syncpack has canonical pin group `astro-robots -> 2.3.1`.
- Astro config wiring: `astro-config-parser` integrations contains `robots()` imported from `astro-robots`.
- Accepted import shape: `import robots from "astro-robots"; integrations: [robots()]`.
- Fail when `robots()` is missing, imported from another package, wrapped in an unresolved helper, or spread from a dynamic array.
- `site` must pass `TS-ASTRO-CONFIG-11`.
- Do not accept hand-authored `public/robots.txt` for the default contract.

`TS-ASTRO-CONFIG-16` - llms discovery present:

- File-tree input from workspace crawl.
- Pass only when the app root contains included file `public/llms.txt`.
- Fail on missing `public/llms.txt`.
- Do not accept route-generated `llms.txt` for the default contract.
- `@nuasite/checks@0.18.0` also catches missing `llms.txt` after build, but G3TS still checks the source file so agents get a pre-build error.

`TS-ASTRO-CONFIG-17` - SEO component packages available:

- Package presence: `package.json` dependencies or devDependencies contains `astro-seo`.
- Package version policy: Syncpack has canonical pin group `astro-seo -> 1.1.0`.
- Package presence: `package.json` dependencies or devDependencies contains `schema-dts`.
- Package version policy: Syncpack has canonical pin group `schema-dts -> 2.0.0`.
- Forbidden package policy: Syncpack bans `@codemint/astro-meta`, `astro-seo-meta`, and `astro-seo-schema`.
- G3TS does not inspect source for `<SEO>` usage. Rendered title/description/canonical/OG/Twitter correctness is delegated to `@nuasite/checks@0.18.0`.

`TS-ASTRO-CONFIG-18` - content adapter route contract:

- Rule owner package: `g3ts-eslint-plugin-astro-pipeline@0.1.5`.
- Rule name: `astro-pipeline/require-approved-content-adapter-in-routes`.
- Package presence: `package.json` dependencies or devDependencies contains `g3ts-eslint-plugin-astro-pipeline`.
- Package version policy: Syncpack has canonical pin group `g3ts-eslint-plugin-astro-pipeline -> 0.1.5`.
- ESLint facts come from `eslint-config-parser`.
- Live probes: `AstroSource`, `TsSource`, and `TsxSource`.
- Pass only when each required `AstroSource`, `TsSource`, and `TsxSource` probe has plugin `astro-pipeline`, rule `astro-pipeline/require-approved-content-adapter-in-routes` at `error`, `routeGlobs` covering all actual page routes, `endpointGlobs` covering all actual endpoints, and non-empty `approvedContentAdapterModules`.
- The rule implementation fails a public page route when it does not import at least one module listed in `approvedContentAdapterModules`.
- Endpoints are excluded by `endpointGlobs`; they are governed by the existing route-scoped pipeline rules.

`TS-ASTRO-CONFIG-19` - direct no-authored-copy ESLint plugin wired:

- Package presence: `package.json` dependencies or devDependencies contains `eslint-plugin-i18next`.
- Package version policy: Syncpack has canonical pin group `eslint-plugin-i18next -> 6.1.4`.
- ESLint facts come from `eslint-config-parser` probes, not from manual config parsing.
- Live probes: `AstroSource`, `TsSource`, and `TsxSource`.
- Pass only when each required `AstroSource`, `TsSource`, and `TsxSource` probe has plugin `i18next`, rule `i18next/no-literal-string` at `error`, and the exact strict options object listed under "Source copy in public UI/source".
- Fail when any required probe is ignored by ESLint.
- Fail when broad allowlists weaken text detection.

`TS-ASTRO-CONFIG-20` - MDX ESLint lane wired:

- Package presence: `package.json` dependencies or devDependencies contains `eslint-plugin-mdx`.
- Package version policy: Syncpack has canonical pin group `eslint-plugin-mdx -> 3.7.0`.
- ESLint facts come from `eslint-config-parser`.
- Add `MdxContent` probe to Astro ingestion.
- Probe path selection is exact:
- first included app-local `content/**/*.mdx`
- else first included app-local `src/content/**/*.mdx`
- else synthetic `content/__g3ts_probe__.mdx`
- Pass only when the required `MdxContent` probe has plugin `mdx` and rule `mdx/remark` at `error`.
- Fail when the `MdxContent` probe is ignored by ESLint.
- Direct `eslint-mdx` package presence does not satisfy this rule because the rule checks effective ESLint plugin `mdx` and rule `mdx/remark`.

`TS-ASTRO-CONFIG-21` - required Astro integrations present:

- Package presence is checked for `@astrojs/react`, `@astrojs/mdx`, `@astrojs/sitemap`, `astro-robots`, and `@nuasite/checks`.
- Astro config facts come from `astro-config-parser`.
- Pass only when integrations contain these exact module calls:
- default import call from `@astrojs/react`: `import react from "@astrojs/react"; integrations: [react()]`
- default import call from `@astrojs/mdx`: `import mdx from "@astrojs/mdx"; integrations: [mdx()]`
- default import call from `@astrojs/sitemap`: `import sitemap from "@astrojs/sitemap"; integrations: [sitemap()]`
- default import call from `astro-robots`: `import robots from "astro-robots"; integrations: [robots()]`
- default import call from `@nuasite/checks`: `import checks from "@nuasite/checks"; integrations: [checks({...})]`
- named or aliased named import call from `@nuasite/checks`: `import { checks as siteChecks } from "@nuasite/checks"; integrations: [siteChecks({...})]`
- Fail when any integration is missing, imported from a different module, hidden behind an unresolved wrapper, or provided only by a dynamic spread.

`TS-ASTRO-CONFIG-22` - JSON-LD presence check delegated to Nuasite custom check:

- `@nuasite/checks@0.18.0` already validates invalid JSON-LD but not missing JSON-LD.
- Package presence: `package.json` dependencies or devDependencies contains `g3ts-astro-nuasite-checks`.
- Package version policy: Syncpack has canonical pin group `g3ts-astro-nuasite-checks -> 0.1.0`.
- The app imports `structuredDataPresentCheck` from `g3ts-astro-nuasite-checks`.
- The app does not define this check inline.
- The package implementation is:

```ts
import type { Check } from "@nuasite/checks";

export const structuredDataPresentCheck: Check = {
  kind: "page",
  id: "g3/structured-data-present",
  name: "Structured Data Present",
  domain: "seo",
  defaultSeverity: "error",
  description: "Every public page must render at least one JSON-LD block.",
  essential: true,
  run(ctx) {
    return ctx.pageData.jsonLd.length === 0
      ? [
          {
            message: "Page is missing JSON-LD structured data",
            suggestion: "Render a schema-dts typed JSON-LD object in the page head."
          }
        ]
      : [];
  }
};
```

- `checks()` options must contain `customChecks: [structuredDataPresentCheck]`.
- `structuredDataPresentCheck` must be imported from `g3ts-astro-nuasite-checks`.
- G3TS checks only that the custom check is wired; Nuasite executes it after build.

## Style Family Follow-Up

Future style/CSS/UI family owns these policies, not Astro:

- `eslint-plugin-tailwindcss` installation and version.
- `tailwindcss/no-arbitrary-value` effectiveness.
- Tailwind config policy.
- Design token reuse.
- Unknown utility class policy.
- Class ordering.

This must not be implemented in the Astro family.

## Files To Modify Later

Likely G3TS files:

- `packages/parsers/astro-config-parser`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs`
- New rule files under `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src`
- Rule-specific sidecar tests under `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/*_tests`

Likely ESLint plugin files:

- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/tests`

Likely landing app files for validation:

- `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing/package.json`
- `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing/.syncpackrc`
- `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing/eslint.config.mjs`
- `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing/astro.config.mjs`
- `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing/src/ui/BaseLayout.astro`

## Verification Later

G3TS package verification:

- Parser tests for `astro-config-parser`.
- Astro ingestion tests.
- Astro config-check tests.
- `apps/guardrail3-ts` workspace tests.

Plugin verification:

- `npm test`.
- `npm pack --dry-run`.

Landing verification:

- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`.
- `pnpm --filter landing run lint`.
- `pnpm --filter landing run typecheck`.
- `pnpm --filter landing run build`.
- Confirm `@nuasite/checks` fails build on missing rendered SEO requirements after it is wired.
