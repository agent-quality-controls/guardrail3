Summary
- Fixed release dry-run so nested package validation no longer fails with the fake error "manifest path ... does not exist" when the validate path is relative.
- Added a regression test that reproduces the bug through public release ingestion.

Decisions made
- Fixed the release helper locally instead of changing workspace-crawl path semantics across the repo.
- Made the dry-run command run from the manifest directory and pass `Cargo.toml`, so it no longer depends on whether the stored path is relative or absolute.
- Reproduced the bug through release ingestion with a relative nested crawl path, not by testing command strings in isolation.

Key files for context
- `.plans/2026-04-15-133417-release-dry-run-relative-manifest-fix.md`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/basic.rs`

Next steps
- Continue the `g3rs-clippy-filetree-checks` package cleanup. The fake release bug is gone; the remaining release findings there are now normal package work.
- If release dry-run messages stay noisy because cargo prints lock chatter first, tighten the stderr excerpt later so it shows the real failure cause.
