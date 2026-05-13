# Goal

Reduce the G3RS behavior replay fixtures to the minimum fixture count that still exposes every intended rule branch at each behavior layer.

The target is not fewer files at any cost. The target is maximum independent signal per fixture without shadowing.

# Scope

Audit and compact the behavior fixture layers currently implemented:

- `L00`
- `L10`
- `L20`
- `L30` through `L39`
- `R00` through `R30` under `behavior/fixtures/g3rs-validate-repo`

# Merge Rule

Merge fixtures when all of these are true:

- The failures are visible at the same behavior layer.
- The failures do not prevent each other from being emitted.
- The combined baseline has no Error or Warn rows except rows listed in `required_results`.
- The combined fixture does not need a deeper valid structure than either original fixture.
- The combined fixture still has a clear reason to exist.

# Split Rule

Split fixtures when any of these are true:

- One failure prevents another failure from being emitted.
- One failure changes parsing or discovery so another branch is not reachable.
- The fixture emits any Error or Warn row that is not part of that fixture's required contract.
- The fixture needs two mutually exclusive states.
- The fixture proves a higher-layer behavior while claiming to be a lower-layer fixture.

# Final Fixture Set

- `L30-guardrail-config-valid-required-inputs-missing`
  - Covers missing modern toolchain file, legacy-only toolchain file, missing rustfmt config, nested rustfmt override, missing workspace lints, missing clippy coverage, missing deny coverage, missing Cargo.lock, ignored Cargo.lock, and garde checks that cannot run without clippy policy inputs.
  - Does not cover test mutation config because the missing Cargo.lock state prevents that branch from being emitted.

- `L31-root-policy-conflicts-and-nextest-inputs`
  - Covers root rustfmt dual-file conflict, nightly rustfmt key without modern toolchain config, clippy same-root conflict, deny same-root conflict, and missing nextest config.
  - Uses otherwise valid clippy and deny config bodies so the fixture does not emit unrelated clippy or deny policy errors.

- `L32-test-required-inputs-missing`
  - Covers missing `.cargo/mutants.toml` and missing mutation hook.
  - Remains separate because adding `.cargo/mutants.toml` is required for the later mutation-profile branch and would hide this branch.

- `L33-release-profile-and-mutants-inputs`
  - Covers both toolchain files present, missing library dependency allowlist, missing garde dependency, missing mutation profile, missing mutation hook, and missing release files.
  - Remains separate from `L30` because it needs Cargo.lock present and `.cargo/mutants.toml` present.

- `L37-workspace-topology-input-conflicts`
  - Covers nested workspace, nested guardrail config, missing declared member, undeclared child workspace, escaping member path, workspace-local file placement, and cargo missing-member warnings.
  - Does not run release, deps, fmt, garde, or test because topology path escape and missing members pollute those families with unrelated missing-input errors.

# Removed Fixtures

- `L30-toolchain-legacy-file-present` merged into `L33-release-profile-and-mutants-inputs`.
- `L30-toolchain-legacy-only-file-present` merged into `L30-guardrail-config-valid-required-inputs-missing`.
- `L30-fmt-dual-file-conflict` merged into `L31-root-policy-conflicts-and-nextest-inputs`.
- `L30-fmt-per-crate-override` merged into `L30-guardrail-config-valid-required-inputs-missing`.
- `L30-clippy-same-root-conflict` merged into `L31-root-policy-conflicts-and-nextest-inputs`.
- `L30-deny-shadowing` merged into `L31-root-policy-conflicts-and-nextest-inputs`.
- `L30-release-member-input-failures` removed from the L30-L39 layer because release input failures from missing workspace members are pollution from topology-invalid state.
- `L30-topology-*` branch fixtures merged into `L37-workspace-topology-input-conflicts`.
- `L33-test-mutants-profile-missing` merged into `L33-release-profile-and-mutants-inputs`.
- `L34-deps-library-policy-missing` merged into `L33-release-profile-and-mutants-inputs`.
- `L35-garde-dependency-missing` merged into `L33-release-profile-and-mutants-inputs`.
- `L36-garde-clippy-inputs-missing` merged into `L30-guardrail-config-valid-required-inputs-missing`.
- `L38-fmt-nightly-toolchain-input-missing` merged into `L31-root-policy-conflicts-and-nextest-inputs`.
- `L39-test-nextest-input-missing` merged into `L31-root-policy-conflicts-and-nextest-inputs`.

# Required Verification

For every proposed merge:

- Run the merged fixture command manually before editing the manifest.
- Confirm every Error and Warn row is intended.
- Add every intended Error and Warn row to `required_results`.
- Reject the merge if any unlisted Error or Warn row appears.

After edits:

- Regenerate baselines with `python3 scripts/behavior/generate-baselines.py`.
- Run `scripts/behavior/verify-all.sh`.
- Run a pollution scan that fails if any L00-L39 or R00-R30 Error/Warn row is absent from `required_results`.
- Run `g3rs validate --path apps/guardrail3-rs`.
- Run `g3rs validate-repo`.
- Send adversarial review against this plan, the manifest, fixtures, baselines, and active rule code.

# Files To Modify

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-235800-g3rs-l30-adversarial-gaps.md`
- `.plans/2026-05-12-235800-g3rs-l30-adversarial-gaps.md.manifest.toml`
- `behavior/fixtures/g3rs`
- `behavior/baselines/g3rs`
- `scripts/behavior/verify-fixtures.py` only if fixture metadata needs a real state vocabulary change
