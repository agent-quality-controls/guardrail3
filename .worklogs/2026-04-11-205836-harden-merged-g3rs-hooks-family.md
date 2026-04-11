## Summary

Hardened the merged `g3rs-hooks` family across source, config, file-tree, and ingestion after repeated adversarial passes. This closed the remaining public-family migration gaps, fixed real parser and ingestion boundary bugs, and tightened end-to-end tests until the attack agents converged on no remaining real issue.

## Decisions made

- Built the missing merged non-source lanes instead of keeping them stubbed.
  - Added `g3rs-hooks-config-checks` for `HOOK-RS-06`, `HOOK-RS-14`, and `HOOK-RS-15`.
  - Added `g3rs-hooks-file-tree-checks` for `HOOK-SHARED-01`, `02`, `03`, `05`, `06`, `07`, `08`, `09`, `12`, and `17`.
  - Kept one public merged family boundary under `packages/rs/hooks`.

- Fixed source-lane bugs at the parser/rule boundary rather than adding rule-local band-aids.
  - Single-line constant `if` branches now keep all semicolon-separated commands.
  - Fail-open line attribution now uses real global line mapping for helper bodies instead of first-text-match lookup.
  - Helper-recursive shell safety checks and inert-text handling stayed in the parser/source layer, not duplicated in ingestion tests.

- Made file-tree ingestion collect structural modular artifacts even when `core.hooksPath=hooks`.
  - Source/config lanes still use the selected active hook surface.
  - File-tree lane now inventories `.githooks/pre-commit.d/*` as owned structural artifacts without treating them as active modular execution.

- Made hooksPath lookup fail closed for actual git-rooted workspaces.
  - If `.git` exists and `git config core.hooksPath` cannot be evaluated, ingestion now errors instead of silently guessing.
  - Non-git temp roots still stay quiet.

- Locked the owned active-hook surface explicitly.
  - `.githooks` and `hooks` are the only owned active-hook surfaces.
  - Other `core.hooksPath` values are treated as out-of-contract and are not analyzed as active hook paths.

## Key files for context

- `.plans/2026-04-11-192634-hooks-merged-hardening-and-non-source-lanes.md`
- `.plans/todo/checks/hooks/shared.md`
- `.plans/todo/checks/hooks/rs.md`
- `packages/parsers/hook-shell-parser/crates/parser/runtime/src/lib.rs`
- `packages/parsers/hook-shell-parser/crates/parser/runtime/src/support.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/selection.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_21_no_fail_open_wrappers/mod.rs`

## Next steps

- If hook work continues, the remaining work is no longer public-family extraction.
- The next hook-level work would be internal cleanup only:
  - flatten the remaining old shared-vs-rs internal module split if desired
  - keep adding exact e2e proofs whenever a new hook command shape is accepted
  - decide whether old app hook code should be retired now that merged package lanes exist
