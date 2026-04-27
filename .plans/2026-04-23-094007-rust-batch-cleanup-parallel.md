Goal
- Drive the current Rust cleanup batch to a clean repo state with no known unresolved production-path bugs from the active attack surface.

Approach
- Fix `g3rs-test/test-support-generic` module-alias helper resolution in `rs/test`.
- Run adversarial reviews in parallel on `rs/test`, `rs/code`, and hooks/parser against the latest committed state.
- For every concrete finding, add a red regression, fix it at the parser/support/rule boundary that owns the semantics, verify with package tests and `g3rs validate`, and commit as a standalone bug fix with a worklog.
- Repeat until attack agents return no concrete findings and `git status --short` is clean.

Key decisions
- Keep write scopes disjoint across agents: one writing worker, the rest read-only attack passes.
- Prefer parser/support boundary fixes over rule-local band-aids when the bug is semantic.
- Do not leave in-flight diffs uncommitted between bug-fix batches.

Files to modify
- Initially unknown beyond the active target:
  - packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule.rs
  - packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_18_test_support_generic/rule_tests/cases.rs
- Additional files only as required by attack findings.
