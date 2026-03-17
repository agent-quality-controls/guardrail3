# Clean generate dry-run output + fix garde comment

**Date:** 2026-03-17 22:52
**Scope:** diff.rs, init.rs

## Summary
1. Generate --dry-run now shows concise summary: "Would create clippy.toml (94 lines)" instead of dumping entire file contents
2. Fixed unclear "libraries are pure types" → "no input boundary validation for shared packages"
