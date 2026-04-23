## Summary

Built the first substantial `ts/astro` slice and the companion `eslint-plugin-astro-pipeline` slice. The work added a shared Astro config parser, moved Astro ingestion to shared-parser consumption, hardened the ESLint plugin against real bypasses, and corrected several Astro-family contract bugs that the attack passes surfaced.

## Decisions Made

- Added a shared parser package at `packages/parsers/astro-config-parser` instead of letting the Astro family parse `astro.config.*` itself.
  - Why: this matches the repo architecture already used on the Rust side and keeps parsing separate from family ingestion.
  - Rejected: ad hoc AST parsing inside `g3ts-astro-ingestion`.

- Kept `ts/astro` focused on Astro-specific setup and source-policy contracts, not generic ESLint/package ownership.
  - Why: Astro should assert Astro-specific requirements on shared config surfaces, not own those file types outright.

- Removed live execution of `TS-ASTRO-FILETREE-05`.
  - Why: the rule had no real ingestion facts and was effectively dead in production.
  - Rejected: keeping a live rule wired while ingestion hardcoded `cross_root_side_loaders = []`.

- Removed live execution of render-validator config rules `TS-ASTRO-CONFIG-04` and `TS-ASTRO-CONFIG-08`.
  - Why: current ingestion has no real cross-family policy fact that enables render-validator enforcement, so those rules were dead-on-arrival in real scans.
  - Rejected: keeping dead rules active behind a permanently false flag.

- Moved Astro ingestion from root-only discovery to per-app-root discovery.
  - Why: nested Astro apps in monorepos were being checked against the workspace root, which is a correctness bug.
  - Rejected: keeping `APP_ROOT_REL_PATH = "."`.

- Treated ignored ESLint probes as absent lanes.
  - Why: requiring Astro/plugin rules on ignored files is a false positive.
  - Rejected: counting requested probes as live lanes even when ESLint ignores them.

- Extended the ESLint plugin import-closure walker to follow:
  - `require()` edges
  - `.astro` helper modules
  - `.mdx` helper modules
  - extensionless `.mdx` resolution
  - Why: closure-based rules were missing real bypasses.

- Narrowed `no-side-loader-imports` cross-root behavior.
  - Why: cross-root utilities that do not touch content should not be flagged.
  - Rejected: blanket cross-root prohibition.

## Key Files For Context

- Plan
  - `.plans/2026-04-23-151845-ts-astro-family-plan.md`

- Shared parser
  - `packages/parsers/astro-config-parser/crates/runtime/src/parser.rs`
  - `packages/parsers/astro-config-parser/crates/runtime/src/parser_tests/cases.rs`

- Shared ESLint parser extension
  - `packages/parsers/eslint-config-parser/crates/types/src/document.rs`
  - `packages/parsers/eslint-config-parser/crates/runtime/src/parser_tests/cases.rs`

- Astro family
  - `packages/ts/astro/g3ts-astro-types/src/types.rs`
  - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
  - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
  - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run_tests/cases.rs`
  - `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs`
  - `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/support.rs`
  - `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs`
  - `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/run.rs`

- ESLint plugin
  - `packages/ts/eslint-plugin-astro-pipeline/src/utils/import-closure.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/utils/ast-helpers.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-fs-read.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-glob.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-direct-astro-content-in-routes.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-runtime-mdx-eval.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-side-loader-imports.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/tests/no-side-loader-imports.test.ts`

## Verification

- `npm test` in `packages/ts/eslint-plugin-astro-pipeline`
- `cargo test -q --manifest-path packages/parsers/astro-config-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-file-tree-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`

## Next Steps

- Finish cross-module alias-graph handling in the ESLint plugin for implemented rules:
  - fs-read aliases re-exported from helper modules
  - imported runtime `Function` aliases
  - imported `glob` aliases
  - direct route-side `require("astro:content")`
  - cross-root mirrors that bypass ESM `astro:content`

- Reintroduce render-validator rule IDs only after a real policy/fact source exists.

- Reintroduce `TS-ASTRO-FILETREE-05` only after ingestion can produce real `cross_root_side_loaders` facts without ad hoc parsing.
