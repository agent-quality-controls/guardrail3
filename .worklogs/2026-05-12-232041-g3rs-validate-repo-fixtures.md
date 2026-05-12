# Summary

Added repo-level behavior replay fixtures and baselines for `g3rs validate-repo`.

The fixture stack now covers invalid repo roots, crawlable repos without adoption markers, hooks reachability without root Cargo, marker-pair policy, root topology reachability, and default repo-root resolution.

# Decisions Made

- Kept `validate-repo` fixtures in `behavior/fixtures/g3rs-validate-repo` because repo-level command behavior has different gates from `g3rs validate --path`.
- Copied fixtures to a temporary directory before replay so bare `validate-repo` cannot accidentally use the outer guardrail3 checkout as its repo root.
- Added `R15-hooks-reachable-no-root-cargo` because clean no-adoption output cannot prove Hooks ran.
- Used explicit fixture metadata `git_init = true` for R15 so replay creates a temporary Git repo without tracking a nested `.git` directory.
- Fixed marker-pair checking to report both half-adoption directions: `guardrail3-rs.toml` without workspace `Cargo.toml`, and workspace `Cargo.toml` without `guardrail3-rs.toml`.
- Tightened baseline verification so stale nested JSON files fail, R20 marker-pair finding lines must match exact expected paths, R15 must show hooks without topology, and R30 must show topology without marker-pair noise.

# Key Files

- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs-validate-repo`
- `behavior/baselines/g3rs-validate-repo`
- `scripts/behavior/baseline_common.py`
- `scripts/behavior/verify-fixtures.py`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-all.sh`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/marker_pairs.rs`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/marker_pairs_tests/cases.rs`

# Verification

- `scripts/behavior/verify-all.sh`
- `python3 -m py_compile scripts/behavior/baseline_common.py scripts/behavior/generate-baselines.py scripts/behavior/verify-baselines.py scripts/behavior/verify-fixtures.py`
- `git diff --check`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-validate-command marker_pairs -- --nocapture`
- `cargo build --quiet --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs --bin g3rs`
- `apps/guardrail3-rs/target/debug/g3rs validate --path apps/guardrail3-rs`
- `g3rs validate-repo`

# Adversarial Review

- Round 1 found missing hooks observability, missing inverse marker-pair coverage, and shallow extra-baseline detection.
- Round 2 found weak R20 exact-path verification and stale R20 hook metadata.
- Round 3 found no blockers after those fixes.

# Next Steps

- Continue fixture migration with the next unlock layer only after defining which public behavior is exposed at that layer.
