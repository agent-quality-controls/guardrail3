# Rust Tool Migration Audit

## Finding 1 - Delete `RS-CODE-CONFIG-12`

- Rule: `RS-CODE-CONFIG-12`
- Current file:
  - `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- Current behavior:
  - reads workspace Cargo lint policy
  - emits inventory when `unsafe_code = "forbid"`
  - emits error when `unsafe_code = "deny"`
  - emits nothing when `unsafe_code` is missing
- Existing owners already covering the same contract:
  - `RS-CARGO-CONFIG-01`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule.rs`
    - requires the `unsafe_code` entry to exist
  - `RS-CARGO-CONFIG-02`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs`
    - requires `unsafe_code = "forbid"`
  - `RS-CARGO-CONFIG-09`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_09_no_weakened_overrides.rs`
    - prevents member crates from weakening inherited workspace lint policy
- Exact edit:
  1. Delete `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/`.
  2. Remove `mod rs_code_config_12_unsafe_code_lint;` from `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/lib.rs`.
  3. Remove `crate::rs_code_config_12_unsafe_code_lint::check(file, &mut results);` from `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/run.rs`.
  4. Delete `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/rs_code_config_12_unsafe_code_lint/rule.rs`.
  5. Remove `pub mod rs_code_config_12_unsafe_code_lint;` from `packages/rs/code/g3rs-code-config-checks/crates/assertions/src/lib.rs`.
  6. Remove `RS-CODE-CONFIG-12` references from:
     - `packages/rs/code/g3rs-code-config-checks/README.md`
     - `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- Why this does not weaken enforcement:
  - missing `unsafe_code` entry is already caught by `RS-CARGO-CONFIG-01`
  - weak root level is already caught by `RS-CARGO-CONFIG-02`
  - weak member override is already caught by `RS-CARGO-CONFIG-09`

## Finding 2 - `RS-CODE-SOURCE-13` overlaps existing `todo!` and `unimplemented!` ownership, but is not a safe delete under the current hook contract

- Rule: `RS-CODE-SOURCE-13`
- Current file:
  - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule.rs`
- Current behavior:
  - AST-scans each Rust source file
  - warns on `todo!`
  - warns on `unimplemented!`
  - warns on `unreachable!` outside test context
- Existing owners for `todo!` and `unimplemented!`:
  - `RS-CARGO-CONFIG-01`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule.rs`
    - requires `todo` and `unimplemented` in the Clippy lint policy
  - `RS-CARGO-CONFIG-02`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs`
    - requires both lints to be set to `deny`
  - `RS-HOOKS-SOURCE-04`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_02_clippy_step_present/rule.rs`
    - requires a real `cargo clippy` command in the hook
  - `RS-HOOKS-SOURCE-10`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
    - requires one `cargo clippy` command to deny warnings
- Exact edit if you want single ownership for `todo!` and `unimplemented!`:
  1. Edit `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule.rs`.
  2. Delete only the `"todo" | "unimplemented" => ...` match arm.
  3. Keep the `unreachable` branch.
  4. Update tests in:
     - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule_tests/direct.rs`
     - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_13_todo_macros/rule_tests/false_positives.rs`
  5. Update `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs` so it no longer expects `RS-CODE-SOURCE-13` from `todo!` coverage cases.
- Why this may weaken enforcement today:
  - `RS-HOOKS-SOURCE-04` only proves that some `cargo clippy` command exists
  - `RS-HOOKS-SOURCE-10` only proves that one `cargo clippy` command denies warnings
  - neither hook rule proves coverage of every target and every feature combination
  - `RS-CODE-SOURCE-13` sees source files directly, so it still catches `todo!` and `unimplemented!` in code that current hook execution might not cover
- Decision:
  - if current hook coverage is accepted as sufficient, narrow `RS-CODE-SOURCE-13` to `unreachable!` only
  - if current hook coverage is not accepted as sufficient, keep `RS-CODE-SOURCE-13` unchanged

## Finding 3 - `RS-CODE-SOURCE-16` overlaps existing `panic!` ownership, but is not a safe delete under the current hook contract

- Rule: `RS-CODE-SOURCE-16`
- Current file:
  - `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_16_panic_macro/rule.rs`
- Current behavior:
  - AST-scans each non-test Rust source file
  - warns on `panic!`
- Existing owners for `panic!`:
  - `RS-CARGO-CONFIG-01`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule.rs`
    - requires `panic` in the Clippy lint policy
  - `RS-CARGO-CONFIG-02`
    - file: `packages/rs/cargo/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs`
    - requires `panic = "deny"`
  - `RS-HOOKS-SOURCE-04`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_02_clippy_step_present/rule.rs`
    - requires a real `cargo clippy` command in the hook
  - `RS-HOOKS-SOURCE-10`
    - file: `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
    - requires one `cargo clippy` command to deny warnings
- Exact edit if you want single ownership for `panic!`:
  1. Delete `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_16_panic_macro/`.
  2. Remove `mod rs_code_ast_16_panic_macro;` from `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/lib.rs`.
  3. Remove `crate::rs_code_ast_16_panic_macro::check(&rule_input, &mut results);` from `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/run.rs`.
  4. Delete `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/rs_code_ast_16_panic_macro/rule.rs`.
  5. Remove `pub mod rs_code_ast_16_panic_macro;` from `packages/rs/code/g3rs-code-source-checks/crates/assertions/src/lib.rs`.
  6. Remove `RS-CODE-SOURCE-16` references from:
     - `packages/rs/code/g3rs-code-source-checks/README.md`
     - `packages/rs/code/g3rs-code-ingestion/crates/assertions/src/run.rs`
- Why this may weaken enforcement today:
  - the same hook-coverage gap from Finding 2 applies here
  - current hook rules do not prove that Clippy runs across every target and every feature combination
  - `RS-CODE-SOURCE-16` still catches `panic!` in source files that current hook execution might not cover
- Decision:
  - if current hook coverage is accepted as sufficient, delete `RS-CODE-SOURCE-16`
  - if current hook coverage is not accepted as sufficient, keep `RS-CODE-SOURCE-16`

## Immediate Order

1. Delete `RS-CODE-CONFIG-12`.
2. Decide whether current hook-driven Clippy coverage is sufficient to replace source-lane detection.
3. If yes, narrow `RS-CODE-SOURCE-13` and delete `RS-CODE-SOURCE-16`.
4. If no, keep `RS-CODE-SOURCE-13` and `RS-CODE-SOURCE-16` unchanged.
