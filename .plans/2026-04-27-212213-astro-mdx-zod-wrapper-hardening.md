# Goal

Make Astro MDX pages fail at build time when MDX component props are malformed, and prevent MDX authors from bypassing approved validated wrappers.

# Scope

- Astro MDX content apps only.
- Source-level enforcement belongs to `g3ts-eslint-plugin-astro-pipeline`.
- G3TS must enforce that the new plugin rules are active on the MDX lane and approved component-map lane.
- Runtime validation belongs in the app's MDX component-map wrappers through Zod.

# Required App Pattern

Approved MDX component-map modules are configured in `guardrail3-ts.toml`:

```toml
[ts.astro.mdx]
component_maps = ["src/mdx-components.tsx"]
```

Each approved component-map module must:

- Import `z` from `zod`.
- Define local Zod schemas for every exported MDX component.
- Define or import `parseMdxComponentProps`.
- Export only validated MDX wrapper components.
- Each exported MDX wrapper must accept raw props and call `parseMdxComponentProps("<WrapperName>", schema, rawProps)`.
- The wrapper may then map parsed props into presentational UI components.

Example:

```tsx
const faqPropsSchema = z.object({
  items: z.array(z.object({
    question: z.string().min(1),
    answer: z.string().min(1),
  })).min(1),
});

export function FAQ(rawProps: unknown): ReactElement {
  const props = parseMdxComponentProps("FAQ", faqPropsSchema, rawProps);

  return <FaqSection items={props.items.map((item) => ({ q: item.question, a: item.answer }))} />;
}
```

# ESLint Plugin Rules

## `astro-pipeline/mdx-imports-only-approved-components`

Target:

- Files matched by `mdxContentGlobs`.

Inputs:

- `approvedMdxComponentModules`
- `approvedMdxComponentNames`

Behavior:

- Every runtime import in MDX must resolve to one of `approvedMdxComponentModules`.
- Every imported specifier from an approved component-map module must be listed in `approvedMdxComponentNames`.
- Namespace imports from approved component-map modules are forbidden.
- Default imports from approved component-map modules are forbidden.
- Type-only imports remain irrelevant for MDX content and should be rejected in MDX content unless there is a concrete allowed use case later.

Why:

- Existing `mdx-component-imports-from-approved-map` proves the module is approved, but not that the imported names are approved validated wrappers.

## `astro-pipeline/mdx-component-map-no-raw-ui-exports`

Target:

- Files matched by `approvedMdxComponentModules`.

Inputs:

- `rawUiModuleGlobs`

Behavior:

- The rule must fail closed when `rawUiModuleGlobs` is empty.
- The error message must tell the agent to configure `rawUiModuleGlobs` explicitly.
- Forbid direct re-export declarations from raw UI modules:
  - `export { FaqSection } from "@project/ui"`
  - `export * from "@project/ui"`
- Forbid exporting imported raw UI component bindings directly:
  - `import { FaqSection } from "@project/ui"; export { FaqSection };`
  - `export const FAQ = FaqSection;`
- Allow wrappers that import raw UI components but export a distinct local function that validates props first.

Why:

- MDX must consume validated article wrappers, not design-system components with loose content props.

## `astro-pipeline/mdx-component-wrapper-requires-zod-parse`

Target:

- Files matched by `approvedMdxComponentModules`.

Inputs:

- `approvedMdxComponentNames`.
- `mdxPropsParserName`.

Behavior:

- The rule must fail closed when `approvedMdxComponentNames` is empty.
- The rule must fail closed when `mdxPropsParserName` is empty.
- The error message must tell the agent to configure both options explicitly.
- Every exported function/component whose name is listed in `approvedMdxComponentNames` must call `mdxPropsParserName`.
- The module must not export runtime values that are not listed in `approvedMdxComponentNames`, except names explicitly listed in `allowedMdxComponentMapExports`.
- `allowedMdxComponentMapExports` exists only for non-component map exports such as `mdxComponents`; it has no defaults.
- The call must include:
  - first arg: string literal equal to the exported wrapper name
  - second arg: a local schema identifier
  - third arg: the raw props parameter or an object derived directly from it
- The schema identifier must be initialized in the same module from a `z.object(...)` expression or a chained expression rooted at `z.object(...)`.
- The file must import `z` from `zod`.
- The wrapper must not type its public raw props parameter as a rich public props interface without runtime parsing. The accepted public parameter type is `unknown` or a small raw object whose values are parsed immediately.

Why:

