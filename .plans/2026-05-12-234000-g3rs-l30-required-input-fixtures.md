# Goal

Add behavior replay coverage for every `g3rs validate` rule that can be exposed when:

- a Rust workspace root exists
- `guardrail3-rs.toml` exists and parses
- family execution is no longer blocked
- required family input files or activation configs are missing

# Approach

- Keep L30 focused on missing required inputs.
- Use separate fixtures when one missing input would hide another rule.
- Use family-filtered commands so the baseline shows the exact family rule instead of a noisy all-family dump.
- Make each fixture source file facade-only unless the fixture specifically needs source content to activate a rule.
- Add fixture-local PATH support for test mutation fixtures so `cargo-mutants` is controlled by the fixture, not by the host machine.
- Make all L30 fixtures baseline-required.

# Fixture Contract

- `L30-guardrail-config-valid-required-inputs-missing`
  - Valid workspace root.
  - Valid `guardrail3-rs.toml`.
  - Missing `rust-toolchain.toml`.
  - Missing `rustfmt.toml`.
  - Missing workspace lint tables.
  - Missing clippy config.
  - Missing deny config.
  - Missing `Cargo.lock` for service profile.

- `L31-release-required-inputs-missing`
  - Valid publishable crate.
  - Missing repo `LICENSE`.
  - Missing `release-plz.toml`.
  - Missing `cliff.toml`.
  - Missing crate `README.md`.

- `L32-test-required-inputs-missing`
  - Valid Rust workspace with mutation testing active through `[profile.mutants]`.
  - Mutation testing active through `[profile.mutants]`.
  - Fixture-local fake `cargo-mutants` exists.
  - Missing `.cargo/mutants.toml`.
  - Missing executable mutation hook step.

- `L33-test-mutants-profile-missing`
  - Valid Rust workspace.
  - Mutation testing active through `.cargo/mutants.toml`.
  - Fixture-local fake `cargo-mutants` exists.
  - Missing `[profile.mutants]`.
  - Missing executable mutation hook step.

# Files To Modify

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs/L30-guardrail-config-valid-required-inputs-missing`
- `behavior/fixtures/g3rs/L31-release-required-inputs-missing`
- `behavior/fixtures/g3rs/L32-test-required-inputs-missing`
- `behavior/fixtures/g3rs/L33-test-mutants-profile-missing`
- `behavior/baselines/g3rs`
- `scripts/behavior/baseline_common.py`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-fixtures.py`

# Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/*.py`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`
