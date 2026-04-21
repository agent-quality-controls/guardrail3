Summary

- Repaired the first hooks package-boundary seam by moving hook command resolution into the shared shell parser and deleting rule-local shell interpretation from the highest-risk hook rules.
- The hook parser now owns environment-sensitive command traversal, and the hooks package now consumes `ResolvedCommand` facts instead of re-tokenizing raw shell text for the basic Rust step-presence rules.

Decisions made

- Added a parser-side stateful visitor API instead of adding more hook-local helpers.
  - Why: the shell parser already owned command resolution. Extending that API keeps shell semantics in one place and stops rule packages from drifting into mini interpreters.
  - Rejected: adding another hook-local shared support layer. That would only centralize duplication inside the wrong package.
- Rewrote `RS-HOOKS-SOURCE-09` and `RS-HOOKS-SOURCE-25` to use the parser visitor with explicit environment state.
  - Why: these two rules had already produced real false positives from local shell interpretation.
- Rewrote `RS-HOOKS-SOURCE-03` through `07` to match `ResolvedCommand` instead of tokenizing `line.command_text`.
  - Why: these were simpler versions of the same boundary leak. The parser already resolves wrappers, path-qualified commands, and env-prefixed invocations.
- Deleted `hook_rs_12_cargo_dupes_step_present/support.rs`.
  - Why: it was dead code and contained a second unused shell parser. Keeping it would preserve exactly the complexity this repair is trying to remove.

Key files for context

- `.plans/2026-04-21-214642-rust-package-boundary-repair.md`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/engine.rs`
- `packages/parsers/hook-shell-parser/crates/runtime/src/command_query/api_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/support.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_01_fmt_step_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_05_cargo_machete_step_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule.rs`

Next steps

- Finish the hooks seam repair by auditing the remaining hook rules for any rule-local command interpretation beyond argument-level matching.
- Move the duplicated hook-shell logic out of `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hook_shell.rs` onto the shared hook parser.
- After hooks and test stop duplicating shell semantics, start the next package-boundary pass on `topology`, `apparch`, and `release`, where normalization still lives inside check packages.
