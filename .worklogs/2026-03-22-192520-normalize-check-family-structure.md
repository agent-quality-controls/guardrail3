# Normalize Rust Check Family Structure

**Date:** 2026-03-22 19:25
**Scope:** `AGENTS.md`, `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`, `.plans/todo/checks/rs/{fmt,cargo,clippy,deny}.md`, `.plans/by_file/rs/deny-toml.md`, `apps/guardrail3/crates/app/rs/checks/rs/{fmt,toolchain,cargo,clippy,deny}/**`

## Summary
Normalized the active Rust config-check families to the strict final structure: one production file per rule ID and one collocated `*_tests.rs` file per rule. This removed grouped family test files, removed grouped `deny` concern modules, split `clippy` and `deny` into exact rule files, and rewired the family orchestrators so the active code now matches the documented architecture contract.

## Context & Problem
The new Rust-check architecture had drifted away from the intended implementation contract. Even though the plans said “one rule per file,” several families still used shortcuts:
- one family-wide test file (`fmt_tests.rs`, `toolchain_tests.rs`, `cargo_tests.rs`, `clippy_tests.rs`, `deny_tests.rs`)
- grouped rule files such as `rs_clippy_thresholds.rs`
- grouped concern files in `deny` such as `rs_deny_bans.rs` and `rs_deny_sources.rs`

The user explicitly rejected those shortcuts and tightened the requirement:
- one rule file per rule
- one test file per rule
- no grouped modules
- no family-wide test files

This work completed that structural rewrite for the currently active Rust config families and hardened the docs so the same deviation is less likely to recur.

## Decisions Made

### Make the structural contract explicit in docs
- **Chose:** Tighten `AGENTS.md`, the checker architecture plan, and the active family plans to state that grouped rule files and grouped family test files are forbidden.
- **Why:** The older wording was not strict enough and allowed “technically close enough” implementations.
- **Alternatives considered:**
  - Leave the docs as-is and only rewrite code — rejected because the same slippage would recur.
  - Encode the rule only in `AGENTS.md` — rejected because the per-family plans also need the mapping contract.

### Normalize existing families instead of treating only `deny` as special
- **Chose:** Fix `fmt`, `toolchain`, `cargo`, `clippy`, and `deny` to the same structural rule.
- **Why:** Leaving older families in the shortcut shape would make the architecture inconsistent and undermine the rule immediately.
- **Alternatives considered:**
  - Rewrite only `deny` and leave the earlier families for later — rejected because the user explicitly wanted every minor deviation fixed, not just the newest family.

### Rewrite `deny` completely instead of incrementally patching grouped modules
- **Chose:** Rewire `mod.rs` to numbered `RS-DENY-01..30` modules, add per-rule sidecar tests, and delete the grouped `rs_deny_*` concern files and `deny_tests.rs`.
- **Why:** The grouped implementation was a direct structural violation and could not be “partially acceptable.”
- **Alternatives considered:**
  - Keep grouped internals hidden behind `mod.rs` — rejected because it still violates the one-rule/one-file contract.
  - Commit a temporary half-split version — rejected because the user explicitly rejected shortcuts.

### Keep family-level `test_support.rs` helpers but not grouped test suites
- **Chose:** Retain small shared family test helpers like `cargo/test_support.rs`, `clippy/test_support.rs`, and `deny/test_support.rs`.
- **Why:** They avoid duplication without bundling multiple assertions into one test module. They are helper substrate, not hidden grouped rule tests.
- **Alternatives considered:**
  - Duplicate fixture builders into every single rule test file — rejected because it adds noise without strengthening the architectural invariant.
  - Treat helper files as forbidden too — rejected because the actual forbidden pattern is grouped rule logic or grouped family test suites.

## Architectural Notes
The active Rust config families now follow the same concrete shape:
- `mod.rs` contains only orchestration and module declarations
- `facts.rs` and `inputs.rs` contain shared family extraction/types
- optional family support modules (`clippy_support.rs`, `deny_support.rs`, `test_support.rs`) contain shared baseline/test helpers only
- each `RS-*` rule has its own production file
- each production rule file mounts exactly one neighboring `*_tests.rs` file

`deny` also now mirrors the same architectural pattern as the other families:
- coverage/placement/shadowing split into `RS-DENY-01..03`
- each later policy concern split into its own rule file through `RS-DENY-30`
- grouped concern files deleted entirely

The final structural audit count is:
- `fmt`: 8 production / 8 tests
- `toolchain`: 4 / 4
- `cargo`: 10 / 10
- `clippy`: 22 / 22
- `deny`: 30 / 30

## Information Sources
- `AGENTS.md`
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `.plans/todo/checks/rs/{fmt,cargo,clippy,deny}.md`
- `.plans/by_file/rs/deny-toml.md`
- current family code under `apps/guardrail3/crates/app/rs/checks/rs/{fmt,toolchain,cargo,clippy,deny}`
- `cargo test --lib checks::rs::{fmt,toolchain,cargo,clippy,deny} --quiet`
- `npx gitnexus analyze`
- `npx gitnexus impact check --repo guardrail3 --direction upstream`
- `npx gitnexus impact collect --repo guardrail3 --direction upstream`
- `.worklogs/2026-03-22-170103-clippy-completeness-finish.md`
- `.worklogs/2026-03-22-164943-clippy-audit-fixes.md`

## Open Questions / Future Considerations
- The same structural rule should be enforced when the remaining Rust source families (`source`, `hexarch`, `deps`, `garde`, `test`, `release`) are built.
- `RS-CLIPPY-19` remains intentionally temporary as a typo-like managed-key detector; that policy is unchanged by this structural rewrite.
- The repo should eventually gain a guardrail that forbids grouped rule/test module patterns automatically so this class of drift is caught by code instead of by review.

## Key Files for Context
- `AGENTS.md` — current repo instructions and architecture contract
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — checker architecture and the explicit one-rule/one-test contract
- `.plans/todo/checks/rs/deny.md` — full `deny` rule inventory and structural contract
- `apps/guardrail3/crates/app/rs/checks/rs/deny/mod.rs` — canonical example of the strict numbered-module orchestration after the rewrite
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/mod.rs` — normalized `clippy` orchestrator after removing grouped rule/test files
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/mod.rs` — normalized family with parent/child rule inputs and per-rule tests
- `apps/guardrail3/crates/app/rs/checks/rs/deny/test_support.rs` — shared deny test fixture helpers, showing the allowed helper pattern
- `.worklogs/2026-03-22-170103-clippy-completeness-finish.md` — previous clippy completion checkpoint

## Next Steps / Continuation Plan
1. Stage and commit this normalization batch with the worklog so the new architecture has a clean structural baseline.
2. After the commit, start the next active Rust family from a clean tree, most likely one of the remaining source families rather than another config family.
3. When building the next family, enforce the same contract from the first file:
   - write `mod.rs`, `facts.rs`, `inputs.rs`
   - add exactly one production file per rule
   - add exactly one `*_tests.rs` per rule immediately
   - avoid grouped temporary files entirely.
