## Goal

Create the first clean `packages/ts/eslint` family package set:

- `g3ts-eslint-types`
- `g3ts-eslint-ingestion`
- `g3ts-eslint-config-checks`

End state for this pass:

- the three package roots exist under `packages/ts/eslint`
- each root follows the validated Rust package pattern already used in `packages/rs`
- each root passes `guardrail3-rs validate`
- the family exports minimal real contracts and placeholder runtime entry points
- no `guardrail3-ts` app wiring yet
- no actual ESLint parser/rule inventory implementation yet

## Approach

1. Mirror the structural package seam from `rs/clippy`.
   - `types` owns the shared family contract
   - `ingestion` owns discovery/orchestration entry points
   - `config-checks` owns the future rule-execution entry point

2. Keep the first contracts minimal but real.
   - `g3ts-eslint-types` defines the typed ESLint config/check input surface
   - `g3ts-eslint-ingestion` exposes `ingest_for_config_checks`
   - `g3ts-eslint-config-checks` exposes `check`

3. Keep the first ingestion behavior intentionally narrow.
   - detect only root-level `eslint.config.{js,mjs,cjs,ts,mts,cts}` candidates via `g3rs-workspace-crawl`
   - choose the first preferred root candidate
   - do not attempt true JS/TS config parsing yet

4. Make the placeholder runtime honest.
   - `config-checks` returns no findings for now because no rule files are implemented yet
   - the package docs should say scaffold/placeholder clearly
   - do not fake parseability or rule enforcement claims in docs

5. Validate every new root with `guardrail3-rs validate` and fix the package shape until all three are clean.

## Key Decisions

- Build the family packages before `guardrail3-ts`.
  - Reason: the user explicitly wants family packages first.

- Use `g3ts-*` naming.
  - Reason: this is the current repo direction in the TS migration plans.

- Skip `filetree-checks` in the first ESLint pass.
  - Reason: the current family plan only locked `types + ingestion + config-checks`, and adding a fourth root now would widen scope before the family even has a typed config path.

- Do not copy legacy grouped rule code into this first scaffold.
  - Reason: the legacy ESLint implementation is boundary-mixed and would immediately pollute the new package seam.

## Latest Ideas, Not Final Decisions

- `ts/eslint` may eventually need a `filetree-checks` lane if active-config shadowing becomes a first-class rule surface.
- `g3ts-eslint-types` may later absorb typed `package.json`-derived plugin presence facts, or those may stay as a cross-family input from `ts/package`.
- real ESLint config parsing may need a dedicated parser package rather than living directly in `g3ts-eslint-ingestion`.

## Files To Modify

- `packages/ts/eslint/g3ts-eslint-types/**`
- `packages/ts/eslint/g3ts-eslint-ingestion/**`
- `packages/ts/eslint/g3ts-eslint-config-checks/**`

## Verification

- `cargo test` in each new package root
- `cargo fmt --check` in each new package root
- `guardrail3-rs validate --path <each root>`
