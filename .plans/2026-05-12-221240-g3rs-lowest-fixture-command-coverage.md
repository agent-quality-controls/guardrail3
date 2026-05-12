# Goal

L00-L20 G3RS behavior fixtures must replay every `g3rs validate` command shape that is still observable at those layers.

These layers intentionally stop before crawl, family checks, staged-file reads, and cargo gates. The fixture command set must prove those early failures dominate public `validate` flags.

# Scope

Included in this slice:

- `g3rs validate --path .`
- `g3rs validate --path . --inventory`
- `g3rs validate --path . --family fmt --inventory`
- `g3rs validate --path . --family apparch --inventory`
- `g3rs validate --path . --family <all supported families> --inventory`
- `g3rs validate --path . --staged --inventory`
- `g3rs validate --path . --rules-only --inventory`
- `g3rs validate --path README.md --inventory` for L00 only, because L00 owns the non-directory workspace-root failure branch.

Excluded from this slice:

- `g3rs validate-repo`: separate repo-level command stack.
- cargo gate output: hidden by L00-L20 failures.
- family runner output: hidden by L00-L20 failures.
- parser schema details beyond invalid-config failure: belongs in parser fixtures or later G3RS layers.

# Approach

- Update L00, L10, and L20 `fixture.toml` command lists with the public `validate` command shapes above.
- Regenerate baselines for every command listed in those fixtures.
- Keep `scripts/behavior/verify-baselines.py` unchanged unless it fails to verify the expanded command set.
- Run `scripts/behavior/verify-all.sh`.
- Run `g3rs validate --path apps/guardrail3-rs`.
- Run `g3rs validate-repo`.
- Send an adversarial verifier to confirm the fixture command coverage matches this plan.

# Files

- `behavior/fixtures/g3rs/L00-workspace-root-not-found/fixture.toml`
- `behavior/fixtures/g3rs/L10-workspace-root-found-guardrail-config-missing/fixture.toml`
- `behavior/fixtures/g3rs/L20-workspace-root-found-guardrail-config-invalid/fixture.toml`
- `behavior/baselines/g3rs/L00-workspace-root-not-found/*.json`
- `behavior/baselines/g3rs/L10-workspace-root-found-guardrail-config-missing/*.json`
- `behavior/baselines/g3rs/L20-workspace-root-found-guardrail-config-invalid/*.json`
- `.worklogs/<timestamp>-g3rs-lowest-fixture-command-coverage.md`

# Done

- L10-L20 each have seven baseline command records.
- L00 has eight baseline command records.
- `scripts/behavior/verify-all.sh` passes.
- G3RS self-validation passes.
- Adversarial verifier finds no blocker for L00-L20 command coverage.
