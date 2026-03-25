# Cut Over Test And Hook Family Crates

**Date:** 2026-03-25 03:02
**Scope:** `apps/guardrail3/crates/app/rs/families/hooks-shared`, `apps/guardrail3/crates/app/rs/families/hooks-rs`, `apps/guardrail3/crates/app/rs/families/test`

## Summary
Promoted the remaining shell-dependent Rust family crates from shim wrappers into real crate-owned source trees. `hooks-shared` now owns the shared shell parser substrate, and both `hooks-rs` and `test` consume that crate directly instead of path-including the legacy `checks/hooks/shell.rs` module.

## Context & Problem
After the prior non-shell family sweeps, the remaining Rust families still depended on one legacy seam: `checks/hooks/shell.rs`. `hooks-shared`, `hooks-rs`, and `test` were real workspace members only in the Cargo sense; their source ownership still lived under `checks/hooks/*` and `checks/rs/test`.

That left three problems:
- the last Rust families were still wrapper crates rather than real owners
- the shared hook-shell parser still had no explicit crate owner
- the split still preserved old fake nested module assumptions for shell parsing

The plan called out shell parsing as the remaining shared substrate before the last Rust family cutover. This batch closes that seam rather than creating another temporary compatibility crate.

## Decisions Made

### Make `hooks-shared` the owner of the shell parser substrate
- **Chose:** copy `checks/hooks/shell.rs` and `shell_tests.rs` into `families/hooks-shared/src` as `hook_shell.rs` and `shell_tests.rs`, then expose it from the crate root.
- **Why:** `hooks-shared` is the natural long-term owner for hook-level shared semantics. Both `hooks-rs` and `test` need the parser, and giving it to `hooks-shared` matches the plan’s “shared hook-support substrate” requirement without inventing another crate.
- **Alternatives considered:**
  - Keep path-including `checks/hooks/shell.rs` — rejected because that preserves the exact legacy ownership seam the split is trying to remove.
  - Create a brand new parser-only crate — rejected because it would add extra workspace surface beyond the planned end-state when `hooks-shared` already fits the responsibility.

### Promote the three remaining shell-facing families in one sweep
- **Chose:** copy the full source trees for `checks/hooks/shared`, `checks/hooks/rs`, and `checks/rs/test` into the crate roots under `families/hooks-shared/src`, `families/hooks-rs/src`, and `families/test/src`.
- **Why:** the user explicitly asked to stop doing one-family-at-a-time iterations. These three crates shared the same blocker, so they were the right final broad sweep.
- **Alternatives considered:**
  - Promote `hooks-shared` first and defer `hooks-rs` / `test` to later — rejected because it would leave the same shell seam half-cut and extend the review cycle.
  - Preserve local `mod.rs` wrappers indefinitely — rejected because the split goal is real crate ownership, not just copied code behind another wrapper.

### Rewrite imports onto real crate owners and prune stale manifest baggage
- **Chose:** rewrite copied sources from `crate::domain::report` and fake nested hook-shell paths to direct crate imports, then remove unused wrapper-era dependencies from the manifests.
- **Why:** the copied family crates should reflect the true dependency surface, otherwise `unused-crate-dependencies` keeps proving the crates are still lying about their ownership.
- **Alternatives considered:**
  - Add underscore imports to silence every stale dependency — rejected for `hooks-shared`, `hooks-rs`, and `test` because the extra dependencies were not actually needed after the cutover.
  - Keep fake `domain`/`app` wrapper modules in the new crate roots — rejected because that would just relocate the legacy seam.

### Fix the shell parser instead of weakening extracted tests
- **Chose:** keep the copied `hook_shell` test suite and fix two parser bugs exposed by the move:
  - assignment lines with whitespace only inside quotes were being misclassified as executable commands
  - `if true; then` / `elif true; then` constant-condition control lines were being counted as executable commands
- **Why:** the move copied the parser byte-for-byte, so the failing tests were revealing real behavior, not migration noise. The extracted owner crate is the right place to harden that parser contract.
- **Alternatives considered:**
  - Delete or relax the failing tests during extraction — rejected because that would throw away the most valuable validation in the shared shell substrate.
  - Treat the behavior as acceptable and update expected results — rejected because the parser is intended to recognize executable command context, not constant shell control scaffolding.

