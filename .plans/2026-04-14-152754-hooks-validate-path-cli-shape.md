Goal

- Make the hooks validate-step detector match the real `guardrail3-rs` CLI shape.
- Stop accepting dead `--staged`, `rs validate`, and `--config` forms.

Approach

- Rewrite the hook golden tests to use `g3rs validate --path ...` as the positive command shape and add explicit negative cases for dead legacy forms.
- Fix the matcher in `hook_rs_08_guardrail_validate_staged_present/mod.rs` to require:
  - command name `g3rs`
  - first subcommand `validate`
  - a `--path` argument, either split or attached
  - no help/version flags
- Re-run hooks tests and app tests, then commit as a standalone bug fix.

Key decisions

- Fix in the hook rule, not in the app.
  - Reason: the rule is the stale compatibility surface.
- Keep wrapper support (`env`, `exec`, path-qualified command, command substitution) because those are shell-shape concerns, not CLI drift.

Files to modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/mod.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/tests/golden.rs`
