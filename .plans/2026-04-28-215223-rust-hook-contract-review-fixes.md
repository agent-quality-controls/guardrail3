Goal

- Close the adversarial findings from the first Rust hook-contract hardening review.
- The repo hook must satisfy the hardened all-family validation contract.
- Hook contract checks must not become weaker than the old hook checks.
- Newly added packages and dependencies must validate under the existing Rust guardrails or have explicit, narrow waivers.

Approach

- Fix `.githooks/pre-commit` to run all-family `g3rs validate --path "$REPO_ROOT"` when Rust or Rust guardrail config changed.
- Update `guardrail_validate_staged_present` so family-filtered `g3rs validate` does not count as staged guardrail validation.
- Update config-trigger tests so `--family hooks` is a failing scenario.
- Keep `required_contract_command_present` conservative: no `--family` filters satisfy `G3RsValidatePath`.
- Fix dependency allowlists for new hook-contract dependencies in app and hook packages.
- Fix or explicitly waive hook-contract package validation drift:
  - If a package is structurally valid under current family rules, add missing local config.
  - If a current family rule is too broad for tiny contract packages, add a narrow waiver in package config rather than hiding the failure globally.
- Restrict modular script command aggregation so only scripts reachable from the actual pre-commit dispatcher can satisfy contract command presence.
- Reuse old shell command checks where required-command presence would otherwise weaken behavior:
  - Clippy deny warnings must respect old override and RUSTFLAGS behavior.
  - Cargo dupes exclude-tests strictness remains enforced by the old dedicated rule; required-command presence must not claim to replace that rule.
- Harden critical fail-open detection for `|| exit 0` and non-terminating wrappers.
- Add tests before or with every fix for the specific adversarial failure.

Key decisions

- All-family validation is the contract. The plan no longer accepts family filters covering owner sets. This avoids a second mini dependency solver inside hook parsing.
- `contract-trigger-coverage` remains warning-only for now because it explicitly states current parser limits. It must not be used as the sole proof that hooks are correct.
- Existing old rules remain active. The new contract rule proves family-owned commands are present; old dedicated rules still enforce stricter command-specific behavior where they exist.

Files to modify

- `.githooks/pre-commit`
- `apps/guardrail3-rs/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-source-checks/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-config-checks/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-types/guardrail3-rs.toml`
- representative or generated `packages/rs/*/g3rs-*-hook-contract/guardrail3-rs.toml`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/guardrail_validate_staged_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/config_changes_trigger_validation/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_critical_command_not_fail_open/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/contract_critical_command_not_fail_open/rule_tests/golden.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`
- command and config test files under `packages/rs/hooks/g3rs-hooks-source-checks` and `packages/rs/hooks/g3rs-hooks-config-checks`
