# Step 21: Migrate T30-T35 (process.env + any + file length) to tree-sitter

## Checks
- T30: process.env direct access
- T31: `: any` type usage
- T32-T33: file length > 300 — KEEP GREP (line counting)
- T34-T35: IDE/coverage suppressions — keep grep (comment patterns)

## Task (1 agent)

1. Rewrite process.env check to use `ast_helpers::find_process_env`
2. tree-sitter finds MemberExpression nodes where object=`process`, property=`env`
3. Rewrite `: any` check to use `ast_helpers::find_any_types`
4. tree-sitter finds TypeAnnotation nodes with `any` keyword
5. T32-T33 and T34-T35 stay grep-based (appropriate for line counting and comment scanning)

## Verification
```bash
cargo test
sh golden-tests/compare.sh
cargo test --test adversarial_grep_attacks -- typescript
```
