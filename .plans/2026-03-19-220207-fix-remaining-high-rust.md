# Fix remaining HIGH Rust findings

**Date:** 2026-03-19 22:02
**Task:** 5 HIGH fixes in Rust validation

1. R32: require `// reason:` prefix, not just any `//` comment
2. R30/R37: detect always-true cfg_attr conditions (cfg_attr(all(), ...))
3. R58: handle UseTree::Glob for `use std::fs::*`
4. R30: visit inner attributes on inline mod blocks
5. F-04-03: check dev-dependencies and build-dependencies for arch violations
