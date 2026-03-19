# Add T-PKG-04: pnpm engine constraint check

**Date:** 2026-03-19 17:40
**Scope:** `src/app/ts/validate/package_check.rs`

## Summary
Added check that `engines` field includes a `pnpm` version constraint. Found by diffing ts-rust-railway (missing `pnpm` in engines) against steady-parent (has both `node` and `pnpm`).

## Context
ts-rust-railway had `"engines": {"node": ">=24"}` but no pnpm constraint, while steady-parent has `"engines": {"node": ">=22", "pnpm": ">=10"}`. Since `.npmrc` requires `package-manager-strict-version=true`, the pnpm engine constraint should also be enforced.
