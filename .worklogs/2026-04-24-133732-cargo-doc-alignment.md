## Summary

Aligned the live cargo family docs with the cargo implementation that actually exists under `packages/rs/cargo`. Removed stale references to non-existent app-local cargo family paths and rewrote the docs around the current root-scoped ingestion contract.

## Decisions made

- Documented current code, not the older routed-family plan.
  - Reason: the live by-family cargo file was pointing at `apps/guardrail3/.../families/cargo`, which does not exist in this repo.
- Kept the historical cargo ledger separate.
  - Reason: `.plans/todo/checks/rs/cargo.md` still has useful rule history, but it is not the current implementation map.
- Documented the real member discovery semantics.
  - Included:
    - `[workspace].members` literal and glob expansion
    - `[workspace].exclude`
    - path normalization
    - deduplication
    - invalid member/exclude patterns degrading into ingestion failures
- Documented the hard root-missing boundary.
  - Reason: `CargoTomlNotFound` is a hard ingestion error before filetree input exists, so the filetree README should not imply every failure is represented inside `g3rs-cargo/input-failures`.

## Key files for context

- `.plans/2026-04-24-133241-cargo-doc-alignment.md`
- `.plans/by_family/rs/cargo.md`
- `packages/rs/cargo/g3rs-cargo-ingestion/README.md`
- `packages/rs/cargo/g3rs-cargo-config-checks/README.md`
- `packages/rs/cargo/g3rs-cargo-filetree-checks/README.md`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/run.rs`
- `packages/rs/cargo/g3rs-cargo-ingestion/crates/runtime/src/select.rs`
- `packages/rs/cargo/g3rs-cargo-types/src/types.rs`

## Verification

- Readback diff review of:
  - `.plans/by_family/rs/cargo.md`
  - `packages/rs/cargo/g3rs-cargo-ingestion/README.md`
  - `packages/rs/cargo/g3rs-cargo-config-checks/README.md`
  - `packages/rs/cargo/g3rs-cargo-filetree-checks/README.md`
- Consistency grep:
  - no remaining active cargo doc claims that cargo lives under the stale app-local family path
- Adversarial doc review against live code:
  - first pass found three real gaps:
    - missing member-discovery semantics
    - missing hard `CargoTomlNotFound` ingestion boundary
    - misleadingly conditional `guardrail3-rs.toml` wording
  - second pass found two real overstatements:
    - `guardrail3-rs.toml` read timing
    - missing malformed `[workspace].exclude` in filetree failure docs
  - final pass result:
    - `No concrete findings.`

## Next steps

- Keep the cargo docs aligned if the family broadens back to routed multi-root discovery later.
- If the pending adversarial re-review is clean, commit this as a docs-only alignment change.
