# Goal
Test the full package-level path for a real `tree-sitter` need in `packages/rs/clippy/g3rs-clippy-config-checks`: add the dependency, add a package-local `deny.toml` wrapper exception for the transitive `regex`, and observe which layers signal.

# Approach
1. Add `tree-sitter` to `crates/runtime/Cargo.toml`.
2. Add a minimal `use tree_sitter as _;` in `crates/runtime/src/lib.rs` so the workspace lint `-D unused-crate-dependencies` does not block the experiment.
3. Add a local wrapper allowance for `regex` in this package's `deny.toml`.
4. Run `cargo deny check bans --config deny.toml` in the package.
5. Run `guardrail3-rs validate --path ... --family deny` and full package validation to see whether package-local allow wiring is accepted or whether a deny-family rule rejects it.

# Key Decisions
- Do not use a temp copy. The point is to see the real package-level signals in the live workspace.
- Keep the exception package-local. Do not touch the shared deny family baseline.
- Keep the experiment narrow: one dependency, one local wrapper allowance, one no-op import to satisfy the lint.

# Files To Modify
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/deny.toml`
