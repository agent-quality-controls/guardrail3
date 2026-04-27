## Summary

Corrected two adversarially found audit flaws before regenerating the authoritative zero-error report. `g3rs-topology-ingestion` now keeps shared topology contracts out of runtime and out of the local ingestion types crate, and the five parser packages again surface their centralized `#[allow(clippy::disallowed_methods)]` escape hatches as visible warnings instead of hiding them.

## Decisions made

- Kept `g3rs-topology-ingestion-types` narrow.
  - It now owns only `G3RsTopologyIngestionError`.
  - Rejected keeping topology data contract re-exports there because assertions can depend directly on `g3rs-topology-types` and do not need a package-local proxy.
- Kept parser `g3rs-code/ast-04-item-level-allow-with-reason` findings visible.
  - Rejected the attribute-form `reason = "..."` workaround because the current rule only inventories comment-style reasons and the workaround turned real warnings invisible.
- Regenerated the full audit with the correct root set.
  - Included `packages/rs/*` roots such as `packages/rs/g3rs-workspace-crawl`, not just `packages/rs/*/*`.

## Key files for context

- `.plans/2026-04-18-060133-zero-error-audit-corrections.md`
- `.worklogs/2026-04-18-060133-zero-error-audit-report.txt`
- `packages/rs/topology/g3rs-topology-ingestion/crates/runtime/src/lib.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/assertions/src/run.rs`
- `packages/rs/topology/g3rs-topology-ingestion/crates/types/src/lib.rs`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rustfmt-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/rustfmt-toml-parser/crates/runtime/src/parser.rs`

## Next steps

- Keep the five parser warning-only packages warning-only until a rule contradiction appears.
- If parser escape hatches ever need to be expressed in attribute-form, extend the `g3rs-code/ast-03-item-level-allow-without-reason` / `04` parser explicitly instead of relying on syntax the rule does not inventory today.
- Keep using the full root set from `.worklogs/2026-04-18-060133-zero-error-audit-report.txt` for future zero-error audits.
