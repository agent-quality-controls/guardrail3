## Summary

Hardened the extracted `g3rs-garde` AST lane after the attack pass. The lane now proves more real `crawl -> ingest_for_ast_checks -> check` paths, handles aliased and explicit generic `GuardrailConfig` parse sites, and pins the remaining selection and applicability branches with focused tests.

## Decisions made

- Kept garde applicability in the AST runtime, but proved more activation shapes through ingestion.
  - Why: the runtime already owns parsed-source adoption markers, so the correct fix was broader pipeline coverage, not moving that logic back into ingestion.

- Extended `RS-GARDE-AST-08` only for narrow same-function shapes.
  - Added aliased `toml::from_str`, explicit generic `toml::from_str::<GuardrailConfig>(...)`, explicit `try_into::<GuardrailConfig>()`, local rebinding before `.validate()`, and control-flow traversal for closures and loops.
  - Rejected broader dataflow. The rule still stays a same-function, parse-before-validate guardrail.

- Split the AST ingestion tests into focused sidecars.
  - Why: the old `ingest_tests/ast.rs` had become one mixed file for selection, gating, failures, and coexistence. The new `ast/` directory makes attack gaps and root-cause ownership clearer.

- Moved the `RS-GARDE-10` sidecar hook onto the rule file.
  - Why: that package was still the odd one out against the repo's per-rule sidecar test pattern.

## Key files for context

- `.plans/2026-04-10-112353-harden-garde-ast-lane.md`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/parse/guardrail_config.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/support.rs`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/rs_garde_ast_01_struct_derive_validate/rule_tests/`
- `packages/rs/garde/g3rs-garde-ast-checks/crates/runtime/src/rs_garde_ast_08_guardrail_config_validate_call/rule_tests/`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/select.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/ast/`

## Verification

- `cargo test --workspace -q` in `packages/rs/garde/g3rs-garde-ast-checks`
- `cargo test --workspace -q` in `packages/rs/garde/g3rs-garde-ingestion`
- `git diff --check`
- final 4-angle attack pass found only one last derived-`Validate` adoption proof gap, then no new concrete rule or lane blocker after that fix

## Next steps

1. Build the garde file-tree lane once the AST lane stops moving.
2. Decide whether the ingestion-package test pattern should also be normalized away from grouped `basic.rs` / `pipeline.rs` files.
3. Reuse the same AST-08 alias/rebinding test shapes when the next source-level Rust family adds parse-before-validate rules.
