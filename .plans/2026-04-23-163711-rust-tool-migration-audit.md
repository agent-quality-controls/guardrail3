# Rust Tool Migration Audit

## Finding 1 - Delete `g3rs-code/unsafe-code-lint`

- Rule: `g3rs-code/unsafe-code-lint`
- Current file:
  - `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- Current behavior:
  - reads workspace Cargo lint policy
  - emits inventory when `unsafe_code = "forbid"`
  - emits error when `unsafe_code = "deny"`
  - emits nothing when `unsafe_code` is missing
- Existing owners already covering the same contract:
  - `g3rs-cargo/workspace-lints`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule.rs`
    - requires the `unsafe_code` entry to exist
  - `g3rs-cargo/lint-levels`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs`
    - requires `unsafe_code = "forbid"`
  - `g3rs-cargo/no-weakened-overrides`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_09_no_weakened_overrides.rs`
    - prevents member crates from weakening inherited workspace lint policy
- Exact edit:
  1. Delete `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/`.
  2. Remove `mod rs_code_config_12_unsafe_code_lint;` from `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/lib.rs`.
  3. Remove `crate::rs_code_config_12_unsafe_code_lint::check(file, &mut results);` from `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/run.rs`.
  4. Delete `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_12_unsafe_code_lint/rule.rs`.
  5. Remove `pub mod rs_code_config_12_unsafe_code_lint;` from `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/lib.rs`.
  6. Remove `g3rs-code/unsafe-code-lint` references from:
     - `packages/rs/code/g3rs-code-config-checks/README.md`
     - `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- Why this does not weaken enforcement:
  - missing `unsafe_code` entry is already caught by `g3rs-cargo/workspace-lints`
  - weak root level is already caught by `g3rs-cargo/lint-levels`
  - weak member override is already caught by `g3rs-cargo/no-weakened-overrides`

## Finding 2 - `g3rs-code/ast-13-todo-macros` overlaps existing `todo!` and `unimplemented!` ownership, but is not a safe delete under the current hook contract

- Rule: `g3rs-code/ast-13-todo-macros`
- Current file:
  - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule.rs`
- Current behavior:
  - AST-scans each Rust source file
  - warns on `todo!`
  - warns on `unimplemented!`
  - warns on `unreachable!` outside test context
- Existing owners for `todo!` and `unimplemented!`:
  - `g3rs-cargo/workspace-lints`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule.rs`
    - requires `todo` and `unimplemented` in the Clippy lint policy
  - `g3rs-cargo/lint-levels`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs`
    - requires both lints to be set to `deny`
  - `g3rs-hooks/hook-rs-02-clippy-step-present`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_02_clippy_step_present/rule.rs`
    - requires a real `cargo clippy` command in the hook
  - `g3rs-hooks/hook-rs-09-clippy-denies-warnings`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
    - requires one `cargo clippy` command to deny warnings
- Exact edit if you want single ownership for `todo!` and `unimplemented!`:
  1. Edit `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule.rs`.
  2. Delete only the `"todo" | "unimplemented" => ...` match arm.
  3. Keep the `unreachable` branch.
  4. Update tests in:
     - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule_tests/direct.rs`
     - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule_tests/false_positives.rs`
  5. Update `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs` so it no longer expects `g3rs-code/ast-13-todo-macros` from `todo!` coverage cases.
- Why this may weaken enforcement today:
  - `g3rs-hooks/hook-rs-02-clippy-step-present` only proves that some `cargo clippy` command exists
  - `g3rs-hooks/hook-rs-09-clippy-denies-warnings` only proves that one `cargo clippy` command denies warnings
  - neither hook rule proves coverage of every target and every feature combination
  - `g3rs-code/ast-13-todo-macros` sees source files directly, so it still catches `todo!` and `unimplemented!` in code that current hook execution might not cover
- Decision:
  - if current hook coverage is accepted as sufficient, narrow `g3rs-code/ast-13-todo-macros` to `unreachable!` only
  - if current hook coverage is not accepted as sufficient, keep `g3rs-code/ast-13-todo-macros` unchanged

## Finding 3 - `g3rs-code/ast-16-panic-macro` overlaps existing `panic!` ownership, but is not a safe delete under the current hook contract

- Rule: `g3rs-code/ast-16-panic-macro`
- Current file:
  - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_16_panic_macro/rule.rs`
- Current behavior:
  - AST-scans each non-test Rust source file
  - warns on `panic!`
- Existing owners for `panic!`:
  - `g3rs-cargo/workspace-lints`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule.rs`
    - requires `panic` in the Clippy lint policy
  - `g3rs-cargo/lint-levels`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs`
    - requires `panic = "deny"`
  - `g3rs-hooks/hook-rs-02-clippy-step-present`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_02_clippy_step_present/rule.rs`
    - requires a real `cargo clippy` command in the hook
  - `g3rs-hooks/hook-rs-09-clippy-denies-warnings`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
    - requires one `cargo clippy` command to deny warnings
- Exact edit if you want single ownership for `panic!`:
  1. Delete `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_16_panic_macro/`.
  2. Remove `mod rs_code_ast_16_panic_macro;` from `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/lib.rs`.
  3. Remove `crate::rs_code_ast_16_panic_macro::check(&rule_input, &mut results);` from `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run.rs`.
  4. Delete `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/rs_code_ast_16_panic_macro/rule.rs`.
  5. Remove `pub mod rs_code_ast_16_panic_macro;` from `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/lib.rs`.
  6. Remove `g3rs-code/ast-16-panic-macro` references from:
     - `packages/rs/code/g3rs-code-source-checks/README.md`
     - `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- Why this may weaken enforcement today:
  - the same hook-coverage gap from Finding 2 applies here
  - current hook rules do not prove that Clippy runs across every target and every feature combination
  - `g3rs-code/ast-16-panic-macro` still catches `panic!` in source files that current hook execution might not cover
- Decision:
  - if current hook coverage is accepted as sufficient, delete `g3rs-code/ast-16-panic-macro`
  - if current hook coverage is not accepted as sufficient, keep `g3rs-code/ast-16-panic-macro`

## Immediate Order

1. Delete `g3rs-code/unsafe-code-lint`.
2. Decide whether current hook-driven Clippy coverage is sufficient to replace source-lane detection.
3. If yes, narrow `g3rs-code/ast-13-todo-macros` and delete `g3rs-code/ast-16-panic-macro`.
4. If no, keep `g3rs-code/ast-13-todo-macros` and `g3rs-code/ast-16-panic-macro` unchanged.