### Keep `hooks-rs` on the original recursion boundary
- **Chose:** restore `#![recursion_limit = "1024"]` in the new `hooks-rs` crate root.
- **Why:** the old wrapper crate had that attribute. Smaller values (`256`, `512`) still overflowed in lib-test compilation. Much larger values (`4096`) made compilation impractically slow.
- **Alternatives considered:**
  - Leave the recursion attribute out — rejected because lib-test compilation immediately overflowed.
  - Keep `4096` — rejected because it changed the problem from overflow to an extremely slow compile with no clear benefit.

## Architectural Notes
- `hooks-shared` is now the explicit owner of:
  - shared hook facts / inputs / rules
  - shell parsing (`hook_shell`)
  - shell parser tests
- `hooks-rs` now depends on `guardrail3-app-rs-family-hooks-shared` for parser types/functions instead of path-including the old shell module.
- `test` now depends on `guardrail3-app-rs-family-hooks-shared` for hook-script parsing in its facts collector.
- The copied crate roots were flattened into real `src/lib.rs` owners; the transient copied `src/mod.rs` files were deleted.
- A repo-wide grep after the cutover finds no remaining live `#[path = "...shell.rs"]`, `checks/hooks/shell`, `app::rs::validate`, or `rs_validate` references under the active Rust family/runtime surfaces.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — overall workspace split target and explicit hook-support substrate requirement.
- `.worklogs/2026-03-25-024318-arch-code-garde-hexarch-release-cutover.md` — prior broad family sweep and next-step handoff into the shell tranche.
- `apps/guardrail3/crates/app/rs/checks/hooks/shared/*` — original shared hook family source promoted into `hooks-shared`.
- `apps/guardrail3/crates/app/rs/checks/hooks/rs/*` — original Rust hook family source promoted into `hooks-rs`.
- `apps/guardrail3/crates/app/rs/checks/rs/test/*` — original Rust test family source promoted into `test`.
- `apps/guardrail3/crates/app/rs/checks/hooks/shell.rs` and `apps/guardrail3/crates/app/rs/checks/hooks/shell_tests.rs` — original shell parser substrate that is now owned by `hooks-shared`.

## Open Questions / Future Considerations
- `guardrail3-app-rs-family-hooks-rs --lib` compiles quickly, but its full `--lib` test target remains a heavy compile even after restoring the original recursion boundary. That is now a compile-performance issue, not a structural ownership issue.
- `app/hooks/mod.rs` is still broader than the end-state thin crate surface and still carries a nested compatibility wrapper shape, even though it now composes the family crates directly.
- The old `checks/hooks/*` and `checks/rs/test/*` trees still remain in the repo as compatibility/transition debt. This batch moved ownership, not deletion.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/lib.rs` — real shared hook family crate root.
- `apps/guardrail3/crates/app/rs/families/hooks-shared/src/hook_shell.rs` — shell parser owner and the two parser fixes from this batch.
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/lib.rs` — real Rust hook family crate root with restored recursion boundary.
- `apps/guardrail3/crates/app/rs/families/test/src/lib.rs` — real Rust test family crate root.
- `apps/guardrail3/crates/app/hooks/mod.rs` — current app-hooks integration surface, still worth thinning in a later pass.
- `.worklogs/2026-03-25-024318-arch-code-garde-hexarch-release-cutover.md` — previous sweep that handed off into this one.

## Next Steps / Continuation Plan
1. Commit only the three family crate directories plus this worklog:
   - `apps/guardrail3/crates/app/rs/families/hooks-shared/**`
   - `apps/guardrail3/crates/app/rs/families/hooks-rs/**`
   - `apps/guardrail3/crates/app/rs/families/test/**`
2. Thin `apps/guardrail3/crates/app/hooks/mod.rs` so `app-hooks` stops carrying nested compatibility modules now that the family crates are real owners.
3. Re-sync to the split plan and choose the next broad non-TS structural cuts:
   - `app/commands`
   - `app/rs/generate`
   - any remaining root-facade reductions that still keep the monolith on the hot path
4. Keep root tests moving off the facade / legacy trees so the crate split continues to pay down compile/test bottlenecks instead of only relocating source ownership.
