# Fix Code Rule 8 Plan Severity

**Date:** 2026-04-09 10:10
**Scope:** `.plans/todo/checks/rs/code.md`

## Summary
Updated the `RS-CODE-08` ledger entry so the plan matches the migrated rule behavior. The rule is a normal-output warning, not an info/inventory item.

## Context & Problem
After migrating the comment/reason-heavy `code` AST rules, one documented mismatch remained: `RS-CODE-08` was described in the plan as info/inventory, but both the legacy implementation and the new package implementation emit it as a warning in normal output.

## Decisions Made

### Make the plan match the implementation
- **Chose:** Change the plan entry for `RS-CODE-08` from `Info` / inventory wording to `Warn` / normal-output wording.
- **Why:** The code already matches legacy behavior, and the user explicitly confirmed that `rule 8` should be warn.
- **Alternatives considered:**
  - Change the implementation to info/inventory — rejected because the user said the rule should be warn.
  - Leave the mismatch in place — rejected because stale plan text would mislead the next migration pass.

## Architectural Notes
This is a spec cleanup only. No runtime behavior changed.

## Information Sources
- `.plans/todo/checks/rs/code.md` — stale severity/source-of-truth text for `RS-CODE-08`
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_08_cfg_attr_allow_inventory/rule.rs` — current migrated behavior
- User clarification in session: `rule 8 should be warn`

## Open Questions / Future Considerations
- None for this fix. The plan and implementation now agree on `RS-CODE-08`.

## Key Files for Context
- `.plans/todo/checks/rs/code.md` — rule ledger for the `code` family
- `packages/rs/code/g3rs-code-ast-checks/crates/runtime/src/rs_code_ast_08_cfg_attr_allow_inventory/rule.rs` — migrated `rule 8`
- `.worklogs/2026-04-09-095636-migrate-code-ast-reason-comment-rules.md` — prior migration and the mismatch note that triggered this cleanup

## Next Steps / Continuation Plan
1. Audit the remaining unmigrated `code` rules and split them into:
   - still-single-file AST
   - single-file AST but profile-sensitive
   - not this AST lane
2. Pick the next smallest batch from the still-single-file AST group.
