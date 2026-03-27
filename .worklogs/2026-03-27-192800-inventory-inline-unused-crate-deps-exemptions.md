# Inventory Inline Unused Crate Dependencies Exemptions

**Date:** 2026-03-27 19:28
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow_tests/false_positives.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow_tests/inventory.rs`

## Summary
Fixed an `RS-CODE-02` false negative: inline-module `#![allow(unused_crate_dependencies)]` exemptions were silently skipped. The rule now inventories both crate-level and inline-module uses of that one approved broad exemption, with tests updated to treat the inline form as visible rather than ignored.

## Context & Problem
During the continuing `RS-CODE` attack pass, I checked the interaction between `RS-CODE-01` and `RS-CODE-02`. `RS-CODE-01` intentionally suppresses findings for `unused_crate_dependencies`, including module-wide inline `#![allow(unused_crate_dependencies)]`. But `RS-CODE-02` only inventoried crate-level instances. That created a silent hole: the one allowed broad exemption could still be applied at inline-module scope without any visibility. That is exactly the kind of detector gap the attack phase is supposed to find.

## Decisions Made

### Treat inline-module unused_crate_dependencies as part of the approved exemption inventory
- **Chose:** Extend `RS-CODE-02` to inspect `find_inline_mod_allows(...)` in addition to crate-level inner attributes.
- **Why:** The exemption is already intentionally allowed at module scope by `RS-CODE-01`, so it should stay visible in the audit inventory instead of disappearing.
- **Alternatives considered:**
  - Forbid inline-module `unused_crate_dependencies` entirely in `RS-CODE-01` — rejected because that is a policy change, not a detector fix, and it contradicts the current allowed-exemption design.
  - Leave the hole and rely on crate-level inventory only — rejected because it creates an avoidable false negative.

### Update tests to distinguish “not a hit” from “allowed but inventoried”
- **Chose:** Change the old “false positives” expectation so a file with inline-module `unused_crate_dependencies` now expects exactly one `RS-CODE-02` hit.
- **Why:** That case is no longer a false positive once the rule is correctly inventorying it.
- **Alternatives considered:**
  - Add only a new test and leave the old one semantically stale — rejected because the existing test would then misdescribe the contract.

## Architectural Notes
This fix keeps the separation between:

- `RS-CODE-01`: broad suppression is bad, except the one approved universal exemption
- `RS-CODE-02`: the approved universal exemption must still be visible as inventory

That keeps the “allowed” case from becoming “invisible,” which is consistent with the broader guardrail design: approved escape hatches still need explicit audit visibility.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_01_crate_level_allow.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow.rs`
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_01_crate_level_allow_tests/bypasses.rs`
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow_tests/*`

## Open Questions / Future Considerations
- The broader `RS-CODE` attack pass is still ongoing. This commit only fixes one concrete false negative.
- `RS-CODE-32` still has a known structural limitation around method-name-only `.expect(...)` detection, but there is not yet a concrete in-repo case or a deterministic type-aware fix.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow.rs` — live rule fix
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow_tests/false_positives.rs` — adjusted expectation for the inline allowed case
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow_tests/inventory.rs` — direct inventory proof for crate-level plus inline-module cases
- `.worklogs/2026-03-27-183815-merge-rs-code-28-into-27.md` — prior nearby `RS-CODE` detector checkpoint

## Next Steps / Continuation Plan
1. Commit this `RS-CODE-02` detector fix and keep `apps/guardrail3/target/` untracked.
2. Continue the adversarial `RS-CODE` pass on untouched rules, prioritizing concrete false positives/false negatives over repo cleanup.
3. Once `RS-CODE` no longer yields obvious detector bugs, begin `RS-CLIPPY` stabilization with README + self-host split + attack review.
