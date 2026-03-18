# Step 10: Migrate R30-R31 (Crate-Level Allow) to syn

## Goal
Replace grep-based `check_crate_level_allow` with syn-based AST inspection.

## Current Implementation
File: `src/rs/validate/allow_checks.rs`, function `check_crate_level_allow`
- Scans each line for `#![allow(` pattern using string concatenation
- Can false-positive on string literals containing the pattern

## New Implementation

### Task (1 agent, max 10 changes)

1. Read `src/rs/validate/allow_checks.rs` — understand current check_crate_level_allow
2. Read `src/rs/validate/ast_helpers.rs` — understand the find_crate_level_allows helper
3. Rewrite `check_crate_level_allow` to:
   - Parse file with `ast_helpers::parse_file`
   - If parse fails → fall back to grep (don't break on non-Rust files)
   - If parse succeeds → use `ast_helpers::find_crate_level_allows`
   - For each found: check if there's a `// reason:` comment on the same line in the SOURCE TEXT (syn strips comments, so look at the source line for the span)
   - Produce same CheckResult structure (R30 for unjustified, R31 for justified)
4. Keep the old grep function as `check_crate_level_allow_grep` (renamed, not deleted — used as fallback)
5. Update any tests that test the old function name

## Verification

```bash
# All tests pass
cargo test

# Golden self-validation unchanged
sh golden-tests/compare.sh

# Adversarial fixture: string literal with #![allow(] NOT flagged
cargo test --test adversarial_grep_attacks -- string_literal
```

## On Failure
If golden tests show a diff, the migration changed behavior on real code. Compare the diff: if it's a FALSE POSITIVE removal (string literal was incorrectly flagged), that's an IMPROVEMENT — update the golden. If it's a MISSED DETECTION (real allow not caught), that's a BUG — fix the syn implementation.
