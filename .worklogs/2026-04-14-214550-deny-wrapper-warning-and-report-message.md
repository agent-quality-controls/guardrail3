# Summary
Changed `g3rs-deny/wrappers` so a narrow package-local wrapper exception on a managed banned crate warns instead of hard-failing. Also fixed the CLI report renderer so it prints the rule message text instead of hiding it behind the short title.

# Decisions Made
- Softened only the narrow local-wrapper case. Rejected weakening malformed wrapper entries or the family baseline itself.
- Kept the default deny baseline strict. Rejected putting `tree-sitter` back into the shared `regex` wrapper baseline.
- Fixed the report layer rather than rewriting rule titles. Rejected title-only output because it drops the actual actionable explanation from the rule.
- Rolled back the `tree-sitter` experiment in the clippy package after verifying the full path. Rejected keeping the dependency because the user said the package does not actually need it.

# Key Files For Context
- `.plans/2026-04-14-213310-deny-managed-wrapper-warning.md`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers_tests/managed_wrappers.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers_tests/project_specific_wrappers.rs`
- `apps/guardrail3-rs/crates/io/outbound/report/src/lib.rs`

# Next Steps
- The clippy config-checks package is back to only `test` family failures.
- If continuing package-by-package, the next work is to decide which `test` findings are valid policy versus overreach, then fix them or fix the family.
