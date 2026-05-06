## Summary

Integrated the independent verifier plan into the app-owned pre-commit hook. The hook keeps repository-owned shared checks inline and delegates Rust and TypeScript language checks to `scripts/g3rs/verify` and `scripts/g3ts/verify` as separate lines.

## Decisions

- Kept `.githooks/pre-commit` as app-owned shell composition because `g3rs` and `g3ts` must stay independent.
- Replaced inline Rust and TypeScript check blocks with separate verifier calls to avoid duplicating language policy in the hook.
- Used `--scope "$REPO_ROOT"` for the TypeScript verifier in this repo because the TypeScript guardrail packages are spread under the repository root rather than one app directory.
- Removed superseded verifier plans from the working tree and kept only the active guarantee-first plan.

## Key Files

- `.plans/2026-05-06-130854-independent-verifier-guarantees.md`
- `.githooks/pre-commit`
- `scripts/g3rs/verify`
- `scripts/g3ts/verify`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- `packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs`

## Next Steps

- Run adversarial review against the active plan and implementation.
- Fix any reported coupling, missing tests, or misleading verifier rule messages.
