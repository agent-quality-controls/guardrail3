# Fix mapper legality call signature

**Date:** 2026-04-02 12:24

## Summary
FamilyMapper::new called legality::collect(tree, structure) but legality now
takes only &RustStructureFacts. Fixed call to collect(structure).

## Remaining leaks (from test attack round 1)
- Mapper stores &ProjectTree and &RustStructureFacts — should only store &LegalityFacts
- RustRunContext carries &ProjectTree to all families
- All runners use ctx.tree for surface construction
- scoped_files.rs takes &ProjectTree directly
