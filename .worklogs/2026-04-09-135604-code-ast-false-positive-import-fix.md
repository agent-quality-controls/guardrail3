## Summary

Fixed a stale import in one `RS-CODE-06` false-positive test file after the broader code AST hardening landed. This restores a clean `cargo test --workspace -q` run for the extracted `g3rs-code-ast-checks` workspace.

## Decisions made

- Kept the fix as a stand-alone test bug commit.
  - Why: the breakage was a narrow leftover from test churn, not a production-rule change.
  - Rejected: bundling it into earlier hardening commits after the fact, because those are already committed working states.

## Key files for context

- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_06_garde_skip_with_comment/rule_tests/false_positives.rs`
- `.worklogs/2026-04-09-135026-code-ast-test-hardening.md`
- `.worklogs/2026-04-09-135458-code-ast-ingest-tests.md`

## Next steps

- Continue the next AST family only after running another adversarial pass on the new lane.
- If more `code` work happens, keep the extracted package and ingestion workspace tests green before adding more rules.
