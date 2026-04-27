Goal
- Fix shared hook command-query helper resolution so nested helper chains and later helper redefinitions follow shell semantics, and land the hook-rule regressions that depend on that shared behavior.

Approach
- Add red regressions in the shared command-query API tests for:
- multi-hop helper chaining through root-level helpers called from inside another helper body
- later helper redefinition overriding an earlier noop at the call site
- Add red regressions in the affected hook rules for the same surfaces where they are user-visible:
- `g3rs-hooks/hook-shared-10-shell-error-handling` config-trigger helper chaining/redefinition
- `g3rs-hooks/hook-shared-21-no-fail-open-wrappers` fail-open wrapper detection through helper chains/redefinitions
- `g3rs-hooks/hook-shared-13-no-unconditional-exit-zero` unconditional `exit 0` detection through helper chains/redefinitions
- Fix helper lookup at the owning boundary:
- shared command-query engine should resolve the active helper definition for the actual call site, not the first matching definition
- nested helper traversal should preserve the root call-site line number when consulting root-level helpers
- If `g3rs-hooks/hook-shared-10-shell-error-handling` still duplicates the old bug in local support code, align its support helper resolution with the same shell-order semantics
- Verify with targeted package tests, touched-package `g3rs validate`, worklog, and a standalone bug-fix commit.

Key decisions
- Keep the main fix in shared parser runtime rather than loosening individual hook rules.
- Keep hook-side edits limited to closely related support where the rule still owns its own helper traversal.
- Prefer a small helper-resolution utility over scattering more ad hoc `.find(...)` lookups.

Files to modify
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support/text.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule_tests/golden.rs`
