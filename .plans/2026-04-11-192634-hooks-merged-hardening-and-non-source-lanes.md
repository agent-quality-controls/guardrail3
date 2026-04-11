Goal
- Fix the merged `g3rs-hooks` source-lane bugs found by the adversarial audit.
- Extract the remaining old hook rules into the merged public family so `g3rs-hooks` is not source-only behind merged-family names.

Approach
- Read the old `hooks-shared` and `hooks-rs` app families and port only the still-live rule logic into the merged packages.
- Add failing tests first for the reported source bugs:
  - fallback `hooks/pre-commit` must not activate `.githooks/pre-commit.d`
  - inline `# ... --no-verify` comments must fire
  - inert mentions beyond `#` and `echo` must fire for `HOOK-SHARED-18`
  - weak `pnpm ... --frozen-lockfile` matching must not false-pass
  - `grep ... && g3rs rs validate --staged .` must satisfy config-trigger coverage
  - add stale-read fail-closed ingestion tests
  - add `|| echo ...` fail-open coverage
- Fix the source and ingestion code at the parser or lane boundary, not rule by rule if a shared fix is cleaner.
- Add merged-family non-source packages:
  - `packages/rs/hooks/g3rs-hooks-config-checks`
  - `packages/rs/hooks/g3rs-hooks-file-tree-checks`
- Extend `g3rs-hooks-ingestion` to produce:
  - config input for Rust tool availability checks
  - file-tree input for hook existence, hooksPath, layout, executability, inventories, trust
- Port old app rules:
  - config: `HOOK-RS-06`, `HOOK-RS-14`, `HOOK-RS-15`
  - file-tree: `HOOK-SHARED-01`, `02`, `03`, `05`, `06`, `07`, `08`, `09`, `12`, `17`
- Add end-to-end pipeline tests for the new config and file-tree lanes plus a clean merged baseline.
- Rerun adversarial review and close all remaining findings before final commit.

Key decisions
- Use the merged `g3rs-hooks` family boundary, not separate public `hooks-rs` and `hooks-shared` packages.
  - Rejected reviving the old public split because the previous merge explicitly collapsed that boundary.
- Put tool-availability rules into the hooks config lane.
  - Rejected leaving them app-only because that would keep the merged family incomplete.
  - Rejected inventing a fourth public hook lane.
- Keep rule IDs unchanged.
  - Rejected renumbering because this task is migration hardening, not ledger redesign.
- Keep `g3rs` as the binary name and `guardrail3-rs.toml` as the config trigger file name.
  - Rejected re-opening the filename question because only the binary-name correction is established in-session.

Files to modify
- `packages/rs/hooks/g3rs-hooks-ingestion/**`
- `packages/rs/hooks/g3rs-hooks-source-checks/**`
- `packages/rs/hooks/g3rs-hooks-config-checks/**` (new)
- `packages/rs/hooks/g3rs-hooks-file-tree-checks/**` (new)
- `.plans/todo/checks/hooks/shared.md`
- `.plans/todo/checks/hooks/rs.md`
