Summary:
Fixed RS-CODE-SOURCE-31 so re-export aliases imported with `use super::Alias` normalize to the concrete public struct target, and verified the same boundary also handles the reversed alias-import order regression.

Decisions made:
- Kept the fix in `rule.rs` at the binding/normalization boundary instead of special-casing the final qualified-name comparison.
- Seeded local binding collection from the module-visible alias map and resolved binding targets through module-qualified bindings so `super::Alias` collapses to the real struct.
- Added two focused regressions in the shared test slice to cover the direct re-export alias case and the reversed-order alias import case.

Key files for context:
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.plans/2026-04-22-224856-rs-code-source-31-reexport-alias-normalization-fix.md`

Next steps:
- Stage only the rule, test, plan, and worklog files for this fix.
- Commit as a standalone RS-CODE-SOURCE-31 bug fix.
