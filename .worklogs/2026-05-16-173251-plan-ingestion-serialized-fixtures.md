# Plan Ingestion Serialized Fixtures

## Summary

Investigated why `g3rs-code-ingestion` still has 49 kept tests after the earlier serialization work. The serializer exists, but the fixture suite only records two fixture repos, so it does not expose the branches covered by the remaining code-ingestion unit tests.

## Decisions Made

- Kept the previous code-ingestion fixture-output crate as valid infrastructure.
- Classified the missing work as fixture coverage, not failed serialization transport.
- Planned one consistent fixture-output crate shape for every Rust ingestion family.
- Required direct `serde::Serialize` on owned Rust types instead of adapters, mirror structs, Python field mapping, or hand-written JSON.
- Required ledger disposition changes only after fixture3 approved output proves the behavior.

## Key Files

- `.plans/2026-05-16-173104-all-ingestion-serialized-fixtures.md`
- `.plans/2026-05-16-173104-all-ingestion-serialized-fixtures.md.manifest.toml`
- `fixture3.yaml`
- `behavior/golden/g3rs-code-ingestion/approved.normalized.json`
- `behavior/migration/g3rs-kept-test-disposition.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/fixture-output/src/run.rs`

## Next Steps

- Finish `g3rs-code-ingestion` fixture coverage first.
- Add fixture-output crates and fixture3 suites family by family.
- Update kept-test dispositions only when approved fixture output covers each row.
- Run family cargo checks, fixture3 checks, behavior verification, and `g3rs validate repo` after each wave.
