Goal
- Close the remaining known missing branch coverage in `g3rs-arch/feature-gated-exports` and `g3rs-apparch/types-purity/09`.

Approach
- Add a source-rule test for `g3rs-arch/feature-gated-exports` where exports are gated directly on `all`.
- Add the missing fail-closed policy tests:
  - `g3rs-apparch/types-purity` with `Unreadable`
  - `g3rs-apparch/logic-purity` with `ParseError`
- Re-run the touched packages and `g3rs validate`.

Key decisions
- Keep this as test-only hardening.
  - The production branches already exist; the gap is unproved behavior.

Files to modify
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_08a_feature_gated_exports_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_08_types_purity_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_09_logic_purity_tests/cases.rs`
