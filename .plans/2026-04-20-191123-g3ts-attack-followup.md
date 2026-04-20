Goal

Close the `guardrail3-ts` attack findings so the app does not print false clean output on runner failure and the minimal TS app contract is actually proved end-to-end.

Approach

- Add failing tests first in the app layer:
  - prove execute returns empty stdout when no family run succeeded and at least one family run failed
  - prove the real CLI stack (`PackageRuntime -> CliFamilyRunner -> PlainTextReportRenderer`) emits the ESLint missing-config finding for a TS root without `eslint.config.*`
  - prove the CLI rejects a Rust family name such as `fmt`
  - prove omitting `--family` still resolves to `eslint` only
- Fix the execute path in the architecturally correct place:
  - `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
  - keep successful family output when some families succeed
  - suppress fake clean stdout when zero families succeeded and at least one family errored
- Re-run app tests, format, and `g3rs validate` on the app root.
- Run one more adversarial pass against the changed app tests and execute/report behavior.

Key decisions

- Fix in `execute.rs`, not the renderer.
  - The renderer should keep rendering a clean empty report as `No findings.` when it is actually given a successful empty report.
  - The false-success bug is that execute currently renders a report in a failure-only run.
- Add the end-to-end app test in CLI `run_tests`, not package-level ESLint tests.
  - The missing proof is app wiring, not ESLint family semantics.

Files to modify

- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute_tests/cases.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/selection_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`
