Goal

Fix the `arch` config contradiction where rule 05 rejects dependencies to sibling crates already marked `shared = true`, even though rule 06 exists to allow that exact case.

Approach

- Add a direct test that proves `RS-ARCH-CONFIG-05` wrongly fires on:
  - `runtime -> shared types`
  - `assertions -> shared types`
- Keep the current special test-only edges unchanged.
- Change `RS-ARCH-CONFIG-05` so it stands down when the target crate is marked `shared = true`.
- Leave `RS-ARCH-CONFIG-06` as the rule that decides:
  - shared target -> allowed
  - non-shared target -> error
- Re-run the arch package tests.
- Re-run `fmt-ingestion` arch validation to confirm the contradiction is gone.

Key decisions

- Fix the contradiction in rule 05, not by weakening rule 06.
- Treat `shared = true` as the real contract for intentional sibling dependencies.
- Keep the change minimal: do not change unrelated boundary logic.

Files to modify

- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_05_no_boundary_crossing.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_05_no_boundary_crossing_tests/mod.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/test_support.rs`
