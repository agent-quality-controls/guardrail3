# Clean clippy rule facades — complete helpers extraction

**Date:** 2026-04-03 13:21

## Summary
Completed the helpers extraction for 24 clippy split rules: cleaned mod.rs
facades (removed #[cfg(test)] blocks), added `mod helpers;` to tests/mod.rs,
updated test imports from super::super:: to super::helpers::.

Previous commit created helpers.rs files but didn't stage the mod.rs cleanups
or test import updates. This commit completes the extraction.
