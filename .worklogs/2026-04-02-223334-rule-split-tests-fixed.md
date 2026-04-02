# Fix test compilation after rule.rs split — all tests pass

**Date:** 2026-04-02 22:33

## Summary
Fixed all test compilation errors from the mod.rs → rule.rs split.
Added #[cfg(test)] imports to 117 mod.rs facades. Fixed pub(crate)
re-exports, dangling cfg attributes, test helper visibility.

All tests pass (excluding pre-existing CLI failure).
