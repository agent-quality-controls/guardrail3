# Step 20: Migrate T23-T29 (eslint-disable + ts-ignore) to tree-sitter

## Checks
- T23-T26: eslint-disable patterns
- T27: @ts-ignore
- T28-T29: @ts-expect-error

## Task (1 agent)

1. Rewrite source scan for eslint-disable to use `ast_helpers::find_eslint_disables`
2. tree-sitter identifies Comment nodes — only search within actual comments, not strings
3. Rewrite @ts-ignore/@ts-expect-error to use `ast_helpers::find_ts_directives`
4. Fallback to grep if tree-sitter parse fails

## Key distinction
eslint-disable in a COMMENT → violation (or info if it has a reason)
eslint-disable in a STRING → not a violation (it's data, not a directive)

## Verification
```bash
cargo test
sh golden-tests/compare.sh
cargo test --test adversarial_grep_attacks -- typescript
```
