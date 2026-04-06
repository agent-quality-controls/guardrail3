# RS-DEPS Implementation Handoff

Owner root:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps`

## What This Handoff Is For

This is a real implementation lane, not a family-structure lane and not a test-only lane.

`RS-DEPS` is already live. This packet is now a closure record for the last inventory completion step and the hardening decisions that landed with it.

Do **not** spend this lane cleaning wider repo dependency findings unless a result proves the rule is wrong.

Priority that was completed here:

1. implement `RS-DEPS-CONFIG-05`
2. preserve existing family ownership boundaries
3. prove it with exact regressions
4. update the plan so `deps` is no longer inventory-incomplete

## Read First

- `/Users/tartakovsky/Projects/websmasher/guardrail3/AGENTS.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/deps.md`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps/README.md`

## Current Snapshot

Live code roots:

- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/deps/crates/assertions/src`

Current inventory status:

- `RS-DEPS-01..12` implemented

Planned contract:

- more than `25` unique direct dependency names on one crate across direct dependency sections/tables

Explicit remaining family caveat:

- target-specific dependency tables are still outside `RS-DEPS-CONFIG-01..07`
- `RS-DEPS-CONFIG-05` explicitly owns target-specific direct-dependency counting

## Scope You Own

This lane owned:

- implementing `RS-DEPS-CONFIG-05`
- the minimal dependency inventory/fact extensions required for correct counting
- exact threshold-edge regressions
- plan cleanup in `deps.md`

You do **not** own:

- release-family dependency policy
- banned-crate lockfile policy beyond `RS-DEPS`
- a broad rewrite of `RS-DEPS-CONFIG-01..11`

## Main Rule To Implement

### `RS-DEPS-CONFIG-05`

- one finding per crate/root whose direct dependency count exceeds `25`
- count unique direct dependency names
- count across direct dependency sections/tables that are part of the intended contract
- do not double-count renamed aliases when they resolve to the same package name
- be explicit about how `workspace = true` and workspace-path/internal dependencies affect the count

Resolved behavior now frozen in code/tests:

- target-specific dependency tables are counted for this rule
- renamed aliases deduplicate by resolved package name
- external `workspace = true` dependencies count
- internal workspace-path dependencies do not count
- non-workspace vendored external path dependencies do count
- malformed dependency inputs fail closed through `RS-DEPS-11` instead of producing a partial direct-dependency count

## Architecture Constraints

Stay inside the live family architecture:

- `crates/runtime/src/facts.rs`
- `crates/runtime/src/inputs.rs`
- one rule file for `RS-DEPS-CONFIG-05`
- rule-specific sidecar tests

Do not:

- collapse the rule into `RS-DEPS-CONFIG-01..07`
- broaden target-specific table policy silently
- count non-owned/internal/workspace path dependencies just to reach the cap

## Suggested Execution Order

Completed execution shape:

1. dependency normalization was extended once in family facts instead of per-rule parsing
2. `RS-DEPS-CONFIG-05` was added as a dedicated rule file
3. regressions now cover:
   - exactly `25`
   - `26`
   - duplicate aliases
   - workspace/path/internal edges
   - malformed manifests that must fail closed
4. docs were updated to remove the planned-only state

## Verify With

```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deps --lib

cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family deps --format json
```

## Outcome

- `RS-DEPS-CONFIG-05` is implemented
- family tests pass
- the detailed deps ledger no longer marks the rule planned
- target-table ownership is explicit: counted for `RS-DEPS-CONFIG-05`, still out of scope for `RS-DEPS-CONFIG-01..07`
