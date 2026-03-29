# Continue RS-DENY Hardening

**Date:** 2026-03-29 13:01
**Scope:** `apps/guardrail3/crates/app/rs/families/deny/**`

## Summary
Continued the `RS-DENY` hardening pass by tightening multiple rule implementations and attack tests around coverage ownership, local override precedence, fail-closed behavior, and parity checks. This commit is a rule-hardening checkpoint, not the final deny-family cleanup: library tests are green, but family self-hosting still has remaining `RS-DENY-01` and `RS-TEST-16` debt.

## Context & Problem
The `deny` family had already been structurally migrated, but the adversarial follow-up left a broad set of in-progress changes across many rules and sidecars. Those edits were coherent enough to preserve, and they materially improve the rules, but they do not yet deliver a fully zero-finding self-host on the family root.

The main constraint here was to separate real rule/attack hardening from unrelated worktree leftovers. I kept the commit scoped to `deny` only and left the unrelated lockfile and formatting churn out for a later cleanup commit.

## Decisions Made

### Preserve the in-progress `deny` rule hardening as its own checkpoint
- **Chose:** commit the `deny` family changes now instead of waiting for the remaining self-hosting debt to be fixed.
- **Why:** the work already strengthens the rules and expands the attack matrix across many files; leaving it mixed in the worktree would make later cleanup harder to audit.
- **Alternatives considered:**
  - Hold everything until `deny` reaches full `0/0/0` self-validation — rejected because that would bury a substantial, already-useful hardening sweep inside a much later commit.
  - Fold `deny` together with unrelated `deps`/`fmt`/`hexarch` leftovers — rejected because they are separate scopes and would make the history less legible.

### Keep the remaining self-hosting failures visible
- **Chose:** record the still-open `RS-DENY-01` and `RS-TEST-16` failures in the worklog instead of trying to mask them.
- **Why:** these are real issues. `RS-DENY-01` still does not treat the family-owned package roots as covered, and the stricter `RS-TEST-16` parser now catches many sidecars that still own semantic assertions directly.
- **Alternatives considered:**
  - Downgrade or bypass the checks to make the family appear clean — rejected because it would hide real debt.

## Architectural Notes
This checkpoint keeps the deny family on the current migrated shape:
- rule code remains in `crates/runtime`
- proof helpers remain in `crates/assertions`
- generic fixtures remain in `test_support`

The changes in this commit mostly refine rule semantics and owned attack coverage. They do not yet finish the family’s self-hosting story. There are still two distinct remaining tasks:
- fix `RS-DENY-01` coverage ownership for the family’s own package roots
- move the newly exposed semantic assertions out of deny sidecars to satisfy `RS-TEST-16`

## Information Sources
- `apps/guardrail3/crates/app/rs/families/deny/**` — live family runtime, assertions, and sidecar tests
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny --lib`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/deny --family deny --inventory --format json`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/deny --family test --inventory --format json`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/deny --family arch --inventory --format json`

## Open Questions / Future Considerations
- `RS-DENY-01` still reports the family’s own `crates/runtime`, `crates/assertions`, and `test_support` roots as uncovered.
- `RS-TEST-16` now reports 33 deny-family sidecars that still own semantic result assertions.
- The unrelated `Cargo.lock` diff and tiny formatting-only edits in `deps`, `fmt`, and `hexarch` remain intentionally outside this commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/rs_deny_01_coverage.rs` — family-root coverage logic and current self-hosting blocker
- `apps/guardrail3/crates/app/rs/families/deny/crates/assertions/src/common.rs` — shared deny assertion helpers touched by the hardening pass
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts_tests/mod.rs` — deny facts-level test coverage
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — checker now exposing deny sidecar semantic ownership debt
- `.worklogs/2026-03-27-214045-rs-deny-stabilization.md` — earlier structural deny migration
- `.worklogs/2026-03-27-220018-rs-deny-policy-and-schema-hardening.md` — earlier policy/schema hardening decisions

## Next Steps / Continuation Plan
1. Fix `RS-DENY-01` so the deny family’s own package roots are recognized as covered by the family-root deny config without relaxing allowed-location/shadowing semantics.
2. Use the live `RS-TEST-16` output for `families/deny` as the migration checklist and move semantic result assertions out of each flagged sidecar into the sibling `crates/assertions` modules.
3. Re-run:
   - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny --lib`
   - `apps/guardrail3/target/debug/guardrail3 rs validate .../families/deny --family deny --inventory --format json`
   - `apps/guardrail3/target/debug/guardrail3 rs validate .../families/deny --family test --inventory --format json`
4. Only after both families (`deny` and `test`) are clean for the family root, run another adversarial pass focused on local override ownership and malformed profile fail-closed behavior.
