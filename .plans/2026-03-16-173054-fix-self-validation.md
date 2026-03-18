# Fix all self-validation errors (except R-ARCH-04 restructure)

**Date:** 2026-03-16 17:30
**Task:** Make guardrail3 pass its own validation (except the apps/ restructure)

## Fixes needed
1. Exclude tests/fixtures from source scan (R32, R34, R42, R58 on fixture files)
2. Exclude test files from R58 (tests need direct std::fs for temp dirs)
3. Add garde dependency + derive Validate on CLI structs (R-GARDE-01, R-GARDE-05)
4. Split 5 oversized files (R38)
5. Consolidate main.rs imports (R40)
6. Remove banned crate from lockfile (R50)
7. Fix std::fs in generate.rs and main.rs (R58)
