# Summary

Normalized the active G3TS hooks packages so they pass G3RS package-shape checks instead of relying on legacy single-crate layouts. Fixed the TS CLI crawl boundary so G3TS can validate TypeScript app roots without requiring a Rust `Cargo.toml`, while keeping the Rust workspace crawler strict by default.

# Decisions Made

- Split hook ingestion and hook source checks into facade/runtime/assertions package shapes because G3RS requires shared assertions for sidecar tests.
- Kept `g3rs_workspace_crawl::crawl()` strict for Rust workspace/package roots and added `crawl_any_root()` for TypeScript/non-Rust project roots.
- Canonicalized the crawled root inside the crawler to avoid symlink/non-canonical app-root mismatch when hook ingestion computes app package roots relative to Git root.
- Encapsulated G3TS hook DTO fields behind constructors/getters so runner code no longer mutates public fields directly.
- Kept landing app hook behavior unchanged; the real landing run still reports the app-side missing style trigger routing.

# Key Files

- `.plans/2026-04-30-185850-g3ts-hooks-package-shape.md`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/crawl.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/io/outbound/packages/src/runtime.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/Cargo.toml`
- `packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/Cargo.toml`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-types/src/types.rs`
- `packages/ts/hooks/g3ts-hooks-contract-types/src/contract.rs`

# Verification

- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-ingestion/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/ts/hooks/g3ts-hooks-source-checks/Cargo.toml --workspace --offline`
- `cargo test --manifest-path packages/rs/g3rs-workspace-crawl/Cargo.toml --workspace --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `g3rs validate --path packages/rs/g3rs-workspace-crawl`
- `g3rs validate --path apps/guardrail3-ts`
- `g3rs validate --path` for every `packages/ts/hooks/g3ts-hooks-*` package
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force --offline`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family hooks --inventory`

# Adversarial Review

- First review found canonical path fragility for TS app roots reached through non-canonical paths.
- Added `crawl_any_root()` canonicalization coverage and a hook ingestion regression for symlinked TS app roots without `Cargo.toml`.
- Second review reported no remaining MUST FIX gaps.

# Next Steps

- Fix the landing app hook script so staged style files route through the required validation path.