- TypeScript does not reliably validate MDX prop payloads as content. Every MDX component exposed by the component map must validate its own props during Astro static render.

## `astro-pipeline/no-raw-mdx-images`

Target:

- Files matched by `mdxContentGlobs`.

Inputs:

- `approvedMdxImageComponents`.

Behavior:

- Forbid Markdown image syntax in MDX:
  - `![alt](src)`
- Forbid raw HTML/JSX image elements:
  - `<img ... />`
- Forbid importing image helpers except approved image wrapper names from approved component-map modules.
- Allow approved image wrapper components, for example:
  - `<ArticleImage src="..." alt="..." />`
- The rule has no default image wrapper names. Apps must configure `approvedMdxImageComponents` explicitly.

Why:

- Article images must go through the approved R2/image policy wrapper.

# G3TS Checks

Add Astro MDX config checks:

## `TS-ASTRO-MDX-CONFIG-35`

- Requires ESLint MDX lane to enforce `astro-pipeline/mdx-imports-only-approved-components` at error severity.
- Requires non-empty `approvedMdxComponentModules`.
- Requires non-empty `approvedMdxComponentNames`.
- Applies to MDX content probe.

## `TS-ASTRO-MDX-CONFIG-36`

- Requires approved component-map lane to enforce `astro-pipeline/mdx-component-map-no-raw-ui-exports` at error severity.
- Requires the probe target to be one configured `[ts.astro.mdx].component_maps` source.

## `TS-ASTRO-MDX-CONFIG-37`

- Requires approved component-map lane to enforce `astro-pipeline/mdx-component-wrapper-requires-zod-parse` at error severity.
- Requires the rule options to include non-empty `approvedMdxComponentNames` and non-empty `mdxPropsParserName`.

## `TS-ASTRO-MDX-CONFIG-38`

- Requires ESLint MDX lane to enforce `astro-pipeline/no-raw-mdx-images` at error severity.
- Requires non-empty `approvedMdxImageComponents`.

# Build-Time Contract

Existing checks already cover the execution path:

- `TS-ASTRO-SETUP-CONFIG-02` requires `astro check`.
- `TS-ASTRO-SETUP-CONFIG-33` requires `eslint`.
- `TS-ASTRO-SEO-CONFIG-13` requires `astro build` with Nuasite.

The new MDX validation depends on `astro build` rendering every static MDX page. If an MDX article passes malformed props into an `Article*` wrapper, Zod throws during render and `astro build` fails.

# Files To Modify

- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/utils/options.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/index.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/mdx-imports-only-approved-components.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/mdx-component-map-no-raw-ui-exports.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/mdx-component-wrapper-requires-zod-parse.ts`
- `packages/ts/g3ts-eslint-plugin-astro-pipeline/src/rules/no-raw-mdx-images.ts`
- Plugin tests for all four rules.
- `packages/ts/astro/mdx/g3ts-astro-mdx-types/src/types.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/eslint.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/run.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/ts_astro_config_35_mdx_import_names.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/ts_astro_config_36_no_raw_ui_exports.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/ts_astro_config_37_mdx_component_wrapper_zod_parse.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-config-checks/crates/runtime/src/ts_astro_config_38_no_raw_mdx_images.rs`
- Landing app ESLint config and `src/mdx-components.tsx` after plugin release/install.

# Verification

- Plugin unit tests:
  - bad MDX import module fails
  - approved module with unapproved specifier fails
  - namespace import fails
  - raw markdown image fails
  - raw `<img>` fails
  - raw UI re-export fails
  - approved component-map export without parser fails
  - approved component-map export with wrong parser first arg fails
  - approved component-map export with non-Zod schema fails
  - runtime export not listed in `approvedMdxComponentNames` fails
  - valid wrapper passes
- G3TS Rust tests:
  - missing each new rule reports Error
  - warning-level rule reports Error
  - empty option lists report Error
  - valid effective config reports Info
- Landing verification:
  - install new plugin and local G3TS
  - run `g3ts validate --path apps/landing --family astro --inventory`
  - run `pnpm --filter landing run lint`
  - run `pnpm --filter landing run build`
  - create a temporary bad MDX copy with raw `<img>` and prove lint fails
  - create a temporary bad wrapper without Zod parse and prove lint fails
  - create a temporary bad MDX prop shape and prove build fails

# Non-Goals

- Do not build a general MDX type checker inside G3TS.
- Do not parse MDX in Rust.
- Do not validate all article content semantics in G3TS.
- Do not make content/link checking part of this slice.
