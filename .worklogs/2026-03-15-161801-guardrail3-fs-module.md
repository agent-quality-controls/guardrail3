# guardrail3 — Centralized fs module + ban reason strings

**Date:** 2026-03-15 16:18
**Scope:** src/fs.rs (new), 18 files migrated, src/modules/clippy.rs

## Summary
Created centralized filesystem module. All std::fs calls now go through src/fs.rs. Updated clippy ban reason strings to tell agents to create their own centralized fs module.

## Decisions Made
- **fs not io** — module is specifically for filesystem operations, not general IO
- **Ban reason strings updated** — now say "BANNED: Create a centralized fs module and route all filesystem operations through it" instead of generic "Use centralized io module"
- **Service profile maintained** — guardrail3 enforces same fs bans it validates others for, with centralized module as the pattern
