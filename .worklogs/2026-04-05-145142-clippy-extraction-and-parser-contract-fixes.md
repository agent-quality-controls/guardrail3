# Extract Clippy Content Checks And Align Parser Consumers

**Date:** 2026-04-05 14:51
**Scope:** `packages/g3-clippy-content-checks/`, `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/`, `packages/g3-fmt-content-checks/`, `packages/g3-toolchain-content-checks/`, `packages/g3-deny-content-checks/`, `apps/guardrail3/Cargo.lock`, `.plans/2026-04-04-142819-family-checks-packages.md`

## Summary
Extracted the first `clippy` content-check slice into `packages/g3-clippy-content-checks` and rewired the app family to call it for typed `clippy.toml` inputs. Then fixed the extracted `fmt`, `toolchain`, and `deny` packages to compile against the finalized parser types so the real `guardrail3` binary path runs again instead of stopping in compile errors.

## Context & Problem
The repo had reached two overlapping states:

- `fmt`, `toolchain`, and `deny` content-check packages already existed, but later parser tightening changed several parser surfaces from loose strings into typed enums or inheritable wrappers.
- `clippy` extraction had started but was still uncommitted.

That left the package tests mostly green in isolation while the real validator binary path (`cargo run -p guardrail3 -- rs validate ...`) failed to compile through stale `.as_deref()` assumptions. At the same time, the user wanted the extracted family work committed only if the repo genuinely compiled and the validator path worked again.

## Decisions Made

### Extract only the typed-content `clippy` slice
- **Chose:** Create `g3-clippy-content-checks` for rules `RS-CLIPPY-02`, `03`, `09`, `10`, `11`, `17`, `21`, and `22`, and keep structural/parse-policy ownership in the app family.
- **Why:** These rules operate on valid parsed `clippy.toml` content and fit the content-package boundary cleanly. Coverage, same-root precedence, cargo-config override handling, and typed parse rejection remain orchestrator concerns.
- **Alternatives considered:**
  - Move more `clippy` rules immediately — rejected because `RS-CLIPPY-24` still depends on cargo config discovery and `RS-CLIPPY-25` must own typed parse failure.
  - Keep all moved rules in the app until the full family is extracted — rejected because the package boundary is already clear for this slice and delaying it would not reduce risk.

### Keep malformed typed parse ownership in the app
- **Chose:** Route typed schema failure for `clippy.toml` through `RS-CLIPPY-25` and ensure the extracted package only sees valid `ClippyToml`.
- **Why:** The architecture requires content packages to receive typed parsed files, not optional/error-state inputs.
- **Alternatives considered:**
  - Let moved rules keep diagnosing raw wrong-type shapes — rejected because that leaks parser failure ownership into content rules.
  - Pass raw TOML or partial typed state into the package — rejected because it breaks the clean parser/orchestrator/content boundary.

### Adapt existing content packages to finalized parser contracts
- **Chose:** Update the extracted `fmt`, `toolchain`, and `deny` packages to use the finalized parser APIs instead of the earlier stringly assumptions.
- **Why:** Parser packages were intentionally tightened to model the real file contracts. The content packages must consume those typed contracts rather than pinning the repo to stale parser shapes.
- **Alternatives considered:**
  - Loosen parsers back to string surfaces — rejected because parser fidelity was the whole point of the previous parser work.
  - Add ad hoc conversion helpers in app families instead of fixing packages — rejected because the stale assumptions lived inside the packages themselves.

## Architectural Notes
The important boundary in this batch is:

- parser packages model files faithfully
- app families discover files, parse them once, and own malformed parse signaling
- content-check packages only consume valid typed parsed files

For `clippy`, that means `g3-clippy-content-checks` takes parsed `ClippyToml` and the app family still owns `RS-CLIPPY-25`.

For the parser-compatibility fixes:

- `cargo-toml-parser` now exposes `InheritableValue<String>` for package-level inherited fields like `edition` and `rust-version`
- `rustfmt-toml-parser` now exposes typed enums for `edition` and `style_edition`
- `deny-toml-parser` now exposes typed `AdvisoryScope`

The extracted packages were updated to read those contracts directly instead of pretending they were still plain strings.

## Information Sources
- Existing extraction plan:
  - `.plans/2026-04-04-142819-family-checks-packages.md`
- Clippy app family runtime:
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/run.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_25_config_parseable/rule.rs`
- Extracted packages:
  - `packages/g3-clippy-content-checks/`
  - `packages/g3-fmt-content-checks/`
  - `packages/g3-toolchain-content-checks/`
  - `packages/g3-deny-content-checks/`
- Finalized parser types:
  - `packages/cargo-toml-parser/crates/parser/types/src/cargo_toml.rs`
  - `packages/rustfmt-toml-parser/crates/parser/types/src/rustfmt_toml.rs`
  - `packages/deny-toml-parser/crates/parser/types/src/advisories.rs`
- Verification commands:
  - `cargo test --workspace --manifest-path packages/g3-clippy-content-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3-fmt-content-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3-toolchain-content-checks/Cargo.toml`
  - `cargo test --workspace --manifest-path packages/g3-deny-content-checks/Cargo.toml`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-fmt --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-toolchain --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny --lib`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate <temp repo> --family fmt --family toolchain --family deny --family clippy --format json`
- Prior worklogs:
  - `.worklogs/2026-04-05-134932-content-checks-package-todos.md`
  - `.worklogs/2026-04-05-141702-parser-contract-tightening-batch.md`
  - `.worklogs/2026-04-05-143457-parser-contract-fixes-batch-2.md`

## Open Questions / Future Considerations
- `g3-clippy-content-checks` still covers only the first typed-content slice. The remaining content rules should be migrated rule-by-rule, keeping `RS-CLIPPY-24` app-side until the package intentionally expands to consume cargo config files.
- The moved `clippy` slice has good package tests, but app-level smoke coverage for “package wired correctly through family run.rs” is still indirect rather than explicit.
- `g3-deny-content-checks` still needs direct package tests; its app-family tests are currently carrying most of the confidence.

## Key Files for Context
- `packages/g3-clippy-content-checks/crates/runtime/src/run.rs` — package entrypoint for the extracted clippy slice.
- `packages/g3-clippy-content-checks/crates/types/src/lib.rs` — public typed package boundary for clippy content checks.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/run.rs` — app/package bridge for moved clippy rules.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_25_config_parseable/rule.rs` — app-owned typed parse failure rule.
- `packages/g3-fmt-content-checks/crates/runtime/src/inputs.rs` — updated cargo/rustfmt accessor logic against typed parser surfaces.
- `packages/g3-toolchain-content-checks/crates/runtime/src/rs_toolchain_03_msrv_consistency/rule.rs` — updated Cargo rust-version handling through inheritable values.
- `packages/g3-deny-content-checks/crates/runtime/src/advisories/rs_deny_05_advisories_baseline.rs` — typed advisory-scope consumption after parser tightening.
- `.plans/2026-04-04-142819-family-checks-packages.md` — current extraction ledger for content packages.
- `.worklogs/2026-04-05-143457-parser-contract-fixes-batch-2.md` — parser finalization context that caused the downstream package fixes.

## Next Steps / Continuation Plan
1. Add explicit app-family smoke tests for one passing and one failing migrated `clippy` rule so the package bridge is pinned at the family layer, not only through package tests.
2. Add direct runtime tests inside `g3-deny-content-checks` so deny package behavior is exercised locally instead of relying mostly on app-family tests.
3. Start the next content package with the same boundary discipline: typed parsed files only, structural discovery and parse rejection left in the app family.
