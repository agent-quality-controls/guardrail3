# Worklog - code config lane hardening

## Summary

Fixed the `code` config-lane ownership bug that let foreign nested repos bleed
into `RS-CODE-07` and `RS-CODE-12`. Added the missing negative, boundary, and
fail-closed tests, then reran the 4-agent `test-attack` until it converged with
no blocker.

## Decisions made

- Added explicit owned-config root selection in `g3rs-code-ingestion`.
  - Why: the bug was in selection, not in the rules.
  - Chosen shape:
    - always allow root-level config files
    - if root `Cargo.toml` is a workspace, allow member roots from
      `[workspace].members`
    - scan config files only under those owned roots

- Reused workspace-member selection logic locally instead of importing another
  family package.
  - Why: keeps the fix in the `code` family boundary and avoids cross-family
    coupling.

- Expanded tests in both places:
  - ingestion/pipeline tests for ownership, negative cases, counts, and
    fail-closed behavior
  - checks-package rule tests to match the stronger config-family pattern with
    `helpers.rs`, `golden.rs`, `missing.rs`, and `wrong.rs`

## Key files for context

- `.plans/2026-04-09-192941-code-config-lane-hardening.md`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_scope.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_comments.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/unsafe_code_lints.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule_tests/`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule_tests/`

## Verification

- `cargo test --workspace -q` in `packages/rs/code/g3rs-code-config-checks`
- `cargo test --workspace -q` in `packages/rs/code/g3rs-code-ingestion`
- final 4-agent `test-attack`:
  - completeness: no blocker remains
  - missing scenarios: no blocker remains
  - pattern parity: no meaningful deviation remains
  - false positives: no blocker remains

## Next steps

1. Build the remaining `code` file-tree lane for `RS-CODE-35`.
2. If `code` config grows beyond `07` and `12`, keep using owned-root
   selection instead of repo-wide filename scans.
