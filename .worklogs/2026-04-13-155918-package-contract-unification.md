## Summary

Unified the public package input contract across the finished audited families that had drifted. `code` and `hooks` now have family-level `-types` crates, `garde` source types now live in `g3rs-garde-types`, and `clippy` now exposes the same placeholder source-lane contract shape as the older audited families.

## Decisions made

- Preserved the older audited family contract instead of inventing a new one.
  - One family `g3rs-<family>-types` crate owns the public lane input types.
  - Lane-local `crates/types` crates remain for compatibility but now re-export from the family crate.
- Left `hexarch` and `topology` out of this pass.
  - Reason: the user explicitly called them unfinished exceptions.
- Added the missing `clippy` source-lane placeholder and ingestion stub instead of deleting placeholder lanes from older families.
  - Reason: the stable contract across the audited families includes honest lane-shaped stubs for unsupported lanes.
- Kept rule runtimes unchanged except for the `clippy` public ingestion export.
  - Reason: this task was boundary unification, not rule logic refactoring.

## Key files for context

- `.plans/2026-04-13-155024-package-contract-unification.md`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/basic.rs`

## Next steps

- If the contract should be stricter than the older audited pattern, the next architectural decision is whether to remove placeholder unsupported lanes from all finished families instead of preserving them.
- If package naming is also part of the stable contract, the next separate pass is normalizing `file-tree` vs `filetree` across package roots.
- Revisit `hexarch` and `topology` only after their lane boundaries are actually settled.
