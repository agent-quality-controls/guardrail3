Goal
- Make `HOOK-SHARED-10` accept equivalent fail-closed shell option spellings such as `set -eu -o pipefail` without false warnings.

Approach
- Read the current rule and tests to confirm the bug surface and keep the fix local to the shell-option matcher.
- Add rule tests first for equivalent `set` spellings that are semantically the same fail-closed contract this rule already accepts.
- Update `has_shell_error_handling_line(...)` in `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_10_shell_error_handling/rule.rs` to normalize and validate accepted option combinations instead of exact-string matching.
- Re-run the touched package tests and `g3rs validate` scoped to the touched hook/parser packages.

Key decisions
- Fix the rule matcher, not the parser.
  - Why: the parser already preserves raw source lines; the bug is the rule's overly narrow semantic check.
- Accept only forms that clearly express the existing contract: `-e` must be enabled, and `pipefail` may appear either inline in the short-option cluster or via `-o pipefail`.
  - Rejected: broad shell-option parsing or support for unrelated options that are not needed to prove this bug.
- Keep the production change inside the existing rule file.
  - Why: this is a single-rule bug, and adding shared helpers would increase surface area without reuse pressure.

Files to modify
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_10_shell_error_handling/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_10_shell_error_handling/rule_tests/golden.rs`
