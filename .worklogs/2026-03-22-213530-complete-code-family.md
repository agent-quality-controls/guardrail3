# Complete RS-CODE Family

**Date:** 2026-03-22 21:35
**Scope:** `.plans/todo/checks/rs/code.md`, `apps/guardrail3/crates/app/rs/checks/rs/code/*`

## Summary
Finished the `rs/code` family so it now implements `RS-CODE-01..29` with one production file per rule and one test file per rule. The completion pass added the remaining suppression/bypass rules, library-profile code-shape rules, and the missing profile-plumbing needed to gate library-only checks correctly.

## Context & Problem
The previous session renamed `rs/source` to `rs/code` and committed only the first implementation slice through `RS-CODE-16` plus `RS-CODE-19`. The family still lacked the rest of the planned rule inventory, and the per-file input dropped `profile_name`, which blocked the library-only rules (`RS-CODE-25..29`). The user asked to finish the whole family rather than leave it as another partial specimen.

## Decisions Made

### Complete the rule inventory now instead of widening to another family
- **Chose:** finish `RS-CODE-17..29` before moving on to anything else.
- **Why:** the family was already the active target and leaving it partial would repeat the earlier drift problem where plans and implementation stop matching.
- **Alternatives considered:**
  - Start `hexarch` or another family first — rejected because `rs/code` still had obvious missing rule IDs.
  - Commit another partial slice — rejected because the user explicitly asked to finish the family.

### Thread `profile_name` through the per-file input
- **Chose:** extend `RustCodeFileFacts` and `RustCodeFileInput` with `profile_name`.
- **Why:** `RS-CODE-25`, `26`, `27`, and `29` are library-profile only, and those decisions belong at orchestrator/facts level rather than inside individual rule discovery.
- **Alternatives considered:**
  - Infer library-ness from file path alone — rejected because profile ownership already lives in `guardrail3.toml` and root/package resolution.
  - Pass the whole policy map into rules — rejected because that would violate the “smallest typed input” rule.

### Keep rule semantics in per-rule files and use `parse.rs` only for shared extraction
- **Chose:** add new AST extractors in `parse.rs` for impl-level allows, deny/forbid attrs, include macros, `#[path]`, public error types, lib.rs facade patterns, and trait size.
- **Why:** the family still needs parse-once shared extraction, but the actual rule decisions must stay visible in the rule files rather than hidden behind grouped policy helpers.
- **Alternatives considered:**
  - Put grouped “quality” or “library” logic into one shared rule helper — rejected because this would recreate the bundling pattern we just removed from earlier families.
  - Reuse only old `rs/validate/*` checks — rejected because the new family needs one-rule/one-file structure and library-profile gating.

### Split cfg-attr behavior between conditional inventory and disguised-bypass enforcement
- **Chose:** keep `RS-CODE-08` for genuinely conditional `cfg_attr(..., allow(...))` inventory, and let `RS-CODE-18` own always-true/disguised-unconditional cfg-attr allows.
- **Why:** this avoids duplicate responsibility between “inventory real conditionals” and “error on bypass disguised as conditional”.
- **Alternatives considered:**
  - Let `RS-CODE-03` continue handling always-true cfg-attr cases — rejected because `RS-CODE-18` is the stronger, dedicated bypass rule.

### Use narrow, explicit semantics for the new library-only rules
- **Chose:** implement the plan literally for:
  - `RS-CODE-25` weak public `Result` error types
  - `RS-CODE-26` glob re-exports in `lib.rs`
  - `RS-CODE-27` facade-only `lib.rs`
  - `RS-CODE-28` inline public modules in `lib.rs`
  - `RS-CODE-29` trait method-count pressure
- **Why:** the repo had design notes for these items but no old validator implementation to migrate, so the current plan is the only stable contract.
- **Alternatives considered:**
  - Broaden `RS-CODE-25` to include `anyhow` immediately — rejected because the active plan froze the narrower `String` / `Box<dyn Error>` shape.
  - Convert `RS-CODE-26` into a clippy-config-only rule — rejected because the active `rs/code` plan defines it as an AST rule in `lib.rs`.

## Architectural Notes
- `rs/code` now matches the same family structure as the finished config families:
  - `mod.rs` orchestrator
  - `facts.rs`
  - `inputs.rs`
  - `parse.rs`
  - `rs_code_XX_*.rs`
  - `rs_code_XX_*_tests.rs`
- The family now has 29 production rule files and 29 sidecar test files.
- `facts.rs` now resolves profile context for each Rust file so the family can gate library-only rules without oversized rule inputs.
- `parse.rs` still reuses a few legacy AST helpers for already-migrated semantics, but the newly-added remaining rules use family-local extractors and rule-local decisions.

## Information Sources
- `AGENTS.md` — current architecture and worklog rules.
- `.plans/todo/checks/rs/code.md` — canonical `RS-CODE` inventory and severity/ownership.
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/facts.rs` — profile/root resolution reference.
- `apps/guardrail3/crates/app/rs/validate/allow_checks.rs` and `ast_helpers.rs` — old helper behavior for existing allow/cfg_attr semantics.
- Explorer audit notes during this session:
  - existing coverage around `cfg_attr` and `std::fs` glob holes
  - lack of old validator/tests for `RS-CODE-25..29`

## Open Questions / Future Considerations
- `rs/code` still silently skips unreadable or unparsable Rust files in `mod.rs`. That is unchanged in this pass and may deserve an explicit parse-failure surface later if the plan adds such a rule.
- `facts.rs` now contains its own profile/root resolution logic similar to `clippy`/`deny`. If the repo later extracts a shared Rust policy-root resolver, `rs/code` should adopt it.
- `RS-CODE-18` now handles the hardened “always true” forms we discussed, but the exact boundary of syntactic truthiness remains a policy surface if more cfg patterns are added later.

## Key Files for Context
- `AGENTS.md` — project source of truth and repo workflow rules.
- `.plans/todo/checks/rs/code.md` — full `RS-CODE` contract and statuses.
- `apps/guardrail3/crates/app/rs/checks/rs/code/mod.rs` — family orchestrator and rule fan-out order.
- `apps/guardrail3/crates/app/rs/checks/rs/code/facts.rs` — per-file discovery and profile resolution.
- `apps/guardrail3/crates/app/rs/checks/rs/code/inputs.rs` — typed rule inputs.
- `apps/guardrail3/crates/app/rs/checks/rs/code/parse.rs` — shared AST extraction helpers used by the family.
- `.worklogs/2026-03-22-211249-rename-source-to-code-and-start-code-family.md` — prior rename/start checkpoint for this family.

## Next Steps / Continuation Plan
1. Run an adversarial audit against `rs/code` the same way we did for `clippy` and `deny`, using the finished plan and old source-scan notes to look for bypasses or over-broad heuristics.
2. If that audit is clean, move to the next heavy Rust family, most likely `rs/hexarch`, using the same one-rule/one-test structure.
3. If a shared Rust policy-root resolver is extracted later, replace the duplicated profile/root logic in `rs/code/facts.rs` with the shared abstraction.
