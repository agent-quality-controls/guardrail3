Summary

- Fixed the hooks validate-step detector to match the real `guardrail3-rs validate --path ...` CLI instead of the dead `--staged` and `rs validate` shapes.
- Expanded the golden tests to cover the live command shape, dead `--config` forms, and malformed `--path` usage.

Decisions made

- Tightened the matcher to the actual CLI grammar after `validate`.
  - Accepted: `--path`, `--path=...`, optional `--family`, optional `--inventory`.
  - Rejected: `rs validate`, `--staged`, `--config`, and malformed `--path` without a value.
- Updated both the direct validate-step tests and the config-trigger tests.
  - Reason: both suites were still encoding the dead command shape.

Key files for context

- `.plans/2026-04-14-152754-hooks-validate-path-cli-shape.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/mod.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/tests/golden.rs`
- `apps/guardrail3-rs/crates/io/inbound/cli/src/main.rs`

Next steps

- Continue the repo-wide stale-doc cleanup outside `hexarch`.
- Keep hook command detection aligned to the real `guardrail3-rs` CLI rather than preserving compatibility with deleted app surfaces.
