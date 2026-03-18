# deny.toml coverage map

**Date:** 2026-03-18 19:51
**Scope:** apps/guardrail3/src/commands/coverage/deny.rs

## Summary
Added deny.toml coverage map with tree and JSON output. Same structure as clippy coverage but with deny-specific details (bans count, advisory ignores) and no shadows field (deny.toml uses CWD-only resolution, no walk-up shadowing).

## Decisions Made
- **No shadows field:** cargo-deny uses CWD only, no walk-up. Shadowing is impossible. Simpler model than clippy.
- **Same JSON schema pattern:** scopes array, workspace/package tagged union, tool-specific config details field (`deny_toml` vs `clippy_toml`).

## Key Files for Context
- `apps/guardrail3/src/commands/coverage/deny.rs` — deny coverage map
- `apps/guardrail3/src/commands/coverage/clippy.rs` — reference implementation
- `.plans/by_file/rs/deny-toml.md` — deny.toml per-file plan
