## Summary

Ran a second adversarial pass on the extracted hook source lanes and added tests for wrapper, path-qualified, called-function, and quoted-heredoc edge cases. The failing tests exposed one parser bug and one broader rule-family bug class in `hooks-shared`; both are now fixed.

## Decisions made

- Fixed the root parser bug in `hook-shell-parser` instead of patching individual hook rules.
  - `<<` inside quoted grep patterns like `'<<<<<<<'` was being misread as a heredoc start.
  - That broke later function parsing and command resolution.
  - Rejected: rule-local workarounds that would leave the parser wrong.

- Moved affected `hooks-shared` source rules onto resolved-command matching.
  - `HOOK-SHARED-15`, `18`, `20`, and `21` were still checking flat executable lines and missing wrappers or path-qualified commands.
  - Rejected: narrow string checks for each wrapper style.

- Widened `HOOK-SHARED-21` input so the rule can resolve one softened line against the full parsed script.
  - This lets fail-open detection work for helper-function calls, not just direct commands.

## Key files for context

- `.plans/2026-04-10-191506-hook-source-second-test-attack.md`
- `packages/parsers/hook-shell-parser/crates/parser/runtime/src/support.rs`
- `packages/parsers/hook-shell-parser/crates/parser/runtime/src/command_query.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks/crates/runtime/src/shell_safety/hook_shared_18_executable_command_context_only/mod.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks/crates/runtime/src/shell_safety/hook_shared_20_concrete_lockfile_command/mod.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/mod.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks/crates/runtime/src/workflow/hook_shared_15_merge_conflict_step_present/mod.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks/crates/runtime/src/workflow/hook_shared_16_file_size_step_present/mod.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Next steps

- Run another attack pass when `hooks-shared` config or file-tree lanes are extracted.
- Reuse the resolved-command parser helpers for any future shell-source rules before adding custom token scans.
