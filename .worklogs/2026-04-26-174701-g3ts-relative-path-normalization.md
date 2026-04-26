# Summary

Fixed the G3TS relative `--path` normalization bug. `g3ts validate --path apps/landing` and the equivalent absolute path now produce the same Astro result set.

# Decisions

- Fixed path normalization at the `guardrail3-ts-packages::PackageRuntime` crawl boundary instead of inside Astro ingestion, because every family consumes the shared crawl.
- Used `current_dir().join(root)` for relative paths instead of `std::fs::canonicalize`, so nonexistent-path errors still come from the crawler and no direct filesystem dependency is added.
- Added a CLI regression test that exercises the default runtime crawler with a relative validate path.
- Left the app-level `g3rs validate --path apps/guardrail3-ts` dependency/apparch allowlist failures untouched because they predate this path bug and are not introduced by this patch.

# Key Files

- `apps/guardrail3-ts/crates/io/outbound/packages/src/runtime.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`
- `.plans/2026-04-26-174007-g3ts-relative-path-normalization.md`

# Verification

- `cargo test --workspace` in `apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path apps/landing --family astro --inventory` from the websmasher app repo
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- `g3rs validate --path apps/guardrail3-ts` still fails on existing dependency/app-architecture allowlist findings unrelated to this patch

# Next Steps

- Landing still needs to fix `TS-ASTRO-CONFIG-28` by narrowing `content_adapter` or moving schema-only modules outside the configured adapter root.
