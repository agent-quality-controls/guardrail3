# Goal

Expand L40 behavior replay coverage to cover required inputs that exist but are invalid.

# Layer Boundary

L40 starts after:

- workspace root exists
- `guardrail3-rs.toml` is valid
- required files exist

L40 must stop before:

- delegated tool missing checks
- delegated tool policy failures
- project source policy failures

# Fixture Strategy

- Keep `L40-required-inputs-present-invalid` for malformed root policy/config files that can coexist without hiding each other:
  - `rust-toolchain.toml`
  - `rustfmt.toml`
  - `clippy.toml`
  - `deny.toml`
- Add a separate malformed root `Cargo.toml` fixture because a broken root manifest shadows most family discovery.
- Add a separate malformed test config fixture for `.cargo/mutants.toml` if it produces a visible test-family input failure without leaking L50+ behavior.
- Add a separate malformed release config fixture for `release-plz.toml` and `cliff.toml` if those produce visible release-family input failures without leaking L50+ behavior.
- Add a separate malformed descendant/member `Cargo.toml` fixture only if it produces topology or member-family input failures without breaking root workspace discovery.

# Verification Contract

- Every L40 Error/Warn row must be listed in `required_results`.
- L40 baselines must not contain volatile paths, timings, or target hashes.
- L40 fixtures must not emit:
  - delegated tool missing findings
  - cargo gate failures caused by intentionally invalid project source
  - project-policy source findings unrelated to invalid config inputs

# Files To Modify

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `behavior/fixtures/g3rs`
- `behavior/baselines/g3rs`
- `scripts/behavior/verify-baselines.py` only if the existing closed-finding verifier cannot express the L40 contract.
