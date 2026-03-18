# Step 14: Migrate R58 (Direct std::fs Usage) to syn

## Check
R58: detect `use std::fs` imports that bypass the centralized fs module.

## Task (1 agent)

1. Rewrite `check_direct_fs_usage` to use `ast_helpers::find_std_fs_imports`
2. syn walks ItemUse nodes, checks if the path starts with `std::fs`
3. Must handle: `use std::fs;`, `use std::fs::read_to_string;`, `use std::fs::{read, write};`
4. Must NOT flag: `use crate::fs;`, `// use std::fs`, `"use std::fs"` in strings

## Verification
```bash
cargo test
sh golden-tests/compare.sh
cargo test --test adversarial_grep_attacks
```
