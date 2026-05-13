Summary

- Planned Stage 5 release workflow replay coverage.
- The plan identifies the real blocker: release repo-root checks exist but are not reachable from `g3rs validate` because the runner does not call them and release repo-root ingestion is still stubbed.

Decisions made

- The implementation must first make `g3rs-release-repo-root-checks` reachable through the public CLI.
- `repo_root_result` should reuse existing release collection output instead of duplicating workflow parsing.
- `L70-release-workflow-policy-violated` should omit workflow files to trigger the three missing-workflow warnings with minimal fixture shape.
- `g3rs-release/release-profile-inventory` should be treated as intentional Info inventory because the rule has no warning/error branch.

Key files for context

- `.plans/2026-05-13-194226-g3rs-release-workflow-replay-fixture.md`
- `apps/guardrail3-rs/crates/logic/family-runner-process/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-repo-root-checks/crates/runtime/src/run.rs`
- `behavior/coverage/g3rs-rule-coverage.toml`

Next steps

- Implement the plan.
- Run the release ingestion, repo-root checks, behavior, and real `g3rs validate` verification listed in the plan.
- Send adversarial reviewers against the original plan and final code before reporting completion.
