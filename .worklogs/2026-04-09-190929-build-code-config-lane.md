# Worklog - build code config lane

## Summary

Built the missing `code` config lane. Added the new `g3rs-code-config-checks`
package for `RS-CODE-07` and `RS-CODE-12`, and replaced
`g3rs-code-ingestion::ingest_for_config_checks` with a real crawl-to-checks
mapping path.

## Decisions made

- Added a dedicated `g3rs-code-config-checks` package instead of trying to
  stuff config rule execution into `g3rs-code-ingestion`.
  - Why: checks stay lane-specific, ingestion stays a mapper.

- Used one family-level config input object with two fact collections:
  - exception comments
  - workspace `unsafe_code` lint facts
  - Why: the runtime can fan that into tiny rule calls while keeping the public
    lane boundary simple.

- Used `cargo-toml-parser` for `RS-CODE-12`.
  - Why: this is config semantics and should stay on a structured parser.

- Kept raw line scanning for `RS-CODE-07`.
  - Why: the rule is explicitly about comment inventory across config files, so
    structured TOML alone is not enough.

- Made unreadable or malformed owned config files fail ingestion.
  - Why: the new lane should not silently drop owned config inputs.

## Key files for context

- `.plans/2026-04-09-190211-build-code-config-lane.md`
- `packages/rs/code/g3rs-code-config-checks/crates/types/src/lib.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_comments.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/unsafe_code_lints.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Verification

- `cargo test --workspace -q` in `packages/rs/code/g3rs-code-config-checks`
- `cargo test --workspace -q` in `packages/rs/code/g3rs-code-ingestion`

## Next steps

1. Build the `code` file-tree lane for `RS-CODE-35`.
2. Decide later whether `code` needs a real `g3rs-code-types` family crate so
   the lane input types stop living behind checks-types re-exports.
3. Expand config-lane parity coverage if more `code` config rules are moved.
