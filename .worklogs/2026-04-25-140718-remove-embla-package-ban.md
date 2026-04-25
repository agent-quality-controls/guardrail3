# Summary

Removed `embla-carousel*` from the active TypeScript package-family Syncpack forbidden dependency policy. Updated ingestion and config-check tests so expected messages and missing-ban fixtures only include the remaining forbidden dependency list.

# Decisions Made

- Removed only the active package-family policy and tests. Historical worklogs and legacy audit notes still describe previous state and were not rewritten.
- Kept the Syncpack delegated-policy shape unchanged. This change only removes an unused dependency pattern from the required ban list.

# Key Files For Context

- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run.rs`
- `packages/ts/package/g3ts-package-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/package/g3ts-package-config-checks/crates/runtime/src/run_tests/cases.rs`
- `.plans/2026-04-25-140535-remove-embla-package-ban.md`

# Verification

- `cargo test -q --manifest-path packages/ts/package/g3ts-package-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/package/g3ts-package-config-checks/Cargo.toml --workspace`
- `rg "embla-carousel" -n packages/ts/package/g3ts-package-ingestion packages/ts/package/g3ts-package-config-checks`
- `git diff --check`

# Next Steps

- None for this slice.
