# Toolchain README And Host Suffix Hardening

**Date:** 2026-03-27 23:05
**Scope:** `apps/guardrail3/crates/app/rs/families/toolchain/README.md`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency.rs`, `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency_tests/mod.rs`

## Summary
Updated the family README so it reflects the actual enforced policy, self-hosting semantics, and current verification path. In the same batch, fixed a cross-rule inconsistency for valid rustup toolchain names with host suffixes: `RS-TOOLCHAIN-CONFIG-01` and `RS-TOOLCHAIN-CONFIG-02` now agree that pinned stable channels such as `1.85.0-x86_64-unknown-linux-gnu` are acceptable and still participate in MSRV comparison.

## Context & Problem
After the previous attack-hardening batches, the README was lagging behind reality. It still described the family at a high level but did not explain the now-expanded malformed-input enforcement, the distinction between self-inventory and actual self-hosting debt, or the direct family-workspace verification command.

The next attack rounds also exposed a concrete detector mismatch:

- `RS-TOOLCHAIN-CONFIG-01` had been hardened to accept stable and pinned-stable channel names with host suffixes.
- `RS-TOOLCHAIN-CONFIG-02` still parsed versions too narrowly and silently dropped MSRV comparison for the same accepted pinned-stable forms.

That meant the family could accept a valid channel in one rule and fail to compare it in the next, which is exactly the kind of inter-rule inconsistency the attack phase is meant to catch.

## Decisions Made

### Treat rustup host-suffixed stable names as valid stable/pinned-stable inputs end-to-end
- **Chose:** Keep `RS-TOOLCHAIN-CONFIG-01` acceptance for host-suffixed stable names and update `RS-TOOLCHAIN-CONFIG-02` version parsing so it strips only the host suffix after first classifying the channel family.
- **Why:** Toolchain names like `stable-x86_64-unknown-linux-gnu` and `1.85.0-x86_64-unknown-linux-gnu` are valid rustup forms and should not become false positives or silent skips.
- **Alternatives considered:**
  - Reject all host-suffixed forms — rejected because that would be a false-positive policy narrower than real Rust toolchain naming.
  - Keep accepting them in `RS-TOOLCHAIN-CONFIG-01` but skip `RS-TOOLCHAIN-CONFIG-02` — rejected because it leaves the inter-rule inconsistency in place.

### Update the README to document real policy and verification behavior
- **Chose:** Expand the family README with current rule behavior, malformed-input handling, self-hosting expectations, and the direct workspace test command.
- **Why:** The family is now stable enough that the README should match the real contract rather than only the initial structural split.
- **Alternatives considered:**
  - Leave the README minimal and rely on code/tests for truth — rejected because the handoff explicitly called for a family README that matches reality.
  - Document only structure and omit behavioral details — rejected because the meaningful changes in this family are now mostly about trust and detector semantics.

## Architectural Notes
This batch did not change the family shape. It tightened semantic consistency:

- channel classification and MSRV comparison now agree on the same accepted pinned-stable namespace
- the README now documents the family as it actually behaves after the attack-hardening passes

The direct family workspace test flow remains the preferred local verification command while unrelated top-level workspace issues exist elsewhere in the repo.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/toolchain/README.md`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components_tests/mod.rs`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency_tests/mod.rs`
- `.worklogs/2026-03-27-230141-toolchain-attack-followup.md`

## Open Questions / Future Considerations
- There are likely more rustup-specific naming corners beyond the currently covered host-suffix cases, especially around archive dates and more exotic suffix combinations. If future attacks keep surfacing channel-name parsing issues, the family may benefit from a dedicated parser helper rather than accreting ad hoc string logic.
- The README is now accurate for the current family contract, but if the plan later broadens `RS-TOOLCHAIN` beyond root-level scope, it will need another update.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/toolchain/README.md` — current family contract and verification guidance
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components.rs` — accepted/stable/pinned channel classification
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency.rs` — version extraction for pinned-stable MSRV comparison
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_01_channel_components_tests/mod.rs` — host-suffix acceptance coverage for `RS-TOOLCHAIN-CONFIG-01`
- `apps/guardrail3/crates/app/rs/families/toolchain/crates/runtime/src/rs_toolchain_config_02_msrv_consistency_tests/mod.rs` — host-suffix MSRV comparison coverage
- `.worklogs/2026-03-27-230141-toolchain-attack-followup.md` — prior attack-hardening context

## Next Steps / Continuation Plan
1. Commit this README refresh plus the host-suffix/MSRV consistency fix as one scoped toolchain-family follow-up commit.
2. Continue adversarial rounds on remaining channel-name corners and mixed malformed modern+legacy inputs, keeping fixes local to the toolchain family only.
3. If another couple of rounds do not reveal a concrete detector bug, stop widening this family and move to the next family handoff.
