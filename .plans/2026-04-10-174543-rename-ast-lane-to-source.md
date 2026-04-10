# Rename AST Lane To Source

## Goal

Rename the current Rust "AST" lane to "source" so package names, public types, and ingestion entrypoints match the actual boundary:

- source checks
- config checks
- file-tree checks

The repo should stop exposing new Rust checks packages as `*-ast-checks` and stop exposing ingestion entrypoints as `ingest_for_ast_checks`.

## Approach

1. Rename the concrete extracted packages:
   - `g3rs-code-ast-checks` -> `g3rs-code-source-checks`
   - `g3rs-garde-ast-checks` -> `g3rs-garde-source-checks`
   - `g3rs-test-ast-checks` -> `g3rs-test-source-checks`

2. Rename their internal workspace crates:
   - `*-ast-checks-runtime` -> `*-source-checks-runtime`
   - `*-ast-checks-types` -> `*-source-checks-types`
   - `*-ast-checks-assertions` -> `*-source-checks-assertions`

3. Rename the public lane API everywhere it is already part of the new package model:
   - `ingest_for_ast_checks` -> `ingest_for_source_checks`
   - `*AstChecksInput` -> `*SourceChecksInput`

4. Update dependent ingestion packages and family types so the source lane is named consistently, including placeholder families that do not implement source checks yet.

5. Update plans, READMEs, TODOs, worklogs, and scripts where they describe the current lane names as active architecture.

6. Verify every affected package workspace independently.

## Key decisions

### Rename the lane API, not just the package directories
- Chose: rename packages, public input types, and ingestion function names together.
- Why: keeping `ingest_for_ast_checks` and `*AstChecksInput` after the package rename would preserve the same boundary mistake under new directory names.

### Keep rule IDs unchanged
- Chose: keep rule IDs such as `RS-CODE-*`, `RS-GARDE-*`, `RS-TEST-*`.
- Why: the lane rename is architectural naming, not rule inventory churn.

### Rename placeholder AST types too
- Chose: rename placeholder `*AstChecksInput` types in non-source families.
- Why: the lane name must stay coherent even where implementation is still stubbed.

## Files to modify

- `packages/rs/*/g3rs-*-ast-checks/**`
- `packages/rs/*/g3rs-*-ingestion/**`
- `packages/rs/*/g3rs-*-types/**`
- `apps/guardrail3/Cargo.lock`
- `.plans/**`
- `.worklogs/**`
- package READMEs / TODOs that describe the active lane names
