# Summary

Completed the `fmt` family package migration. `fmt` now has package-owned config and filetree lanes, with stateful ingestion that preserves rule-owned blocker behavior instead of failing early.

## Decisions made

- Moved `RS-FMT-07` into `g3rs-fmt-config-checks`.
  - Why: it checks parsed rustfmt `ignore` content plus typed escape hatch metadata, so it belongs in config.
  - Rejected: keeping it app-side.
- Added `g3rs-fmt-filetree-checks` for `RS-FMT-FILETREE-01`, `RS-FMT-FILETREE-05`, and `RS-FMT-FILETREE-08`.
  - Why: these rules inspect root presence and nested file placement, not config contents.
- Changed `g3rs-fmt-ingestion` to preserve blocker states.
  - Why: `RS-FMT-CONFIG-01`, `03`, and `04` need malformed or missing root inputs to stay visible as rule results.
  - Rejected: returning ingestion errors for malformed root rustfmt, missing Cargo, and missing toolchain.
- Accepted root `.rustfmt.toml` when `rustfmt.toml` is absent.
  - Why: that matches the live app family behavior.

## Key files for context

- `.plans/2026-04-12-144736-fmt-family-migration.md`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_07_ignore_escape_hatch/rule.rs`
- `packages/rs/fmt/g3rs-fmt-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/select.rs`

## Next steps

- Re-audit the next partial Rust family from live app rule bodies, not stale plan docs.
- If the user asks for adversarial review, run `test-attack` agents against `fmt` migration scope:
  - app rule inventory vs package rule coverage
  - config/filetree lane distribution
  - filetree false positives for nested config discovery
