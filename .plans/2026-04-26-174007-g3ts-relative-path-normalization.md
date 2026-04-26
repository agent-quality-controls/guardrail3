# Goal

Make `g3ts validate --path apps/landing` behave the same as `g3ts validate --path /absolute/path/to/apps/landing`.

# Bug

Relative `--path` values are passed through to `g3_workspace_crawl`.
The crawl then stores `root_abs_path` and entry `abs_path` values as relative paths.
Astro ESLint ingestion passes that relative root into the ESLint parser, which breaks effective ESLint config probing.

# Fix

- Normalize the workspace root once in `guardrail3-ts-packages::PackageRuntime`.
- Use an absolute path for the crawl root.
- Preserve existing error handling for missing roots.
- Add a CLI regression test proving relative validate paths reach the real crawler successfully.
- Reinstall local `g3ts` after the fix.

# Files

- `apps/guardrail3-ts/crates/io/outbound/packages/src/runtime.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`

# Verification

- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- Run from `/Users/tartakovsky/Projects/websmasher/websmasher`:
  - `g3ts validate --path apps/landing --family astro --inventory`
  - `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- Both commands should report the same Astro result set.
