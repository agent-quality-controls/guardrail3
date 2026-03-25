# Rust Generate Owner Cutover

**Date:** 2026-03-25 10:36
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/generate`, `apps/guardrail3/crates/adapters/inbound/cli/check.rs`, `apps/guardrail3/crates/adapters/inbound/cli/diff.rs`, `apps/guardrail3/crates/adapters/inbound/cli/generate.rs`, `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`, `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers_tests.rs`, `apps/guardrail3/crates/main.rs`

## Summary
Promoted the Rust write set into a real `crates/app/rs/generate` workspace crate and switched the Rust-facing CLI commands onto it. This also fixed two behavioral mismatches the split exposed: `rs diff` / `rs check` were using the whole-stack generator instead of the Rust-owned artifact set, and `rs hooks-install` was still emitting mixed-stack hook content.

## Context & Problem
The workspace split plan explicitly calls for one explicit owner of the Rust write set under `crates/app/rs/generate` with `owned_artifacts.rs` as the concrete owner file. Before this batch, actual Rust generation still lived in the inbound CLI adapter via `generate_helpers.rs`, and Rust-facing commands depended on whole-project generation surfaces. That meant the ownership boundary was still wrong, and it also created real behavior drift:
- `rs diff` and `rs check` were comparing against mixed-stack output instead of the Rust-owned artifact set.
- `rs hooks-install` used the same mixed-stack hook installer as the TS path, even though the plan says the Rust hook install belongs to the Rust write set and must not emit TS-owned steps.

## Decisions Made

### Promote a real `app/rs/generate` crate instead of keeping CLI-owned planners
- **Chose:** Create `crates/app/rs/generate` with `src/owned_artifacts.rs` and move Rust artifact planning there.
- **Why:** The plan’s next owner after `app/commands` is the Rust write set. The old `generate_helpers.rs` file was still the live owner, so the split was incomplete until that moved.
- **Alternatives considered:**
  - Leave the planner in CLI and only add a facade crate over it — rejected because that would preserve the real ownership in the wrong place.
  - Move only the override loader first and leave artifact planning in CLI — rejected because it would not fix `rs diff` / `rs check` / `rs hooks-install`.

### Make Rust-facing commands consume Rust-owned artifacts only
- **Chose:** Point `rs generate`, `rs diff`, `rs check`, and `rs hooks-install` at `guardrail3-app-rs-generate`.
- **Why:** Those commands are Rust-facing surfaces, so they should consume the Rust write-set owner directly. This also fixes the existing whole-project leakage in `check.rs` and `diff.rs`.
- **Alternatives considered:**
  - Keep `diff.rs` and `check.rs` on the old mixed `generate_expected(...)` and only move `run_rs(...)` — rejected because the user-visible behavior would still violate the plan’s ownership boundary.
  - Move `ts generate` at the same time — rejected because the active roadmap is Rust-only and the plan keeps mixed-stack hook/product surfaces as separate follow-up work.

### Split Rust-only hook install from the mixed-stack hook entrypoint
- **Chose:** Add `run_rs_hooks(...)` for `RsCommands::HooksInstall`, while leaving the existing mixed `run_hooks(...)` entrypoint for the TS side.
- **Why:** `main.rs` was dispatching both `rs hooks-install` and `ts hooks-install` through the same function. If I made that shared function Rust-only, I would silently break the TS command. The clean fix was to split the entrypoints and route Rust to the Rust owner.
- **Alternatives considered:**
  - Keep one shared `run_hooks(...)` and pass a mode flag through CLI — rejected because the point of the cut is to stop CLI from owning Rust write-set semantics.
  - Change both RS and TS commands to Rust-only hook output for now — rejected because that would be a bad regression outside the active scope.

### Keep the Rust hook builder honest even though the shared pre-commit module is still mixed
- **Chose:** Build the Rust-owned hook artifact in `app/rs/generate` from `domain/modules/pre_commit` constants, then strip TS-owned sections before appending Rust duplication checks.
- **Why:** A focused test showed `build_pre_commit_script(true, false)` still emitted TypeScript validation from the shared base script. I wanted a real Rust-only artifact now, without turning this batch into a full `pre_commit.rs` refactor.
- **Alternatives considered:**
  - Relax the test and accept the shared builder output — rejected because the hook content was genuinely wrong for `rs hooks-install`.
  - Fully split `domain/modules/pre_commit.rs` into per-stack segment builders in this batch — rejected because it is a larger cross-cutting change than needed for the current owner promotion.

## Architectural Notes
This batch makes `app/rs/generate` the real owner of the Rust artifact set:
- Rust config planning lives in `src/owned_artifacts.rs`
- local override loading lives in `src/overrides.rs`
- crate-local tests verify both config planning and Rust-only hook output

The inbound CLI adapter is thinner now:
- `check.rs` and `diff.rs` no longer compute Rust expectations through mixed-stack generation
- `generate.rs` keeps TS generation and mixed hook handling, but Rust generation and Rust-only hook content now come from the promoted crate
- the old CLI-local `generate_helpers.rs` owner is deleted

This is also a functional correction, not just a crate move. After the change, the Rust-facing commands align with the plan’s write-set boundary instead of reusing whole-project helpers.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — explicit Phase 3 owner target for `crates/app/rs/generate`
- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs` — prior mixed owner of Rust generation and hook installation
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` — prior owner of Rust artifact planning
- `apps/guardrail3/crates/adapters/inbound/cli/check.rs` and `apps/guardrail3/crates/adapters/inbound/cli/diff.rs` — callers that were still consuming mixed generation
- `apps/guardrail3/crates/domain/modules/pre_commit.rs` — shared hook script constants and builder
- Subagent exploration from Linnaeus during this batch — confirmed the smallest viable cut and the exact blocking callsites before implementation
- `.worklogs/2026-03-25-102654-app-commands-owner-cutover.md` — prior split step immediately before promoting the Rust write-set owner

## Open Questions / Future Considerations
- `generate.rs::run(...)` and `generate_expected(...)` still represent whole-project generation, and there is no current top-level CLI command using them. They remain cleanup debt.
- The shared `pre_commit.rs` module still has mixed-stack ownership baked into the base script. I corrected the Rust-facing artifact at the new owner boundary, but the shared builder still deserves a deeper refactor later.
- `adapters/inbound/cli` is still a module tree in the root crate rather than a promoted crate.
- `domain/modules/guide.rs` remains legacy duplication from the previous batch.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/generate/src/owned_artifacts.rs` — canonical Rust write-set owner and hook-content builder
- `apps/guardrail3/crates/app/rs/generate/src/overrides.rs` — override loading/validation extracted from CLI
- `apps/guardrail3/crates/app/rs/generate/src/owned_artifacts_tests.rs` — focused crate-local tests proving deny profile generation and Rust-only hook output
- `apps/guardrail3/crates/adapters/inbound/cli/generate.rs` — thinner CLI adapter after Rust owner cutover
- `apps/guardrail3/crates/adapters/inbound/cli/check.rs` — Rust check now using Rust-owned expected artifacts
- `apps/guardrail3/crates/adapters/inbound/cli/diff.rs` — Rust diff now using Rust-owned expected artifacts
- `apps/guardrail3/crates/main.rs` — split dispatch between `run_rs_hooks(...)` and mixed `run_hooks(...)`
- `apps/guardrail3/crates/domain/modules/pre_commit.rs` — still the shared script source that the Rust owner now post-processes
- `.worklogs/2026-03-25-102654-app-commands-owner-cutover.md` — prior batch that promoted `app/commands`

## Next Steps / Continuation Plan
1. Keep shrinking the root/CLI surface by promoting the next real owner from the plan, likely `adapters/inbound/cli` or more of `app/rs/validate` depending on the highest remaining monolith edge.
2. Revisit `domain/modules/pre_commit.rs` and split the shared hook builder into explicit Rust-only / TS-only / mixed assembly instead of post-processing the shared base in `app/rs/generate`.
3. Continue moving root tests off the product facade and onto promoted crates, especially around CLI/help/generation behavior where crate-local tests can now prove ownership boundaries directly.
