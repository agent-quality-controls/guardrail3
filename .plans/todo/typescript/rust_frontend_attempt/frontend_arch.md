# RS-FRONTEND-ARCH — Dioxus/frontend architecture checker

**Input:** Rust frontend crate structure + Cargo.toml + frontend/content config + generated artifact layout
**Parser:** `ProjectTree` structure + TOML + targeted Rust-file discovery
**Current code:** None yet — new family needed for Rust frontend architecture

## Scope

This family is for Rust-rendered frontend apps, primarily Dioxus-based apps and the crates around them.

It is the Rust replacement for the useful structural part of the old TS frontend architecture plans. It should enforce frontend-specific structure and dependency direction, not generic Rust code quality.

## Rules

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-FRONTEND-ARCH-01 | Error | Frontend app root is identifiable and explicitly typed as a Rust frontend/content app | Planned |
| RS-FRONTEND-ARCH-02 | Error | One canonical frontend root layout exists (`routes/`, `components/`, `layouts/`, `content/`, `generated/`, `assets/`, `theme/` or repo-approved equivalent) | Planned |
| RS-FRONTEND-ARCH-03 | Error | No duplicate/shadow frontend roots or ad hoc parallel directories that bypass the canonical layout | Planned |
| RS-FRONTEND-ARCH-04 | Warn | Loose files in container directories are banned except approved sentinels | Planned |
| RS-FRONTEND-ARCH-05 | Error | UI/runtime crates must not parse raw content files directly | Planned |
| RS-FRONTEND-ARCH-06 | Error | Content compiler/pipeline crates must not depend on runtime UI crates | Planned |
| RS-FRONTEND-ARCH-07 | Error | Frontend/runtime crates must not depend directly on DB/backoffice/backend-only adapters | Planned |
| RS-FRONTEND-ARCH-08 | Warn | Shared UI crates must not accumulate route/runtime/content-compiler concerns in one crate | Planned |
| RS-FRONTEND-ARCH-09 | Info | Frontend root inventory: app roots, UI crates, pipeline crates, generated roots | Planned |
| RS-FRONTEND-ARCH-10 | Error | Input failures for frontend architecture discovery/config parsing fail closed | Planned |

## Notes

- This is not a Rust copy of TS hex-arch.
- The point is explicit frontend structure and boundary direction for Dioxus/content systems.
- Escape hatches such as duplicate roots or special directories should require explicit documented ownership in config.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| Exact TS `src/modules/{domain,ports,application,adapters}` layout | Wrong abstraction for Dioxus/frontend work |
| ESLint/boundaries plugin presence | JS tooling mechanism, not Rust frontend policy |
| Next.js route-wrapper rules | Framework-specific legacy TS behavior |
