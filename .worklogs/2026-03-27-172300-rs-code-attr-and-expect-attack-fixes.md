# Tighten RS-CODE Attribute And Expect Attack Coverage

**Date:** 2026-03-27 17:23
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_03_item_level_allow_without_reason_tests/false_positives.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason_tests/false_positives.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason_tests/inventory.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_32_test_expect_message_quality_tests/false_positives.rs`

## Summary
Continued adversarial review of `RS-CODE` found two more concrete parser/contract bugs. I fixed `RS-CODE-32` so it no longer flags arbitrary helper functions named `expect`, and I fixed the shared reason/allow parsing so multiline documented `#[allow(...)]` attrs are classified correctly and `RS-CODE-04` now matches its inventory contract.

## Context & Problem
The first `RS-CODE` attack pass had already hardened the filesystem rules, but most of the family was still unchallenged. Sampling live repo findings and cross-checking them against the implementation uncovered:

- `RS-CODE-32` treated any path ending in `expect` as an `Option/Result.expect(...)` surface, so a user-defined helper like `helpers::expect("ok")` in a test would false-positive.
- `RS-CODE-03` / `RS-CODE-04` misclassified multiline documented `#[allow(...)]` attributes because the parser stored the attribute start line while `same_line_reason` looked only at that recorded line.
- `RS-CODE-04` was documented in `.plans/todo/checks/rs/code.md` as audit-trail inventory, but the implementation emitted ordinary non-inventory findings.
- Reason extraction also truncated comment text after the second `//`, which made inventory text lossy when the reason itself contained a URL.

These were detector/contract bugs, not repo cleanup issues.

## Decisions Made

### Restrict `RS-CODE-32` to actual method-style `expect(...)`
- **Chose:** Drop the generic `ExprCall` path-ending-in-`expect` detection and keep only method-call detection.
- **Why:** The repo does not use UFCS-style `::expect(...)`, while the generic path-based matcher created a clear false-positive class for unrelated helper functions named `expect`.
- **Alternatives considered:**
  - Keep generic path matching and try to infer `Option`/`Result` UFCS — rejected because that still relies on weak naming heuristics and leaves room for unrelated helper false positives.
  - Support both method calls and full type-resolution of UFCS — rejected because the family does not have type resolution and the added complexity was not justified by live usage.

### Record allow-attribute line numbers at the attribute end line
- **Chose:** Use `span_end_line(attr.span())` for `collect_allow_lints` and `collect_cfg_attr_allow_lints`.
- **Why:** For multiline attributes, the “same-line reason” contract applies to the line where the attribute closes, not the line where it starts. Using the end line aligns parsing with the written rule semantics.
- **Alternatives considered:**
  - Invent a special multiline reason scan separate from line tracking — rejected because the contract is still same-line; only the recorded line was wrong.
  - Treat multiline allows as always undocumented — rejected because the repo already contains legitimate documented multiline allow surfaces and the plan does not forbid them.

### Make `RS-CODE-04` actually inventory-only
- **Chose:** Set `inventory: true` for documented item-level allows and update test expectations accordingly.
- **Why:** The plan explicitly describes `RS-CODE-04` as audit-trail inventory. Emitting normal findings contradicted the contract and made documented allows look like ordinary active findings.
- **Alternatives considered:**
  - Change the doc to match the old implementation — rejected because the inventory model is the better architecture for documented exceptions.
  - Keep non-inventory findings for visibility — rejected because it blurs the distinction between “bad code to fix” and “documented escape hatch to track.”

### Preserve full reason text after the first comment marker
- **Chose:** Change `same_line_reason` from `split("//").nth(1)` to `split_once("//")`.
- **Why:** Reasons containing URLs or later `//` text were being truncated in inventories. The first `//` is the comment boundary; everything after it belongs to the reason string.
- **Alternatives considered:**
  - Leave truncation alone because detection still worked — rejected because the inventory text is part of the contract and should be faithful.

## Architectural Notes
These fixes tighten the “policy vs inventory” distinction inside `RS-CODE`:

- undocumented item-level allows remain `RS-CODE-03` errors,
- documented item-level allows are now properly treated as `RS-CODE-04` inventory,
- multiline formatting no longer changes semantic classification,
- and helper names no longer leak into `RS-CODE-32`’s message-quality policy.

This is consistent with the broader Rust family approach already used in `test`, `arch`, `cargo`, and `hexarch`: inventory rules should be explicit and structurally distinct from active findings.

## Information Sources
- `.plans/todo/checks/rs/code.md` — rule contract, especially `RS-CODE-03`, `RS-CODE-04`, and `RS-CODE-32`
- `apps/guardrail3/crates/app/rs/families/code/README.md` — family-level expectations
- `apps/guardrail3/tests/adversarial_fixtures.rs` — live multiline documented allow examples
- `/tmp/rs_code_attack_inventory_after.json` and `/tmp/rs_code_attack_inventory_after2.json` — repo-wide inventories used to confirm live bucket changes
- prior worklogs:
  - `.worklogs/2026-03-27-161428-rs-code-attack-fixes.md`
  - `.worklogs/2026-03-27-171025-rs-code-fs-attack-fixes.md`

## Open Questions / Future Considerations
- `RS-CODE-03` / `RS-CODE-04` still miss allow attributes generated inside `macro_rules!` bodies. There is a known adversarial fixture for this. Fixing it deterministically likely needs a contract decision about whether macro-token scanning belongs in this family or whether the gap is explicitly accepted.
- `RS-CODE-24` remains a large bucket. Current sampling suggests most of it is real `#[path]` debt rather than detector drift, but it still deserves a dedicated adversarial pass.
- `RS-CODE-32` is now less noisy structurally, but the message-quality policy still needs more sampling to decide whether any one-word operation names should be accepted or whether the current strictness is the desired standard.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs` — same-line reason parsing
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs` — shared allow-attribute line extraction
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs` — `RS-CODE-32` expect-call parsing
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason.rs` — inventory behavior for documented allows
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_03_item_level_allow_without_reason_tests/false_positives.rs` — multiline documented allow regression
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason_tests/false_positives.rs` — URL reason and multiline inventory regressions
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_32_test_expect_message_quality_tests/false_positives.rs` — helper-`expect` regression
- `.worklogs/2026-03-27-171025-rs-code-fs-attack-fixes.md` — prior `RS-CODE` attack-fix checkpoint

## Next Steps / Continuation Plan
1. Continue the adversarial pass on `RS-CODE-24` and `RS-CODE-32`, focusing on live repo hits and any remaining parser loopholes.
2. Decide whether the macro-body allow gap for `RS-CODE-03/04` should be enforced or explicitly documented as out of scope.
3. Once the `code` family looks stable under attack, move the same stabilization flow to `rs/release`.
