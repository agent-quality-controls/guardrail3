Summary
- Removed the stale tracked `parse/fs_visitors.rs` file left behind after the `fs_visitors/` directory split.
- The verified working tree already depended on the directory form; this commit makes `HEAD` match that validated state.

Decisions made
- Fixed the mistake with a follow-up deletion instead of amending the previous commit.
- Kept the change isolated to the missed tracked file so the repository returns to a clean, validated state.

Key files for context
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors/mod.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs
- .worklogs/2026-04-22-231722-rs-code-source-15-21-lexical-scope-fix.md

Next steps
- Land the `RS-TEST-SOURCE-17` owned-assertions alias-chain fix from the active agent.
- Land the two active parser/hooks bug fixes: function-tail brace parsing and escaped-hash comment parsing.
