## Summary

Fixed the `RS-CODE-SOURCE-31` follow-up false negative where nested public structs were still skipped if the inherent impl used `crate::...` or `self::...` qualification. The rule now normalizes impl self-type paths before comparing them to the qualified struct identity.

## Decisions made

- Kept the normalization inside the rule's inherent-impl matcher.
  - Why: this is still rule-local impl identity work, not a new parser lane.
- Added both `crate::...` and `self::...` proofs.
  - Why: the bug class was qualified self-type syntax, not one isolated spelling.

## Key files for context

- `.plans/2026-04-22-220333-rs-code-qualified-impl-normalization-fix.md`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_31_public_struct_named_fields/rule_tests/shared.rs`

## Next steps

- Fix the concrete `rs/release` workflow false positive where `--manifest-path .../Cargo.toml` is matched by filename only and can credit the wrong crate.
- Keep the hooks and code attack queue running in parallel for the next bug after that.
