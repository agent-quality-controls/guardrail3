# RS-ARCH-03: Add directory scan for #[path]-wired modules

**Date:** 2026-04-01 23:55

## Summary

RS-ARCH-03 was only finding 46 violations (foo.rs convention) because it only scanned `mod foo;` declarations. Directories wired via `#[path]` attributes have no corresponding `mod dirname;` declaration — the module name differs from the directory name. Added a second scan pass that checks all directories with .rs files under crate src/ trees.

## Changes

- `module_layout.rs`: Split collection into two passes:
  - Pass 1: mod declaration scan (existing behavior, catches foo.rs convention)
  - Pass 2: directory scan — finds directories under crate src/ trees with .rs files and no mod.rs, regardless of whether a mod declaration points to them
  - Added `is_under_crate_src()` to scope pass 2 to actual crate source trees
  - Added exclusion for src/ directories containing lib.rs/main.rs (crate entry points, not module directories)

- `rs_arch_03_mod_rs_required.rs`: Updated error message for directories found by scan (no mod declaration) to indicate they're likely wired via #[path].

## Results

RS-ARCH-03 violations went from 46 → 171 (46 foo.rs + 125 #[path]-wired directories).

## Key Files
- `facts/module_layout.rs` — two-pass collection
- `facade/rs_arch_03_mod_rs_required.rs` — error message for scan-found dirs
