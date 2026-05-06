Summary
- Fixed the remaining independent verifier review gaps for G3TS and G3RS hook checks.
- G3TS path validation no longer accepts unrelated suffix paths, category detection works for scoped app crawls, and category command checks no longer accept echoed tokens or `prettier --write`.
- G3RS cargo dupes threshold and test-exclusion contracts now report separate rule IDs.

Decisions made
- Kept forbidden-command checks on executable command facts, but used relaxed category command scanning for verifier category discovery because the shipped G3TS verifier routes concrete commands through local shell functions.
- Changed `scripts/g3ts/verify` from a final `case` dispatcher to a single top-level `run_verifiers` call after the pre-commit staged-file gate so parser-backed source checks can prove the real script.
- Split cargo dupes thresholds and `--exclude-tests` into separate Rust hook contracts because they are separate assertions and need separate diagnostics.

Key files for context
- `scripts/g3ts/verify`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/commands.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`

Next steps
- Run another adversarial review pass against `.plans/2026-05-06-130854-independent-verifier-guarantees.md` and the modified hook packages.
- If reviewers find no remaining gaps, keep the verifier work as the convergence point for the independent G3RS/G3TS scripts.
