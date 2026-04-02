# Structure consumes ProjectTree, legality drops tree dependency

**Date:** 2026-04-02 12:18

## Summary

Structure now takes ProjectTree by value (consuming it). StructureFacts carries
forward content map, root path, and directory structure. Legality no longer
receives or imports ProjectTree — reads content from StructureFacts only.
Legality's Cargo.toml no longer depends on domain-project-tree.

## Changes

### structure/src/lib.rs
- `collect(tree: ProjectTree)` — takes by value, not by reference
- `RustStructureFacts` now carries: content (BTreeMap), root (PathBuf),
  structure (BTreeMap<String, DirEntry>)
- Added `file_content()`, `matching_dir_rels()`, `root_path()`, `dir_structure()`
  accessors for legality to use

### legality/src/lib.rs
- `collect(structure: &RustStructureFacts)` — no tree parameter
- All internal functions changed from `tree: &ProjectTree` to `structure: &RustStructureFacts`
- Uses `structure.file_content()` instead of `tree.file_content()`
- Uses `structure.matching_dir_rels()` instead of `tree.matching_dir_rels()`

### legality/Cargo.toml
- Removed `guardrail3_domain_project_tree` dependency entirely

## Verification
`cargo check -p guardrail3-app-rs-legality` passes clean.
Legality has zero access to ProjectTree — not in code, not in dependencies.

## Remaining leaks
- Mapper still receives &ProjectTree and &RustStructureFacts — next target
- Runtime context stores &ProjectTree — breaks on tree consumption
