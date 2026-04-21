## Summary

Closed the `ts/jscpd` proof gaps found by the attack pass. The family behavior did not need architectural changes; this iteration strengthened the tests around unreadable-root handling, exact result payloads, selected-family ordering, and the real CLI wiring path.

## Decisions made

- Kept the fixes in tests and assertions only.
  - Why: the attack found proof debt, not a `ts/jscpd` rule bug.
  - Rejected: widening the family or changing production behavior just to satisfy the tests.

- Proved unreadable-root handling through the live crawl branch.
  - Why: the gap was specifically in ingestion behavior, so the right fix was a real unreadable fixture with `chmod 000` on Unix.
  - Rejected: constructing `Unreadable` state directly in config-check fixtures, because that would not prove ingestion.

- Moved selected-family proof to the `validate-command` layer.
  - Why: a `run` sidecar is not allowed to reach sibling CLI internals like `FamilyArg`.
  - Rejected: keeping a `run` sidecar test that parsed CLI args locally, because `g3rs validate` correctly flagged that as a boundary escape.

- Strengthened the real CLI wiring test to exact output.
  - Why: substring assertions were the last remaining false-positive risk in the app-path proof.

## Key files for context

- `.plans/2026-04-21-141526-ts-jscpd-attack-followup.md`
- `.worklogs/2026-04-21-140048-ts-jscpd-wave1.md`
- `packages/parsers/jscpd-json-parser/crates/runtime/src/parser_tests/cases.rs`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/assertions/src/run.rs`
- `packages/ts/jscpd/g3ts-jscpd-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/jscpd/g3ts-jscpd-config-checks/crates/runtime/src/run_tests/cases.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/selection_tests/cases.rs`
- `apps/guardrail3-ts/crates/logic/validate-command/crates/runtime/src/execute_tests/cases.rs`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/parsers/jscpd-json-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/jscpd/g3ts-jscpd-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/jscpd/g3ts-jscpd-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/parsers/jscpd-json-parser/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/jscpd/g3ts-jscpd-ingestion/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/jscpd/g3ts-jscpd-config-checks/Cargo.toml`
- `cargo fmt --all --check --manifest-path apps/guardrail3-ts/Cargo.toml`
- `g3rs validate --path packages/parsers/jscpd-json-parser`
- `g3rs validate --path packages/ts/jscpd/g3ts-jscpd-ingestion`
- `g3rs validate --path packages/ts/jscpd/g3ts-jscpd-config-checks`
- `g3rs validate --path apps/guardrail3-ts`

## Adversarial review

- Follow-up completeness re-review: no remaining concrete blocker.
- Follow-up app-wiring re-review: no remaining issue.

## Next steps

- `ts/jscpd` is converged for wave 1.
- Move to the next TS family, not more `jscpd` expansion, unless a new target repo exposes a real policy gap.
