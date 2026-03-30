# RS-TEST Implementation Handoff

Owner root:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test`

## What This Handoff Was For

This was a family-closure lane, not a fresh inventory lane.

`RS-TEST` is now closed as a family-local implementation lane, but it is still a valid hardening lane. The remaining work is mostly cross-family follow-up plus adversarial tightening against bypasses and false positives.

Do **not** invent new `RS-TEST-*` rules unless the plan is explicitly expanded.

Priority:

1. harden the live family against concrete bypasses and false positives
2. resolve stale plan language vs live code
3. fix any semantic bugs discovered in that attack pass

## Read First

- `/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/test.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/README.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/check_review/test_hardening/34-test-family-rewrite-agent-brief.md`

## Current Snapshot

Live code roots:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src`

Inventory status:

- `RS-TEST-01..18` are implemented

Closure status:

- `RS-TEST-01..18` are implemented
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib` passes
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family test --format json` currently returns `0` errors and `0` warnings
- the last family-local semantic bug found during closure was `RS-TEST-17` over-reporting success inventory for external harnesses that did not actually call owned assertions; that gap is now covered by a regression test
- generated/cache Rust under `target/**` is now excluded from owned-root analysis and should stay that way

Remaining follow-up is external to this lane:

- cargo still has assertions-owned runtime execution routing in `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/src/lib.rs`
- other families may still need their own semantic-ownership cleanup, but that is no longer `rs/test` implementation work

## Scope You Own

This packet is closed for implementation, but still useful for hardening.

What remains useful here:

- preserve the closure record
- keep the family-specific hardening backlog explicit
- point follow-up work at the external families that still need `RS-TEST`-driven cleanup
- keep the detailed ledger and README aligned if the family contract changes later

You do **not** own:

- rewriting other families just because `RS-TEST` checks them
- inventing new `RS-TEST-*` rules
- a repo-wide cleanup of every non-test family that currently fails `RS-TEST`

## Closure Notes

- `RS-TEST-16` is enforced and covered for proof-bearing exports plus sidecar semantic-proof leaks.
- `RS-TEST-17` now reports success inventory only when an external harness actually proves through owned assertions.
- `RS-TEST-18` is enforced for runtime/assertions imports, route-construction infrastructure, canned fixture helpers, and semantic result helpers.
- `RS-TEST-10` and `RS-TEST-14` remain part of the closure proof and passed in the latest family-local run.
- generated Rust under `target/**` is not part of owned-root analysis and should not be allowed to reactivate family rules.

## Current Hardening Targets

1. keep `RS-TEST-16/17/18` resistant to aliasing, wrapper, and indirect semantic-helper bypasses
2. keep non-owned/generated Rust surfaces out of family discovery
3. keep fail-closed inputs and executable-line hook parsing exact

## External Follow-Up

If work resumes from this packet, it is usually in another family, unless the work is a proven `rs/test` hardening fix.

Current concrete carry-forward:

1. remove cargo assertions-owned runtime execution routing in `apps/guardrail3/crates/app/rs/families/cargo/crates/assertions_common/src/lib.rs`
2. rerun `RS-TEST` against the touched family after that cleanup
3. update the touched family's README/plan rather than reopening `rs/test`

## Verify With

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family test --format json
```

## Done Meant

- `rs/test` no longer has material family-local closure debt hidden behind “implemented”
- the family remains clean on validator errors/warnings after hardening changes
- family tests pass
- the plan text matches the live implementation state
- any remaining cross-family follow-up is explicit and scoped outside this lane
