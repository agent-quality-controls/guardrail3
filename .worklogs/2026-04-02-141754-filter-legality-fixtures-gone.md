# Filter legality output, remove .structure() accessor — fixtures gone

**Date:** 2026-04-02 14:17

## Summary

Closed the last leak: LegalityFacts no longer exposes raw .structure().
Replaced with specific filtered accessors. Structure data filtered to legal
roots with fixture/target/snapshot exclusion. Test fixtures eliminated from
all family output (217 → 0).

## Changes

### legality/src/lib.rs
- Removed `pub fn structure() -> &RustStructureFacts` accessor
- Added specific accessors: content(), dir_structure(), root_path(),
  placement(), overlaps(), input_failures(), matching_dir_rels(), file_content()
- collect() now calls structure.filter_to_roots() with legal workspace root dirs
- Removed public constructor (struct literal only in collect())

### structure/src/lib.rs
- Added filter_to_roots() method: filters structure/content maps to legal roots
  AND excludes fixture/target/snapshot directories via is_excluded_live_root_dir()
- Re-exported DirEntry as pub

### family_mapper/src/rs.rs
- All self.legality.structure().X() calls replaced with self.legality.X()
- scoped_files now takes &RustLegalityFacts

### runtime/src/runners.rs
- All ctx.legality.structure().X() calls replaced with ctx.legality.X()

## Verification
- Zero fixture results in arch output (was 217)
- 2572 real errors across 9 rules (down from 2764 with fixture inflation)
- Full binary builds and runs clean
