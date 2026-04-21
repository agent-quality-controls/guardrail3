## Summary

Fixed `RS-HOOKS-SOURCE-25` so it follows real shell execution semantics instead of whole-line text. The rule now evaluates one reachable shell segment at a time, follows shell-wrapper payloads and function calls, and correctly handles same-line export/unset ordering, pipelines, background jobs, and command substitutions.

## Decisions made

- Fixed the bug at the rule boundary, not in assertions.
  - Why: the attack pass found real check bugs in shell evaluation, not just missing proof.
  - Rejected: adding more narrow text-pattern tests around a broken evaluator.

- Reused the existing segment-aware hook pattern.
  - Mirrored the `hook_rs_09_clippy_denies_warnings` approach with a tiny `EnvState` for `CARGO_TARGET_DIR`.
  - Why: the family already had a stronger shell model for wrappers and segment reachability.
  - Rejected: keeping the ad hoc whole-line scanner and patching one more special case at a time.

- Treated piped and backgrounded cargo as real executions.
  - Why: for this rule, cargo still runs and should require coverage even if it is piped or backgrounded.
  - This intentionally diverges from the earlier copied skip behavior because the target-dir policy is about execution, not about a terminal command result.

- Closed the command-substitution hole in the local helper.
  - Added backtick extraction alongside `$(...)` extraction.
  - Why: otherwise cargo could run inside backticks without the rule seeing it.

## Key files for context

- `.plans/2026-04-21-194142-hook-source-shared-target-dir-warning.md`
- `.plans/2026-04-21-195342-fix-hook-shared-target-dir-shell-semantics.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/support.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule_tests/golden.rs`

## Verification

- `cargo test -q --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml`
- `g3rs validate --path packages/rs/hooks/g3rs-hooks-source-checks`

## Adversarial review

- First attack pass found real bugs:
  - same-line `export` / `unset` ordering was wrong
  - quoted shell-wrapper payloads were opaque
  - echoed fake assignments on a shared line could suppress the warning
  - cargo wrapper functions were not handled correctly
- Fixed all of those and added proving tests.
- Follow-up pass then found one more real semantic gap:
  - piped/background cargo and backtick substitutions were not covered
- Fixed those too and expanded tests again.
- Final follow-up pass found no remaining concrete check bug.
- Remaining notes were only broader shell-model limits or optional coverage additions, not blockers for this rule iteration.

## Next steps

- If this rule goes deeper later, the next worthwhile additions are explicit proof for `env -u`, `env -S`, `command`, and `exec` wrapper branches.
- Broader shell constructs like `source` / `.` / `eval` still require a larger family-level decision about whether hook rules should model external sourced payloads at all.
