## Summary

Fixed the `garde` family so applicability is decided inside the package instead of in the `guardrail3-rs` app. When a workspace has neither a `garde` dependency nor a `guardrail3.toml`, the family now becomes `Inactive` and returns no findings instead of surfacing a package ingestion error.

## Decisions made

- Added explicit family applicability to `g3rs-garde-types`:
  - `Inactive`
  - `Active`
- Rejected app-side gating.
  - Reason: the app must stay thin and only invoke families.
  - Family-local applicability belongs in the family package.
- Kept `Cargo.toml` as a required root input for applicability.
  - Reason: the package must still fail closed on unreadable or malformed root cargo state.
- Made `guardrail_toml` optional in source inputs.
  - Reason: inactive inputs must be representable honestly without synthetic placeholder files.

## Key files for context

- `packages/rs/garde/g3rs-garde-types/src/lib.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/src/run.rs`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/src/run.rs`
- `apps/guardrail3-rs/crates/io/outbound/packages/src/families/garde.rs`

## Next steps

- Keep `deps` signaling on missing `guardrail3-rs.toml`; that is intentional package policy.
- If more families need conditional applicability, use the same package-owned `Inactive` pattern instead of pushing decisions into the app.
