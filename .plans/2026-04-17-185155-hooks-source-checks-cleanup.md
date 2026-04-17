Goal
- Clean `packages/rs/hooks/g3rs-hooks-source-checks` to current package shape and reach `No findings.` without changing rules.

Approach
- Normalize root and member manifests: add missing root policy/config files, explicit publish/include/docs.rs/shared metadata, and local package policy in `guardrail3-rs.toml`.
- Remove old boundary leaks: stop runtime depending on local `crates/types`, stop local types from depending on `g3rs-hooks-types`, and feature-gate root facades.
- Normalize assertions crate shape: remove crate-wide allow attributes, make the exported assertion surface direct and feature-gated, and keep proof-bearing checks in exported functions.
- Convert runtime rule modules off facade-owned logic/tests: move production logic from `mod.rs` into sibling files, replace `src/**/tests` trees with owned `*_tests/` sidecars, and keep `mod.rs` facade-only.
- Re-run `cargo test` and `guardrail3-rs validate --path packages/rs/hooks/g3rs-hooks-source-checks`, then commit with a worklog.

Key decisions
- Follow the cleaned hooks/garde package pattern instead of inventing a hooks-specific exception.
- Keep package-local fixes only; if validation exposes an actual contradiction, stop and report it before changing any rule.

Files to modify
- `packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-source-checks/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-source-checks/{clippy.toml,deny.toml,rust-toolchain.toml,rustfmt.toml}`
- `packages/rs/hooks/g3rs-hooks-source-checks/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/{runtime,assertions,types}/Cargo.toml`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/assertions/src/**`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/**`
