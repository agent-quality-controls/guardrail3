# Summary

Documented the hook-contract architecture for G3RS and G3TS, then wrote a Rust-first implementation plan for migrating G3RS hooks away from hardcoded family policy.

# Decisions Made

- Added a dedicated `g3rs-<family>-hook-contract` package pattern because ingestion, checks, and types are not the right owner for hook policy.
- Kept `hook_contract()` parameterless until a concrete implemented rule proves parameterization is unavoidable.
- Planned Rust migration before TypeScript hooks so G3RS does not remain the legacy model.
- Planned to keep old hardcoded G3RS hook rules until contract-driven replacements have parity tests.

# Key Files

- `.plans/2026-04-28-185643-g3ts-hooks-family.md`
- `.plans/2026-04-28-192015-g3rs-hook-contract-implementation.md`

# Next Steps

- Implement slice 1: shared hook contract types, `g3rs-fmt-hook-contract`, contract aggregation in hooks orchestration, and `g3rs-hooks/required-contract-command-present` for `CargoFmtCheck`.
- Run G3RS tests and adversarial review against the plan before removing any legacy hardcoded hook rules.
