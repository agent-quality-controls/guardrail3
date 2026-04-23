Goal
- Close the remaining known missing branch coverage in `RS-ARCH-SOURCE-08` and `RS-APPARCH-CONFIG-08/09`.

Approach
- Add a source-rule test for `RS-ARCH-SOURCE-08` where exports are gated directly on `all`.
- Add the missing fail-closed policy tests:
  - `RS-APPARCH-CONFIG-08` with `Unreadable`
  - `RS-APPARCH-CONFIG-09` with `ParseError`
- Re-run the touched packages and `g3rs validate`.

Key decisions
- Keep this as test-only hardening.
  - The production branches already exist; the gap is unproved behavior.

Files to modify
- `packages/rs/arch/g3rs-arch-source-checks/crates/runtime/src/rs_arch_08a_feature_gated_exports_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_08_types_purity_tests/cases.rs`
- `packages/rs/apparch/g3rs-apparch-config-checks/crates/runtime/src/rs_apparch_config_09_logic_purity_tests/cases.rs`
