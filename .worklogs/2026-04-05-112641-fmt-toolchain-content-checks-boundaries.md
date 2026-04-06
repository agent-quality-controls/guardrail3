# Extract FMT Content Checks And Correct Toolchain Package Boundaries

**Date:** 2026-04-05 11:26
**Scope:** `packages/g3rs-fmt-config-checks/**`, `packages/g3rs-toolchain-config-checks/**`, `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/**`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/**`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/assertions/src/rs_toolchain_01_exists.rs`, `apps/guardrail3/Cargo.lock`, `packages/g3rs-toolchain-config-checks/Cargo.lock`

## Summary
Extracted `fmt` content validation into the new `g3rs-fmt-config-checks` package and rewired the app `fmt` family to call it for `RS-FMT-CONFIG-01`, `RS-FMT-CONFIG-02`, `RS-FMT-CONFIG-03`, and `RS-FMT-CONFIG-04`. Corrected the previously extracted `g3rs-toolchain-config-checks` boundary so the package receives full parsed files instead of scoped Cargo state, and normalized both packages so internal rule functions take direct parameters rather than internal `*Input` structs.

## Context & Problem
The family-extraction direction is to move content-only validation into standalone packages under `packages/` while keeping the app as the orchestrator for discovery, authoritative-file selection, missing/parse blockers, and filetree semantics. Toolchain had already been extracted once, but that first cut still passed scoped `cargo_rust_version` state into the package, which violated the agreed rule that content-check packages receive parsed files, not derived substate. Separately, `fmt` needed its first real extraction pass using the new parser packages and the same content-only boundary.

During the same discussion, the internal rule layering was clarified too: a package may expose typed checker inputs, but rule functions themselves should not grow a second layer of private `FooRuleInput` structs. That internal indirection was starting to drift toward the same kind of oversized type jungle the extraction is supposed to prevent.

## Decisions Made

### Extract `fmt` As A Content-Only Package With One Typed Checker Input
- **Chose:** build `packages/g3rs-fmt-config-checks/` and give it one typed public input containing parsed `RustfmtToml`, `CargoToml`, and `RustToolchainToml`, plus their authoritative relative paths.
- **Why:** `fmt` content rules need those three parsed files semantically, while the app still owns deciding which files are authoritative and whether they exist or parse. One checker input keeps the package boundary stable as rules grow, without pushing discovery into the package.
- **Alternatives considered:**
  - Keep `fmt` entirely in-app for now — rejected because `fmt` is small enough to be a clean extraction specimen and the parser layer already exists.
  - Pass only scalar facts like edition/channel — rejected because the user explicitly wanted packages to receive the parsed files they need, not scoped substate.

### Keep `fmt` Filetree And Waiver Semantics In The App
- **Chose:** extract only `RS-FMT-CONFIG-01`, `RS-FMT-CONFIG-02`, `RS-FMT-CONFIG-03`, and `RS-FMT-CONFIG-04`; keep `RS-FMT-01`, `RS-FMT-05`, `RS-FMT-07`, and `RS-FMT-08` in the app.
- **Why:** existence, override placement, dual-config conflicts, and `guardrail3.toml` waiver matching are orchestrator/filetree concerns rather than pure config-content checks.
- **Alternatives considered:**
  - Move `RS-FMT-07` immediately too — rejected because it depends on app-side guardrail waiver loading, not just `rustfmt.toml` semantics.
  - Move the whole family — rejected because that would blur the orchestrator/content boundary on the first extraction.

### Correct Toolchain Package Inputs To Full Parsed Files
- **Chose:** replace `G3CargoRustVersion` with parsed `CargoToml` in `g3rs-toolchain-config-checks`, while keeping app-side parse blockers in the toolchain family.
- **Why:** the agreed package contract is “parsed files in, content checks out.” Missing/invalid Cargo files are app/orchestrator failures; once the package is called, it should see the full parsed file set it needs.
- **Alternatives considered:**
  - Keep the extracted Cargo state enum — rejected because it bakes app-owned parse/blocker semantics into the package boundary.
  - Push Cargo parse failures into the package — rejected because content packages are not supposed to own missing/malformed-file routing.

### Split Toolchain Public Checker Inputs By Rule Ownership
- **Chose:** expose `G3RsToolchainConfigChannelComponentsInput` for `RS-TOOLCHAIN-CONFIG-01` and `G3RsToolchainConfigMsrvConsistencyInput` for `RS-TOOLCHAIN-CONFIG-02`, instead of one aggregate package input.
- **Why:** the two extracted toolchain rules do not actually need the same file set. Splitting the public checker inputs keeps the package contract honest without reintroducing discovery logic.
- **Alternatives considered:**
  - Keep a single aggregate `G3ToolchainContentChecksInput` — rejected because it forced unnecessary Cargo coupling into `RS-TOOLCHAIN-CONFIG-01`.
  - Recreate the old in-app `ToolchainRootInput` in the package — rejected because it mixed filetree and content semantics again.

### Remove Internal Rule-Specific Input Structs
- **Chose:** keep typed checker inputs only at the package API boundary and have internal rule functions take direct parameters.
- **Why:** the user explicitly rejected building a second internal layer of long-lived typed rule-input structs. Direct params preserve rule purity without creating another type jungle inside the package.
- **Alternatives considered:**
  - Keep `FmtSettingsInput`, `FmtNightlyKeysInput`, `FmtEditionMismatchInput`, and similar private structs — rejected because they added indirection without buying architectural clarity.
  - Inline all package input field access directly inside each rule with no helper layer at all — partially rejected; shared helper functions like `cargo_edition()` and `rustfmt_table()` still remain where they genuinely reduce duplication.

### Preserve Toolchain Family Behavior For Invalid Root Cargo Files
- **Chose:** keep root detection in the app even when `Cargo.toml` is semantically invalid, and continue surfacing the `RS-TOOLCHAIN-CONFIG-02` blocker rather than dropping the root entirely.
- **Why:** the first typed-parser rewrite accidentally lost the invalid-root case because the family stopped seeing the root as a workspace. That was a regression in routing semantics, not a package-boundary improvement.
- **Alternatives considered:**
  - Let invalid Cargo manifests silently remove the root from toolchain evaluation — rejected because it suppresses a real blocking error.
  - Move workspace-root inference into the package — rejected because root discovery belongs in the app.

## Architectural Notes
The extraction pattern is now sharper:

- app family:
  - discovers authoritative files
  - parses them once
  - emits missing/parse-blocker results
  - chooses whether to call the package
- content package:
  - receives typed parsed files only
  - runs pure content checks
  - returns `G3CheckResult`/`GrdzCheckResult` values

For `fmt`, the app bridge now constructs `G3RsFmtConfigChecksInput` only when all required parsed files are available. For `toolchain`, the app bridge calls the package in two pieces:

- `RS-TOOLCHAIN-CONFIG-01` gets parsed `RustToolchainToml`
- `RS-TOOLCHAIN-CONFIG-02` gets parsed `RustToolchainToml` plus parsed `CargoToml`

This avoids a false “single aggregate input everywhere” rule while still keeping package APIs typed and explicit.

Internally, both extracted packages now follow the same layering:

- public typed checker input at the package boundary
- shared helper functions where genuinely useful
- rule functions that take direct parameters

That keeps the packages aligned with the app-side rule philosophy: rules should be pure and small, but they do not need a second domain-model layer just to call a function.

## Information Sources
- `AGENTS.md`
- `.worklogs/2026-04-04-193852-toolchain-content-checks-rewire.md`
- `.worklogs/2026-04-04-215523-rename-parser-packages-and-add-cargo-toml-parser.md`
- `.worklogs/2026-04-04-231941-guardrail3-rs-toml-parser.md`
- `.plans/2026-04-04-142819-family-checks-packages.md`
- `.plans/by_family/rs/fmt.md`
- `packages/cargo-toml-parser/**`
- `packages/rustfmt-toml-parser/**`
- `packages/rust-toolchain-toml-parser/**`
- existing app family implementations under `apps/guardrail3/crates/app/rs/families/fmt/**` and `apps/guardrail3/crates/app/rs/families/toolchain/**`

## Open Questions / Future Considerations
- The app-side `fmt` family still carries the current waiver-driven `RS-FMT-07` path. If waiver semantics are later extracted, that should happen only after the waiver model itself is stabilized.
- Other extracted families should follow the same boundary rule immediately: parsed files at the package boundary, direct params at the internal rule boundary.
- `toolchain` still has some local helper/test shape inherited from the older in-app family structure. It now behaves correctly, but there is room for cleanup once more families move and the extracted package patterns stabilize.

## Key Files for Context
- `packages/g3rs-fmt-config-checks/crates/types/src/lib.rs` — public `fmt` checker input contract with parsed file inputs
- `packages/g3rs-fmt-config-checks/crates/runtime/src/run.rs` — `fmt` package entrypoint wiring extracted rules
- `packages/g3rs-toolchain-config-checks/crates/types/src/lib.rs` — corrected toolchain checker input contracts
- `packages/g3rs-toolchain-config-checks/crates/runtime/src/run.rs` — toolchain package entrypoints and parameter fanout
- `packages/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule.rs` — MSRV rule now reads parsed Cargo manifest directly
- `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/run.rs` — app-side `fmt` bridge and blocker ownership
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/run.rs` — app-side `toolchain` bridge and blocker ownership
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/discover.rs` — root selection and invalid-root preservation for toolchain
- `.worklogs/2026-04-04-193852-toolchain-content-checks-rewire.md` — prior toolchain extraction state before this boundary correction

## Next Steps / Continuation Plan
1. Commit and stabilize this `fmt` + corrected `toolchain` extraction state before starting another family, so the package-boundary rule does not drift again.
2. Apply the same pattern to the next config family: package gets parsed files, app owns missing/parse blockers, internal rule functions take direct params.
3. When extracting the next family, decide explicitly whether one aggregate checker input is justified or whether the package should expose multiple checker entrypoints like toolchain now does.
