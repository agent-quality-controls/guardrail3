# Unify garde config checks into one input type and one check function

**Date:** 2026-04-06 20:16
**Scope:** g3rs-garde-config-checks, g3rs-garde-config-ingestion, apps/guardrail3 garde family

## Summary
Same fix as toolchain: replaced two split input types and two check functions with one unified `G3RsGardeConfigChecksInput` and one `check` function. Clippy fields are `Option` — clippy ban checks run when present, skip when absent. Updated the ingestion to return the checks input directly instead of its own result struct. Deleted the ingestion result type.

## Key Files
- `packages/g3rs-garde-config-checks/crates/types/src/lib.rs` — unified input type
- `packages/g3rs-garde-config-checks/crates/runtime/src/run.rs` — single check function
- `packages/g3rs-garde-config-ingestion/crates/runtime/src/run.rs` — returns checks input directly
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/run.rs` — updated app caller
