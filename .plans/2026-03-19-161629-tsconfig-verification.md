# Empirically verify tsconfig.json resolution

**Date:** 2026-03-19 16:16
**Task:** Verify how `tsc` resolves tsconfig.json — walk-up, shadowing, extends behavior

## Goal
Understand exactly how tsc finds and resolves tsconfig.json so the coverage map in `coverage/tsconfig.rs` has correct resolution semantics.

## Input Information
- steady-parent has: root `tsconfig.base.json`, per-app `tsconfig.json` files that extend it
- Root file is named `tsconfig.base.json` (not `tsconfig.json`) — tsc walk-up only finds `tsconfig.json`
- All app tsconfigs use `"extends": "../../tsconfig.base.json"` — explicit path, not walk-up
- Prior session established: all Rust tools walk up, nearest wins, complete replacement

## Approach

### Tests to run
1. **Walk-up test:** Run `tsc --showConfig` from `apps/web/` — does it find `apps/web/tsconfig.json`?
2. **Walk-up from subdir:** Run `tsc --showConfig` from `apps/web/src/` — does it walk up to `apps/web/tsconfig.json`?
3. **No tsconfig at root:** The root has `tsconfig.base.json` not `tsconfig.json` — run `tsc --showConfig` from root to see what happens
4. **Intermediate shadow test:** Create a temporary `tsconfig.json` at `apps/` level, run from `apps/web/src/` — does intermediate shadow?
5. **Extends behavior:** Does extends MERGE or REPLACE? Run `tsc --showConfig` from app dir to see resolved config
6. **Per-file resolution:** Does tsc resolve per-file or per-project? (Unlike eslint which is per-file, tsc uses a project model)

### Key decisions
- Testing on steady-parent (websmasher/websmasher) as established in prior session
- Using `tsc --showConfig` to see resolved config without compiling
- Will create/remove temp files for shadow tests

## Risks
- tsc version matters — check which version is installed
- Next.js apps may have special tsc behavior via plugins
