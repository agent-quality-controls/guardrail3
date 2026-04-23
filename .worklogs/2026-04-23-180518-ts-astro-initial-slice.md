Summary

Implemented the initial `ts/astro` family and the repo-owned `eslint-plugin-astro-pipeline`, wired the Astro family into `apps/guardrail3-ts`, and closed the first adversarially-found rule and contract bugs. The slice is mechanically green across the plugin, the Astro crates, and the TS app workspace.

Decisions made

- Kept source-policy in the ESLint plugin, not in `g3ts`.
  - Implemented plugin rules for:
    - `no-authored-content-fs-read`
    - `no-authored-content-glob`
    - `no-direct-astro-content-in-routes`
    - `no-side-loader-imports`
    - `no-runtime-mdx-eval`
  - Reason: these are AST/source-boundary checks and belong in lint/plugin space.

- Narrowed `TS-ASTRO-FILETREE-05` instead of reintroducing raw string scanning.
  - Live ingestion currently leaves `cross_root_side_loaders` empty.
  - The family plan and implementation plan now say the live discovery is deferred until there is a real parser-owned or plugin-owned source fact.
  - Reason: the earlier string-literal scan was a boundary bug.

- Strengthened the config contract instead of weakening it.
  - `TS-ASTRO-CONFIG-07` now requires the Astro pipeline plugin to be both present and effective at error severity.
  - The ESLint surface model is lane-aware for TS and TSX instead of unioning both lanes into one bag.
  - Added `TS-ASTRO-CONFIG-01` for Astro package presence.

- Added the missing planned live-config rule.
  - `TS-ASTRO-FILETREE-03` now exists and uses a dedicated `live_collection_roots` input lane.

- Fixed TS app/workspace breakage that blocked verification.
  - Repointed stale TS ingestion crates from the nonexistent `packages/shared/g3-workspace-crawl` path to `packages/rs/g3rs-workspace-crawl` with the correct package rename.
  - Updated TS ingestion/runtime imports from the old `G3Workspace*` names to `G3RsWorkspace*` aliases where needed.

Key files for context

- Plans
  - `.plans/2026-04-23-151845-ts-astro-family-plan.md`
  - `.plans/2026-04-23-163555-ts-astro-implementation.md`

- Plugin
  - `packages/ts/eslint-plugin-astro-pipeline/src/index.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/configs/recommended.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-fs-read.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-authored-content-glob.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-direct-astro-content-in-routes.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-side-loader-imports.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/rules/no-runtime-mdx-eval.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/utils/ast-helpers.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/src/utils/import-closure.ts`
  - `packages/ts/eslint-plugin-astro-pipeline/tests/*.test.ts`

- Astro family
  - `packages/ts/astro/g3ts-astro-types/src/types.rs`
  - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
  - `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
  - `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/*.rs`
  - `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/*.rs`

- TS app wiring
  - `apps/guardrail3-ts/Cargo.toml`
  - `apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs`
  - `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
  - `apps/guardrail3-ts/crates/io/outbound/report/crates/runtime/src/plain_text.rs`

Next steps

- Implement a shared Astro config parser under `packages/parsers` and use it from `g3ts-astro-ingestion`.
  - Required for:
    - `TS-ASTRO-CONFIG-05` Astro ESLint wiring in `astro.config.*`
    - `TS-ASTRO-CONFIG-08` `@nuasite/checks` integration wiring in `astro.config.*`
    - any future Astro integration/package coupling checks

- Replace the root-collapsed Astro model with real app-root discovery.
  - Current ingestion still emits a single app root at `"."`.
  - Follow-up should model multiple/nested Astro app roots and per-root package/eslint/config surfaces.

- Decide whether to broaden plugin closure resolution for repo alias imports.
  - Current `collectImportClosure()` follows relative/absolute imports only.
  - If the repo standardizes `@/` or `~/` alias imports in Astro apps, the plugin needs an alias-aware resolver contract.

- Decide whether to broaden the plugin to catch path-building and alias indirection beyond the current slice.
  - Remaining candidate work:
    - `path.join(...)`/`new URL(...)` in filesystem reads
    - `import.meta.glob` variable indirection
    - additional runtime-MDX alias patterns
