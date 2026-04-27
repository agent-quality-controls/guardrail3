## Goal

Fix `g3rs-hooks/hook-rs-17-shared-target-dir-present` so it reasons about real shell execution semantics instead of whole-line text, and closes the concrete attack cases: same-line segment ordering, shell-wrapper payloads, fake echoed assignments, and wrapper functions.

## Approach

- Add failing golden tests first for the concrete break cases found in the attack pass:
  - `export ...; unset ...; cargo ...` on one line
  - `unset ...; CARGO_TARGET_DIR=... cargo ...` on one line
  - `echo "CARGO_TARGET_DIR=..." && cargo ...`
  - `bash -lc 'export ...; cargo ...'`
  - `cargo() { export ...; command cargo "$@"; }` then `cargo ...`
- Replace the ad hoc whole-line evaluation in `hook_rs_17_shared_target_dir_present` with the same segment-aware pattern already used by `hook_rs_09_clippy_denies_warnings`.
  - Track `CARGO_TARGET_DIR` in a tiny env state.
  - Evaluate one reachable segment at a time.
  - Recurse into shell-wrapper payloads and called functions.
  - Use command-token semantics, not raw substring presence, for cargo coverage.
- Keep the rule boundary the same: warning inventory when covered, warning finding when cargo is uncovered, quiet when no cargo runs.
- Re-run tests, formatter, validator, then another adversarial pass.

## Key decisions

- Fix at the rule boundary, not in tests or assertions.
  - Why: the attack found real check bugs, not just proof gaps.
- Reuse existing hook-source semantic helpers as the pattern.
  - Why: the family already has a stronger shell semantics implementation in `hook_rs_09_clippy_denies_warnings`.
  - Rejected: patching more special-case text scans into the new rule.
- Keep the message and severity unchanged.
  - Why: the bug is in detection semantics, not in policy intent.

## Files to modify

- `.plans/2026-04-21-195342-fix-hook-shared-target-dir-shell-semantics.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule_tests/golden.rs`
- `.worklogs/<timestamp>-fix-hook-shared-target-dir-shell-semantics.md`
