# Verify tsconfig.json resolution

**Date:** 2026-03-19 16:20
**Scope:** `src/commands/coverage/tsconfig.rs`, `src/app/crawl.rs`, `src/app/project_map.rs`

## Summary
Empirically verified tsconfig.json resolution with TypeScript 5.9.3 on steady-parent. Fixed crawler to split `tsconfig.json` from `tsconfig.base.json` — only the former participates in walk-up resolution.

## Context & Problem
The coverage map for tsconfig was untested. The crawler lumped `tsconfig.json` and `tsconfig.base.json` into the same vec, causing `tsconfig.base.json` to falsely cover all 55 source directories via walk-up.

## Decisions Made

### Split crawler field
- **Chose:** Two fields: `tsconfigs` (walk-up) and `tsconfig_bases` (extends-only)
- **Why:** tsc walk-up only discovers `tsconfig.json` exactly — `tsconfig.base.json` is never auto-discovered, only reachable via explicit `extends`
- **Alternatives considered:**
  - Filter in coverage module's `config_files()` — can't do, trait returns `&[PathBuf]` (reference to crawl result)
  - Single field with a filter flag — over-engineering for one tool

## Empirical Verification Results

| Test | Result |
|---|---|
| Walk-up from `apps/web/` | Finds `apps/web/tsconfig.json` |
| Walk-up from `apps/web/src/modules/domain/` | Walks up to `apps/web/tsconfig.json` |
| Walk-up from root (only `tsconfig.base.json`) | ERROR TS5081 — not discovered |
| Walk-up from `apps/` (no tsconfig) | ERROR TS5081 — not discovered |
| Intermediate shadow at `apps/tsconfig.json` | Intercepts walk-up, `strict: false` wins |
| Intermediate shadow with own tsconfig | No effect — own tsconfig found first |
| Extends behavior | Deep merge — base settings inherited, local overrides per-key |
| Without extends | No inheritance — only the found file's settings |
| Filename specificity | Only `tsconfig.json` — NOT `tsconfig.base.json` |

**Key finding vs Rust tools:** tsconfig has `extends` for explicit deep-merge inheritance. Rust tools have complete replacement (no merging). But the walk-up mechanism itself is identical — nearest `tsconfig.json` wins and shadows.

## Open Questions
- `strict: false` in details for apps that inherit strict via extends — needs `tsc --showConfig` for resolved values (rich details pass)
- 2 uncovered dirs at root — correct (root has `tsconfig.base.json` not `tsconfig.json`)
