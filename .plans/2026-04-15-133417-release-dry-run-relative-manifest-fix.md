Goal
- Fix g3rs-release/publish-dry-run so nested package validation does not fail with a fake "manifest path does not exist" error when the validate path is relative.

Approach
- Add a release ingestion regression test that crawls a nested package through a relative path and proves the dry-run failure is not allowed to be a fake missing-manifest error.
- Fix `run_publish_dry_run` so it runs from the manifest directory and passes `Cargo.toml`, instead of depending on whatever path form the crawler stored.
- Re-run release ingestion tests and validate the real package again.

Key decisions
- Fix the release helper locally instead of widening this into a global workspace-crawl path rewrite.
- Reproduce the bug through public release ingestion, not by testing cargo command strings in isolation.

Files to modify
- .plans/2026-04-15-133417-release-dry-run-relative-manifest-fix.md
- packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs
- packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/basic.rs
