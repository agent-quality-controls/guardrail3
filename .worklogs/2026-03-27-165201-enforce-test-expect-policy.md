# Enforce Test Expect Policy

**Date:** 2026-03-27 16:52
**Scope:** `apps/guardrail3/crates/domain/modules/clippy`, `apps/guardrail3/crates/app/rs/families/clippy`, `apps/guardrail3/crates/app/rs/families/code`, `apps/guardrail3/tests/fixtures/r_arch_01/golden`

## Summary
Added explicit Clippy policy for test-only `expect`/`unwrap` behavior and introduced a new `RS-CODE-32` rule that enforces useful `expect(...)` messages in test contexts only. This completes the ownership split: Clippy decides where `expect` is allowed, `RS-CODE` decides whether the allowed test message is actually worth keeping.

## Context & Problem
After removing `RS-CODE-14`, the repo no longer duplicated Clippy’s raw `unwrap()` / `expect()` detection. That left one unresolved requirement from the user:

1. `expect()` should be allowed in tests but not in normal code.
2. test `expect(...)` messages must still be useful.

Clippy already solves the first problem with `expect_used`, `unwrap_used`, `allow-expect-in-tests`, and `allow-unwrap-in-tests`. It does not solve the second problem, because it cannot judge whether a test message like `"ok"` or `"mkdir"` is informative.

So the right architecture was:
- enforce the allow/deny surface in `RS-CARGO` + `RS-CLIPPY`
- keep only the message-quality rule in `RS-CODE`

## Decisions Made

### Put test allowance policy in Clippy config, not source scanning
- **Chose:** require `allow-expect-in-tests = true` and `allow-unwrap-in-tests = false` in generated and validated `clippy.toml`.
- **Why:** this is exactly what Clippy’s built-in configuration supports. Re-implementing it in custom AST code would duplicate tool-native capability again.
- **Alternatives considered:**
  - Reintroduce source scanning for production-vs-test `expect()` placement — rejected because it duplicates Clippy.
  - Put the allow/deny policy in `RS-CODE` only — rejected because policy belongs in lint/config ownership, not AST scanning.

### Add one narrow custom rule for test `expect(...)` message quality
- **Chose:** add `RS-CODE-32` for test-context `expect(...)` messages only.
- **Why:** message usefulness is the part Clippy cannot enforce. Keeping the rule narrow preserves the clean separation from Clippy-owned allow/deny behavior.
- **Alternatives considered:**
  - No custom rule at all — rejected because it would allow junk messages like `"ok"` and `"mkdir"`, which defeats the point of allowing `expect()` in tests.
  - A general `expect()` scanner across all code — rejected because production placement is already Clippy’s job.

### Keep the rule strict but fix local weak examples rather than weakening it
- **Chose:** tighten a shared golden fixture that used `expect("plan")` and keep the rule flagging similar low-signal messages.
- **Why:** the new repo-wide sample showed the rule catching exactly the kind of junk messages we want to eliminate (`"strip"`, `"ft"`, `"tempdir"`, `"mkdir"`, `"symlink"`). Weakening the rule to accommodate obviously weak local fixtures would have been backwards.
- **Alternatives considered:**
  - Loosen the rule until `"plan"` passed — rejected because that would also legitimize many real junk messages.
  - Keep `"valid id"` / `"valid job"` as failures — rejected because those are short but still specific enough to be workable in tests.

## Architectural Notes
The resulting ownership split is now:

- `RS-CARGO`
  - requires `clippy::expect_used = "deny"`
  - requires `clippy::unwrap_used = "deny"`
- `RS-CLIPPY`
  - requires `allow-expect-in-tests = true`
  - requires `allow-unwrap-in-tests = false`
  - keeps debug/print test relaxations off
- `RS-CODE`
  - enforces only the custom quality rule for test `expect(...)` messages

`RS-CODE-32` applies in:
- real test files
- `#[cfg(test)]` islands inside normal files

It does **not** police normal-code `expect(...)`; that is intentional and left to Clippy.

## Information Sources
- Local code:
  - `apps/guardrail3/crates/domain/modules/clippy/settings.rs`
  - `apps/guardrail3/crates/app/rs/families/clippy/src/rs_clippy_17_test_relaxations.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs`
  - `apps/guardrail3/tests/fixtures/r_arch_01/golden/apps/backend/crates/app/commands/src/lib.rs`
- Official Clippy docs:
  - `unwrap_used` docs, including `allow-unwrap-in-tests` config
  - `expect_used` docs
  - Clippy lint configuration docs
- Prior worklogs:
  - `.worklogs/2026-03-27-163642-remove-rs-code-14-duplicate-clippy-scan.md`
  - `.worklogs/2026-03-27-161428-rs-code-attack-fixes.md`

## Open Questions / Future Considerations
- `RS-CODE-32` currently accepts literal strings and `concat!` of string literals, and rejects indirect expressions. That is deliberate for auditability, but if the repo strongly prefers named constants for test messages, the rule may need a later expansion.
- Repo-wide `RS-CODE-32` is already a large live bucket (`426` hits in the sampled inventory). The sampled hits looked legitimate, but this should be treated as real cleanup pressure, not just a detector experiment.
- `guardrail3-domain-modules` still has an unrelated broken `cspell_tests.rs` / `serde_json` test issue; it was observed during this slice but not changed here.

## Key Files for Context
- `apps/guardrail3/crates/domain/modules/clippy/settings.rs` — canonical generated Clippy settings, now including test `expect`/`unwrap` policy.
- `apps/guardrail3/crates/app/rs/families/clippy/src/rs_clippy_17_test_relaxations.rs` — enforcement of exact test-relaxation config policy.
- `apps/guardrail3/crates/app/rs/families/clippy/src/rs_clippy_13_local_policy_root_baseline.rs` — local policy-root completeness, now requiring the new test keys too.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/rs_code_32_test_expect_message_quality.rs` — new custom rule for useful test `expect(...)` messages.
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/parse/visitors.rs` — AST visitor for test-context `expect(...)` call discovery.
- `.plans/todo/checks/rs/code.md` — updated code-family rule inventory.
- `.plans/todo/checks/rs/clippy.md` — updated Clippy-family policy inventory.
- `.worklogs/2026-03-27-163642-remove-rs-code-14-duplicate-clippy-scan.md` — immediate predecessor establishing the ownership split.

## Next Steps / Continuation Plan
1. Sample more of the live `RS-CODE-32` repo findings and decide whether any additional message heuristics are needed, but only if concrete false positives show up.
2. Decide whether to stabilize `rs/release`, `rs/garde`, or a cargo-family self-compliance pass next.
3. If `RS-CODE-32` turns out to be the next large repo cleanup campaign, batch-fix obvious junk messages in test files (`"ok"`, `"mkdir"`, `"tempdir"`, `"symlink"`, etc.) before touching subtler buckets.
