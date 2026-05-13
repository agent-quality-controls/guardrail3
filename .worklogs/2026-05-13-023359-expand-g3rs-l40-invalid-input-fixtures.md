# Summary

Expanded the G3RS behavior replay L40 layer so required inputs that exist but are invalid are replayed as closed fixtures.

The L40 root-config fixture now covers malformed `rust-toolchain.toml`, `rustfmt.toml`, `clippy.toml`, `deny.toml`, `.cargo/mutants.toml`, `release-plz.toml`, and `cliff.toml`. Added separate L41 and L42 fixtures for malformed root `Cargo.toml` and malformed member `Cargo.toml` because those parse failures shadow different family branches.

# Decisions

- Kept malformed root `Cargo.toml` separate because it prevents most families from constructing normal rule inputs.
- Kept malformed member `Cargo.toml` separate because root discovery still works and exposes topology, cargo, and release fail-closed behavior for descendant inputs.
- Changed L40 to `--rules-only` so malformed required inputs are tested without cargo gate noise from broken toolchain or rustfmt files.
- Reused the known valid L50 `clippy.toml` and `deny.toml` policy files in L41 and L42 so these fixtures isolate Cargo parse failures instead of leaking delegated policy failures.

# Key Files

- `.plans/2026-05-13-022617-expand-g3rs-l40-invalid-input-fixtures.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L40-required-inputs-present-invalid`
- `behavior/fixtures/g3rs/L41-root-cargo-toml-invalid`
- `behavior/fixtures/g3rs/L42-member-cargo-toml-invalid`
- `behavior/baselines/g3rs/L40-required-inputs-present-invalid/command-00.json`
- `behavior/baselines/g3rs/L41-root-cargo-toml-invalid/command-00.json`
- `behavior/baselines/g3rs/L42-member-cargo-toml-invalid/command-00.json`

# Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/*.py`
- `git diff --check`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`

# Next Steps

- Continue to the next fixture layer only after identifying which additional behaviors are unshadowed by L40-L42.
