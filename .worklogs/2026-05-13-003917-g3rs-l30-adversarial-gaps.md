# Summary

Expanded the G3RS behavior replay L30 layer after adversarial review found missing same-layer checks.

The L30-L39 fixture set now covers missing required inputs, activation-file conflicts, misplaced workspace-local config files, release ingestion failures, and workspace topology facts that are visible after a valid `guardrail3-rs.toml`.

# Decisions Made

- Added separate fixtures for each file-tree or topology branch instead of merging them into the base L30 fixture, because combined fixtures would hide which rule branch produced each finding.
- Kept clippy and deny conflict fixtures on full clean policy files, because minimal clippy/deny configs would trigger unrelated config-policy findings.
- Extended fixture metadata allowed invalid states with concrete L30 concepts: activation file conflict, workspace-local file misplaced, and workspace topology invalid.
- Kept `L32` focused on mutation-config inputs and moved nextest missing input to `L39`, because the earlier async-test shape polluted the layer with test architecture findings.

# Key Files For Context

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-235800-g3rs-l30-adversarial-gaps.md`
- `.plans/2026-05-12-235800-g3rs-l30-adversarial-gaps.md.manifest.toml`
- `behavior/fixtures/g3rs/L30-*`
- `behavior/fixtures/g3rs/L31-release-required-inputs-missing`
- `behavior/fixtures/g3rs/L32-test-required-inputs-missing`
- `behavior/fixtures/g3rs/L33-test-mutants-profile-missing`
- `behavior/fixtures/g3rs/L34-deps-library-policy-missing`
- `behavior/fixtures/g3rs/L35-garde-dependency-missing`
- `behavior/fixtures/g3rs/L36-garde-clippy-inputs-missing`
- `behavior/fixtures/g3rs/L37-workspace-member-inputs-missing`
- `behavior/fixtures/g3rs/L38-fmt-nightly-toolchain-input-missing`
- `behavior/fixtures/g3rs/L39-test-nextest-input-missing`
- `scripts/behavior/verify-fixtures.py`

# Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/baseline_common.py scripts/behavior/generate-baselines.py scripts/behavior/verify-baselines.py scripts/behavior/verify-fixtures.py`
- `git diff --check`
- L30-L39 pollution scan over generated baselines
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`
- Final adversarial review: PASS

# Next Steps

- Build the next behavior fixture layer only after extracting the active rules that become visible once required inputs are present but invalid.
