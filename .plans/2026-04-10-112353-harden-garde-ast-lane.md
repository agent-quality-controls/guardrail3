## Goal

Fix the real garde AST lane bugs found by the attack pass and close the missing end-to-end coverage gaps.

## Approach

1. Prove the remaining bugs with tests first.
   - Add pipeline tests for:
     - clean root
     - no-garde-dependency roots that still show real garde adoption markers
     - unreadable Rust source
     - unreadable `guardrail3.toml`
     - excluded test/fixture paths
     - the remaining excluded path shapes: `__tests__`, `*_test.rs`, `*_tests.rs`
     - mixed valid + broken files
   - Add rule-local tests for:
     - `RS-GARDE-AST-01` primitive-only false positive prevention
     - alias / alternate derive coverage for `Deserialize`, `Parser`, `Args`, and `FromRow`
     - `RS-GARDE-AST-08` aliased `toml::from_str`, `try_into::<GuardrailConfig>()`,
       rebinding before `.validate()`, and `#[cfg(test)]` skip

2. Fix the AST gating contract.
   - Extend the AST input with garde dependency state from `Cargo.toml`.
   - Let the AST runtime compute adoption markers from parsed files.
   - If a root has neither garde dependency nor source adoption markers, emit only `RS-GARDE-10` input failures and skip source rules.

3. Fix the policy-loss false positive.
   - When `guardrail3.toml` is unreadable or malformed, keep `RS-GARDE-10` but do not emit `RS-GARDE-AST-04` findings that depend on missing escape-hatch data.

4. Tighten selection coverage.
   - Pin the current excluded-path matrix in tests.
   - Keep the extraction root-scoped, but prove what enters and what stays out.
   - Split the current monolithic AST ingestion test file into focused sidecars.

5. Clean the small pattern drift that is cheap to remove.
   - Use plain `mod tests;` in `RS-GARDE-10`.

## Key decisions

- Keep cargo parsing in ingestion for AST applicability.
  - Why: dependency presence is root metadata, not AST logic.

- Keep adoption-marker detection in the AST runtime.
  - Why: it depends on parsed source semantics, and parsing belongs there.

- Extend the narrow `RS-GARDE-AST-08` parser only where tests prove real code shapes.
  - Why: aliased `toml::from_str`, rebinding through a local `let`, and simple control-flow
    wrappers are still in-scope for the current same-function contract.

- Treat unreadable or malformed `guardrail3.toml` as `RS-GARDE-10` only for escape-hatch-dependent logic.
  - Why: running `RS-GARDE-AST-04` without policy data creates false positives.

## Files to modify

- `packages/rs/garde/g3rs-garde-source-checks/crates/types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/types/Cargo.toml`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/test_support.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_10_input_failures/mod.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_01_struct_derive_validate/rule_tests/*`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/rs_garde_ast_08_guardrail_config_validate_call/rule_tests/*`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/select.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/ast.rs`
