# Split Rust Runtime And Product Surface

**Date:** 2026-03-30 13:20
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/runtime/**`, `apps/guardrail3/crates/bin/guardrail3/Cargo.toml`, `apps/guardrail3/crates/bin/guardrail3/src/main.rs`, `apps/guardrail3/crates/adapters/inbound/cli/**`, `apps/guardrail3/crates/app/rs/README.md`

## Summary
Moved the Rust runtime from the `app/rs` container root into a real `app/rs/runtime` crate and replaced unconditional family linking with feature-gated family dispatch. Then finished the decoupling at the product layer by adding binary and CLI-adapter features so a release-only build excludes TS, hooks, Rust generation, and coverage-only extras.

## Context & Problem
The repo was using separate family crates, but the build graph was still effectively monolithic. `guardrail3` depended on one runtime crate rooted at `crates/app/rs`, and that runtime crate unconditionally depended on every Rust family crate. Separately, the top-level binary and inbound CLI adapter still unconditionally pulled in `app-ts`, `app-hooks`, `app-rs-generate`, and coverage-only family crates. The result was that `--family release` was only a runtime choice, not a compile-time isolation boundary.

The user explicitly wanted the good architecture, not a narrow workaround. That meant fixing both layers:
- the Rust runtime boundary itself
- the product boundary above it

## Decisions Made

### Move the runtime crate under `app/rs/runtime`
- **Chose:** Promote `apps/guardrail3/crates/app/rs/runtime` as the real `guardrail3-app-rs-runtime` crate and remove `apps/guardrail3/crates/app/rs/Cargo.toml`.
- **Why:** `app/rs/` is a namespace/container. Keeping the runtime package at the container root blurred ownership and made the directory layout lie about the intended architecture.
- **Alternatives considered:**
  - Keep `app/rs/Cargo.toml` in place and only add features there — rejected because it preserved the collapsed container/runtime boundary.
  - Remove the runtime crate concept entirely — rejected because the system still needs one orchestrator crate to build shared scope, select families, dispatch, and assemble reports.

### Make family inclusion a compile-time runtime concern
- **Chose:** Add per-family Cargo features to `guardrail3-app-rs-runtime` and dispatch through a feature-gated registry/runners layer.
- **Why:** Separate family crates only matter operationally if the runtime does not link all of them unconditionally. The registry/runners split keeps the runtime thin and turns family inclusion into a real compile-time boundary.
- **Alternatives considered:**
  - Keep one large `match` with all families imported directly — rejected because it leaves unrelated family compile failures on the hot path.
  - Split into separate binaries immediately — rejected because the repo still wants one CLI and one report model.

### Decouple the product surface, not just the runtime
- **Chose:** Add binary/CLI product features for `product-ts`, `product-hooks`, `product-rs-generate`, `product-coverage-clippy`, and `product-coverage-deny`.
- **Why:** After the runtime split, a release-only build still linked TS, hooks, generation, and coverage helpers through `guardrail3` and `adapters/inbound/cli`. The runtime refactor alone was not enough.
- **Alternatives considered:**
  - Stop after the runtime feature work — rejected because it would still compile unrelated product crates in a release-only binary.
  - Hide unsupported commands only at runtime — rejected because the point was compile-time decoupling, not better error strings alone.

### Keep the full product as the default build
- **Chose:** Preserve full capability in default features while allowing lean builds through `--no-default-features`.
- **Why:** The repo still ships one primary CLI, and the user did not ask to make the default product smaller or remove commands from normal builds.
- **Alternatives considered:**
  - Make lean builds the default — rejected because it would silently change the normal shipped product surface.

## Architectural Notes
- `apps/guardrail3/crates/app/rs/runtime` is now the real Rust orchestrator crate.
- `runtime/src/lib.rs` owns shared scope build, config loading, family selection, mapper construction, dispatch, and report assembly.
- `runtime/src/runners.rs` owns family-specific adapter calls.
- `runtime/src/registry.rs` owns “compiled into this build” lookup.
- The runtime now isolates Rust family compilation.
- The binary and CLI adapter now isolate TS/hooks/generation/coverage compilation.
- A release-only build now excludes:
  - `guardrail3-app-ts`
  - `guardrail3-app-hooks`
  - `guardrail3-app-rs-generate`
  - `guardrail3-app-rs-family-clippy`
  - `guardrail3-app-rs-family-deny`

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md`
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md`
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/Cargo.toml`
- `cargo tree --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 --no-default-features --features family-release -e normal`

## Open Questions / Future Considerations
- Older handoff and test-hardening docs under `.plans/` still mention `apps/guardrail3/crates/app/rs/runtime.rs` and `apps/guardrail3/crates/app/rs/Cargo.toml`.
- The runtime now supports compile-time family isolation, but the runner layer still lives as modules inside the runtime crate; it could later become separate adapter crates if the team wants even cleaner ownership.
- Full family-by-family rollout is structurally available now, but only `release` has been exercised end-to-end as the pilot path in this session.

## Key Files for Context
- `apps/guardrail3/Cargo.toml` — workspace membership; now points at `crates/app/rs/runtime`
- `apps/guardrail3/crates/app/rs/runtime/Cargo.toml` — runtime crate features and optional family deps
- `apps/guardrail3/crates/app/rs/runtime/src/lib.rs` — thin Rust orchestrator
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — per-family runner adapters
- `apps/guardrail3/crates/app/rs/runtime/src/registry.rs` — compiled-family lookup
- `apps/guardrail3/crates/bin/guardrail3/Cargo.toml` — product-level feature forwarding
- `apps/guardrail3/crates/bin/guardrail3/src/main.rs` — top-level command gating and dispatch
- `apps/guardrail3/crates/adapters/inbound/cli/Cargo.toml` — CLI adapter feature ownership
- `apps/guardrail3/crates/adapters/inbound/cli/cli.rs` — gated command surface
- `apps/guardrail3/crates/app/rs/README.md` — updated architecture note for the runtime location
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — plan that motivated the `app/rs/runtime` boundary

## Next Steps / Continuation Plan
1. Update the stale `.plans/**` handoff docs that still point at `apps/guardrail3/crates/app/rs/runtime.rs` and `apps/guardrail3/crates/app/rs/Cargo.toml`.
2. Move the next Rust family through the same lean-build proof path, then parallelize the remaining family runner verification with agents.
3. Decide whether runner adapters should remain runtime modules or become separate crates once more families are migrated and the registry shape stabilizes.
