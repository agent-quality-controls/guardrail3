Summary
- Fixed the arch config rule mismatch that blocked the shared test shape.
- Arch now allows the exact test-only edges needed for `runtime`, `assertions`, and `test_support`, while still rejecting normal cross-crate dependencies.

Decisions made
- Fixed `arch`, not `test`, because the test family shape was correct and the arch rules were applying production dependency rules to test-only helper crates.
- Kept the exception narrow. Only these edges stand down:
  - `crates/assertions -> crates/runtime`
  - `crates/runtime --dev-dependencies-> crates/assertions`
  - `crates/runtime --dev-dependencies-> crates/test_support`
- Kept a control test that still fails for a normal non-child dependency so this does not become a general escape hatch.

Key files for context
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_05_no_boundary_crossing.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_06_shared_flag_required.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_05_no_boundary_crossing_tests/mod.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_06_shared_flag_required_tests/mod.rs`

Next steps
- If another package uses the same `runtime + assertions + test_support` split, it should now pass arch without extra rule changes.
- If a new helper crate role appears, add direct tests first before widening the exception.
