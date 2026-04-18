Goal
- Make `packages/rs/release/g3rs-release-ingestion` validate cleanly under current rules without changing the rules.

Approach
- Finish package shell normalization: root policy files, publish metadata, root config, and shared boundaries.
- Split `crates/runtime/src/ingest.rs` into smaller sibling modules by responsibility while keeping `ingest.rs` as a facade/entry file.
- Convert `crates/runtime/src/ingest_tests` to facade-only `mod.rs` plus helper files and move semantic result assertions into the shared assertions crate.
- Re-run package tests and `guardrail3-rs validate --path ...` until the package is clean.

Key decisions
- Keep fixes package-local. If a rule contradicts the cleaned shape, stop and handle that as a separate bug.
- Use one responsibility per helper module when splitting `ingest.rs`: root parsing/member collection, crate normalization, dependency edges, path/file utilities, tooling/dry-run.

Files to modify
- `packages/rs/release/g3rs-release-ingestion/Cargo.toml`
- `packages/rs/release/g3rs-release-ingestion/guardrail3-rs.toml`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/*`
- `packages/rs/release/g3rs-release-ingestion/crates/assertions/src/*`
- package README/policy files as needed
