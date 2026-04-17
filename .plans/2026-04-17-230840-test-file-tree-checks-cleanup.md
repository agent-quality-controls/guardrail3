Goal
- Make `packages/rs/test/g3rs-test-file-tree-checks` validate clean under all current families without changing rules.

Approach
- Normalize the package shell: root `publish = false`, split `types` and `runtime` features, add root policy files, and add `guardrail3-rs.toml`.
- Normalize member crate metadata: explicit `publish = false`, `include`, docs.rs metadata, and feature-gated facades.
- Remove the runtime dependency on the local `types` member where the root facade or shared family types should be used instead.
- Split `crates/runtime/src/parse/mod.rs` and `crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs` into facade-only directories with leaf files below the size cap.
- Convert every `tests/mod.rs` tree to the owned `*_tests/` leaf-file shape and move semantic result checks into the assertions crate.
- Replace `crates/runtime/src/test_helpers.rs` with either owned per-rule support files or shared assertions helpers so `lib.rs` stops owning ad hoc test modules.

Key decisions
- Keep the package-local `crates/types` only if it still carries real file-tree-check-specific boundary types after cleanup. If it is just a re-export shell, remove or minimize it instead of preserving the extra crate out of habit.
- Follow the same leaf-owner pattern used in the cleaned ingestion package: facade `mod.rs` files only, leaf production files own `*_tests`.
- Prefer moving test proof logic to `crates/assertions` over adding more runtime-local helper modules.

Files to modify
- `packages/rs/test/g3rs-test-file-tree-checks/Cargo.toml`
- `packages/rs/test/g3rs-test-file-tree-checks/src/lib.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/guardrail3-rs.toml`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/Cargo.toml`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/lib.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/parse/**`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/**`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/**`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_10_input_failures/**`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/**`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/assertions/**`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/types/**`
