# TypeScript Check Families

This is the canonical planning root for TypeScript check families.

The old material under:
- `.plans/todo/typescript/**`

is still useful, but it is audit/reference material, not the authoritative family contract.

## Current problem

The implemented TypeScript validator is still grouped into broad runtime buckets:
- architecture
- content
- tests

and old mixed report sections such as:
- config files
- plugin packages
- tool configuration
- source scan

That is the TypeScript equivalent of the old grouped Rust validator model. It is too coarse to be a clean target architecture.

It is also too root-centric.
TypeScript repos can contain:
- a validation root
- a workspace/package-manager root
- many TS app/package roots
- local config roots for `eslint`, `tsconfig`, `stylelint`, and `jscpd`

The family plans must define which roots each family owns.

## Canonical family set

The TypeScript families are:

1. `ts/arch`
2. `ts/eslint`
3. `ts/tsconfig`
4. `ts/npmrc`
5. `ts/package`
6. `ts/fmt`
7. `ts/spelling`
8. `ts/typecov`
9. `ts/size`
10. `ts/jscpd`
11. `ts/css`
12. `ts/code`
13. `ts/hexarch`
14. `ts/libarch`
15. `ts/content`
16. `ts/i18n`
17. `ts/seo`
18. `ts/tests`

Sibling TS-adjacent plans remain:
- `.plans/todo/typescript/checks/hooks/ts.md`
- `.plans/todo/typescript/checks/deploy/ts.md`

Those are reference-only for now.
They are not part of the canonical active TS family set.

## Family intent

- `ts/arch`
  - repo-global TS root placement, ownership, and overlap
- `ts/eslint`
  - `ESLint` config, plugin presence, plugin wiring, rule baseline
- `ts/tsconfig`
  - compiler strictness and TypeScript compiler policy
- `ts/npmrc`
  - pnpm/npm policy root settings
- `ts/package`
  - `package.json` policy, scripts, engines, and banned dependency surface
- `ts/fmt`
  - formatting config and formatter tool wiring
- `ts/spelling`
  - spelling config and spelling tool wiring
- `ts/typecov`
  - type-coverage config, scripts, and threshold policy
- `ts/size`
  - size-budget config, scripts, and threshold policy
- `ts/jscpd`
  - duplication-detection config and duplication-policy wiring
- `ts/css`
  - CSS quality and CSS accessibility config
- `ts/code`
  - direct TypeScript/TSX source scanning rules
- `ts/hexarch`
  - service-app and extension-app hex architecture and boundary enforcement
- `ts/libarch`
  - library/package structure and package-boundary architecture
- `ts/content`
  - content pipeline/model, content-root placement, content-only API/image safety
- `ts/i18n`
  - locale routing, locale/message completeness, translation-usage correctness
- `ts/seo`
  - sitemap, robots, metadata, and static route/SEO surface
- `ts/tests`
  - test-quality rules

## Current code mapping

Today the implementation still lives under:
- `apps/guardrail3/crates/app/ts/validate/**`

That directory should be treated as legacy-grouped implementation input when planning these families.

Approximate current mapping:

- `config_files.rs`
  - currently mixes `eslint`, `tsconfig`, `npmrc`, `package`, `jscpd`
- `package_deps.rs`
  - currently mixes `eslint`, `fmt`, `spelling`, `typecov`, `size`, `css`, `jscpd`
- `tool_config_checks.rs`
  - currently mixes `fmt`, `spelling`, `typecov`, and `size`
- `stylelint_check.rs`
  - currently belongs to `ts/css`
- `eslint_audit.rs` + `ts_arch_checks.rs`
  - currently belong to `ts/hexarch`
- `source_scan.rs`, `ts_comment_checks.rs`, `ts_code_analysis.rs`
  - currently belong to `ts/code`
- `jscpd_check.rs`
  - currently mixes `ts/jscpd` with early content-site checks and must be split
- `test_checks.rs`
  - currently belongs to `ts/tests`
- `i18n_check.rs`
  - currently belongs to `ts/i18n`

There is no current cohesive runtime owner for repo-global TS root placement.
That is the missing `ts/arch` surface.

