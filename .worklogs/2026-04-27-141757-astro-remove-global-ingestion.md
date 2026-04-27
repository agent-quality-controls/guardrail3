Summary:
- Removed the remaining flat Astro aggregate packages: `g3ts-astro-config-checks`, `g3ts-astro-file-tree-checks`, and `g3ts-astro-ingestion`.
- Rewired setup, content, mdx, seo, and state ingestion packages to own their public ingestion entrypoints and use `g3ts-astro-check-support` only for shared parser/fact support.

Decisions made:
- Kept shared parser-normalizer code in `g3ts-astro-check-support` because it is support code, not a family package.
- Rejected keeping `g3ts-astro-ingestion` because it made the split fake: every per-area ingestion package still depended on one global ingestion family.
- Added temporary source-size waivers to support for the moved parser-normalizer module; the visible package boundary is now correct, and deeper internal support-module extraction can happen without recreating a global family.

Key files for context:
- `packages/ts/astro/g3ts-astro-check-support/src/ingestion.rs`
- `packages/ts/astro/g3ts-astro-check-support/src/ingestion_select.rs`
- `packages/ts/astro/setup/g3ts-astro-setup-ingestion/src/run.rs`
- `packages/ts/astro/content/g3ts-astro-content-ingestion/src/run.rs`
- `packages/ts/astro/mdx/g3ts-astro-mdx-ingestion/src/run.rs`
- `packages/ts/astro/seo/g3ts-astro-seo-ingestion/src/run.rs`
- `packages/ts/astro/state/g3ts-astro-state-ingestion/src/run.rs`

Verification:
- `cargo test --workspace` in `packages/ts/astro/g3ts-astro-check-support`
- `cargo test --workspace` in `apps/guardrail3-ts`
- `g3rs validate --path .../packages/ts/astro/g3ts-astro-check-support`
- `g3rs validate --path` for each Astro setup/content/mdx/seo/state ingestion package
- `g3rs validate --path .../apps/guardrail3-ts`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`
- `rg "g3ts-astro-(config-checks|file-tree-checks|ingestion)|g3ts_astro_(config_checks|file_tree_checks|ingestion)" apps packages/ts/astro -g 'Cargo.toml' -g '*.rs' -g 'guardrail3-rs.toml'`

Next steps:
- If we keep hardening internals, split `g3ts-astro-check-support/src/ingestion.rs` into smaller support modules without changing family package ownership.
