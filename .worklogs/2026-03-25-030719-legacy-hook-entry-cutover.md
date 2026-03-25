# Cut Legacy Hook Calls Over To App-Hooks

**Date:** 2026-03-25 03:07
**Scope:** `apps/guardrail3/crates/app/rs/validate/mod.rs`

## Summary
Rewired the remaining legacy Rust validator hook calls to use the new `app-hooks` top-level API. This keeps the old validator path aligned with the thin `guardrail3-app-hooks::check(...)` surface introduced in the previous commit.

## Context & Problem
After thinning `app-hooks`, the live crate API was:
- `guardrail3_app_hooks::check(...)`

But the legacy validator still reached through the old nested path:
- `crate::app::rs::checks::hooks::check(...)`

Leaving those callsites behind would keep one more fake surface alive even though the real hook composition point now lives in `app-hooks`.

## Decisions Made

### Route legacy validation through the real hook crate surface
- **Chose:** update both legacy hook-report callsites in `app/rs/validate/mod.rs` to call `crate::app::hooks::check(...)`.
- **Why:** the root facade already reexports `guardrail3_app_hooks` as `crate::app::hooks`, so this removes the compatibility lie without changing the user-visible product surface.
- **Alternatives considered:**
  - Leave the old nested path until the whole legacy validator is removed — rejected because it would preserve a stale entrypoint after the real owner is already in place.
  - Import `guardrail3_app_hooks` directly here — rejected because the root facade already exposes the right product-facing path.

### Use the direct outbound-traits owner in the same file
- **Chose:** switch `FileSystem` and `ToolChecker` imports in this module from the old root-facade path to `guardrail3_outbound_traits`.
- **Why:** this module was already being touched, and the direct crate owner is the correct dependency edge after the split.
- **Alternatives considered:**
  - Keep the root-facade import for now — rejected because it unnecessarily preserves another facade dependency in an actively edited file.

## Architectural Notes
- This change does not make the legacy validator “current”; it only keeps the compatibility path aligned with the real crate graph.
- The live hook composition boundary is now consistent across:
  - `app-hooks`
  - the thin root facade
  - the legacy Rust validator compatibility path

## Information Sources
- `.worklogs/2026-03-25-030528-app-hooks-thin-api.md` — prior commit that introduced the thin `app-hooks` API.
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — the remaining legacy callsites.

## Open Questions / Future Considerations
- The broader `app/rs/validate/*` compatibility tree still exists and should eventually shrink or disappear as more callers move onto the real runtime/family crates.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — legacy validator orchestrator still in the root crate.
- `.worklogs/2026-03-25-030528-app-hooks-thin-api.md` — hook API thinning commit that this follow-up aligns with.

## Next Steps / Continuation Plan
1. Commit this small alignment patch cleanly with only:
   - `apps/guardrail3/crates/app/rs/validate/mod.rs`
   - this worklog
2. Continue re-syncing the remaining compatibility paths to real crate owners before attempting another larger facade reduction.
