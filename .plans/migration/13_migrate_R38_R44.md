# Step 13: Migrate R38-R44 (Structure + Code Quality) to syn

## Checks
- R38-R39: file length > 500 effective lines — KEEP GREP (line counting doesn't benefit from AST)
- R40-R41: use count > 20 — migrate to syn (count ItemUse nodes, not grep)
- R42: unsafe — migrate to syn (find ExprUnsafe, not grep for "unsafe" text)
- R43: todo!/unimplemented! — migrate to syn (find ExprMacro, not grep)
- R44: .unwrap()/.expect() — migrate to syn (find ExprMethodCall, not grep)

## Task (1 agent, max 10 changes)

Split into sub-tasks if needed:

### Sub-task A: R40-R41 (use count)
1. Rewrite `check_use_count` to use `ast_helpers::count_use_statements`
2. Fallback to grep if parse fails

### Sub-task B: R42 (unsafe)
1. Rewrite `check_unsafe` to use `ast_helpers::find_unsafe_usage`
2. Must distinguish: `unsafe { }` block vs `let s = "unsafe"` string

### Sub-task C: R43 (todo/unimplemented)
1. Rewrite `check_todo_macros` to use `ast_helpers::find_forbidden_macros`
2. Must distinguish: `todo!()` macro vs `// TODO:` comment vs `let todo = 1`

### Sub-task D: R44 (unwrap/expect)
1. Rewrite `check_unwrap_expect` to use `ast_helpers::find_unwrap_expect`
2. Must distinguish: `.unwrap()` method call vs `"unwrap"` string vs `unwrap_result` field name

## Verification
```bash
cargo test
sh golden-tests/compare.sh
cargo test --test adversarial_grep_attacks
```

## On Failure
These checks are the most likely to have golden diff — real projects may have comments containing "unsafe" or "todo" that grep currently flags. Each diff should be reviewed: false positive removal = improvement.
