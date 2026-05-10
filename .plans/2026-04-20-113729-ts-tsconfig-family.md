## Goal

Create `packages/ts/tsconfig` as the TypeScript compiler-policy family.

End state for wave 1:

- root-aware discovery of `tsconfig.base.json` and local `tsconfig.json` files
- parseability checks
- inheritance checks
- strict-baseline checks
- no write path yet

## Local Intent

The current local intent is not "one repo-root option bag". It is a root-aware family:

- `tsconfig.base.json` at the repo root is the shared base
- per-app/package `tsconfig.json` files either extend the base or carry the strict flags inline

The by-file notes split the surface into:

- merge-managed `tsconfig.base.json`
- validate-only per-app/package `tsconfig.json`

The legacy checker bundles:

- existence
- parseability
- strict booleans
- `target`
- `module`
- `moduleResolution`
- extra-option inventory

That bundle is too broad for the first migration wave.

## External 2026 Delta

Several old assumptions are stale:

- TypeScript 5.0 allows `extends` arrays, not only a single parent config.
- TypeScript 5.6 added stricter options such as:
  - `noUncheckedSideEffectImports`
  - `strictBuiltinIteratorReturn`
- TypeScript 6.0 deprecates or removes older knobs such as:
  - `baseUrl`
  - legacy `moduleResolution` values
  - the ability to force `esModuleInterop: false`
- Current TS docs differentiate app/bundler guidance from library guidance more clearly.

## Approach

1. Build a root-aware orchestrator that discovers:
   - root `tsconfig.base.json`
   - local `tsconfig.json`
2. Parse once into typed JSONC-aware or JSON-compatible structures.
3. Fan out into pure rules by concern:
   - exists/parseable
   - extends/inheritance
   - strict-baseline
4. Keep the first wave read-only.

## Wave 1 Rule Cut

Wave 1 should include:

- root `tsconfig.base.json` exists
- root `tsconfig.base.json` parses
- local `tsconfig.json` surfaces parse
- local `tsconfig.json` either:
  - extends the base, or
  - carries the required strict booleans inline
- the strict baseline checks the current planned 12 booleans only

Wave 1 should not yet enforce:

- `target`
- `lib`
- `module`
- `moduleResolution`
- `declaration`
- `declarationMap`
- `noEmit`

Those are real policy surfaces, but they are a second-wave decision because they differ between apps and publishable packages.

## Key Decisions

- Keep wave 1 read-only.
  - Reason: write/merge semantics for JSONC comments are still unresolved.

- Keep the strict baseline narrow at first.
  - Reason: the 12 boolean flags are already locally documented and are the least ambiguous cross-project contract.

- Support both string and array `extends`.
  - Reason: TS 5.0 made single-parent assumptions obsolete.

## Latest Ideas, Not Final Decisions

- `moduleResolution: bundler` may be correct for app roots, but not for published library/package roots.
- Some TS 5.6+ strict flags may deserve inclusion in wave 1, but they are not yet part of the local 12-flag contract.
- `tsconfig.base.json` may eventually become merge-managed, but validate-only is the safer first cut.

## Risks / Open Questions

- If comments must be preserved, `serde_json` is not enough for write-mode later.
- The old known-key inventory is stale relative to TS 6.0.
- We still need an explicit app-vs-library applicability matrix for the later module/emit rules.

## Files To Modify

- `packages/ts/tsconfig/**`
- `apps/guardrail3-ts/**` for wiring only after the family exists
- top-level workspace manifests as needed

## Source Inputs

- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/todo/checks/ts/README.md`
- `.plans/by_file/ts/tsconfig.md`
- `legacy/apps/guardrail3-current/crates/app/ts/validate/packages/tsconfig_check.rs`
- `legacy/apps/guardrail3-current/crates/app/ts/validate/packages/config_files.rs`
- `legacy/apps/guardrail3-current/crates/app/core/project_map.rs`
- TypeScript docs:
  - tsconfig overview
  - TS 5.0 release notes
  - TS 5.6 release notes
  - TS 6.0 release notes
  - module guidance
