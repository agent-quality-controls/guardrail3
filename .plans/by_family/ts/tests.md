# TS-TESTS

Status: current family contract, legacy-grouped implementation, but not a TS analogue to `RS-TEST`.

Implementation roots:

- `apps/guardrail3/crates/app/ts/validate/test_checks.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/tests.md` as the detailed family ledger until the cutover is complete

Current state:

- test-quality logic already exists, but still lives under the grouped TS validator layout
- this family is already cohesive in code and mostly needs plan reconciliation rather than a new implementation split
- compared with Rust, this family is about test-quality policy, not test-architecture enforcement or self-host hardening

Historical/supplemental references:

- `.plans/todo/checks/ts/tests.md`

Rule inventory:

- `T-TEST-01` — mutation-testing config exists.
  What it should do: detect Stryker config and warn when it is absent.
  What it is for: keep mutation testing visible as a test-quality amplifier rather than just raw test-count theater.
- `T-TEST-02` — real TS test files exist.
  What it should do: require at least one `.test.ts`, `.spec.ts`, `.test.tsx`, or `.spec.tsx` file under the root.
  What it is for: fail roots that nominally have TS code but no actual test surface.
- `T-TEST-03` — a test runner is configured.
  What it should do: require a Vitest or Jest config surface.
  What it is for: ensure tests run with intentional configuration rather than ad hoc local defaults.
- `T-TEST-04` — `.skip()` requires a same-line reason.
  What it should do: inventory `.skip()` calls with a same-line `// reason` comment and warn on undocumented skips.
  What it is for: make disabled tests auditable and harder to abandon permanently.
- `T-TEST-05` — `.only()` is forbidden in committed code.
  What it should do: error on `test.only`, `describe.only`, or `it.only`.
  What it is for: prevent silently skipping the rest of the suite.

Current code mapping:

- `apps/guardrail3/crates/app/ts/validate/test_checks.rs`
  - `check(...)` orchestrates the family.
  - `check_stryker_config(...)` implements `T-TEST-01`.
  - `check_test_files_exist(...)` implements `T-TEST-02`.
  - `check_test_runner_config(...)` implements `T-TEST-03`.
  - `check_skip_without_reason_content(...)` and helpers implement `T-TEST-04`.
  - `check_only_in_source_content(...)` and helpers implement `T-TEST-05`.
- `apps/guardrail3/crates/app/ts/validate/ast_helpers.rs`
  - provides the tree-sitter-based test method call detection used by `T-TEST-04` and `T-TEST-05`.

Current doc/code reconciliation notes:

- the old ledger is directionally correct but broader than the live implementation
- current code does not yet implement several planned checks mentioned by the old ledger, including:
  - mutation-test package/script/config coherence beyond config presence
  - test-runner package/script/config coherence beyond config presence
  - coverage-threshold configuration
  - `test.todo()` or equivalent unfinished-test inventory
  - minimum assertion-bearing test surface
- that makes `ts/tests` another family where the intended contract is broader than the current runtime
- compared with `RS-TEST`:
  - this family should not try to become a validator self-hosting or test-boundary meta-family
  - its strongest analogue is only the “test quality policy” slice, not the Rust family-splitting architecture
  - package/config presence checks should stay explicit if they remain here, otherwise they should move toward `TS-PACKAGE`
- the biggest likely missing rule class is “assertion-bearing test surface”; test files existing is much weaker than tests actually exercising assertions or expectations
- anti-bypass coverage is still thin; `.skip()` and `.only()` are not enough by themselves for a hardened family

Next planning focus:

- separate package/tool presence checks from direct test-source quality rules
- decide which of the old planned test-quality rules are real contract requirements versus future expansion ideas before demoting the old ledger
- explicitly state that `TS-TESTS` is a product test-quality family, not a TS counterpart to the Rust family-hardening/test-ownership system
- decide whether to add explicit rules for:
  - assertion-bearing test surface
  - unfinished tests such as `todo`
  - malformed/unreadable test config fail-closed behavior
