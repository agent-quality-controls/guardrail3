# Forbid Lib-Owned Sidecars

## Goal

Make G3RS reject crate facade `lib.rs` files that own `lib_tests/` sidecars, then fix the new G3TS style package to match that rule.

## Approach

- Add G3RS tests proving `src/lib.rs` with `#[path = "lib_tests/mod.rs"] mod lib_tests;` is invalid for owned sidecar shape.
- Add G3RS tests proving `g3rs-arch/no-path-attr` does not exempt `lib.rs -> lib_tests`.
- Update the owned-sidecar contract helper so `lib.rs` returns no owned sidecar contract.
- Update the no-path-attr owned-sidecar exemption so `lib.rs` is not exempt.
- Move `g3ts-style-config-checks` runtime tests from crate `lib.rs` to the implementation module `run.rs`.

## Key Decisions

- This does not try to enforce one semantic rule per production file. That may not be universally enforceable yet.
- This only fixes the universal rule: crate facade `lib.rs` must stay a facade and must not own semantic test sidecars.
- `#[path = "..._tests/mod.rs"]` remains allowed for non-facade implementation modules where G3RS already expects file-owned sidecars.

## Files To Modify

- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/owned_sidecar_shape/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/owned_sidecar_shape/rule_tests/cases.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/no_path_attr.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/no_path_attr_tests/cases.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/*`
