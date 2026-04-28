# Rust Hook Contract Adversarial Fixes

## Goal

Close concrete gaps found by adversarial review after `30d314a3a feat: add rust hook contracts`.

## Fixes

1. `G3RsValidatePath` must mean a real all-family validation command.
   - Reject `g3rs validate --path ... --family hooks` for family contracts that require architecture/config validation.
   - Accept either no `--family` filters or a command whose family filters cover every owner that requires `G3RsValidatePath`.
   - Reject detached `--path ""` in both new and legacy guardrail command parsers.

2. Rust cargo lockfile command must be Rust-specific.
   - Replace `ConcreteLockfileCommand` semantics from `pnpm install --frozen-lockfile` to a concrete Cargo lockfile check.
   - Use `cargo update --locked --workspace` or an equivalent Cargo command that fails when `Cargo.lock` is out of sync.
   - Update cargo hook contract tests and required-command tests.

3. Trigger coverage must stop lying.
   - Either prove trigger routing with existing parser support or keep the rule conservative without claiming command presence.
   - Update `.githooks/pre-commit` so `release-plz.toml`, `cliff.toml`, `.cargo/config`, and `.cargo/config.toml` enter the same `g3rs validate` condition if contracts require them.
   - Add README trigger patterns to the release hook contract if release source checks use README files.

4. Strengthen tests.
   - Positive required-command tests must assert non-empty exact findings, not `all(...)` on possibly empty results.
   - Add mixed `cargo dupes` valid-plus-invalid coverage.
   - Add `G3RsValidatePath` duplicate-owner aggregation coverage.
   - Add config tool coverage for g3rs, gitleaks, cargo-machete, cargo-dupes, and path-qualified tools.
   - Add process-runner test that `run(SupportedFamily::Hooks)` injects requirements into checks enough to surface contract findings.

5. Fix dependency policy metadata.
   - Update `apps/guardrail3-rs/guardrail3-rs.toml` allowed dependencies for new hook contract crates if current G3RS dependency rules require it.
   - Update each new hook-contract package `guardrail3-rs.toml` with explicit allowed dependencies for runtime and contract types if package-local validation requires it.

## Verification

- `cargo fmt --manifest-path apps/guardrail3-rs/Cargo.toml --all`
- `cargo clippy --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --all-targets --all-features -- -D warnings`
- `cargo test --manifest-path apps/guardrail3-rs/Cargo.toml --workspace --offline --locked`
- Targeted hook source/config tests
- `g3rs validate --path . --family hooks`
- Full pre-commit through `git commit`
- Second adversarial review until clean
