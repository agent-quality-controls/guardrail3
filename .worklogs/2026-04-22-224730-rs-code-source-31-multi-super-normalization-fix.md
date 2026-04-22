## Summary

Fixed `RS-CODE-SOURCE-31` so multi-`super` inherent impl paths normalize correctly when checking shared public structs with named public fields. Added a red regression for `super::super::Input` and kept the broader glob-reexport alias regression as coverage on the same rule surface.

## Decisions made

- Fixed the normalization in the rule-owned impl-path binding layer.
  - Why: the live bug was in inherent-impl path matching, not in public-struct discovery.
- Kept the glob-reexport alias regression in the same sidecar test file.
  - Why: it exercises the same `RS-CODE-SOURCE-31` name-resolution surface and stayed green under the real fix.
- Removed stale batch plan files instead of carrying them forward.
  - Why: the repo should stay clean and only retain the plan/worklog artifacts that match landed work.

## Key files for context

- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`
- `.plans/2026-04-22-224228-rs-code-source-31-multi-super-normalization-fix.md`

## Next steps

- Run another adversarial pass on `rs/code` and the updated `hook_shared_13` rule to surface the next real production-path miss.
