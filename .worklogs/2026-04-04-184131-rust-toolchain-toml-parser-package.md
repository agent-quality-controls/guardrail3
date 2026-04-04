# Normalize rust-toolchain-toml Parser Package

**Date:** 2026-04-04 18:41
**Scope:** `packages/rust-toolchain-toml/`, `packages/rust-toolchain-toml/crates/parser/{runtime,assertions,types}`, root facade/workspace manifests

## Summary
Built `packages/rust-toolchain-toml` into the intended facade package plus internal parser subcrates layout, then normalized it to `crates/parser/{runtime,assertions,types}`. The package now builds cleanly as a workspace, but the repo's current `test` family still reports a discovery-vs-rule mismatch on internal sidecar layout.

## Context & Problem
This session started from the new standalone `rust-toolchain.toml` parser package but the internal structure was in flux and naming was inconsistent. The goal was to make the package arch-shaped: published facade at the package root, internal parser/types split under `crates/`, and then further normalize parser internals to the same subsystem convention used elsewhere in the repo.

The main complication was that the Rust `test` family is currently internally inconsistent. Its discovery code still recognizes `*_tests/mod.rs` sidecars and runtime-owned sibling assertions crates, while parts of `RS-TEST-02` describe a newer `<module>/tests/mod.rs` shape. That mismatch affected every attempt to settle the runtime crate layout.

## Decisions Made

### Facade Package At Root
- **Chose:** Keep `packages/rust-toolchain-toml/` as the publishable facade crate and workspace root.
- **Why:** This matches the intended package architecture for released parser libraries in this repo: users depend on the outer facade only, while implementation is split underneath `crates/`.
- **Alternatives considered:**
  - Virtual workspace root only — rejected because it bypasses the facade-package architecture.
  - Single flat crate at package root — rejected because the chosen architecture intentionally splits facade/runtime/types.

### Normalize Internal Layout To `crates/parser/{runtime,assertions,types}`
- **Chose:** Move parser implementation to `crates/parser/runtime`, keep reusable assertion helpers in `crates/parser/assertions`, and move shared parsed models to `crates/parser/types`.
- **Why:** This matches the subsystem layout used in existing family crates and avoids the asymmetry of `crates/parser/src/parser` next to a top-level `crates/types`.
- **Alternatives considered:**
  - Leave runtime crate at `crates/parser` and types at `crates/types` — rejected because it left the package half-normalized.
  - Introduce additional nesting under `runtime/src/runtime` — rejected because it created unnecessary stutter.

### Keep Sidecar Tests As `parser_tests` For Now
- **Chose:** Restore and keep `runtime/src/parser_tests/` as the owned sidecar directory for `runtime/src/parser.rs`.
- **Why:** The live discovery code in the `test` family still looks for `*_tests/mod.rs` directories. Renaming the sidecar to plain `tests/` made the package more aligned with one rule implementation, but less aligned with actual discovery.
- **Alternatives considered:**
  - Switch to `runtime/src/parser/tests/` — rejected for now because current discovery still keys off `*_tests`.
  - Inline test modules — rejected because that violates the intended sidecar model and is not the long-term target.

### Make Assertions Module Follow The Owned Module Name
- **Chose:** Name the assertions helper module `parser.rs` inside the assertions crate.
- **Why:** The owned semantic assertions are about the runtime parser module, so `parser.rs` is the coherent module name.
- **Alternatives considered:**
  - Keep `parse.rs` — rejected because it mismatched the owned runtime module name and confused proof-site detection.

## Architectural Notes
The package now has three distinct layers:

- Facade crate at package root: exported dependency surface for consumers.
- Runtime parser crate: parsing entrypoints, filesystem boundary, runtime-local sidecar tests.
- Shared types crate: parsed `RustToolchainToml` model and section types, marked `shared = true` for sibling reuse.

This keeps the external API narrow while still allowing the parser implementation and shared model to evolve independently underneath the facade.

The remaining design tension is not inside this package but inside the current `test` family:
- `discover/components.rs` still discovers `*_tests` sidecars and runtime-owned sibling assertions crates.
- `RS-TEST-02` text and part of its logic now describe `<module>/tests/`.
- `RS-TEST-03` still expects assertions as a sibling of the runtime component root it discovers.

So the package can currently satisfy one branch of the family but not the whole combined rule set at once without changing guardrail code.

## Information Sources
- `.worklogs/2026-04-04-150600-session3-handoff.md` — current extraction direction and parser-first toolchain plan.
- `packages/rust-toolchain-toml/` — the evolving parser package itself.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover/components.rs` — current sidecar/assertions discovery behavior.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_02_owned_sidecar_shape/rule.rs` — current owned-sidecar rule implementation.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_03_runtime_assertions_split/rule.rs` — current runtime/assertions split expectations.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/` — existing sibling-crate subsystem convention.

## Open Questions / Future Considerations
- Reconcile the `test` family so discovery and `RS-TEST-02` agree on one internal sidecar shape.
- Decide whether runtime-owned assertions should live under `crates/parser/assertions` or `crates/parser/runtime/assertions`; current repo logic still expects the latter when the component root is the runtime crate.
- Once the package architecture is settled, extract the actual toolchain content checks package on top of this parser/types split.

## Key Files for Context
- `packages/rust-toolchain-toml/Cargo.toml` — facade crate + workspace root wiring.
- `packages/rust-toolchain-toml/src/lib.rs` — consumer-facing facade exports.
- `packages/rust-toolchain-toml/crates/parser/runtime/Cargo.toml` — runtime parser crate manifest and dev-dependency wiring.
- `packages/rust-toolchain-toml/crates/parser/runtime/src/parser.rs` — parser entrypoints and sidecar attachment point.
- `packages/rust-toolchain-toml/crates/parser/runtime/src/parser_tests/mod.rs` — restored sidecar test entrypoint.
- `packages/rust-toolchain-toml/crates/parser/assertions/src/parser.rs` — reusable semantic proof helpers for parser tests.
- `packages/rust-toolchain-toml/crates/parser/types/src/rust_toolchain_toml.rs` — parsed whole-file and section types.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover/components.rs` — current source of truth for sidecar/assertions discovery.
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/structure/rs_test_02_owned_sidecar_shape/rule.rs` — current sidecar rule mismatch.
- `.worklogs/2026-04-04-150600-session3-handoff.md` — upstream architectural context for extracting content checks.

## Next Steps / Continuation Plan
1. Decide whether to keep the package as-is and tolerate current `test`-family failures until guardrail reconciliation, or to change guardrail `test` family discovery/rules first.
2. If the package should be made fully green under `test`, reconcile `discover/components.rs`, `RS-TEST-02`, and `RS-TEST-03` around one owned-sidecar/assertions shape before changing the package again.
3. After the package shape is locked, build the first extracted toolchain content-checks package against `rust-toolchain-toml` facade/types rather than raw TOML.
