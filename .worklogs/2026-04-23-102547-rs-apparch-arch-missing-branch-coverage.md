Summary
- Added the missing direct-`all` export branch proof for `RS-ARCH-SOURCE-08`.
- Added the missing fail-closed policy tests for `RS-APPARCH-CONFIG-08` and `RS-APPARCH-CONFIG-09`.

Decisions made
- Kept this as test-only coverage.
  - The production branches already existed and were reachable; the gap was that they were unproved.

Key files for context
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_08a_feature_gated_exports_tests/cases.rs`
- `packages/rs/arch/g3rs-arch-source-checks/crates/assertions/src/rs_arch_08a_feature_gated_exports.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_08_types_purity_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_09_logic_purity_tests/cases.rs`
- `.plans/2026-04-23-102455-rs-apparch-arch-missing-branch-coverage.md`

Next steps
- Fold the final attack-pass result into the closeout once the tree is clean again.
