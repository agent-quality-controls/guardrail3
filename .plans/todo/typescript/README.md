# TypeScript / Frontend Planning

This folder is active again, but it is no longer the canonical place to define TS family contracts.

Reason:
- the Rust-for-frontend direction is not mature enough yet
- frontend/content guardrails need a real active planning surface again

What lives here:
- revived TypeScript/frontend guardrail docs moved back out of `legacy/`
- TS/frontend audit notes worth reusing
- the Rust frontend/content attempt docs, kept as reference for possible future migration back

Canonical TS family contracts now live under:
- `.plans/todo/checks/ts/`

## Structure

- `checks/`
  - legacy/revived TS/deploy/hook plan docs and audit-era notes
- `audit/`
  - TS/frontend audit notes reactivated for planning use
- `ts/`
  - TS config/npmrc/plugin notes
- `rust_frontend_attempt/`
  - the paused Rust-frontend/content family ideas
  - reference only for now, not active Rust check-family implementation targets

## Current stance

- TypeScript/frontend is no longer treated as dead legacy
- Rust backend/library/app guardrails remain active
- Rust frontend/content family ideas are paused, not deleted

If planning frontend/content work, start here before reading `legacy/`.
If defining the actual family architecture, start at:
- `.plans/todo/checks/ts/README.md`
