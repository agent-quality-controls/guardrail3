Summary
- Fixed the release family so it follows explicit publish intent instead of Cargo's default publish behavior.
- Unpublished crates and all-false workspaces now stand down cleanly, and missing `publish` now fails with one direct rule instead of causing fake release noise.

Decisions made
- Treated missing `publish` as "not release-eligible" in release ingestion. This matches the agreed policy better than Cargo's default and keeps the failure specific.
- Added a new explicit rule, `g3rs-release/publish-must-be-explicit`, instead of overloading the existing release rules. That keeps the message short and precise.
- Gated release dependency rules on the source crate being publishable, not just the target edge shape.
- Gated workspace-root release file and repo release setup checks on `publishable_count > 0`.
- Kept the publishable fixture shortcut only in direct rule test helpers. Real ingestion tests now write `publish = true` explicitly where they mean a published crate.

Key files for context
- `.plans/2026-04-15-124227-release-publish-gating.md`
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_00_publish_must_be_explicit.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_19_no_path_deps_to_unpublishable.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/rs_release_config_20_interdependent_version_consistency.rs`
- `packages/rs/release/g3rs-release-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest_tests/basic.rs`

Next steps
- Add the same explicit publish policy to the remaining release rules that still assume Cargo defaults, if any show up in the next package audits.
- Decide later whether the release family should also enforce explicit publish on the root workspace facade crates outside the package subcrates in a separate dedicated rule pass.
