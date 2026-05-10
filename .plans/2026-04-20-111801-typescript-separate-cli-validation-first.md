## Goal

Plan the TypeScript migration as a separate product:

- separate CLI app: `guardrail3-ts`
- separate family packages under `packages/ts/{family}`
- validation-only first
- same structural discipline as the Rust migration

This plan supersedes the earlier mixed-app assumption.

## Correction to the previous direction

The previous TS migration idea was wrong because it assumed:

- TS families would wire into `guardrail3-rs`
- TS would be another lane of the Rust app

That is not the target.

Correct target:

- `guardrail3-rs` and `guardrail3-ts` are separate apps
- same idea
- same package architecture
- different product surfaces

## Core direction

Build TypeScript the same way Rust was built:

1. family packages first
2. active app wiring second
3. validation only
4. no generation yet

For TS this means:

- `packages/ts/{family}` package groups
- one separate CLI app under `apps/guardrail3-ts`
- the app only orchestrates validation
- all family logic lives in packages

## Important constraint

The TS family packages are still Rust packages.

So every `packages/ts/**` package must itself satisfy Rust guardrails:

- Rust package structure
- Rust config family requirements
- Rust code/source/filetree checks
- Rust app/package architecture checks where applicable

In other words:

- TS validator logic is implemented in Rust
- therefore TS family packages must be clean under the Rust guardrails too

This should be treated as a hard requirement from day one, not cleanup debt for later.

## Current planning inputs

Canonical TS family inventory:

- `.plans/todo/checks/ts/README.md`

Legacy grouped implementation to mine:

- `legacy/apps/guardrail3-current/crates/app/ts/validate`

Representative family contracts:

