# Main Direct Crate Imports

**Date:** 2026-03-25 10:41
**Scope:** `apps/guardrail3/crates/main.rs`

## Summary
Moved the product entrypoint off the root facade for the promoted CLI/app/domain owners. `main.rs` now imports the real member crates directly instead of pulling them through `guardrail3::...`.

## Context & Problem
After promoting `adapters/inbound/cli`, the root facade was no longer the implementation owner of the CLI tree, but `main.rs` still consumed most of the system through `guardrail3::{adapters, app, domain}`. That meant the binary entrypoint was still reintroducing the monolithic facade at the top, even though the underlying crates were already split. The next clean-up step was to make the product path use the promoted crates directly.

## Decisions Made

### Import promoted crates directly from `main.rs`
- **Chose:** Replace `guardrail3::{...}` imports with direct imports from `guardrail3_adapters_inbound_cli`, `guardrail3_app_core`, `guardrail3_app_hooks`, `guardrail3_app_rs_runtime`, `guardrail3_app_ts`, `guardrail3_domain_config`, and `guardrail3_domain_report`.
- **Why:** The point of the workspace split is not just to create manifests, but to make real call paths depend on real owners. `main.rs` is the product entrypoint, so it should consume those owners directly where practical.
- **Alternatives considered:**
  - Leave `main.rs` on the root facade until a later giant cleanup — rejected because it would keep the facade on the hottest product path for no good reason.
  - Remove the root re-export compatibility layer at the same time — rejected because that would widen the blast radius unnecessarily; direct product imports and compatibility re-exports can coexist for now.

## Architectural Notes
This is a thinning step, not a new crate promotion. The important outcome is that the binary path now reaches:
- the promoted CLI crate directly
- the promoted app/runtime crates directly
- the promoted domain crates directly

That reduces the architectural importance of `crates/lib.rs` and keeps the root facade closer to a compatibility surface rather than a required transit hub.

## Information Sources
- `apps/guardrail3/crates/main.rs` — product entrypoint before and after the import cutover
- `.worklogs/2026-03-25-104020-inbound-cli-crate-promotion.md` — previous batch that promoted the inbound CLI crate

## Open Questions / Future Considerations
- The root facade still exists for compatibility and tests, so this is not the end of facade cleanup.
- Root-level tests remain a larger source of monolithic coupling than `main.rs` at this point.

## Key Files for Context
- `apps/guardrail3/crates/main.rs` — binary entrypoint now using direct crate imports
- `.worklogs/2026-03-25-104020-inbound-cli-crate-promotion.md` — prerequisite CLI crate promotion worklog

## Next Steps / Continuation Plan
1. Tackle the root test topology next, starting with broad root tests that still import the facade instead of promoted crates.
2. Keep shrinking the remaining live reasons to route through `guardrail3::...`, especially in product-entry and binary-only test code.
