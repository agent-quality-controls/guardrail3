# Summary

Added L30 behavior replay fixtures for missing required inputs after a Rust workspace and valid `guardrail3-rs.toml` are available.

The L30 baseline set now proves missing toolchain, fmt, cargo lint tables, clippy config, deny config, service lockfile, release files, nextest config, mutants config, mutants profile, and mutation hook wiring.

# Decisions Made

- Split L30 into four fixtures because one fixture cannot expose base config, release, async-test, and mutation-profile branches without hiding required findings.
- Made L30 source facade-only so `g3rs-arch/lib-facade-only` no longer pollutes the missing-input layer.
- Removed `Cargo.lock` from the base L30 fixture and set `profile = "service"` so `g3rs-deps/cargo-lock-present` fails as an error.
- Added fixture-local `path_prepend` support so mutation fixtures use a fake `cargo-mutants` from the fixture instead of the host PATH.
- Added `required_results` verification to baseline replay so specific rule IDs, titles, and files must appear in generated output.

# Key Files

- `.plans/2026-05-12-234000-g3rs-l30-required-input-fixtures.md`
- `.plans/2026-05-12-234000-g3rs-l30-required-input-fixtures.md.manifest.toml`
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

- `python3 -m py_compile scripts/behavior/baseline_common.py scripts/behavior/generate-baselines.py scripts/behavior/verify-baselines.py scripts/behavior/verify-fixtures.py`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`
- `git diff --check`

# Next Steps

- Continue with L40 fixtures only after listing invalid required-input rules that become visible after L30 missing files are present.
