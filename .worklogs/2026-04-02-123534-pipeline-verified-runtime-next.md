# Pipeline verified — runtime/runners leak remaining

**Date:** 2026-04-02 12:35

## Verified boundaries

| Boundary | Status |
|----------|--------|
| Tree → Structure | CLEAN (by-value consumption) |
| Structure → Legality | CLEAN (by-value consumption, no tree dep) |
| Mapper sees only legality | CLEAN (no tree dep in code or Cargo.toml) |
| Runtime context/runners | LEAK — tree field + all runners bypass pipeline |

## Remaining leaks (all in runtime)

1. `context.rs:23` — unconditional `pub(crate) tree: &'a ProjectTree` field
2. `runners.rs:3` — imports deleted RsProjectSurface
3. All 16 runner functions pass ctx.tree to surface builders
4. Runtime lib.rs uses tree after move (line 105 uses &tree after line 87 moves it)

## Next step

Plan the runtime/runner migration: remove tree from context, replace
surface building with FamilyView from legality output.
