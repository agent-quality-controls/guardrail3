## Summary

Fixed adversarial findings from the independent verifier review. The repo hook no longer wires G3TS with an invalid repo-root scope, and verifier source checks no longer pretend to prove mode parsing through raw source text.

## Decisions

- Kept `.githooks/pre-commit` Rust-only in this repo because the active repo scope is Rust guardrails, not a TypeScript app workspace.
- Removed unprovable static verifier mode assertions from hook source checks. Mode behavior is now covered by executable script tests for the shipped verifier scripts.
- Extended cross-tool checks to catch direct neighbor verifier path calls such as `scripts/g3ts/verify` from the Rust verifier and `scripts/g3rs/verify` from the TypeScript verifier.
- Added Rust script behavior tests matching the TypeScript behavior coverage.

## Key Files

- `.githooks/pre-commit`
- `.plans/2026-05-06-130854-independent-verifier-guarantees.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run_tests/cases.rs`

## Next Steps

- Run a second adversarial review against the updated plan and implementation.
- Commit only after the hook and package tests remain clean.
