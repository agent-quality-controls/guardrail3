# JSON output for clippy coverage map + tree display fix

**Date:** 2026-03-18 19:46
**Scope:** apps/guardrail3/src/commands/coverage/clippy.rs, cli.rs, main.rs

## Summary
Added structured JSON output for the clippy coverage map (`--format json`) and fixed tree display bugs.

## Decisions Made

### JSON schema
- **Chose:** Scopes array with tagged union (workspace vs package), each with `clippy_toml` field (not generic `config`)
- **Why:** Field name matches the actual file. Each tool's coverage map uses its own field name (deny_toml, rustfmt_toml, etc.)

### Workspace vs package in scopes
- Workspaces have `crates` array with member packages
- Packages have `covered_by` and `shadows` directly (no nested crates — a package IS the crate)
- Avoids the redundant "package containing one package" pattern

## Key Files for Context
- `apps/guardrail3/src/commands/coverage/clippy.rs` — data model + both renderers
- `.plans/by_file/rs/clippy-toml.md` — clippy.toml per-file design plan
- `.plans/by_file/shared/discovery.md` — crawler design

## Next Steps
1. Build deny.toml coverage map (same pattern, different resolution: CWD only, no walk-up)
2. Build rustfmt.toml coverage map (walk-up like clippy)
3. Then TS side: eslint, tsconfig, stylelint, cspell, npmrc
