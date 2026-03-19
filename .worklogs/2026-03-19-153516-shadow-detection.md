# Shadow detection in coverage maps

**Date:** 2026-03-19 15:35
**Scope:** coverage/engine.rs, plans/by_file/rs/clippy-toml.md

## Summary
Coverage maps now detect when a config file shadows another (nested inside its coverage area). Shadow configs get `is_shadow: true` and `shadows: "<parent path>"`. Parent configs get `shadowed_by: [{ path, steals }]`. Verified empirically: intermediate clippy.toml at crates/adapters/ shadows workspace root for adapters+api crates but not domain/app/ports.

## Verified edge cases for clippy
- Walk-up does NOT stop at workspace boundaries (goes past [workspace] Cargo.toml)
- Per-crate resolution is independent (each crate resolves its own config)
- Intermediate directory config shadows all crates below it (but not siblings)
- Clippy results are CACHED — changing clippy.toml doesn't trigger recompile, need cargo clean
