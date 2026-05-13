Summary:
- Expanded G3RS behavior replay fixtures through L70 project-policy coverage.
- Added six L70 fixtures covering source/test/arch, workspace/package/deps/cargo, apparch, garde, release metadata, and invalid semver.
- Hardened baseline verification so required findings include severity and hook routing checks require real assignment lines.

Decisions made:
- Kept six L70 fixtures because adversarial review found they are separated by real hiding boundaries.
- Merged dependency allowlist coverage into the workspace/package fixture because it does not hide the workspace, cargo, topology, or arch rows.
- Split invalid semver into its own release fixture because it changes release dry-run behavior and can hide non-semver release branches.
- Suppressed delegated cargo gate stdout/stderr in behavior baselines because cargo test, cargo-deny, cargo-dupes, and cargo-machete output adds volatile noise unrelated to G3RS findings.
- Changed `cargo machete` hook command shape to `cargo-machete` because the delegated gate runner executes argv directly and `cargo machete` was routed as an invalid cargo subcommand shape.

Key files for context:
- `.plans/2026-05-13-123846-expand-g3rs-l70-project-policy-fixtures.md`
- `.plans/2026-05-12-183156-guardrail3-behavior-replay-fixture-migration.md.manifest.toml`
- `.plans/2026-05-12-222909-g3rs-validate-repo-fixtures.md.manifest.toml`
- `behavior/fixtures/g3rs/L70-delegated-policy-valid-project-policy-violated`
- `behavior/fixtures/g3rs/L70-workspace-package-policy-violated`
- `behavior/fixtures/g3rs/L70-apparch-policy-violated`
- `behavior/fixtures/g3rs/L70-garde-boundary-policy-violated`
- `behavior/fixtures/g3rs/L70-release-metadata-policy-violated`
- `behavior/fixtures/g3rs/L70-release-invalid-semver-policy-violated`
- `scripts/behavior/verify-baselines.py`
- `scripts/behavior/verify-fixtures.py`
- `apps/guardrail3-rs/crates/logic/validate-command/crates/runtime/src/cargo_gates.rs`

Verification:
- `scripts/behavior/verify-all.sh`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml -p guardrail3-rs-validate-command`
- `g3rs validate --path apps/guardrail3-rs --inventory`
- `python3 -m py_compile scripts/behavior/*.py`
- `git diff --check`
- Adversarial reviewers confirmed six L70 fixtures are not mergeable without hiding rows, required rows are severity-aware, and hook checks no longer pass on comments or echo text.

Next steps:
- Continue fixture migration above L70 with the same rule: merge every branch that can signal independently, split only when one fixture would hide another row.
