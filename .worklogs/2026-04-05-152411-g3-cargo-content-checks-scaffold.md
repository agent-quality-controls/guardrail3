# Scaffold g3-cargo-content-checks

**Date:** 2026-04-05 15:24
**Scope:** `packages/g3-cargo-content-checks/`, `.plans/2026-04-04-142819-family-checks-packages.md`

## Summary
Scaffolded the new `g3-cargo-content-checks` package in the same facade/runtime/assertions/types shape as the existing extracted checks packages, but intentionally stopped at the package boundary and input contract. The package currently has a stub runtime and no app wiring; the main deliverable is the locked split between cargo content rules and app-owned structural routing.

## Context & Problem
The user asked for the next extracted family package to be `g3-cargo-content-checks`, with the same discipline already established for `fmt` and the emerging `clippy` split:

- package owns parsed-file content validation only
- app owns discovery, coverage, and parse-failure routing
- do not wire blindly before the boundary is clear

The existing cargo family runtime mixes:
- parsed manifest policy rules
- missing-member reporting
- malformed-input fail-closed behavior
- legacy `guardrail3.toml` parsing

Before extracting rule code, the package needed a stable contract saying what belongs inside the package and what must remain app-owned.

## Decisions Made

### Split Cargo Rules Into Content vs Structural Ownership
- **Chose:** Keep these in the package:
  - `RS-CARGO-01..09`
  - `RS-CARGO-11..13`
  - `RS-CARGO-15`
- **Why:** These rules are fundamentally about the semantics of parsed Cargo manifests plus small normalized policy inputs such as profile/waiver context.
- **Alternatives considered:**
  - Put all `RS-CARGO-*` rules into the package — rejected because `RS-CARGO-10` and `RS-CARGO-14` are still structural/discovery ownership, not parsed-content checks.
  - Keep cargo entirely app-side until a full migration is ready — rejected because the user explicitly wanted the family package scaffold and boundary locked first.

### Keep `RS-CARGO-10` And `RS-CARGO-14` In The App
- **Chose:** Leave `RS-CARGO-10` missing-member reporting and `RS-CARGO-14` input-failure routing in the app layer.
- **Why:** Both rules depend on discovery/coverage/fail-closed behavior rather than on already-parsed content. Moving them into the package would force the package to own structural tree semantics.
- **Alternatives considered:**
  - Fake these as package rules with optional/malformed inputs — rejected because that weakens the clean app/package boundary and makes the package own parse-routing concerns.

### Normalize Cargo Policy Inputs Instead Of Depending On App Domain Config
- **Chose:** The package input uses package-local normalized policy data:
  - `policy_profile`
  - `lint_allow_waivers`
  instead of depending directly on `guardrail3-domain-config::GuardrailConfig`.
- **Why:** A publishable checks package under `packages/` should not depend on app-domain config internals from `apps/guardrail3`. The app can parse whichever root-local policy file exists and normalize only the fields cargo content rules actually need.
- **Alternatives considered:**
  - Depend on `GuardrailConfig` directly — rejected because it couples the extracted package back to legacy app schema and defeats the purpose of a narrow package boundary.
  - Omit profile/waiver inputs entirely — rejected because `RS-CARGO-03`, `RS-CARGO-12`, `RS-CARGO-13`, and `RS-CARGO-15` clearly need policy context.

### Scaffold First, Leave Runtime As A Stub
- **Chose:** `check()` returns an empty result vector for now, with README/TODO documenting the rule split and next extraction step.
- **Why:** The user explicitly asked not to wire everything blindly and to lock the package input contract first.
- **Alternatives considered:**
  - Copy a large chunk of legacy cargo rule code immediately — rejected because that would blur whether the boundary itself is correct before extraction starts.

## Architectural Notes
The new package mirrors the existing extracted checks package shape:

- root facade crate
- `crates/types` for the package boundary contract
- `crates/runtime` for `check(...)`
- `crates/assertions` for future package-local tests

The package currently consumes:
- `cargo-toml-parser::CargoToml`
- package-local normalized cargo policy profile/waiver structs

It deliberately does **not** own:
- workspace discovery
- missing-member detection
- parse failure reporting
- app-domain config parsing

That matches the project’s direction that extracted family packages should accept already-selected parsed inputs and not reimplement app-level routing.

## Information Sources
- `AGENTS.md` — current repo rules, worklog discipline, and Rust-only direction
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — extracted-check architecture and rule/input discipline
- `.plans/2026-04-04-142819-family-checks-packages.md` — existing family checks package plan note
- `packages/g3-fmt-content-checks/` — specimen facade/runtime/assertions/types package shape
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/run.rs` — current cargo rule fan-out
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/facts.rs` — current cargo fact model
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/inputs.rs` — current cargo rule input shapes
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/workspace_policy/*.rs`
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/member_policy/*.rs`

## Open Questions / Future Considerations
- The package still has no extracted rule modules. The next step is deciding the first small extraction set, likely starting with a workspace-root rule cluster rather than trying to migrate all package-owned cargo rules at once.
- The package plan still talks about `guardrail3.toml`-derived policy because the app runtime is not yet on the future `guardrail3-rs.toml` flow. The package boundary is already normalized enough that the app can translate either legacy or future config into the same package-local profile/waiver types.
- If cargo content rules later need more normalized policy data, add that as package-local types instead of importing app-domain config wholesale.

## Key Files for Context
- `packages/g3-cargo-content-checks/Cargo.toml` — package facade/workspace shape
- `packages/g3-cargo-content-checks/crates/types/src/lib.rs` — locked package input contract
- `packages/g3-cargo-content-checks/crates/runtime/src/run.rs` — current stub runtime entrypoint
- `packages/g3-cargo-content-checks/README.md` — package boundary summary
- `packages/g3-cargo-content-checks/TODO.md` — next extraction target and rule split
- `.plans/2026-04-04-142819-family-checks-packages.md` — updated family checks package boundary note
- `packages/g3-fmt-content-checks/crates/types/src/lib.rs` — specimen extracted content-package contract
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/run.rs` — source rule inventory and current app-side ownership

## Next Steps / Continuation Plan
1. Commit only the new `g3-cargo-content-checks` package, the package plan note update, and this worklog without sweeping in unrelated clippy-family edits.
2. Run an adversarial test-attack pass against the scaffold boundary itself:
   - verify every `RS-CARGO-*` rule really belongs on the chosen side of the split
   - verify the package contract contains all content-owned policy data and no app-owned structural data
   - verify the new package has no accidental app/runtime coupling
3. After that review, extract the first small rule set into `g3-cargo-content-checks-runtime` rather than migrating the entire cargo family in one shot.
