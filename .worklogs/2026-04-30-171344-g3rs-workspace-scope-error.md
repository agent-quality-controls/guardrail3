## Summary

G3RS now rejects non-Rust roots before running family validation. A repository root without a root `Cargo.toml` returns one explicit workspace-scope error instead of producing noisy family findings from nested workspaces.

## Decisions Made

- Added the root `Cargo.toml` invariant to `g3rs-workspace-crawl`, because the crawler is the IO boundary that decides whether a requested path is a valid Rust validation root.
- Added `MissingWorkspaceManifest` as a typed crawl error and implemented `Display` so CLI output is actionable instead of a debug enum dump.
- Kept nested workspace discovery out of this path. G3RS remains scoped to the exact `--path` target.

## Key Files

- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/crawl.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/run.rs`
- `packages/rs/g3rs-workspace-crawl/crates/runtime/src/run_tests/crawl_mechanics.rs`
- `apps/guardrail3-rs/crates/io/outbound/packages/src/runtime.rs`

## Verification

- `cargo test --manifest-path packages/rs/g3rs-workspace-crawl/crates/runtime/Cargo.toml --offline`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --offline`
- `cargo run --manifest-path apps/guardrail3-rs/Cargo.toml -- validate --path . --inventory`
- `cargo run --manifest-path apps/guardrail3-rs/Cargo.toml -- validate --path apps/guardrail3-rs --inventory`
- `cargo run --manifest-path apps/guardrail3-rs/Cargo.toml -- validate --path packages/rs/g3rs-workspace-crawl --inventory`
- `cargo install --path apps/guardrail3-rs/crates/io/inbound/cli/crates/runtime --locked --offline --force`
- `g3rs validate --path . --inventory`

## Next Steps

- Continue full G3RS validation per actual Rust workspace root, not from the repository root.
