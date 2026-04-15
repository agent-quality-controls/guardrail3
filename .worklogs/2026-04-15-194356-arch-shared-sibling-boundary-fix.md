Summary

Fixed the arch config contradiction where rule 05 still rejected dependencies to sibling crates already marked `shared = true`. After the fix, shared sibling crates are handled only by rule 06, and `fmt-ingestion` no longer hits the false `runtime -> types` and `assertions -> types` errors.

Decisions made

- Fixed the contradiction in rule 05, not in rule 06.
- Kept the earlier special test-only edges unchanged:
  - `assertions -> runtime`
  - `runtime` dev-dep on `assertions`
  - `runtime` dev-dep on `test_support`
- Added direct rule tests for the real missing case:
  - `runtime -> shared types`
  - `assertions -> shared types`
- Left the next blocker untouched:
  - `RS-ARCH-CONFIG-07` still enforces the direct-dependency-count cap.

Key files for context

- `.plans/2026-04-15-194356-arch-shared-sibling-boundary-fix.md`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_05_no_boundary_crossing.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_05_no_boundary_crossing_tests/mod.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/test_support.rs`

Next steps

- Decide `RS-ARCH-CONFIG-07` for ingestion runtimes:
  - keep the hard cap
  - or allow/waive orchestrator-heavy runtimes
- After that, continue cleaning `packages/rs/fmt/g3rs-fmt-ingestion`.
