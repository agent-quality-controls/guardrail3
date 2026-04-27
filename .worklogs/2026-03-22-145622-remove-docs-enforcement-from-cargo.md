# Remove Docs Enforcement From Cargo

**Date:** 2026-03-22 14:56
**Scope:** `.plans/todo/checks/rs/cargo.md`, `apps/guardrail3/crates/app/rs/checks/rs/cargo/lint_support.rs`

## Summary
Removed the accidental library-profile enforcement of `missing_docs = "deny"` from the new cargo checker and aligned the cargo plan doc with that policy. The cargo family now only treats `unreachable_pub = "deny"` as the extra library-profile Rust lint requirement.

## Context & Problem
After the cargo audit hardening commit, the user clarified that missing docs are explicitly allowed and should not be enforced anywhere. The new `g3rs-cargo/workspace-lints` implementation still carried a library-profile requirement for `missing_docs = "deny"`, inherited from earlier planning assumptions rather than the current policy.

This was a contract bug, not a test bug: the checker was enforcing something the project does not want.

## Decisions Made

### Remove `missing_docs` from library-profile cargo expectations
- **Chose:** Delete `missing_docs` from `EXPECTED_LIBRARY_RUST_LINTS`.
- **Why:** The user explicitly stated missing docs are allowed. The checker should not encode a documentation policy the project rejects.
- **Alternatives considered:**
  - Leave it in code and only loosen tests — rejected because the implementation would still be wrong.
  - Keep it as a warning instead of an error — rejected because the user said to skip the docs check entirely.

### Update the cargo plan doc immediately
- **Chose:** Edit `.plans/todo/checks/rs/cargo.md` so `g3rs-cargo/workspace-lints` says library profile adds only `unreachable_pub = "deny"` and explicitly notes that `missing_docs` is not enforced.
- **Why:** The plan should match the implemented contract. Leaving the old wording would reintroduce the same bug later.
- **Alternatives considered:**
  - Defer the doc fix — rejected because this is exactly the kind of drift we are cleaning up.

### Correct the stale clippy deny count while touching the plan
- **Chose:** Change the count from 30 to 31 in the same cargo plan row.
- **Why:** The canonical module and validator already agree on 31 deny lints after the prior hardening pass. Keeping the stale count would preserve a known inconsistency.
- **Alternatives considered:**
  - Leave the stale count for later — rejected because the line was already being updated and the count is part of the rule contract.

## Architectural Notes
This change reinforces the intended separation:
- policy belongs in the plan and expectation tables
- rules should derive from those expectations, not from stale historical assumptions

The cargo family still uses the same orchestrator/facts/input structure. This was a policy correction inside the expectation layer, not an architecture change.

## Information Sources
- User clarification in-session that “missing docs are allowed explicitly”
- `.plans/todo/checks/rs/cargo.md` — stale policy wording for `g3rs-cargo/workspace-lints`
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/lint_support.rs` — source of the extra library-profile Rust lint expectations
- `.worklogs/2026-03-22-145320-cargo-fmt-audit-hardening.md` — prior checkpoint that hardened cargo and exposed plan/contract drift issues

## Open Questions / Future Considerations
- If there are any remaining old validator paths that still treat missing docs as required, they should be removed when those families are migrated.
- The approved allow inventory still includes `missing_docs_in_private_items = "allow"`. That is compatible with “missing docs allowed”, but if the project later decides that docs policy should disappear entirely from cargo-lint expectations, that line can also be reconsidered.

## Key Files for Context
- `AGENTS.md` — current Rust-only project scope and architectural direction
- `.plans/todo/checks/rs/cargo.md` — current cargo family contract
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/lint_support.rs` — cargo lint expectation tables
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/rs_cargo_config_01_workspace_lints.rs` — rule consuming the expectation table
- `.worklogs/2026-03-22-145320-cargo-fmt-audit-hardening.md` — previous cargo audit checkpoint

## Next Steps / Continuation Plan
1. Continue with the next Rust family using the same flow: implement family shape, attack it against the plan, then migrate/adapt old adversarial tests where useful.
2. Prefer `rs/clippy` next, because the old adversarial config fixtures already contain relevant coverage and it remains a relatively contained config family.
3. After `rs/clippy`, do `rs/deny`, then start planning the migration path for the heavier arch/source families where the old unit-test corpus is much larger.
