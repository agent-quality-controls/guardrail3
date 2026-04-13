## Summary

Completed the cargo family migration into package-owned config and filetree lanes. Fixed the hybrid-root classification bug, migrated the remaining old app cargo rules, hardened member discovery and guardrail parsing boundaries, and drove the package through repeated adversarial review until both implementation and test-attack agents came back clean.

## Decisions made

- Kept cargo package lanes split as:
  - config for Cargo.toml contents and cargo guardrail policy state
  - filetree for missing member manifests and cargo-family input failures
  - no source lane
- Fixed hybrid-root handling at shared cargo support/ingestion boundaries instead of patching individual rules.
- Moved malformed member `[lints].workspace` out of whole-lane ingestion failure and into member config facts.
  - This lets healthy members still reach config checks while the malformed member is still reported.
- Kept malformed root `workspace.package.edition` / `workspace.package.rust-version` on the root parse-fail path when the typed Cargo parser rejects them.
  - Added regressions to prove there is no fallback to local `[package]` values.
- Tightened `guardrail3.toml` handling so invalid `profile.name` or malformed `escape_hatches` fail closed into `guardrail_parse_error` instead of being silently ignored.

## Key files for context

- `.plans/2026-04-13-093901-cargo-family-migration.md`
- `packages/rs/cargo/g3rs-cargo-types/src/lib.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/support.rs`
- `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/select.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Next steps

- Audit the next remaining partial Rust family against the old app inventory using the same flow:
  - read every live app rule body
  - classify lanes from code, not docs
  - add failing tests first
  - rerun adversarial agents until clean
