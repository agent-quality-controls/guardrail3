# Add CLI tests, adversarial fixtures, fuzz targets

**Date:** 2026-03-15 21:57
**Scope:** Test robustness — CLI integration + adversarial correctness verification

## Summary
- 35 CLI integration tests targeting 48 mutation survivors in command layer
- 16 adversarial source scan fixtures (R30-R58): 0 bugs found
- 17 adversarial config fixtures (R1-R29): 0 bugs found
- Fuzz targets scaffolded (cargo-fuzz, 3 targets, needs lib.rs for imports)
- lib.rs created (required by existing Cargo.toml [lib] section)
- Total: 139 unit + 35 CLI + 16 adversarial source + 17 adversarial config + 11 property = 218 tests
