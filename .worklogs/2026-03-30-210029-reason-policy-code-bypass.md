# Shared Reason Policy And RS-CODE Bypass Hardening

**Date:** 2026-03-30 21:00
**Scope:** `packages/reason-policy/`, `apps/guardrail3/crates/app/rs/families/code/crates/runtime`, `apps/guardrail3/Cargo.lock`, `.plans/todo/checks/rs/code.md`, `.plans/by_family/rs/code.md`, `apps/guardrail3/crates/app/rs/families/code/README.md`, `.plans/todo/check_review/test_hardening/02-code.md`, `.plans/todo/check_review/test_hardening/12-code-execution-plan.md`

## Summary
Moved reason-quality validation out of `RS-CODE` into a new shared Rust workspace under `packages/reason-policy`, then migrated the active `RS-CODE` bypass lanes to consume that shared policy. The `code` family now treats weak placeholder reasons as hard failures instead of accepting any non-empty `// reason:`.

## Context & Problem
The earlier `RS-CODE` hardening pass unified documented bypass behavior so documented `#[allow(...)]`, `#[expect(...)]`, and non-exempt `#[garde(skip)]` stayed visible in normal output. That still left a structural hole: the shared parser only checked for exact same-line `// reason:` syntax plus non-empty text, so trivial placeholders like `temp` or `legacy` counted as documentation.

The follow-up user requirement was to enforce non-triviality and fail weak reasons as errors. In the same conversation, we also aligned on a broader architectural constraint: this policy should not live in `app/rs/runtime`, because that crate is the top-level orchestrator that depends on families. Reusable escape-hatch justification policy needs to be consumable by multiple families without introducing `families -> runtime` coupling.

## Decisions Made

### Create a separate shared reason-policy workspace
- **Chose:** add `packages/reason-policy/` as a Cargo workspace with one member crate `crates/reason-policy`
- **Why:** the repo disallows top-level packages but allows top-level workspaces, and the validator needs to be reusable outside `RS-CODE`
- **Alternatives considered:**
  - `apps/guardrail3/crates/app/rs/runtime` — rejected because `app/rs/runtime` depends on families and is the wrong ownership direction for leaf policy helpers
  - leave the helper in `RS-CODE` — rejected because `deny`, `cargo`, `clippy`, `toolchain`, hook checks, and Rust-enforced TS policies would duplicate or drift

### Keep extraction local and move only text-quality policy
- **Chose:** leave `same_line_reason(...)` inside `RS-CODE` and move only `validate_reason_text(...)` / `reason_text_is_useful(...)`
- **Why:** parsing where a reason lives is family- and syntax-specific; deciding whether the extracted text is acceptable is shared policy
- **Alternatives considered:**
  - move comment parsing too — rejected because other families may extract reasons from different surfaces than Rust trailing comments
  - expose a single giant helper that parses and validates — rejected because it would couple unrelated source formats to one family’s comment parser

### Fail weak reasons as errors on the active bypass lanes
- **Chose:** treat weak reasons as errors in `RS-CODE-03`, `RS-CODE-06`, `RS-CODE-22`, and `RS-CODE-24`, while `RS-CODE-04` only inventories or warns on reasons that pass the shared quality check
- **Why:** once a reason is too weak to justify an escape hatch, it should not count as documented
- **Alternatives considered:**
  - downgrade weak reasons to warnings — rejected because that still blesses low-signal escape hatches
  - keep the old presence-only model — rejected because it allows agents to satisfy policy with meaningless placeholders

### Use a simple initial shared contract
- **Chose:** require at least 12 trimmed characters, at least 2 words, and reject a small placeholder blocklist (`temp`, `temporary`, `legacy`, `todo`, etc.)
- **Why:** this is strict enough to reject the common junk reasons already present in the repo while staying simple and deterministic
- **Alternatives considered:**
  - immediately add category-aware or per-rule-specific validators — rejected as overfitting before other families migrate
  - require 3+ words like `RS-CODE-32` test expect messages — rejected because bypass reasons can be concise but still valid

## Architectural Notes
- `packages/reason-policy` is a leaf utility workspace, not a runtime orchestration layer.
- The shared crate is intentionally Rust-only for now because TypeScript family policy is still enforced by Rust code in this repo.
- The right reuse boundary is:
  - family-local extraction/parsing
  - shared text-quality validation
- This keeps future migrations straightforward: other families can call into the shared validator once they extract their own escape-hatch reason text.

## Information Sources
- `apps/guardrail3/crates/app/rs/runtime/Cargo.toml` and `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` for dependency direction and why `app/rs/runtime` is the wrong home
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/comments.rs` for the prior presence-only reason model
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/inventory/rs_code_32_test_expect_message_quality.rs` for the existing “useful text” precedent
- live `RS-CODE` validation runs against this repo to confirm weak reasons like `CLI command` would now flip into the new error lane

## Open Questions / Future Considerations
- Other families still have local escape-hatch surfaces that should migrate to `packages/reason-policy`.
- The current placeholder list is intentionally small; after cross-family rollout we may want a richer shared `ReasonIssue -> diagnostic message` mapping.
- If the repo ever wants non-Rust consumers, the workspace root can add shared fixtures/spec files without changing the current crate API.

## Key Files for Context
- `packages/reason-policy/Cargo.toml` — top-level workspace for the new shared reason-policy utility
- `packages/reason-policy/crates/reason-policy/src/lib.rs` — canonical shared reason-quality validator
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lint_policy/rs_code_03_item_level_allow_without_reason.rs` — weak allow/expect reasons now fail here
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lint_policy/rs_code_04_item_level_allow_with_reason.rs` — only useful reasons remain in the documented warning lane
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/hygiene/rs_code_06_garde_skip_with_comment.rs` — documented `garde(skip)` now depends on the shared validator
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lint_policy/rs_code_22_deny_forbid_without_reason.rs` — local lint-policy overrides now reject weak reasons
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/cfg_and_paths/rs_code_24_path_attr.rs` — `#[path]` reasons now share the same quality floor
- `.worklogs/2026-03-30-210029-reason-policy-code-bypass.md` — this worklog

## Next Steps / Continuation Plan
1. Search the other Rust families for escape-hatch surfaces that rely on local `allow`, `ignore`, exception comments, or reason strings.
2. For each family, separate extraction from validation and switch only the text-quality decision to `guardrail3-reason-policy`.
3. After the first cross-family pass, tighten the shared crate API if needed so families can report richer diagnostics without inventing custom reason-quality heuristics again.
