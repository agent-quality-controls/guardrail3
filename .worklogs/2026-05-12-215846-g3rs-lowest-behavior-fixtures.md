# Summary

Added executable behavior baselines for the lowest G3RS fixture levels: L00 workspace root missing, L10 guardrail config missing, and L20 guardrail config invalid.

The replay verifier now runs the repo-local candidate `g3rs` binary, honors `run_from = "repo"`, and compares normalized command output against generated JSON baselines.

# Decisions Made

- Baselines are required only for L00-L20 in this slice because those are the first fixture layers whose behavior is now fully exposed.
- L10 and L20 fixtures were stripped to the minimum files needed to expose their layer. Extra Rust source files hid whether the layer itself was doing useful work.
- `g3rs validate` now fails after proving a root `Cargo.toml` exists but before family execution when `guardrail3-rs.toml` is missing or invalid.
- The TOML parser now parses `checks.apparch` because the old opt-out path silently ignored it.
- The behavior verifier builds `apps/guardrail3-rs` and executes that candidate binary instead of trusting `g3rs` from `PATH`.
- `validate-repo` commands are not rejected by the fixture verifier. Future fixtures can replay public repo-root behavior directly.

# Key Files

- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-205756-g3rs-l00-behavior-fixture.md`
- `scripts/behavior/baseline_common.py`
- `scripts/behavior/generate-baselines.py`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-fixtures.py`
- `behavior/baselines/g3rs/L00-workspace-root-not-found/command-00.json`
- `behavior/baselines/g3rs/L10-workspace-root-found-guardrail-config-missing/command-00.json`
- `behavior/baselines/g3rs/L20-workspace-root-found-guardrail-config-invalid/command-00.json`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/execute.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/family_opt_out.rs`
- `packages/parsers/g3rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`

# Verification

- `cargo fmt --manifest-path apps/guardrail3-rs/Cargo.toml --all`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs -p guardrail3-rs-validate-command`
- `cargo test --manifest-path packages/parsers/g3rs-toml-parser/crates/runtime/Cargo.toml`
- `cargo check --manifest-path packages/parsers/g3rs-toml-parser/crates/runtime/Cargo.toml --no-default-features`
- `cargo check --manifest-path packages/parsers/g3rs-toml-parser/crates/types/Cargo.toml --no-default-features`
- `cargo check --manifest-path apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/Cargo.toml --no-default-features`
- `scripts/behavior/verify-all.sh`
- `g3rs validate --path apps/guardrail3-rs`
- `g3rs validate --path packages/parsers/g3rs-toml-parser`
- `g3rs validate-repo`
- `git diff --check`

# Adversarial Review

- First pass found two real false greens: replay used `g3rs` from `PATH`, and `run_from` metadata was ignored.
- Both false greens were fixed before commit.
- Second pass found no blocker for L00-L20.

# Next Steps

- Extend baseline replay to L30 after defining the exact missing-required-input layer.
- Add future fixtures for `validate-repo`, `--family`, `--staged`, and multi-command replay only when those behaviors become the current layer target.
