# Step 12: Migrate R34-R37 (Garde Skip + cfg_attr Allow) to syn

## Checks
- R34-R35: `#[garde(skip)]` without/with reason
- R36: EXCEPTION comment detection (this one might stay grep-based — it's about comments, not attributes)
- R37: `#[cfg_attr(test, allow(...))]` detection

## Task (1 agent, max 10 changes)

1. Rewrite `check_garde_skip` to use `ast_helpers::find_garde_skips`
2. Rewrite `check_cfg_attr_allow` to use `ast_helpers::find_cfg_attr_allows`
3. R36 (EXCEPTION comments) — keep as grep-based, it's scanning for comment patterns which is appropriate
4. Update tests

## Verification
```bash
cargo test
sh golden-tests/compare.sh
cargo test --test adversarial_grep_attacks
```
