Goal:
Fix `RS-CODE-SOURCE-31` so repeated `super::` prefixes normalize to the correct enclosing module path when resolving inherent impl self types.

Approach:
- Add a red-first regression in the shared rule tests using a nested module repro with `impl super::super::Input`.
- Patch the public-surface normalization helper so repeated `super` segments pop multiple module levels instead of only one.
- Keep the fix in parsing/normalization; do not touch the rule matcher.
- Run the `g3rs-code-source-checks` package tests and validate the package path with `g3rs validate`.

Key decisions:
- The parser already carries enough module-path context, so the right fix is to improve normalization rather than add rule-side heuristics.
- The repro should remain valid Rust and nested enough to prove the exact multi-super failure.

Files to modify:
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/attrs/public_surface.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.worklogs/<dated>-rs-code-source-31-multi-super-normalization-fix.md`
