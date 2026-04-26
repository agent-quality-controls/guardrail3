# Summary

Added `TS-ASTRO-FILETREE-12` to enforce configured `[ts.astro].forbidden_state` patterns against discovered app files and directories. File-tree ingestion now derives forbidden-state matches from the parsed app-local Astro policy.

# Decisions

- Kept `TS-ASTRO-FILETREE-11` for the existing hard-coded legacy-state signal instead of removing it in this slice.
- Used the same `globset` matcher style as the config policy rules.
- Matched both included and ignored visible entries so ignored generated state still reports.

# Key Files

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `packages/ts/astro/g3ts-astro-types/src/types.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/run.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_12_configured_forbidden_state.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/run_tests/cases.rs`

# Verification

- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-types`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-ingestion`
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-file-tree-checks`
- `g3rs validate --path packages/ts/astro/g3ts-astro-types`
- `g3rs validate --path packages/ts/astro/g3ts-astro-ingestion`
- `g3rs validate --path packages/ts/astro/g3ts-astro-file-tree-checks`

# Next Steps

- Run the full Astro package test set once more before release.
- Update/release G3TS if the CLI package consumes these package changes.
