# Add expression-level #[allow] detection to ItemAllowVisitor

**Date:** 2026-03-16 12:02
**Task:** Add visit_local and visit_arm to ItemAllowVisitor for expression-level allow detection

## Goal
ItemAllowVisitor should detect #[allow] on let bindings (syn::Local) and match arms (syn::Arm), not just items/impl items/trait items.

## Approach

### Step-by-step plan
1. Add `visit_local` and `visit_arm` methods to ItemAllowVisitor impl
2. Both use `collect_outer_allows` on the node's `.attrs` field (same as existing visitors)
3. Update the test `grep_before_edge_attribute_on_expression` to expect R33 results instead of no hits

### Key decisions
- **Use `collect_outer_allows` directly** — no need for a new helper method since `collect_outer_allows` already does the shared logic (filter outer attrs, extract allow lints)
- The fixture has `// reason:` comments on all allows, so they should be detected as R33 (info) not R32 (error)

## Files to Modify
- `src/rs/validate/ast_helpers.rs` — add visit_local and visit_arm to ItemAllowVisitor
- `tests/adversarial_grep_attacks.rs` — update test expectations