## Project-shape split

TypeScript planning must respect the same three-way split already used informally across the repo:

- service apps
  - `ts/hexarch`
- content sites
  - `ts/content`
  - `ts/i18n`
  - `ts/seo`
- libraries/packages
  - `ts/libarch`

Those are different architectural contracts and should not be collapsed into one family.

## Root ownership model

The TypeScript plans should use these root kinds:

- validation root
  - repo root where `guardrail3.toml` is resolved
- package-manager root
  - root `.npmrc`, workspace `package.json`, root lockfile
- TS package/app root
  - a local directory with its own `package.json` and TS source/config surface
- local config root
  - the nearest root that owns a local `eslint`, `tsconfig`, `stylelint`, or `jscpd` file

The target ownership split is:

- validation root
  - family selection/config only
- package-manager root
  - `ts/npmrc`
  - package-manager parts of `ts/package`
- TS package/app root
  - `ts/arch`
  - `ts/eslint`
  - `ts/tsconfig`
  - `ts/package`
  - `ts/fmt`
  - `ts/spelling`
  - `ts/typecov`
  - `ts/size`
  - `ts/jscpd`
  - `ts/css`
  - `ts/code`
  - `ts/hexarch`
  - `ts/libarch`
  - `ts/content`
  - `ts/i18n`
  - `ts/seo`
  - `ts/tests`

If a family supports local config ownership, the nearest local config root wins for that family.

## Local config ownership

Nearest-local-config ownership applies only to families with their own local config surface:

- `ts/eslint`
  - nearest `eslint.config.*`
- `ts/tsconfig`
  - nearest `tsconfig*.json`
- `ts/fmt`
  - nearest formatter config surface
- `ts/spelling`
  - nearest `cspell` config surface
- `ts/typecov`
  - nearest type-coverage config surface
- `ts/size`
  - nearest size-budget config surface
- `ts/jscpd`
  - nearest `.jscpd.json`
- `ts/css`
  - nearest stylelint config surface

The following families are not “nearest config wins” families:
- `ts/package`
- `ts/code`
- `ts/hexarch`
- `ts/libarch`
- `ts/content`
- `ts/i18n`
- `ts/seo`
- `ts/tests`

## `package.json` ownership map

`package.json` is shared by several families.
The ownership split must be explicit:

- `ts/package`
  - root/package metadata
  - `private`
  - `packageManager`
  - `engines`
  - `prepare`
  - `preinstall`
  - workspace policy
  - `pnpm.overrides`
  - `pnpm.onlyBuiltDependencies`
  - banned dependency declarations
  - generic required scripts such as `lint` and `typecheck`
- `ts/eslint`
  - `eslint` plugin package presence and conflicts
- `ts/fmt`
  - formatter package presence and format script wiring
- `ts/spelling`
  - spelling package presence and spelling script wiring
- `ts/typecov`
  - type-coverage package presence and type-coverage script wiring
- `ts/size`
  - size-budget package presence and size-budget script wiring
- `ts/css`
  - stylelint package/plugin presence
- `ts/tests`
  - test-runner and mutation-test package presence when those are part of the tests contract

No family may claim a `package.json` key already owned by another family.

## Project-shape scope

The active project shapes are:
- `service`
- `content`
- `library`

`extension` is a `ts/hexarch` variant, not a separate family.

## Project-shape ownership

- `ts/arch`
  - repo-global placement and ownership
- `ts/hexarch`
  - `service`
  - `extension`
- `ts/content`
  - `content`
- `ts/libarch`
  - `library`

## Planning rule

From here on:
- family docs under `.plans/todo/checks/ts/*.md` are the target contract
- old TS docs under `.plans/todo/typescript/**` are mined as inputs
- do not use the old grouped runtime/report categories as the planning model

## Next planning step

The next TypeScript planning tasks are:
- define `ts/arch`
- complete rule-by-rule inventories for:
  - `ts/eslint`
  - `ts/tsconfig`
  - `ts/package`
  - `ts/content`
  - `ts/tests`
- then define the TypeScript validation cutover from grouped runtime buckets to family-based execution

That cutover task is separate from the family definitions themselves.
