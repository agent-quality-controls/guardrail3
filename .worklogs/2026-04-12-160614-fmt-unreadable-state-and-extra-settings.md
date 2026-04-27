# Summary

Fixed two fmt package boundary problems after proving them with tests. The package now models unreadable root inputs distinctly from parse errors, and `g3rs-fmt/extra-settings` inventories explicit non-baseline rustfmt keys based on the actual TOML keys present instead of typed re-serialization defaults.

## Decisions made

- Added `Unreadable` variants to all three fmt config input states.
  - Why: unreadable IO and parse failure are different facts and should not be collapsed into one blocker state.
- Kept stale-read and unreadable root files fail-closed.
  - Why: the package model has a path/readability crawl snapshot, not cached file contents.
- Moved explicit rustfmt key extraction into ingestion parsing.
  - Why: `g3rs-fmt/extra-settings` needs the keys the user actually wrote, not defaults introduced by serializing the typed parser struct.
- Removed the empty `skip_macro_invocations` suppression.
  - Why: the rule is about non-baseline config surface, so explicit presence of the key must be inventoried even when the list is empty.

## Key files for context

- `.plans/2026-04-12-155816-fmt-unreadable-state-and-extra-settings.md`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/inputs.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_01_settings/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_02_extra_settings/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_03_nightly_keys_on_stable/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_04_edition_mismatch/rule.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- updated fmt rule tests and `ingest_tests/basic.rs`

## Next steps

- The unreadable-state and extra-settings boundary drift is fixed.
- Remaining fmt hardening, if needed later, is mostly about converting more pipeline tests to exact result vectors.
