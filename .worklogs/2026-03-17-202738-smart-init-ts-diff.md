# Smart init + ts diff command

**Date:** 2026-03-17 20:27
**Scope:** init.rs, cli.rs, main.rs, diff.rs, generate.rs, mod.rs, ts_arch_checks.rs

## Summary
1. ts init now analyzes the project — discovers apps, auto-detects types, generates project-specific config with detection reason comments
2. ts diff command added — dry-run for ts generate, shows what files would change
3. --dry-run on init shows full content preview for new files
