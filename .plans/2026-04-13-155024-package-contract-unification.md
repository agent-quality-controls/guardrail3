## Goal

Unify finished Rust package families behind one stable public input contract:
- one family-level `g3rs-<family>-types` crate owns the public lane input types
- lane-local `crates/types` crates re-export from the family types crate
- ingestion entrypoints expose the same honest lane capability pattern used by the older audited families

Exclude unfinished families from this pass:
- `hexarch`
- `topology`

## Approach

1. Read the older audited families and preserve their contract shape.
   - Use `toolchain`, `fmt`, `cargo`, `deny`, `deps`, and `release` as the contract baseline.
2. Fix families that drifted from that contract.
   - `code`: add a new `g3rs-code-types` crate, move public lane input ownership there, re-export from lane-local types, and point ingestion types at the family crate.
   - `hooks`: add a new `g3rs-hooks-types` crate, move public lane input ownership there, re-export from lane-local types, and point ingestion types at the family crate.
   - `garde`: move the source-lane public types into `g3rs-garde-types`, then make the source lane-local types crate re-export them.
   - `clippy`: add a placeholder source-lane input to `g3rs-clippy-types` and add the matching source ingestion stub and error variant so its public contract matches the audited pattern.
3. Keep rule runtimes unchanged unless the contract boundary forces a small import change.
4. Verify the affected package workspaces compile and tests pass.

## Key Decisions

- Preserve the older audited contract instead of inventing a stricter new one.
  - Reason: the user explicitly said the older packages were personally audited and should be treated as the better contract.
- Do not normalize `hexarch` and `topology` in this pass.
  - Reason: they are unfinished and explicitly called out as exceptions.
- Keep lane-local `crates/types` crates for compatibility, but make them re-export family-level public types.
  - Reason: this unifies the public contract without forcing broad downstream import churn.
- Add `clippy` source placeholder and source ingestion stub instead of deleting other families' placeholders.
  - Reason: the audited contract already exposes honest family-level lane shape even when a lane is unimplemented.

## Files To Modify

- `.plans/2026-04-13-155024-package-contract-unification.md`
- `packages/rs/code/g3rs-code-types/**` (new)
- `packages/rs/code/g3rs-code-config-checks/crates/types/**`
- `packages/rs/code/g3rs-code-source-checks/crates/types/**`
- `packages/rs/code/g3rs-code-file-tree-checks/crates/types/**`
- `packages/rs/code/g3rs-code-ingestion/crates/types/**`
- `packages/rs/hooks/g3rs-hooks-types/**` (new)
- `packages/rs/hooks/g3rs-hooks-config-checks/crates/types/**`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/types/**`
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/crates/types/**`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/types/**`
- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/types/**`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/types/src/error.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/src/lib.rs`
