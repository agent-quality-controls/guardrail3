Summary
- Fixed shared workspace crawling so unignored `dist/` directories are excluded before G3TS or G3RS family ingestion sees them.
- Added a regression test proving `dist/**` is absent even when the app forgot to gitignore it.

Decisions made
- Fixed this in `g3-workspace-crawl` because `dist/` is generated build output, not source owned by a specific TS family or user app.
- Kept `.next`, `.velite`, and `.contentlayer` recovery unchanged because those are explicit ignored sentinels used by state/content policy checks.
- Did not add app-level `.gitignore` requirements because that would leave every consuming family vulnerable to generated-output noise.

Key files for context
- `packages/shared/g3-workspace-crawl/crates/runtime/src/recovery.rs`
- `packages/shared/g3-workspace-crawl/crates/runtime/src/run_tests/ignore_state.rs`

Verification
- `cargo test --manifest-path packages/shared/g3-workspace-crawl/Cargo.toml -p g3-workspace-crawl-runtime dist_is_excluded_from_phase1_without_gitignore`
- `cargo test --manifest-path packages/shared/g3-workspace-crawl/Cargo.toml --workspace`
- `g3rs validate --path packages/shared/g3-workspace-crawl`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --force`

Next steps
- None.
