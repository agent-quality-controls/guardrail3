# Remove Runtime Shim And Cut Over Toolchain Family

**Date:** 2026-03-25 01:19
**Scope:** `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/runtime_tests.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/src/*`

## Summary
Removed the fake `app` / `domain` compatibility tree from the Rust runtime crate and rewired it to use direct crate dependencies. Promoted the `toolchain` family from a `#[path = ...]` wrapper into a real self-owned family crate with its own source and tests under `families/toolchain/src`.

## Context & Problem
The workspace split had created real crates, but two compatibility layers were still on the live Rust path. `guardrail3-app-rs-runtime` still recreated a monolithic `crate::app` / `crate::domain` tree inside `runtime.rs`, and `guardrail3-app-rs-family-toolchain` was still just a thin wrapper over the old `checks/rs/toolchain` module via `#[path = ...]`. That meant the split looked real at the Cargo level while the runtime still behaved like the old monolith internally.

The immediate goal for this batch was to remove the smallest remaining live shim layers without widening into the much dirtier root CLI and TS compatibility surfaces.

## Decisions Made

### Remove the runtime crate's inline monolith shim
- **Chose:** delete the inline `pub mod domain` / `pub mod app` tree from `runtime.rs` and replace all uses with direct imports from `guardrail3_app_core`, `guardrail3_domain_config`, `guardrail3_domain_report`, and the promoted family crates.
- **Why:** this is the narrowest runtime-path cut that proves the split is real in execution code, not only in manifests and reexports.
- **Alternatives considered:**
  - Keep the shim until CLI/root-facade cleanup — rejected because it leaves the runtime path lying about ownership.
  - Remove the root facade first — rejected because that widens blast radius into many unrelated dirty files.

### Make `toolchain` a real family crate before touching larger families
- **Chose:** copy the `toolchain` family sources into `families/toolchain/src`, rewrite imports to direct crate owners, and make `src/lib.rs` the real family entrypoint.
- **Why:** `toolchain` is small, has clean dependencies, and removes one of the remaining `#[path = ...]` family shims with low risk.
- **Alternatives considered:**
  - Start with `cargo` or `clippy` — rejected because they are larger and still tied to more legacy shared surfaces.
  - Rewrite the old `checks/rs/toolchain` tree in place and reexport it — rejected because that preserves the old ownership lie.

### Leave the legacy `checks/rs/toolchain` files in place for now
- **Chose:** do not delete or rewrite the old module tree in this batch; only stop the family crate from depending on it.
- **Why:** the repo is heavily dirty, and the ownership cut is already achieved once the live family crate no longer path-includes the old files.
- **Alternatives considered:**
  - Delete the old tree immediately — rejected because it risks colliding with unrelated in-flight work and test references.

## Architectural Notes
`guardrail3-app-rs-runtime` now behaves like a real orchestrator crate: it walks the project tree via `guardrail3_app_core`, loads config via `guardrail3_domain_config`, reports through `guardrail3_domain_report`, and dispatches directly into the family crates. The runtime no longer reintroduces a fake monolith namespace internally.

`guardrail3-app-rs-family-toolchain` now owns its own discovery, facts, inputs, rules, and rule tests under its crate `src/`. This is still not the final state for all families, but it is a real ownership cut and a usable specimen for converting other path-shim families one by one.

## Information Sources
- `apps/guardrail3/crates/app/rs/runtime.rs` — showed the inline `app` / `domain` shim still on the runtime path.
- `apps/guardrail3/crates/app/rs/Cargo.toml` — confirmed the runtime crate already had direct dependencies on the promoted crates.
- `apps/guardrail3/crates/app/rs/families/toolchain/src/lib.rs` — showed the existing `#[path = "../../../checks/rs/toolchain/mod.rs"]` shim.
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/*` — source of the real family logic and tests moved into the crate.
- `.worklogs/2026-03-25-004019-runtime-applicability-and-rs-ast-split.md` — prior split batch that removed the AST backedge and set up the next runtime-path cleanup.
- `.worklogs/2026-03-25-011145-coverage-baselines-off-legacy-validate.md` — prior split batch that removed the clippy/deny coverage backedges.

## Open Questions / Future Considerations
- The root facade in `apps/guardrail3/crates/lib.rs` is still wide and still makes it easy for tests and CLI code to keep importing through compatibility paths.
- Most family crates are still `#[path = ...]` wrappers over the old `checks/**` tree. `toolchain` is now a real owner, but the rest still need the same treatment.
- The legacy `checks/rs/toolchain` files are now duplicate, unused ownership. They should eventually be removed after callers and tests no longer depend on that path.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/runtime.rs` — Rust runtime orchestration and family dispatch; now imports direct crate owners.
- `apps/guardrail3/crates/app/rs/runtime_tests.rs` — runtime applicability tests and the direct report import pattern.
- `apps/guardrail3/crates/app/rs/Cargo.toml` — runtime crate dependency graph showing the direct crate owners.
- `apps/guardrail3/crates/app/rs/families/toolchain/src/lib.rs` — real toolchain family entrypoint after cutover.
- `apps/guardrail3/crates/app/rs/families/toolchain/src/discover.rs` — family-local discovery now owned by the family crate.
- `apps/guardrail3/crates/app/rs/families/toolchain/src/inputs.rs` — minimal rule input for toolchain checks.
- `apps/guardrail3/crates/app/rs/families/toolchain/src/rs_toolchain_02_channel_and_components.rs` — representative rule file in the new family-owned location.
- `.worklogs/2026-03-25-004019-runtime-applicability-and-rs-ast-split.md` — prior runtime-path cleanup context.
- `.worklogs/2026-03-25-011145-coverage-baselines-off-legacy-validate.md` — prior legacy-validate backedge cleanup context.

## Next Steps / Continuation Plan
1. Repeat the same ownership cut for the next smallest family shim, starting with `cargo` or `fmt`, by copying the real family sources into the crate `src/` and removing the `#[path = ...]` wrapper from its `lib.rs`.
2. Move the shared hook-shell parser out of `checks/hooks/shell.rs` into a real shared owner so the remaining family crates can stop path-including that old tree.
3. Thin the live CLI/product path by switching `main.rs` and `adapters/inbound/cli/*` from root-facade imports to direct crate owners, but keep that as a separate batch because the worktree around CLI is already noisy.
