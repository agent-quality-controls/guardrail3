# Fix RS-CODE Audit Gaps

**Date:** 2026-03-22 22:44
**Scope:** `.plans/todo/checks/rs/code.md`, `apps/guardrail3/crates/app/rs/checks/rs/code/{mod.rs,facts.rs,inputs.rs,parse.rs,rs_code_22_deny_forbid_without_reason.rs,rs_code_30_input_failures.rs,*_tests.rs}`

## Summary
Closed the adversarially-discovered `rs/code` gaps instead of leaving them as “known issues”. The family now surfaces source/config parse failures explicitly, resolves package/library policy context correctly for workspace-member packages, scans nested config files for exception comments, and tightens the AST semantics for the flagged rules (`RS-CODE-18`, `21`, `22`, `23`, `27`).

## Context & Problem
After `RS-CODE-01..29` was finished, adversarial review found that the family still had several real weaknesses:
- `mod.rs` silently skipped unreadable or unparsable Rust files
- `facts.rs` failed open on parse errors in `Cargo.toml` / `guardrail3.toml`
- package-profile resolution missed workspace-member packages under `[rust.packages]`
- nested config files were not scanned for `EXCEPTION` comments
- some AST helpers under-enforced or over-enforced the planned semantics

Those were not cosmetic issues. They were places where the checker could silently miss exactly the kind of guardrail bypasses it is supposed to make visible.

## Decisions Made

### Add an explicit `RS-CODE-30` input-failure rule
- **Chose:** introduce `RS-CODE-30` for source/config input failures that would otherwise fail the family open.
- **Why:** silent skipping is structurally wrong for a foundational guardrail family. A surfaced failure is better than quietly omitting every downstream code rule for that file or policy input.
- **Alternatives considered:**
  - Reuse an existing `RS-CODE-*` ID — rejected because none of the existing rules own orchestrator-level read/parse failures.
  - Keep the failures as internal logs only — rejected because the whole problem was invisibility.

### Treat `[rust.packages]` as applying to package roots, not only standalone roots
- **Chose:** apply package policy to every package root not explicitly claimed by `[rust.apps.*]`, including workspace-member packages.
- **Why:** the audit correctly caught that library-only code rules (`RS-CODE-25..29`) could be skipped for workspace-member packages. That drifted from the generation/config intent where package policy is broader than “standalone package only”.
- **Alternatives considered:**
  - Keep package policy limited to standalone packages — rejected because it preserves the missed-coverage bug.
  - Infer app roots from directory shape alone and always exclude them — rejected because that incorrectly treated unconfigured package members as apps.

### Tighten rule helpers to the actual planned semantics
- **Chose:** narrow or broaden the AST helpers where the audit found real mismatches:
  - `RS-CODE-18` no longer treats `feature = "..."` or plain `unix`/`windows` predicates as always true
  - `RS-CODE-21` now catches grouped `use std::{fs::*, ...}` imports
  - `RS-CODE-22` only gives the special inventory exception to crate-level inner `#![forbid(unsafe_code)]`
  - `RS-CODE-23` now sees traversal through `concat!(...)`
  - `RS-CODE-27` now flags more disallowed `lib.rs` body items instead of only `fn`/`impl`
- **Why:** these were direct plan-vs-code mismatches, not stylistic preferences.
- **Alternatives considered:**
  - Leave helper behavior alone and only change tests — rejected because the problems were in production semantics.
  - Broaden the plan to match the implementation — rejected because the adversarial findings were pointing at real under/over-enforcement, not good policy.

### Scan all relevant config files for `EXCEPTION` comments
- **Chose:** collect exception comments from all discovered managed config files in the tree, not only a fixed repo-root list.
- **Why:** nested app/package configs are part of the active policy surface. Root-only scanning silently dropped the very exception trail the rule is meant to inventory.
- **Alternatives considered:**
  - Keep the root-only list — rejected because it mismatched real config placement.
  - Expand only a few hardcoded extra paths — rejected because it would recreate the same drift in a slightly larger form.

## Architectural Notes
- `RS-CODE` now has 30 rules instead of 29.
- `RS-CODE-30` is orchestrator-owned: it is emitted from read/parse failures before any per-file AST rules run.
- `facts.rs` now carries an explicit `input_failures` surface instead of silently discarding parse problems in policy inputs.
- Package-profile resolution still lives inside `rs/code/facts.rs`, but the logic is now stricter:
  - configured app roots come only from actual `[rust.apps.*]` matches
  - package policy applies to package roots not claimed by configured apps
- The tests added in this pass are intentionally adversarial:
  - grouped `std::fs::*` imports
  - `concat!(\"../\", ...)` path traversal
  - nested config `EXCEPTION` comments
  - workspace-member package profile resolution
  - surfaced `guardrail3.toml` parse failure

## Information Sources
- `AGENTS.md` — repo workflow and architecture constraints.
- `.plans/todo/checks/rs/code.md` — canonical `RS-CODE` contract, updated here to add `RS-CODE-30`.
- `.worklogs/2026-03-22-213530-complete-code-family.md` — prior completion checkpoint for the family.
- Adversarial audit findings from the `rs/code` review pass:
  - source read/parse fail-open in `mod.rs`
  - package-profile/root-resolution drift in `facts.rs`
  - helper mismatches in `parse.rs` and rule files

## Open Questions / Future Considerations
- `rs/code` now surfaces bad input state instead of hiding it, but there is still duplicated policy-root logic across Rust families. If a shared resolver is extracted later, `rs/code` should adopt it.
- Fixture-path exclusion in `discover.rs` remains path-based. That was not changed here because the audit did not prove it wrong for the current repo, but it is still a place to scrutinize if source-family coverage questions come up again.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/code/mod.rs` — orchestrator, now including `RS-CODE-30`.
- `apps/guardrail3/crates/app/rs/checks/rs/code/facts.rs` — package-profile resolution, config exception scanning, and code-family input failures.
- `apps/guardrail3/crates/app/rs/checks/rs/code/inputs.rs` — includes the new `CodeInputFailureInput`.
- `apps/guardrail3/crates/app/rs/checks/rs/code/parse.rs` — AST helper tightening for `RS-CODE-18`, `21`, `22`, `23`, `27`.
- `apps/guardrail3/crates/app/rs/checks/rs/code/rs_code_30_input_failures.rs` — new rule for surfaced family input failures.
- `.plans/todo/checks/rs/code.md` — updated rule inventory and `RS-CODE-30` contract.
- `.worklogs/2026-03-22-213530-complete-code-family.md` — previous family completion baseline before this hardening pass.

## Next Steps / Continuation Plan
1. If `rs/code` gets another adversarial pass, target the remaining non-proven-risk areas first:
   - fixture exclusion in `discover.rs`
   - any remaining shared policy-root duplication across families
2. Move to the next unfinished heavy Rust family (`rs/hexarch` is the likely next target) using the same “finish, then adversarially audit, then close every surfaced gap” cycle.
3. If future audits find orchestrator-level silent skips in other families, add explicit surfaced failure rules instead of tolerating fail-open behavior.
