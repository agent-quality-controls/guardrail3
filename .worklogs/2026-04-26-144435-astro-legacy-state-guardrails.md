# Summary

Implemented the first Astro strict-content hardening slice after the finalized Astro/content boundary plan. Astro content apps now reject legacy Next/Contentlayer state, Velite generated output is scoped to content apps, Contentlayer package bans are required through Syncpack for Astro content apps, and the workspace crawler recovers ignored generated-state directory sentinels so gitignored outputs are visible to guardrails.

# Decisions Made

- Kept Syncpack as the package-policy owner. G3TS still consumes parsed Syncpack facts and enforces canonical Astro-specific required bans.
- Scoped Contentlayer-specific bans and Velite/generated-state file-tree checks to Astro content modes until `[ts.astro].profile = "strict-local-content"` exists.
- Kept `.velite/**` under existing `TS-ASTRO-FILETREE-06` and added `TS-ASTRO-FILETREE-11` for `.next/**`, `.contentlayer/**`, and `contentlayer.config.*`.
- Extended workspace crawl recovery for `.next`, `.velite`, and `.contentlayer` directories because these outputs are usually gitignored but still guardrail-relevant.
- Rejected catch-all Syncpack ban groups by preserving the existing one-canonical-group-per-dependency contract.

# Key Files

- `.plans/2026-04-26-133953-content-astro-boundaries.md`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/crawl.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/recovery.rs`
- `packages/ts/astro/g3ts-astro-ingestion/crates/runtime/src/select.rs`
- `packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/ts_astro_config_10_syncpack_forbidden_deps.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_06_no_velite_output.rs`
- `packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/ts_astro_filetree_11_no_legacy_parallel_state.rs`

# Verification

- `cargo test -q --manifest-path packages/rs/g3rs-workspace-crawl/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-file-tree-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/astro/g3ts-astro-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo fmt --check --manifest-path packages/rs/g3rs-workspace-crawl/Cargo.toml`
- `cargo fmt --check --manifest-path packages/ts/astro/g3ts-astro-ingestion/Cargo.toml`
- `cargo fmt --check --manifest-path packages/ts/astro/g3ts-astro-file-tree-checks/Cargo.toml`
- `git diff --check`

# Adversarial Review

- First pass found rule-scope, rule-ID, wording, test precision, nested-root, ignored-state, and route false-positive gaps.
- Fixed all reported gaps and reran convergence reviewers.
- Final convergence reviewers reported no blocking gaps.

# Next Steps

- Implement `[ts.astro].profile` policy facts so strict content scope is explicit instead of inferred from build/live collection content mode.
- Continue with route class normalization and effective ESLint option checks from the Astro plan.
