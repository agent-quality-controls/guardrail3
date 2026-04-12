# Package Rule ID Normalization

## Summary

Normalized package rule IDs across the lane-bearing Rust families so every package rule now uses the lane-scoped pattern `RS-<FAMILY>-CONFIG-*`, `RS-<FAMILY>-SOURCE-*`, or `RS-<FAMILY>-FILETREE-*`. This removed the leftover legacy app-style IDs from `arch`, `code`, `garde`, `hexarch`, `test`, `topology`, and the merged `hooks` family, and updated package-side tests, assertions, ingestion expectations, and README text to match.

## Decisions made

- Kept the existing numeric suffixes for families whose package lanes did not collide.
  - Why: this minimized churn while still enforcing the package naming rule.
- Renumbered the merged `hooks` package lane IDs from runtime order instead of trying to preserve the old `HOOK-RS` and `HOOK-SHARED` numbers.
  - Why: the old numbers collide inside the merged public family, so preserving them would violate the lane-scoped package contract.
- Split `RS-TEST-10` into lane-specific package IDs after the bulk rename.
  - `RS-TEST-SOURCE-10` now belongs to the source lane.
  - `RS-TEST-FILETREE-10` now belongs to the file-tree lane.
  - Why: one legacy ID existed in two package lanes, which is incompatible with the lane-scoped naming rule.
- Left old app code untouched.
  - Why: this change was scoped to package rule identities and package-side references only.

## Key files for context

- `.plans/2026-04-12-112451-package-rule-id-normalization.md`
- `packages/rs/arch/g3rs-arch-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/source`
- `packages/rs/hexarch/g3rs-hexarch-ingestion/crates/runtime/src/ingest_tests`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest_tests`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Next steps

- If package-to-app bridging is still needed, audit old app expectations against the normalized package IDs before relying on cross-layer result matching.
- Keep new package families on the same lane-scoped ID pattern from the start so this normalization does not repeat.
