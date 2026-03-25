# App Commands Owner Cutover

**Date:** 2026-03-25 10:26
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/commands`, `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs`, `apps/guardrail3/crates/adapters/inbound/cli/modules_cmd.rs`, `apps/guardrail3/crates/adapters/inbound/cli/generate.rs`, `apps/guardrail3/crates/adapters/inbound/cli/check.rs`, `apps/guardrail3/crates/adapters/inbound/cli/map.rs`, `apps/guardrail3/crates/main.rs`

## Summary
Wired the new `app/commands` crate into the real workspace and moved the live owners of shared CLI help text, guide output, and canonical Rust command ids onto it. Updated the remaining callers that were still printing stale top-level command names so command text now comes from one explicit owner, which is the Phase 3 plan target.

## Context & Problem
The workspace split had already promoted the Rust families and runtime crates, but shared command text was still split across the old CLI help generator and `domain/modules/guide.rs`. Several live callsites also still emitted wrong commands such as `guardrail3 show-module`, `guardrail3 generate`, and `guardrail3 init`, even though the real CLI surface is `guardrail3 rs ...`. The crate-split plan explicitly calls out `crates/app/commands` as the owner for Rust namespace / command text, so leaving those strings scattered would keep the split half-done and preserve easy drift.

## Decisions Made

### Make `app/commands` the live owner instead of adding another shim
- **Chose:** Wire the already-created `crates/app/commands` crate into the workspace and import it directly from CLI/product-entry callers.
- **Why:** The plan requires one explicit owner for command ids and shared user-facing messages. Direct imports reduce live ownership in the old CLI/domain paths instead of adding a facade over another facade.
- **Alternatives considered:**
  - Keep the constants in `help_gen.rs` and `domain/modules/guide.rs` and only copy them into `app/commands` later — rejected because it would leave the new crate unused and preserve drift.
  - Re-export `app/commands` back through the root facade first — rejected because the current direction is to thin the root facade, not add fresh dependency shortcuts.

### Canonicalize the stale Rust command strings while touching the owner
- **Chose:** Add canonical Rust command ids in `src/command_ids.rs` and update the current mismatched callers in `generate.rs`, `check.rs`, `map.rs`, and `modules_cmd.rs`.
- **Why:** The work uncovered multiple incorrect top-level command hints. Leaving them behind would mean the new owner exists but does not actually control the user-visible command namespace.
- **Alternatives considered:**
  - Only move the long help / guide text and leave the stale command strings for a later cleanup — rejected because the wrong strings are part of the same ownership problem.
  - Centralize every command string in one pass, including TS and all generated-file banner text — rejected for this batch because the plan target here is the Rust-facing namespace owner, not a whole CLI wording rewrite.

### Keep `domain/modules/guide.rs` as legacy debt for now
- **Chose:** Switch the binary entry point to `guardrail3_app_commands::messages::GUIDE_CONTENT` but leave `domain/modules/guide.rs` in place for now.
- **Why:** Moving the live guide owner out of `domain/modules` fixes the product path without creating an inverted `domain -> app` dependency. The remaining file can be drained or deleted in a follow-up cleanup batch.
- **Alternatives considered:**
  - Make `domain/modules` depend on `app/commands` and re-export the guide text — rejected because it would invert layering.
  - Delete `domain/modules/guide.rs` immediately — rejected because other unreviewed callers or tests may still reference it and this batch was intentionally scoped to the live ownership cutover.

## Architectural Notes
`app/commands` now matches the Phase 3 ownership model more closely: one crate owns the Rust-facing command ids and the shared help/guide text, while CLI adapters and the product entrypoint consume that owner directly. This is a small but important split because it removes a class of duplicated CLI strings from the old monolith and makes future CLI crate promotion cleaner. I deliberately did not re-export `app/commands` through `crates/lib.rs`, because that would widen the root facade again.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — Phase 3 owner targets for `app/commands`
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs` — previous live owner of shared help text
- `apps/guardrail3/crates/domain/modules/guide.rs` — previous live owner of generated guide content
- `apps/guardrail3/crates/adapters/inbound/cli/modules_cmd.rs` — stale `list-modules` hint
- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs` — stale `show-module` / `init` hints
- `apps/guardrail3/crates/adapters/inbound/cli/check.rs` and `apps/guardrail3/crates/adapters/inbound/cli/map.rs` — stale `generate` / `init` hints
- `.worklogs/2026-03-25-030719-legacy-hook-entry-cutover.md` — most recent prior split state before continuing upward into CLI/product ownership

## Open Questions / Future Considerations
- `domain/modules/guide.rs` is now legacy duplication and should eventually be removed once all remaining callers are confirmed off it.
- `adapters/inbound/cli` is still a module tree inside the root crate, not a real promoted crate yet.
- Other command/help strings still exist outside this batch, especially TS-facing wording and generated-file banner text.
- The broader test topology problem remains: root integration/unit harnesses still mask some of the compile-time benefit from the crate split.

## Key Files for Context
- `apps/guardrail3/crates/app/commands/src/command_ids.rs` — canonical Rust command ids used by CLI callers
- `apps/guardrail3/crates/app/commands/src/messages.rs` — canonical shared help and guide text
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs` — CLI help now consuming `app/commands`
- `apps/guardrail3/crates/adapters/inbound/cli/modules_cmd.rs` — module CLI now using canonical command ids
- `apps/guardrail3/crates/main.rs` — product entrypoint now using `app/commands` for guide generation
- `apps/guardrail3/Cargo.toml` — workspace member/dependency wiring for `app/commands`
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — active split plan and owner targets
- `.worklogs/2026-03-25-030719-legacy-hook-entry-cutover.md` — previous split checkpoint immediately before this batch

## Next Steps / Continuation Plan
1. Promote the next explicit owner from the plan: `crates/app/rs/generate`, moving Rust write-set ownership out of CLI helpers and away from the root crate.
2. Keep draining live consumers off the root facade and `domain/modules` duplication, starting with any remaining direct users of `guide.rs` and other shared command text.
3. Revisit root tests that still import broad product surfaces for CLI/help behavior and move them onto the smallest owning crate or leave them as explicit product-entry integration tests only.
