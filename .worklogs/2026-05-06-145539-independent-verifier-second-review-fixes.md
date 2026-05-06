Summary

- Fixed second-pass adversarial findings for independent verifier guarantees.
- G3RS now resolves relative scopes from the git root and treats dot-prefixed Rust config files as relevant pre-commit triggers.
- G3TS hook checks now use typed optional-category facts and exact parsed `--path` values instead of accepting broad scope substrings.

Decisions Made

- Kept mode behavior as executable script tests instead of static source-shape checks.
- Split broad G3RS verifier diagnostics into assertion-specific rule IDs so each finding names one missing or forbidden verifier behavior.
- Modeled G3TS optional verifier categories as typed source-check input facts because style/package/typecov requirements depend on scope state.

Key Files For Context

- scripts/g3rs/verify
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run_tests/cases.rs
- packages/ts/hooks/g3ts-hooks-types/src/types.rs
- packages/ts/hooks/g3ts-hooks-ingestion/crates/runtime/src/run.rs
- packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/commands.rs
- packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run.rs
- packages/ts/hooks/g3ts-hooks-source-checks/crates/runtime/src/run_tests/cases.rs

Verification

- cargo test --manifest-path packages/ts/hooks/g3ts-hooks-source-checks/Cargo.toml --workspace
- cargo test --manifest-path packages/ts/hooks/g3ts-hooks-ingestion/Cargo.toml --workspace
- cargo test --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace
- cargo test --manifest-path packages/rs/hooks/g3rs-hooks-ingestion/Cargo.toml --workspace
- bash -n scripts/g3rs/verify scripts/g3ts/verify .githooks/pre-commit
- g3rs validate --path apps/guardrail3-rs

Next Steps

- Run another adversarial pass against the original verifier plan and current code.
- Commit only if the adversarial pass converges or after fixing any remaining findings.
