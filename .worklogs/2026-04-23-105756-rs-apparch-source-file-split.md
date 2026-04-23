Summary
- Split the oversized apparch ingestion source file without changing source-ingestion behavior.
- Moved reexport and source-path helper logic into a sibling run-level helper module so the package is green under `g3rs validate` again.

Decisions made
- Kept `crates/runtime/src/run/source.rs` as the owning module because its test sidecar and module layout already fit the repo conventions.
- Rejected the `source/mod.rs` conversion because it triggered mod.rs facade rules and broke the existing sidecar shape.
- Extracted only shared helper logic into `crates/runtime/src/run/source_support.rs` instead of changing the ingestion API or test inventory.

Key files for context
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/source_support.rs`
- `packages/rs/apparch/g3rs-apparch-ingestion/crates/runtime/src/run/mod.rs`
- `.plans/2026-04-23-104423-rs-apparch-source-file-split.md`

Next steps
- Fix the remaining real follow-up bugs from the attack review:
  - broaden release workflow token and dry-run detection
  - reject malformed toolchain channel strings like `stable-foo`
