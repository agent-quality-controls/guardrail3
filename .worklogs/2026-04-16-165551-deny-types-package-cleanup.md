Summary

Normalized `packages/rs/deny/g3rs-deny-types` to the current shared `*-types` workspace shape. The package now validates clean as a one-crate workspace with a small facade in `src/lib.rs`, the shared transport records in `src/types.rs`, and explicit root policy files.

Decisions made

- Kept this as a one-crate workspace.
  - Why: this package is just the shared deny type surface, so it does not need child crates to satisfy the current package rules.
  - Rejected: inventing runtime or assertions crates for a pure shared types package.
- Kept the public fields on the transport structs.
  - Why: this package is exactly the shared plain-record case that the current `RS-CODE-SOURCE-31` exception path is meant to allow.
  - Rejected: adding constructors and getters to plain transport records.
- Added the documented `module_name_repetitions` waiver.
  - Why: the family-prefixed shared type names are intentional and already follow the established `*-types` pattern.

Key files for context

- `packages/rs/deny/g3rs-deny-types/Cargo.toml`
- `packages/rs/deny/g3rs-deny-types/src/lib.rs`
- `packages/rs/deny/g3rs-deny-types/src/types.rs`
- `packages/rs/deny/g3rs-deny-types/guardrail3-rs.toml`

Next steps

- Commit this package slice as a standalone cleanup.
- Move to the next package that still fails validation.
- Stop only on the next real contradictory rule or false positive.
