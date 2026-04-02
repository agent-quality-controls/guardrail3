# Fix families after removing discovery methods from surface

**Date:** 2026-04-02 02:02

## Summary

Fixed all five broken families after removing `dirs_with_file` and `all_dir_rels`
from `RsProjectSurface`. Each family now uses `dir_contents`-based recursive
walks or `tree.structure()` iteration instead of discovery methods.

Test fixture leak eliminated: arch sees 0 fixture crates (was 37).

## Changes by family

### arch
- `crate_tree.rs`: new `find_cargo_dirs` + `find_cargo_dirs_recursive` walk within route roots
- `facade_surface.rs`: new `collect_mod_rs_recursive` walk within crate roots
- `module_layout.rs`: new `collect_rs_files_in_dir` + `collect_dirs_recursive`
- `facts/mod.rs`: receives route, passes root_dirs to crate_tree, walks within roots for .rs files

### hexarch
- `facts.rs`: replaced `dirs_with_file` with `tree.structure()` iteration
- `dependency_facts/workspaces.rs`: same pattern

### release
- `facts/cargo_roots.rs`: new `find_dirs_with_cargo_toml` + recursive walk within route roots

### test
- `discover/components.rs`: stack-based walk from src_rel_dir instead of all_dir_rels
- `structure/rs_test_02_owned_sidecar_shape.rs`: walk from src_roots instead of all_dir_rels

### cargo-assertions-common
- `lib.rs`: changed `check_results` to accept `&dyn ProjectTreeDiscovery` since it calls `structure::collect`

## Key Files
- `domain/project-tree/src/lib.rs` — trait split (ProjectTreeView vs ProjectTreeDiscovery)
- `app/rs/family_mapper/src/views.rs` — RsProjectSurface no longer exposes discovery
- Each family's facts collection files
