# Split ProjectTreeView: remove discovery methods from family surface

**Date:** 2026-04-02 01:54

## Summary

Split ProjectTreeView into two traits to enforce the family ingress boundary:
- `ProjectTreeView` — narrow read-only (no walking/discovery)
- `ProjectTreeDiscovery: ProjectTreeView` — adds `all_dir_rels` and `dirs_with_file`

Pre-family stages (structure, placement, ownership, mapper) upgraded to
ProjectTreeDiscovery. RsProjectSurface (family input) only implements
ProjectTreeView — families can no longer walk/discover.

## Breaking changes

Five family crates now fail to compile:
- arch (11 errors) — all_dir_rels, dirs_with_file
- hexarch (5 errors) — dirs_with_file
- release (3 errors) — dirs_with_file
- test (11 errors) — all_dir_rels
- cargo-assertions-common (2 errors)

These families must be fixed to use route roots for discovery instead of
walking the surface. This is the intended break.

## Files changed

### Domain
- `domain/project-tree/src/lib.rs` — split trait, added ProjectTreeDiscovery

### Pre-family pipeline (upgraded to ProjectTreeDiscovery)
- `app/rs/placement/src/roots.rs` — collect() takes &dyn ProjectTreeDiscovery
- `app/rs/ownership/src/{lib.rs,discover.rs}` — collect() takes &dyn ProjectTreeDiscovery
- `app/rs/structure/src/lib.rs` — collect() takes &dyn ProjectTreeDiscovery
- `app/rs/family_mapper/src/rs.rs` — FamilyMapper.tree field is &dyn ProjectTreeDiscovery

### Family surface (narrowed)
- `app/rs/family_mapper/src/views.rs` — removed all_dir_rels and dirs_with_file
  from RsProjectSurface (both inherent methods and trait impl)

## Key Files
- `domain/project-tree/src/lib.rs` — trait definitions
- `app/rs/family_mapper/src/views.rs` — surface API boundary
