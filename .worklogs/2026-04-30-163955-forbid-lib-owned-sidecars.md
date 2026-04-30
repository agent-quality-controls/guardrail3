# Forbid lib-owned sidecars

## Summary

Fixed a G3RS hole where crate facade `lib.rs` could own `lib_tests/` sidecars. G3RS now rejects `lib.rs -> lib_tests`, and the G3TS style config check tests now attach to `run.rs` through `run_tests/`.

## Decisions made

- Updated `g3rs-test/owned-sidecar-shape` so `lib.rs` has no owned sidecar contract.
- Updated `g3rs-arch/no-path-attr` so its test-sidecar exemption does not apply to `lib.rs`.
- Kept non-facade implementation sidecars valid, such as `run.rs -> run_tests`.
- Did not attempt to enforce one semantic rule per file in this change because that is a separate architectural rule and may need a more careful universal definition.

## Key files for context

- `.plans/2026-04-30-163521-forbid-lib-owned-sidecars.md`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/owned_sidecar_shape/rule.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/no_path_attr.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test --manifest-path packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/Cargo.toml --offline owned_sidecar_shape`
- `cargo test --manifest-path packages/rs/arch/g3rs-arch-source-checks/crates/runtime/Cargo.toml --offline no_path_attr`
- `cargo test --manifest-path packages/ts/style/g3ts-style-config-checks/crates/runtime/Cargo.toml --offline`
- `g3rs validate --path packages/rs/test/g3rs-test-file-tree-checks`
- `g3rs validate --path packages/rs/arch/g3rs-arch-source-checks`
- `g3rs validate --path packages/ts/style/g3ts-style-config-checks`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --offline`
- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace --offline`
- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --bin g3ts --force`

## Next steps

- Decide separately whether G3RS can safely enforce one semantic rule per file for check packages.
