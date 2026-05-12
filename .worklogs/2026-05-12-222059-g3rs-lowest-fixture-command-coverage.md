# Summary

Expanded L00-L20 G3RS behavior replay fixtures from one command per layer to every public `g3rs validate` command shape still observable before crawl, family execution, staged-file reads, or cargo gates.

The baseline set now contains 22 records: 8 for L00 and 7 each for L10 and L20.

# Decisions Made

- Added `validate --path README.md --inventory` to L00 because `execute` has a distinct non-directory branch before the missing-`Cargo.toml` branch.
- Added a repeated `--family` command containing all 14 public family names so the fixture proves early failure dominance across the full family enum surface.
- Kept `validate-repo` out of this layer because it is a separate command stack and does not fail on the same workspace-root/config dominance gates.
- Did not add flag-order permutations, duplicate same-family repetitions, or full flag cross-products because they do not expose separate branches at this layer.

# Key Files

- `.plans/2026-05-12-221240-g3rs-lowest-fixture-command-coverage.md`
- `behavior/fixtures/g3rs/L00-workspace-root-not-found/fixture.toml`
- `behavior/fixtures/g3rs/L10-workspace-root-found-guardrail-config-missing/fixture.toml`
- `behavior/fixtures/g3rs/L20-workspace-root-found-guardrail-config-invalid/fixture.toml`
- `behavior/baselines/g3rs/L00-workspace-root-not-found/*.json`
- `behavior/baselines/g3rs/L10-workspace-root-found-guardrail-config-missing/*.json`
- `behavior/baselines/g3rs/L20-workspace-root-found-guardrail-config-invalid/*.json`

# Verification

- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`
- `git diff --check`

# Adversarial Review

- First pass found missing `--path README.md` and repeated/all-family command coverage.
- Both were added.
- Second pass found no blocker for L00-L20.

# Next Steps

- Start L30 only after defining which required inputs are missing and which public `validate` flags remain observable at that layer.
- Build a separate repo-level fixture stack for `validate-repo`; do not mix it into workspace-root layers.
