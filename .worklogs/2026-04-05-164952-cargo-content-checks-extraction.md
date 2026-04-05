# Cargo Content Checks Extraction

**Date:** 2026-04-05 16:49
**Scope:** `packages/g3-cargo-content-checks`, `apps/guardrail3/crates/app/rs/families/cargo`, `apps/guardrail3/Cargo.lock`, `.plans/2026-04-04-142819-family-checks-packages.md`

## Summary
Extracted the first single-file cargo content rules into `g3-cargo-content-checks`, rewired the app cargo family to call the package for those moved rules, and cleaned the app tree so the old root-rule implementations no longer remain live. The cargo family now follows the same boundary used in the other extracted families: structural parse failures stay in the app and typed content checks only run on parsed files.

## Context & Problem
The cargo content-check package had drifted into the wrong architecture. Instead of taking a parsed `Cargo.toml` file, it had started accumulating invented policy/profile/waiver subset types and workspace-level bag inputs. That broke the extraction goal for these packages, which is to validate file content only and leave routing, policy interpretation, and parse failure handling in the app family orchestrator.

The user explicitly called out that the package boundary must be parsed files only, and that content checks are about what is inside a file rather than about cross-workspace orchestration. The cargo family was the only extracted package area still violating that rule.

## Decisions Made

### Rebuilt the cargo package around a single-file contract
- **Chose:** `G3CargoContentChecksInput { cargo_rel_path, cargo }` as the only public input.
- **Why:** Cargo content checks should operate on one parsed `Cargo.toml` at a time. The file itself already tells the package whether it is a workspace root, package root, or both.
- **Alternatives considered:**
  - Workspace root + member manifest bag input — rejected because it turns the package into an orchestrator.
  - Separate public root/member input structs — rejected because the file shape is the same and role can be detected from parsed content.
  - Policy profile / waiver subset structs — rejected because they partialize external files instead of passing parsed files.

### Moved only single-file cargo rules into the package
- **Chose:** move `RS-CARGO-01`, `02`, `05`, `07`, `08`, and `11`.
- **Why:** These are content checks on one `Cargo.toml` file and do not need workspace/member pairing or external policy files.
- **Alternatives considered:**
  - Moving `RS-CARGO-04` too — rejected because it needs workspace/member relationship context and is not a pure single-file content check.
  - Moving `RS-CARGO-03`, `12`, or `15` into the package with subset policy inputs — rejected because those app-side rules still depend on external policy/escape-hatch/profile context not yet represented as full parsed files.

### Kept typed parse rejection structurally owned by `RS-CARGO-14`
- **Chose:** if `cargo-toml-parser` rejects a root `Cargo.toml`, the app records an input failure and does not run remaining root-level semantic rules for that file.
- **Why:** The content package should only receive valid typed parsed files. Malformed schema belongs to the structural/orchestrator layer.
- **Alternatives considered:**
  - Letting moved rules still reason about malformed typed shapes — rejected because it violates the package boundary.
  - Letting app-side semantic rules continue on raw TOML after typed parse failure — rejected because it creates mixed ownership and double-reporting.

### Removed stale app-side cargo rule shells after migration
- **Chose:** cut the moved rule modules and their assertion exports out of the live app tree.
- **Why:** Leaving the old modules compiled or exported creates a false impression that the app still owns those rules and makes future maintenance error-prone.
- **Alternatives considered:**
  - Keeping dead code on disk and referenced in module surfaces — rejected because it already caused compile/dead-code fallout.
  - Leaving old assertion modules exported — rejected because the tree would keep lying about current ownership.

## Architectural Notes
This change reasserts the extracted-family contract already used by `fmt`, `toolchain`, `clippy`, and `deny`:

- app family owns discovery, route selection, parse attempts, and parse-failure reporting
- content package owns only typed file-content validation
- rule functions inside the package remain pure and local
- package boundary uses parsed files, not scoped subsets or bag inputs

For cargo specifically, the package determines how to treat the file by inspecting parsed content (`[workspace]`, `[package]`, etc.) rather than receiving external classification hints.

## Information Sources
- `AGENTS.md` — repo worklog requirements and current family extraction direction
- `.plans/2026-04-04-142819-family-checks-packages.md` — extraction ledger updated for cargo
- `packages/g3-fmt-content-checks` — specimen for extracted content-check package shape
- `packages/g3-clippy-content-checks` — specimen for package/app bridge plus structural parse ownership
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/*` — existing cargo family behavior and tests
- `.worklogs/2026-04-05-145142-clippy-extraction-and-parser-contract-fixes.md` — prior extraction boundary decisions

## Open Questions / Future Considerations
- `RS-CARGO-03`, `12`, and `15` are still app-owned because they depend on policy/escape-hatch/profile context. If they ever move, they should take full parsed external policy files, not subset helper structs.
- `RS-CARGO-04`, `06`, `09`, `10`, `13`, and `14` remain structural or cross-file and should stay app-side unless their architecture changes substantially.
- The remaining dirty tree includes a large deny-package refactor/test migration that should be audited separately before any commit.

## Key Files for Context
- `packages/g3-cargo-content-checks/crates/types/src/lib.rs` — package input contract
- `packages/g3-cargo-content-checks/crates/runtime/src/run.rs` — package entrypoint and moved rule orchestration
- `packages/g3-cargo-content-checks/crates/runtime/src/support.rs` — shared cargo-content helpers
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/run.rs` — app/package bridge for moved cargo rules
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/discover.rs` — root parse failure ownership and typed parse gating
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/member_policy/rs_cargo_14_input_failures/tests/family_bridge.rs` — app-layer bridge smoke tests for moved rules
- `.plans/2026-04-04-142819-family-checks-packages.md` — current cargo move/stay ledger
- `.worklogs/2026-04-05-145142-clippy-extraction-and-parser-contract-fixes.md` — prior extracted-family boundary precedent

## Next Steps / Continuation Plan
1. Audit the remaining dirty tree outside the cargo family, starting with `packages/g3-deny-content-checks`, to determine whether the current deny refactor is coherent enough to commit or still needs structural fixes.
2. If the deny work is sound, create a separate worklog and commit it independently from cargo; do not mix family extractions.
3. If the deny work is not sound, leave it uncommitted and produce a concrete fix list describing which package/runtime/test surfaces still violate the extracted-family architecture.
