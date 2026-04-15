Goal
- Let the shared test shape work without opening general cross-crate coupling.

Approach
- Add direct arch config rule tests for these exact edges:
  - `crates/assertions -> crates/runtime`
  - `crates/runtime --dev-dependencies-> crates/assertions`
  - `crates/runtime --dev-dependencies-> crates/test_support`
  - keep a normal forbidden edge as a control case
- Change the arch config rules so only those exact test-only edges stand down.
- Re-run arch tests and the full clippy package validator.

Key decisions
- Fix `arch`, not `test`.
- Keep the exception narrow by exact crate role and dependency section.
- Do not allow normal `dependencies` from runtime into `assertions` or `test_support`.

Files to modify
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_05_no_boundary_crossing.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/rs_arch_06_shared_flag_required.rs`
- `packages/rs/arch/g3rs-arch-config-checks/crates/runtime/src/test_support.rs`
- new test sidecars for `05` and `06`
