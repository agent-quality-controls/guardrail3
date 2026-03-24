# Freeze Rust Validation Cutover Spec

**Date:** 2026-03-24 14:56
**Scope:** `.plans/todo/checks/2026-03-24-rust-validation-cutover.md`, `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`, `.plans/todo/checks/2026-03-23-rust-hardening-followups.md`

## Summary
Defined the hard cutover contract for Rust validation so `guardrail3 rs validate` runs only the new family-based checker architecture and no longer routes through the legacy `app/rs/validate` stack. Tightened the architecture and follow-up docs so they point at this cutover spec instead of preserving a fake migration/delegation story.

## Context & Problem
The planning work for Rust families and generator parity was already far ahead of the actual runtime. The CLI still validated through the legacy validator tree, which meant self-validation and user-facing behavior were still driven by coarse grouped domains and old `R*` inventories rather than the new `RS-*` family system. That made the plans disconnected from reality and let obvious structural issues, such as fake hexarch folder boundaries with no real Cargo workspace ownership, slip through the active validator path.

The user explicitly clarified that there is no release/migration compatibility requirement: the new checker architecture was never shipped as the public runtime, so the correct move is a hard cutover to the new family runtime, not a transitional dual-path system.

## Decisions Made

### Rust validate becomes family-based only
- **Chose:** `guardrail3 rs validate` must select and execute only the new Rust families (`fmt`, `toolchain`, `clippy`, `deny`, `cargo`, `code`, `hexarch`, `deps`, `garde`, `test`, `release`, `hooks-shared`, `hooks-rs`).
- **Why:** The new rules and plans are organized by family, and the legacy grouped domains (`architecture`, `tests`, `release`, etc.) preserve the wrong runtime semantics. Keeping grouped domains would keep the old validator model alive under a different skin.
- **Alternatives considered:**
  - Keep grouped flags and translate them to families — rejected because it preserves the stale UX and old conceptual model.
  - Keep a compatibility path through `app/rs/validate` — rejected because the user explicitly does not want a migration layer.

### Legacy Rust validator path is removed from runtime
- **Chose:** The runtime contract now forbids calling `crate::app::rs::validate::run(...)` and removes the old Rust-specific hook validation path from the product surface.
- **Why:** The cutover only means something if the runtime stops depending on legacy logic. Otherwise correctness is still defined by the old stack.
- **Alternatives considered:**
  - Delegate old orchestrator to new families — rejected because it keeps old category/report/schema behavior in the runtime.
  - Keep hooks as a separate validate command — rejected because hooks are now first-class Rust families.

### CLI, config, and report schemas must change together
- **Chose:** The cutover spec explicitly names the grouped-schema surfaces that must be rewritten: `cli.rs`, `validate.rs`, `main.rs`, `domain/config/types.rs`, `domain/report/mod.rs`, plus help/init/guide/test surfaces.
- **Why:** Without naming these files, the spec would be “architecturally right” but still leave old grouped flags, old config keys, and old report domain structs in place.
- **Alternatives considered:**
  - Leave these as implied implementation details — rejected because earlier review already showed that ambiguous cutover specs let stale UX survive.

### Scope filtering applies only to source-file analysis surfaces
- **Chose:** `--staged` / `--dirty` / `--files` / `--commits` narrow only source-file inputs. Root/config/tool/policy/architecture rules continue to run in full.
- **Why:** Families like `code`, `garde`, and `test` are mixed-scope. Treating the whole family as “file-scoped” would silently skip root-owned rules.
- **Alternatives considered:**
  - Mark whole families as scoped or unscoped — rejected because reviewers found that this breaks the actual family contracts.

### Hooks are split but not independent
- **Chose:** `hooks-rs` remains a selectable family but selecting it must also run `hooks-shared`.
- **Why:** `HOOK-RS` depends on `HOOK-SHARED` to establish that an effective hook even exists. Running `hooks-rs` alone can produce false-clean silence.
- **Alternatives considered:**
  - Treat `hooks-rs` as fully standalone — rejected because the implemented family contracts do not support that safely.

