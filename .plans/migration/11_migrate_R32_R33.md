# Step 11: Migrate R32-R33 (Item-Level Allow) to syn

## Current Implementation
`check_item_level_allow` in `allow_checks.rs` — scans for `#[allow(` pattern, checks for `//` on same line.

## New Implementation

### Task (1 agent, max 10 changes)

1. Read current `check_item_level_allow`
2. Rewrite to use `ast_helpers::find_item_allows`
3. For each found attribute:
   - Get the line number from the syn span
   - Look at the source text for that line
   - Check if `// reason:` (or just `//`) exists on the same line
   - R32 (error) if no reason, R33 (info) if justified
4. Fallback to grep if syn parse fails
5. Ensure multi-line attributes are handled (syn sees them as one attribute regardless of line breaks)

## Key edge case
Multi-line `#[allow(\n    clippy::foo\n)]` — grep sees `#[allow(` on one line, the lint name on another. syn sees the whole thing as one Attribute. The line number should be the line where `#[allow` starts.

## Verification

```bash
cargo test
sh golden-tests/compare.sh
cargo test --test adversarial_grep_attacks
```

## On Failure
Same as step 10 — diff golden, determine if improvement or regression.
