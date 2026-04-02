# All families migrated to FamilyView — zero compile errors

**Date:** 2026-04-02 13:26

## Summary
Complete migration of all Rust family crates from RsProjectSurface to FamilyView.
218 files changed. Zero compile errors. Zero warnings.

## What was done
- All 16 family runtimes, 12 test_support crates, assertions crates updated
- RsProjectSurface → FamilyView everywhere
- abs_path() now returns Option — all 11+ call sites updated
- DirEntry imported from family_view, not family_mapper
- join_rel() calls updated to FamilyView::join_rel()
- Test helpers build FamilyView via FamilyView::build() or FamilyView::from_tree()
- hooks/mod.rs migrated

## Architecture enforcement
- No family depends on domain-project-tree directly (except cargo-assertions test helper)
- No family has access to ProjectTree
- All families receive &FamilyView which is scoped to legal roots
- Pipeline: walker → structure(tree consumed) → legality(structure consumed) →
  mapper(&legality) → runners build FamilyView → families
