# Package Rule ID Normalization

## Goal

Normalize every package rule ID in `packages/rs` so lane-bearing package checks use lane-scoped IDs instead of old app-family IDs or ad hoc suffixes. The target pattern is `RS-<FAMILY>-CONFIG-*`, `RS-<FAMILY>-SOURCE-*`, and `RS-<FAMILY>-FILETREE-*` for Rust families, with the equivalent family-lane pattern for non-`RS` families.

## Approach

1. Audit every runtime rule ID under `packages/rs` and classify current IDs by family and lane.
2. Decide the exact lane-scoped prefix for each package family from live package structure and existing naming conventions.
3. Rename rule IDs, rule-local expectations, pipeline tests, README references, and any package docs that still mention old IDs.
4. Run the affected package workspaces and grep the final `const ID` surfaces to verify consistency.
5. Write a worklog and commit as a standalone naming fix.

## Key decisions

- Use `FILETREE` as one word.
- Treat this as a package-boundary normalization, not an app-ledger rewrite.
- Keep the old app code untouched unless package tests or bridges require reference updates.
- Rename package IDs consistently across all package lanes instead of fixing `arch` only.

## Files to modify

- `packages/rs/**/crates/runtime/src/**/*.rs`
- `packages/rs/**/README.md`
- `packages/rs/**/crates/runtime/src/**/tests/**/*`
- `packages/rs/**/crates/assertions/src/**/*`
- `packages/rs/**/crates/runtime/src/ingest_tests/**/*`
- `.worklogs/2026-04-12-*-package-rule-id-normalization.md`
