Summary

Closed the `guardrail3-ts` attack findings in the app layer. The execute path no longer prints fake clean stdout when runner failures leave no visible findings, and the app tests now prove the one-family CLI surface and the real ESLint wiring more directly.

Decisions made

- Fixed the false-clean-output bug in `execute.rs`, not in the renderer.
  - Reason: `PlainTextReportRenderer` is still correct for a genuinely clean report.
  - The bug was that execute rendered a report even when the run had family errors and no visible findings.
- Added a small private helper in `execute.rs` to make the stdout policy explicit and directly testable.
  - This avoided pushing the test into the renderer or inventing a broader abstraction.
- Added the real-stack CLI wiring test in `run.rs` through `run_command_with_defaults`.
  - Reason: the missing proof was the app wiring path, not package-level ESLint semantics.
  - `main.rs` now reuses that wiring instead of open-coding the dependencies.

Key files for context

- `.plans/2026-04-20-191123-g3ts-attack-followup.md`
- `.worklogs/2026-04-20-190316-guardrail3-ts-minimal-app.md`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute_tests/cases.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/selection_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/cli_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/main.rs`

Verification

- `cargo test --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `g3rs validate --path apps/guardrail3-ts`
- final adversarial review:
  - `Harvey`: no remaining findings
  - `Kant`: none

Next steps

- Install `g3ts` if we want the binary on `PATH`.
- Start fixing the actual ESLint findings in the external TS apps.
- Build the next TS families: `ts/package`, `ts/npmrc`, `ts/tsconfig`.
