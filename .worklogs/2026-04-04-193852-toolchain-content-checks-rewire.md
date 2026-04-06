# Toolchain Content Checks Rewire

**Date:** 2026-04-04 19:38
**Scope:** `packages/g3rs-toolchain-config-checks/**`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/**`, `apps/guardrail3/Cargo.lock`

## Summary
Created the extracted `g3rs-toolchain-config-checks` package for toolchain content validation and rewired the in-app toolchain family to delegate `RS-TOOLCHAIN-CONFIG-01` and `RS-TOOLCHAIN-CONFIG-02` into it. The app still owns discovery, placement, legacy-file behavior, and result aggregation; the package now owns pure content semantics for channel/components and MSRV consistency.

## Context & Problem
The active extraction direction is to move family content validation into standalone packages under `packages/` while preserving the app as the orchestrator for discovery and filetree concerns. Toolchain is the smallest realistic proving ground because its content logic is relatively self-contained, but its app family already had strong routing/discovery behavior that should not be duplicated. The goal here was to scaffold a real extracted package in the same style as the parser packages, move only the content rules, and prove that the app can call the package without regressing the existing toolchain family behavior.

## Decisions Made

### Use A Dedicated `g3-*` Content Package Name
- **Chose:** `packages/g3rs-toolchain-config-checks`
- **Why:** The naming needs to be specific about both family and concern boundary. This distinguishes extracted content checks from parser crates and leaves room for a future `g3-toolchain-filetree-checks` if that ever becomes useful.
- **Alternatives considered:**
  - `guardrail3-checks-toolchain` — rejected because it was too generic and did not encode the content-vs-filetree split.
  - `toolchain-checks` — rejected because it lost the project namespace and was too ambiguous.

### Mirror The Multi-Crate Package Shape
- **Chose:** top-level facade crate plus internal `types`, `runtime`, and `assertions` crates.
- **Why:** This matches the emerging `packages/` structure and keeps the public contract separate from the runtime implementation. The `types` crate gives the app one stable input contract instead of depending on runtime internals.
- **Alternatives considered:**
  - single crate package — rejected because it would conflate public contract, runtime logic, and test assertions immediately.
  - no `types` crate — rejected because the user explicitly wanted types exposed and the contract separated cleanly.

### Keep The Extracted Input Narrow
- **Chose:** expose `G3ToolchainContentChecksInput` with `toolchain_rel_path`, parsed `toml::Value`, `cargo_rel_path`, and an extracted `G3CargoRustVersion` state enum.
- **Why:** The extracted rules only need toolchain TOML plus Cargo rust-version state. They do not need discovery objects, route objects, filesystem access, or even profile today. Narrowing the input keeps the boundary honest and prevents the package from silently inheriting app responsibilities.
- **Alternatives considered:**
  - pass through the current app `ToolchainRootInput` shape — rejected because it would bake app routing and parse-failure concerns directly into the package contract.
  - include profile now — rejected because the migrated rules do not use it yet, and adding unused contract surface would be premature coupling.

### Preserve App Ownership For Non-Content Semantics
- **Chose:** leave `RS-TOOLCHAIN-01` and `RS-TOOLCHAIN-04` in the app family and bridge only `RS-TOOLCHAIN-CONFIG-01` and `RS-TOOLCHAIN-CONFIG-02`.
- **Why:** existence, same-root legacy shadowing, and all route/discovery concerns are filetree semantics. The package should not rediscover files or decide ownership; it should only validate content handed to it.
- **Alternatives considered:**
  - move the full family immediately — rejected because it would duplicate routing/discovery logic inside the package and violate the extraction boundary.
  - leave local copies of rules 02/03 in the app and add the package in parallel — rejected because it would create split ownership and likely drift.

### Convert Results At The Bridge
- **Chose:** convert `GrdzCheckResult` from the new package into the app’s existing `CheckResult` in the toolchain runtime `run.rs`.
- **Why:** This is the pragmatic migration step that proves end-to-end integration without forcing a global report-model refactor first.
- **Alternatives considered:**
  - replace app `CheckResult` everywhere now — rejected because that is a broader migration than the toolchain extraction itself.
  - duplicate toolchain rule formatting locally and only use the package for hidden helpers — rejected because it would defeat the point of extraction.

## Architectural Notes
The new package boundary is:
- package owns pure content semantics for `RS-TOOLCHAIN-CONFIG-01` and `RS-TOOLCHAIN-CONFIG-02`
- app owns route-bounded discovery, policy-root selection, parse-failure routing, and local non-content rules
- bridge happens in `apps/guardrail3/.../toolchain/.../run.rs`

This keeps the app structure intact:
- `discover.rs` and `facts.rs` still gather route-bounded toolchain/Cargo state
- `run.rs` now transforms that state into the new package input only when a modern toolchain file is actually available and not shadowed by a same-root legacy file
- package results are converted back into app report results at the call boundary

The extracted package also adds direct rule-side tests that the current app family lacked. That improves confidence in the content semantics independently of the app routing tests.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-04-04-150600-session3-handoff.md`
- `packages/rustfmt-toml/Cargo.toml`
- `packages/guardrail3-check-types/crates/guardrail3-check-types/src/*`
- `apps/guardrail3/crates/app/rs/families/toolchain/README.md`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/run.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/facts.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components/rule.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule.rs`

## Open Questions / Future Considerations
- The app toolchain family still carries the old local `rule.rs` files for 02 and 03 in the tree even though the package now owns that behavior. They are no longer compiled through the family, but a cleanup pass should decide whether to delete them or keep them temporarily as migration history.
- The bridge currently converts package results back into app `CheckResult`. If more families migrate soon, a shared conversion helper or broader report-model convergence will become worthwhile.
- Toolchain content checks do not use profile yet. If future toolchain policy becomes profile-sensitive, the package input should grow only when a real rule needs it.

## Key Files for Context
- `packages/g3rs-toolchain-config-checks/Cargo.toml` — package workspace and facade setup
- `packages/g3rs-toolchain-config-checks/crates/types/src/lib.rs` — public extracted input contract
- `packages/g3rs-toolchain-config-checks/crates/runtime/src/run.rs` — package entrypoint
- `packages/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_components/rule.rs` — extracted channel/components logic
- `packages/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule.rs` — extracted MSRV consistency logic
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/run.rs` — app-side bridge and result conversion
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs` — route-bounded toolchain/Cargo facts collection that still stays in-app
- `.worklogs/2026-04-04-193852-toolchain-content-checks-rewire.md` — this worklog

## Next Steps / Continuation Plan
1. Decide whether to delete the now-retired in-app `rs_toolchain_config_01_channel_components/rule.rs` and `rs_toolchain_config_02_msrv_consistency/rule.rs` source files or keep them temporarily during extraction rollout.
2. Add app-level integration coverage proving that the bridge path and the extracted package stay semantically aligned for malformed toolchain TOML, legacy-shadow suppression, and Cargo rust-version blocker cases.
3. Use the same extraction pattern for the next workspace-local family, likely one of `deny` or `clippy`, but keep the boundary strict: discovery stays in-app, content-only rules move.