## Architectural Notes
This work freezes the runtime contract so it matches the already-adopted checker architecture:
- one `ProjectTree`
- family orchestrators
- family sections
- `RS-*` / `HOOK-*` output only

The new cutover doc is intentionally product-facing, not merely structural. It covers:
- runtime entrypoint
- family inventory
- CLI flag semantics
- config key semantics
- mixed-scope family behavior
- ownership model
- hook dependency behavior
- report naming
- acceptance criteria

The checker architecture doc was also corrected to stop claiming there is still a staged migration/delegation phase. The remaining work after this doc is implementation against the cutover contract, not more architecture design for Rust validation.

## Information Sources
- `apps/guardrail3/crates/adapters/inbound/cli/validate.rs` — proved the CLI still routes to `app::rs::validate::run`
- `apps/guardrail3/crates/app/rs/validate/mod.rs` — showed the legacy grouped Rust runtime path
- `apps/guardrail3/crates/app/rs/checks/rs/*/mod.rs` — verified the real family entrypoints and their required inputs
- `apps/guardrail3/crates/app/rs/checks/hooks/{mod.rs,shared/mod.rs,rs/mod.rs}` — clarified hook-family runtime requirements and dependencies
- `apps/guardrail3/crates/domain/config/types.rs` — showed grouped Rust config keys still exist
- `apps/guardrail3/crates/domain/report/mod.rs` — showed grouped Rust runtime category types still exist
- `apps/guardrail3/crates/main.rs` — showed `HooksValidate`, `ValidateDomains`, and `RustCheckCategories` are still live surfaces
- `.plans/todo/checks/rs/{hexarch,garde,test,release,deps}.md` — used to reconcile family ownership/scope semantics
- adversarial reviewer findings from subagents in this session — used to fix mixed-scope family handling, release/deps ownership, hook dependency semantics, and stale surface coverage

## Open Questions / Future Considerations
- The cutover spec is now planning-complete, but implementation will still need careful handling for family-level config resolution across mixed-scope families like `deps` and `release`.
- `libarch` remains out of the runtime family set until there is an implemented checker module for it.
- There is still a large amount of stale runtime/test/help/init code that must be updated or removed during implementation; this worklog records the contract, not the code change.

## Key Files for Context
- `.plans/todo/checks/2026-03-24-rust-validation-cutover.md` — the new hard runtime cutover contract
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — now points at the cutover spec instead of preserving a migration story
- `.plans/todo/checks/2026-03-23-rust-hardening-followups.md` — now treats CLI/runtime routing as an implementation task under the cutover spec
- `apps/guardrail3/crates/adapters/inbound/cli/validate.rs` — current runtime entrypoint that still routes to legacy Rust validation
- `apps/guardrail3/crates/main.rs` — current grouped CLI/config/report routing surface that must be rewritten
- `apps/guardrail3/crates/domain/config/types.rs` — current grouped Rust config schema
- `apps/guardrail3/crates/domain/report/mod.rs` — current grouped Rust report/category types
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/mod.rs` — exemplar family showing the new runtime should call families directly
- `.worklogs/2026-03-24-132341-freeze-generator-specs.md` — earlier planning context for generator-side exact contract work
- `.worklogs/2026-03-24-134856-freeze-generator-ownership-modes.md` — earlier planning context for generator ownership semantics

## Next Steps / Continuation Plan
1. Implement a new Rust validation runner that selects families directly from `app/rs/checks/**` and returns one family section per family.
2. Replace the grouped Rust CLI/config/report schema with family-based selection:
   - update `cli.rs`
   - update `domain/config/types.rs`
   - update `domain/report/mod.rs`
   - remove grouped Rust routing from `main.rs` / `validate.rs`
3. Update public surfaces and tests in the same pass:
   - `help_gen.rs`
   - `init.rs`
   - `domain/modules/guide.rs`
   - Rust validate CLI tests, config tests, and golden snapshots
4. Remove the legacy Rust runtime path and the separate Rust hook validation command once the new runner is wired and verified.
