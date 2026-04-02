# Break: delete RsProjectSurface and ProjectTreeView traits

**Date:** 2026-04-02 12:02

## Summary

Intentional breaking change. Deleted RsProjectSurface from family_mapper and
ProjectTreeView/ProjectTreeDiscovery traits from domain-project-tree. Every
family that accessed the tree is now a compile error. 81 errors across 10+
families.

## What was deleted

- `ProjectTreeView` trait (domain-project-tree) — families used this for tree access
- `ProjectTreeDiscovery` trait (domain-project-tree) — pre-family walking trait
- `RsProjectSurface` struct (family_mapper/views.rs) — the surface families received
- `RsProjectSurface` re-export from family_mapper/lib.rs
- `DirEntry` re-export from family_mapper/lib.rs

## What was preserved

- `ProjectTree` concrete type with all inherent methods — used by pre-family stages
- All route types (RsArchRoute, RsHexarchRoute, etc.) — stay in family_mapper
- Original code saved to legacy/ for reference

## Pre-family stages fixed

Changed from trait references to concrete `&ProjectTree`:
- placement/src/roots.rs
- ownership/src/{lib.rs, discover.rs}
- structure/src/lib.rs
- legality/src/lib.rs
- family_mapper/src/{rs.rs, scoped_files.rs}

## Compile errors (the loopholes)

81 errors across: cargo, clippy, deny, fmt, garde, hexarch, hooks-shared,
libarch, release, toolchain — plus arch, code, test from earlier.

Each error is a family that was accessing the tree and must be migrated to
the new FamilyView input.

## Next step

Fix forward: create FamilyView type, migrate each family to use it instead
of RsProjectSurface.
