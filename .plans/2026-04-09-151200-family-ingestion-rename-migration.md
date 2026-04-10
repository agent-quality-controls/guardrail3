# Family Ingestion Package Rename Migration

## Goal

Make ingestion package names match the architecture we already settled on:

- one ingestion package per family
- three public entrypoints per family ingestion package:
  - `ingest_for_config_checks`
  - `ingest_for_source_checks`
  - `ingest_for_file_tree_checks`

Target naming:

- `g3rs-cargo-ingestion`
- `g3rs-clippy-ingestion`
- `g3rs-deny-ingestion`
- `g3rs-fmt-ingestion`
- `g3rs-garde-ingestion`
- `g3rs-release-ingestion`
- `g3rs-toolchain-ingestion`
- `g3rs-deps-ingestion`
- `g3rs-code-ingestion`

This removes the current naming lie where packages named `*-config-ingestion`
also expose AST and file-tree ingestion entrypoints.

## Current Problem

There are two conflicting shapes in the repo:

1. Family ingestion packages with stale names:
   - `g3rs-*-config-ingestion`
   - these already expose all three `ingest_for_*_checks` entrypoints

2. A lane-specific outlier:
   - `g3rs-code-ast-ingestion`
   - this does not match the agreed family-ingestion architecture

So the architecture is mostly family-ingestion already, but package naming and
the `code` family layout are inconsistent.

## Decisions

### 1. Keep checks packages lane-specific

Do not rename checks packages.

Keep:

- `g3rs-{family}-config-checks`
- `g3rs-{family}-ast-checks`
- `g3rs-{family}-file-tree-checks`

Reason:

- checks packages really are lane-specific
- their names already match reality

### 2. Rename ingestion packages to family-only names

Rename every `g3rs-*-config-ingestion` package to `g3rs-*-ingestion`.

Reason:

- package name should describe real responsibility
- these packages are not config-only anymore

### 3. Fold `code` back into family ingestion

Replace `g3rs-code-ast-ingestion` with `g3rs-code-ingestion`.

`g3rs-code-ingestion` should expose:

- `ingest_for_config_checks`
- `ingest_for_source_checks`
- `ingest_for_file_tree_checks`

Initial implementation:

- real `ingest_for_source_checks`
- stub `ingest_for_config_checks`
- stub `ingest_for_file_tree_checks`

Reason:

- `code` should follow the same family-ingestion contract as every other family
- `g3rs-code-ast-ingestion` is the structural mismatch

### 4. Do the migration in two phases

Do not combine naming cleanup with new `code` config/file-tree implementation.

Phase 1:

- rename packages
- preserve behavior
- keep stub lanes where they already exist
- fold `code` into `g3rs-code-ingestion`

Phase 2:

- build remaining real ingestion lanes:
  - `code` config
  - `code` file-tree
  - other remaining family gaps

Reason:

- naming cleanup is mechanical
- lane implementation is semantic
- separating them keeps failures easy to diagnose

## Approach

### Phase 1 - package/name migration

1. Rename directories:
   - `g3rs-cargo-config-ingestion` -> `g3rs-cargo-ingestion`
   - `g3rs-clippy-config-ingestion` -> `g3rs-clippy-ingestion`
   - `g3rs-deny-config-ingestion` -> `g3rs-deny-ingestion`
   - `g3rs-fmt-config-ingestion` -> `g3rs-fmt-ingestion`
   - `g3rs-garde-config-ingestion` -> `g3rs-garde-ingestion`
   - `g3rs-release-config-ingestion` -> `g3rs-release-ingestion`
   - `g3rs-toolchain-config-ingestion` -> `g3rs-toolchain-ingestion`
   - `g3rs-deps-config-ingestion` -> `g3rs-deps-ingestion`

2. Fold `g3rs-code-ast-ingestion` into:
   - `g3rs-code-ingestion`

3. Update package manifests:
   - package names
   - workspace members
   - dependency names
   - path dependencies

4. Update crate names / facade exports:
   - `g3rs_*_config_ingestion_*` -> `g3rs_*_ingestion_*`
   - `g3rs_code_ast_ingestion_*` -> `g3rs_code_ingestion_*`

5. Keep public function names unchanged:
   - `ingest_for_config_checks`
   - `ingest_for_source_checks`
   - `ingest_for_file_tree_checks`

6. For `code`:
   - move current AST ingestion runtime/types/assertions into `g3rs-code-ingestion`
   - add stub config/file-tree entrypoints and stub types if needed

7. Update tests and call sites.

8. Verify every renamed workspace with `cargo test --workspace -q`.

### Phase 2 - fill remaining real lanes

1. Build `g3rs-code-ingest_for_config_checks`
   for:
   - `RS-CODE-07`
   - `RS-CODE-12`

2. Build `g3rs-code-ingest_for_file_tree_checks`
   for:
   - `RS-CODE-35`

3. Keep checks runtime packages lane-specific.

## Files / Areas To Modify

Primary package roots:

- `packages/rs/cargo/*`
- `packages/rs/clippy/*`
- `packages/rs/deny/*`
- `packages/rs/fmt/*`
- `packages/rs/garde/*`
- `packages/rs/release/*`
- `packages/rs/toolchain/*`
- `packages/rs/deps/*`
- `packages/rs/code/*`

Planning/docs:

- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`
- family plan files that mention `*-config-ingestion`

## Migration Risks

- crate-name churn across many Cargo manifests
- stale path dependencies after directory rename
- duplicate package identities during the `code` fold-over
- outdated docs continuing to describe `*-config-ingestion`

## Done Means

- no ingestion package in `packages/rs/*` is named `*-config-ingestion`
- no lane-specific ingestion package remains for `code`
- all ingestion packages are named `g3rs-{family}-ingestion`
- all family ingestion packages expose exactly:
  - `ingest_for_config_checks`
  - `ingest_for_source_checks`
  - `ingest_for_file_tree_checks`
- tests pass for every renamed ingestion workspace
