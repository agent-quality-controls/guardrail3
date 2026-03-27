# RS-CODE Attack Round 3

**Date:** 2026-03-27 19:43
**Scope:** `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs`, `apps/guardrail3/crates/app/rs/ast/src/ast_helpers_tests.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_02_unused_crate_dependencies_allow.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_03_item_level_allow_without_reason.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_04_item_level_allow_with_reason.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_05_garde_skip_without_comment.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_06_garde_skip_with_comment.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_07_exception_comment_inventory.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_07_exception_comment_inventory_tests/inventory.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_08_cfg_attr_allow_inventory.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_08_cfg_attr_allow_inventory_tests/inventory.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_12_unsafe_code_lint_tests/inventory.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_17_impl_allow_blast_radius.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_18_always_true_cfg_attr_bypass.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_20_extern_allow_tests/direct.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_22_deny_forbid_without_reason.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_25_public_result_error_type.rs`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_32_test_expect_message_quality.rs`

## Summary
This checkpoint hardened several remaining `RS-CODE` detector holes and restored self-host cleanliness for the `code` family. The main fixes were nested `cfg_attr` detection, inline config `EXCEPTION:` inventory, table-shaped `unsafe_code` lint parsing, and same-line `#[path]` reasons on the family’s own sidecar wiring.

## Context & Problem
After earlier attack rounds, `RS-CODE` was much better but still had a few concrete parser/facts gaps:

- `RS-CODE-07` only inventoried whole-line `# EXCEPTION:` / `// EXCEPTION:` comments and missed trailing inline config comments.
- `RS-CODE-12` only recognized string-valued workspace lint entries and silently ignored table-shaped forms such as `unsafe_code = { level = "deny" }`.
- `RS-CODE-20` and `RS-CODE-08` missed nested `cfg_attr(..., cfg_attr(..., allow(...)))` forms.

Separately, the `code` family had started failing its own `RS-CODE-24` checks because a subset of runtime rule files still used split-line `#[path]` + `// reason:` instead of the same-line form the rule actually enforces.

There was also substantial unrelated repo churn in other families (`fmt`, `toolchain`, `clippy`, `hexarch`, and others). This checkpoint intentionally avoided mixing that work into the commit.

## Decisions Made

### Treat Nested `cfg_attr` As A Structural Parse Problem
- **Chose:** recurse through nested `cfg_attr` metadata for `allow(...)` extraction in both the shared AST helper and the code-family helper.
- **Why:** the bug was not rule policy; it was incomplete syntactic traversal. Nested `cfg_attr` should still be seen as the same suppression surface.
- **Alternatives considered:**
  - Leave nested `cfg_attr` unsupported — rejected because it is a real bypass shape.
  - Patch only `RS-CODE-20` locally — rejected because `RS-CODE-08` shares the same underlying pattern and would remain inconsistent.

### Parse `unsafe_code` Lint Levels From Table Shapes
- **Chose:** accept both string and `{ level = ... }` TOML values when collecting workspace `unsafe_code` policy facts.
- **Why:** modern lint tables can be expressed either way, and the family should not silently miss the structured form.
- **Alternatives considered:**
  - Keep string-only parsing — rejected as a concrete false negative.
  - Push this concern to another family — rejected because `RS-CODE-12` explicitly owns the `unsafe_code` lint-level policy.

### Inventory Trailing `EXCEPTION:` Comments Without Changing The Message Contract
- **Chose:** scan config lines for comment markers outside quoted strings and inventory trailing `EXCEPTION:` comments while preserving the original `# ...` / `// ...` text shape.
- **Why:** inline exception comments are legitimate config surfaces, but changing the message format would create unnecessary drift in existing expectations.
- **Alternatives considered:**
  - Only support full-line exception comments — rejected because it misses real documented exceptions.
  - Inventory only the comment body without marker prefixes — rejected after it broke existing test expectations and reduced continuity.

### Make Self-Hosted `#[path]` Reasons Match The Rule Literally
- **Chose:** move the `// reason:` text onto the same line as the `#[path = ...]` attribute in the `code` family’s runtime rule files.
- **Why:** `RS-CODE-24` explicitly requires same-line reasons, and the family should not rely on a loophole or special casing to stay green.
- **Alternatives considered:**
  - Relax `RS-CODE-24` for split-line reasons — rejected because it weakens the rule and adds ambiguity.
  - Ignore the self-hits as acceptable debt — rejected because the family is meant to self-host honestly.

## Architectural Notes
The changes stayed within the detector and self-host surfaces:
- shared AST traversal in `app/rs/ast`
- `code` family facts/parser helpers
- rule-local regression tests

No route, placement, or family-mapper boundaries changed here. This was detector hardening, not architecture refactoring.

## Information Sources
- `.worklogs/2026-03-27-193159-tighten-rs-code-include-and-result-rules.md` — previous `RS-CODE` attack checkpoint.
- `.worklogs/2026-03-27-183815-merge-rs-code-28-into-27.md` — recent `RS-CODE` rule-contract change.
- `apps/guardrail3/crates/app/rs/families/code/README.md` — family contract and self-hosting expectations.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/*` — rule implementations and rule-local tests.

## Open Questions / Future Considerations
- `RS-CODE-16` still has a theoretical false positive on locally-defined macros named `panic`. Fixing that cleanly likely requires real macro-resolution semantics rather than string matching.
- The broader repo still has a lot of `RS-CODE` debt, but at this point it looks more like real code cleanup than detector drift.
- `RS-CLIPPY` is the next family to stabilize, but the worktree already has unrelated `clippy` churn that should be treated carefully.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs` — code-family fact collection, including config comment and workspace lint parsing.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/helpers.rs` — local parser helpers for nested `cfg_attr` allow extraction.
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers.rs` — shared AST helper used by multiple families for cfg-attr inventory.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_07_exception_comment_inventory.rs` — `RS-CODE-07`.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_08_cfg_attr_allow_inventory.rs` — `RS-CODE-08`.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_12_unsafe_code_lint.rs` — `RS-CODE-12`.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_20_extern_allow.rs` — `RS-CODE-20`.
- `.worklogs/2026-03-27-193159-tighten-rs-code-include-and-result-rules.md` — immediately prior attack checkpoint.

## Next Steps / Continuation Plan
1. Start the `RS-CLIPPY` stabilization pass without assuming the current unrelated `clippy` worktree changes are safe to absorb. Read the existing `clippy` family state first and separate detector work from external churn.
2. Add the missing `RS-CLIPPY` family README if it still does not exist, then evaluate whether the family should be migrated to the same self-hosted workspace pattern used by `code`.
3. Before editing `clippy`, run the family’s own tests plus `RS-TEST` / `RS-ARCH` validation on the family root to get a fresh baseline independent of the current dirty tree.