- `.plans/todo/checks/ts/eslint.md`
- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/todo/checks/ts/package.md`
- `.plans/todo/checks/ts/content.md`

## Target package architecture

Use the Rust family pattern as closely as possible.

Per family, likely package set:

- `g3ts-<family>-types`
- `g3ts-<family>-ingestion`
- one or more pure check packages

Likely check lanes:

- `-config-checks`
- `-source-checks`
- `-filetree-checks`

But only where the family really needs that lane.

Examples:

- `ts/eslint`
  - `types`
  - `ingestion`
  - `config-checks`
- `ts/tsconfig`
  - `types`
  - `ingestion`
  - `config-checks`
- `ts/npmrc`
  - likely `types`
  - `ingestion`
  - `config-checks`
- `ts/package`
  - likely `types`
  - `ingestion`
  - `config-checks`
- `ts/code`
  - `types`
  - `ingestion`
  - `source-checks`
- `ts/arch`
  - `types`
  - `ingestion`
  - `filetree-checks`

Do not create fake lanes just for symmetry.

## Target app architecture

Create a separate active app:

- `apps/guardrail3-ts`

It should mirror the current Rust app structure:

- inbound CLI
- logic validate command
- outbound report rendering
- app types / traits
- family runner grouping

But it should only own TS validation orchestration.

It should not own:

- family rule logic
- family parsing logic
- grouped legacy validator logic

## Canonical TypeScript family set

Use the family set already defined in `.plans/todo/checks/ts/README.md`:

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

## What to do first

Start with the highest-impact config-policy families:

1. `ts/eslint`
2. `ts/tsconfig`
3. `ts/npmrc`
4. `ts/package`

This is the right first wave because it forces the important substrate decisions:

- root ownership
- local-config ownership
- typed config parsing
- `package.json` key ownership by family
- app wiring from separate family packages

These are the TS equivalents of the Rust config-family foundation.

## Why this first wave

### `ts/eslint`

High impact because it owns:

- config existence / parseability
- required plugin presence
- required plugin wiring
- core rule baseline

And because it overlaps with `package.json` ownership, which we need to get right early.

### `ts/tsconfig`

High impact because it owns:

- compiler strictness
- inheritance / nearest-config handling
- monorepo local override behavior

This is a clean standalone family and a good early specimen.

### `ts/npmrc`

High impact because it sets package-manager policy at the workspace root and should stay narrowly scoped.

It is a good early test of:

- package-manager root ownership
- root-only family behavior

### `ts/package`

High impact because it owns the generic `package.json` contract and must stay disciplined enough not to swallow tool-family concerns.

This family forces explicit key ownership rules.

## Family-boundary rules to preserve

These should be treated as active direction, not optional cleanup ideas.

### 1. Whole typed config files across family boundaries

Same preference as Rust:

- parse once
- type once
- pass the whole typed config file across family boundaries
- do not centrally slice files into per-family mini-views

Examples:

- `eslint.config.*`
- `tsconfig*.json`
- `.npmrc`
- `.jscpd.json`
- stylelint config
- `package.json`

Inside a family, rules still get small local typed inputs.

### 2. Explicit shared-file ownership

Especially for `package.json`, ownership must stay explicit:

- `ts/package`
  - generic metadata, engines, packageManager, workspace policy, generic scripts, `pnpm.*`, generic banned deps
- `ts/eslint`
  - eslint plugin package presence
- `ts/fmt`
  - formatter package presence
- `ts/spelling`
  - spelling package presence
- `ts/typecov`
  - type-coverage package presence
- `ts/size`
  - size-budget package presence
- `ts/css`
  - stylelint package presence
- `ts/tests`
  - test-runner package presence where it is part of tests policy

Do not let `ts/package` become the dumping ground.

### 3. Local config ownership where appropriate

Preserve nearest-local-config ownership for:

- `ts/eslint`
- `ts/tsconfig`
- `ts/fmt`
- `ts/spelling`
- `ts/typecov`
- `ts/size`
- `ts/jscpd`
- `ts/css`

Do not use nearest-config-wins for:

- `ts/package`
- `ts/code`
- `ts/arch`
- `ts/hexarch`
- `ts/libarch`
- `ts/content`
- `ts/i18n`
- `ts/seo`
- `ts/tests`

## Legacy implementation map to normalize

Treat the old grouped TS validator as migration input only.

Grouped legacy surfaces:

- `eslint/`
  - mostly `ts/eslint`
  - architecture-specific parts move to `ts/hexarch`
- `packages/config_files.rs`
  - split into:
    - `ts/eslint`
    - `ts/tsconfig`
    - `ts/npmrc`
    - `ts/package`
    - `ts/jscpd`
- `packages/package_deps.rs`
  - split into:
    - `ts/eslint`
    - `ts/fmt`
    - `ts/spelling`
    - `ts/typecov`
    - `ts/size`
    - `ts/css`
    - `ts/jscpd`
- `packages/tool_config_checks.rs`
  - split into:
    - `ts/fmt`
    - `ts/spelling`
    - `ts/typecov`
    - `ts/size`
- `packages/stylelint_check.rs`
  - `ts/css`
- `packages/jscpd_check.rs`
  - `ts/jscpd`
  - content-specific logic out to `ts/content`
- `packages/i18n_check.rs`
  - `ts/i18n`
- `source/source_scan.rs`
  - `ts/code`
- `source/ts_comment_checks.rs`
  - `ts/code`
- `source/ts_code_analysis.rs`
  - `ts/code`
- `source/test_checks.rs`
  - `ts/tests`
- `topology/ts_topology_checks.rs`
  - split into:
    - `ts/arch`
    - `ts/hexarch`
    - `ts/libarch`
    - maybe some `ts/content` root ownership pieces

## Recommended migration order

Phase 1 - Config-policy foundation:

1. `ts/package`
2. `ts/npmrc`
3. `ts/tsconfig`
4. `ts/eslint`

Phase 2 - Tool/config families:

5. `ts/fmt`
6. `ts/jscpd`
7. `ts/css`
8. `ts/spelling`
9. `ts/typecov`
10. `ts/size`

Phase 3 - Generic source and tests:

11. `ts/code`
12. `ts/tests`

Phase 4 - Architecture families:

13. `ts/arch`
14. `ts/libarch`
15. `ts/hexarch`

Phase 5 - Product-shape families:

16. `ts/content`
17. `ts/i18n`
18. `ts/seo`

## First concrete deliverables

Before writing any TS family logic:

1. Decide TS top-level package namespace and naming:
   - likely `packages/ts/...`
   - likely `g3ts-...`
2. Create the separate app root:
   - `apps/guardrail3-ts`
3. Define the TS app boundary types:
   - family enum
   - validate request
   - family runner traits
   - report model
4. Write the first detailed package plan for:
   - `ts/package`
5. Write the first detailed package plan for:
   - `ts/eslint`
6. Write the app-wiring plan for the separate TS app.

## Non-final questions

These are still open:

1. Exact package naming:
   - `g3ts-*`
   - or another prefix
2. Whether to create one shared TS crawl/discovery package before family work starts
3. Which legacy TS checks should be migrated versus deleted
4. Whether `ts/content`, `ts/i18n`, and `ts/seo` should move together or independently
5. Whether `guardrail3-ts` should share any app-level shared crates with `guardrail3-rs`, or whether the apps stay fully separate except for lower-level generic support crates

## Files to modify later

Likely future anchors:

- new app root under `apps/guardrail3-ts`
- new TS family packages under `packages/ts/`
- `.plans/todo/checks/ts/*.md` for family-specific detailed migrations
- `legacy/apps/guardrail3-current/crates/app/ts/validate/**` as migration input only
