## Summary

Fixed the remaining `RS-CODE-SOURCE-31` misses where local alias chains and same-file glob imports could hide inherent impls on shared-crate structs. Added red tests for alias chaining and for both `use super::*` and `use crate::api::*` forms, then extended the rule to build per-module same-file struct bindings and reuse them for local `use` resolution.

## Decisions made

- Kept the fix inside the rule matcher.
  - Why: this is still one-file source name resolution owned by `RS-CODE-SOURCE-31`.
- Built same-file struct bindings per module scope.
  - Why: glob imports need a concrete source of bindable names; string heuristics were not enough.
- Resolved later aliases through already-collected local bindings.
  - Why: alias chains are just repeated local rebinding, not a new semantic category.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.plans/2026-04-22-223045-rs-code-public-struct-alias-chain-glob-fix.md`

## Next steps

- Continue the fresh attack pass on `rs/code` after these alias/glob cases land.
