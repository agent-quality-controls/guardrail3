# Extract garde AST checks

**Date:** 2026-04-05 22:17
**Scope:** `packages/g3rs-garde-ast-checks`, `apps/guardrail3/crates/app/rs/families/garde/crates/runtime`, `apps/guardrail3/crates/app/rs/families/garde/crates/assertions`, `.plans/2026-04-04-142819-family-checks-packages.md`

## Summary
Built `g3rs-garde-ast-checks` for the remaining source-analysis garde rules and rewired the app garde family to call it. The app keeps malformed-input ownership in `RS-GARDE-10`, while the new package owns AST parsing and the source-rule checks for `RS-GARDE-AST-01/07/08/09/11/12/13/14`.

## Context & Problem
`g3rs-garde-config-checks` already owned the root-policy config checks, but the remaining garde rules still lived in the app runtime and depended on source parsing, cross-file type knowledge, and `query_as!` inventory. The user explicitly rejected oversized orchestrator-derived fact bags as package inputs and also rejected path/root metadata garbage in the package contract. The workable direction was a second garde package that receives explicit files, reads and parses them itself, and emits the AST-related checks.

## Decisions Made

### Built a separate AST package with explicit file inputs
- **Chose:** `g3rs-garde-ast-checks` with one input type:
  - `G3RsAstFile { rel_path, abs_path }`
  - `G3RsGardeAstChecksInput { source_files, guardrail_toml }`
- **Why:** The AST package needs real files it is allowed to analyze, not precomputed facts and not root/mapper metadata. `guardrail3.toml` was made required because `RS-GARDE-AST-04` needs it and the family will need it anyway.
- **Alternatives considered:**
  - Passing app-normalized AST fact bags — rejected because that just moves orchestrator output into the package boundary.
  - Adding repo-root / scope / mapper inputs — rejected because those are app concerns, not package inputs.
  - Keeping `guardrail3.toml` optional — rejected after the user explicitly allowed making it required for this family.

### Moved all remaining AST garde rules into the new package
- **Chose:** Package ownership for `RS-GARDE-AST-01`, `07`, `08`, `09`, `11`, `12`, `13`, `14`.
- **Why:** These rules all operate on governed Rust source files, and `RS-GARDE-AST-04` additionally reads `guardrail3.toml`. Cross-file checks such as nested validated types are handled internally by analyzing the whole `source_files` set inside the package.
- **Alternatives considered:**
  - Moving only the one-file AST rules first — rejected because the package can cleanly analyze the full governed source set and build the cross-file maps internally.
  - Leaving `RS-GARDE-AST-04` in the app — rejected because once `guardrail3.toml` became a required input, the same package could own it honestly.

### Kept malformed-input ownership in the app
- **Chose:** `RS-GARDE-10` remains app-side and the app skips AST package execution for roots whose required `guardrail3.toml` already failed structurally.
- **Why:** The content/AST packages should assume valid inputs. The app already owns source-read failures, source-parse failures, and config-parse failures. Without the skip, malformed `guardrail3.toml` could double-fire with `RS-GARDE-AST-04`.
- **Alternatives considered:**
  - Letting the AST package report malformed `guardrail3.toml` — rejected because that would blur structural failure ownership.
  - Running the AST package anyway and hoping individual rules stand down — rejected because it produces duplicate or contradictory reporting.

### Deleted the dead app-side AST implementations after wiring
- **Chose:** Remove the old garde `derive_checks` and `inventory` rule directories plus their assertion modules once the bridge was green.
- **Why:** Leaving the old app-side rules in place would mislead future agents about what still owns the behavior and would keep stale tests and assertions around.
- **Alternatives considered:**
  - Keeping the old files as historical references — rejected because the repo already has git history and worklogs for that.

## Architectural Notes
The garde family is now split along the same package boundary pattern as the other extracted families:
- `g3rs-garde-config-checks` owns parsed-config rules over root `Cargo.toml` and covering `clippy.toml`.
- `g3rs-garde-ast-checks` owns source-analysis rules over governed Rust files plus required `guardrail3.toml`.
- the app garde family still owns applicability gating and malformed-input reporting through `RS-GARDE-10`.

The AST package parses source files and policy files itself because that is its honest job. This is different from the config content packages, which receive parsed files. The distinction is intentional: source analysis needs a governed file set, not app-derived semantic bundles.

## Information Sources
- `packages/g3rs-garde-config-checks` — existing garde config-package specimen and current garde split.
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/parse/*` — existing source-analysis logic reused inside the new AST package.
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/run.rs` — family bridge point.
- `.plans/2026-04-04-142819-family-checks-packages.md` — extraction ledger updated during this work.
- `.worklogs/2026-04-05-211234-extract-garde-root-policy-content-checks.md` — prior garde package extraction and typed-clippy fallback fix.

## Open Questions / Future Considerations
- `RS-GARDE-AST-04` still depends on legacy `guardrail3.toml`. If the family later migrates fully to `guardrail3-rs.toml`, the AST package input and internal parser calls will need to switch with the family.
- The new package likely still shares the repo-wide package arch/code debt seen in earlier extracted packages, especially public-field warnings and internal-crate dependency complaints.
- The app `facts` module still defines some source-fact structs that are now mostly historical. They are tolerated with `allow(dead_code)` for now, but can be trimmed later if the family no longer needs them for any remaining tests or structural paths.

## Key Files for Context
- `packages/g3rs-garde-ast-checks/crates/types/src/lib.rs` — public AST package contract.
- `packages/g3rs-garde-ast-checks/crates/runtime/src/run.rs` — package orchestration over source files and guardrail policy.
- `packages/g3rs-garde-ast-checks/crates/runtime/src/support.rs` — shared AST analysis and cross-file indexing helpers.
- `packages/g3rs-garde-ast-checks/crates/runtime/src/parse/mod.rs` — source parsing entrypoint reused from the old family implementation.
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/run.rs` — app/package bridge and `RS-GARDE-10` ownership boundary.
- `apps/guardrail3/crates/app/rs/families/garde/crates/runtime/src/root_policy/rs_garde_10_input_failures/tests/family_bridge.rs` — family-level regression coverage for the new AST bridge.
- `.plans/2026-04-04-142819-family-checks-packages.md` — current extraction ledger for all packages.
- `.worklogs/2026-04-05-211234-extract-garde-root-policy-content-checks.md` — prior garde extraction context.

## Next Steps / Continuation Plan
1. Run package-level `arch` / `code` validation on `packages/g3rs-garde-ast-checks` and record any package-specific debt in `packages/g3rs-garde-ast-checks/TODO.md`.
2. Audit whether the remaining app-side garde `facts` structs can be deleted entirely now that the AST rules moved out of the app runtime.
3. Choose the next extraction target after garde. The likely candidates are `test` or `release`, depending on which family still has a clean config/content split.
