# HOOK-RS — Rust-specific hook step checker (16 rules)

Public package boundary note:
- `HOOK-RS` no longer ships as a separate public package family.
- These rules now execute through the merged `g3rs-hooks` package family together with the old `HOOK-SHARED` source rules.

**Input:** effective pre-commit executable command context + Rust hook tool availability
**Current code:** `crates/app/rs/checks/hooks/rs/**` (old `hook_checks.rs` / `tool_checks.rs` are legacy seed material only)

## Implementation mapping contract

- exactly one `HOOK-RS-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `facts.rs` and `inputs.rs` may contain shared discovery and typed executable-command inputs only

Forbidden:

- grouped family test files such as `hook_rs_tests.rs`
- helper files that hide multiple unrelated rule predicates behind one API

## Discovery / ownership model

`HOOK-RS` is a validation-root family layered on top of `HOOK-SHARED`.

It owns:
- Rust-specific executable command checks in the effective pre-commit hook
- required Rust hook tool availability
- Rust config-change trigger coverage in hook logic

It does not own:
- whether the hook file exists at all
- whether hook dispatching is structurally sound
- whether comments or inert text are being mistaken for real commands

Those belong to `HOOK-SHARED`.

Compatibility note:
- `.githooks/pre-commit` is the preferred hook path
- `hooks/pre-commit` is a compatibility fallback accepted by the current family facts
- if neither exists, `HOOK-SHARED` owns the missing-hook failure and `HOOK-RS` may stay silent

## Executable-command contract

Rust hook rules must evaluate executable command lines only.

That means Rust step presence is not satisfied by:
- comments
- `echo` text
- unrelated helper prose

`HOOK-RS` relies on the same executable-line parsing discipline as `HOOK-SHARED`.

Presence normalization includes the executable command shapes already accepted by the family:
- `env` wrappers
- path-qualified binaries
- `cargo +toolchain` prefixes
- required commands executed through called shell functions
- reachable command substitutions / subshell-style command segments when the command is actually executed

Future verification must use that normalized executable-command model, not plain substring matching.

## Cross-family dependency

`HOOK-RS` depends on `HOOK-SHARED` being correct about:
- pre-commit existence
- effective hook path / trust
- executable command context
- fail-open wrapper detection

So `HOOK-RS` should not duplicate:
- hook-file existence rules
- dispatcher rules
- generic shell-structure rules

It should only own Rust-specific command/tool/config semantics.

## Input integrity / fail-closed expectations

The family depends on:
- readable effective pre-commit content from the shared hook surface
- `ToolChecker` results for required Rust hook tools

Missing or malformed hook content must not create false positives for Rust step presence, and missing tools must not be downgraded to silent skips.

When the shared hook family determines there is no effective pre-commit script, `HOOK-RS` may remain silent on presence rules because the missing-hook failure is already owned by `HOOK-SHARED`.

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| HOOK-RS-01 | H5 (partial) | Warn | cargo fmt --check step present | Implemented |
| HOOK-RS-02 | H5 (partial) | Warn | cargo clippy step present | Implemented |
| HOOK-RS-03 | H5 (partial) | Warn | cargo-deny check step present | Implemented |
| HOOK-RS-04 | H5 (partial) | Warn | cargo test step present | Implemented |
| HOOK-RS-05 | H5 (partial) | Warn | cargo-machete step present | Implemented |
| HOOK-RS-06 | H8 | Error | Required Rust hook tools installed. One result per required tool: gitleaks, cargo-deny, cargo-machete. | Implemented |
| HOOK-RS-07 | H12 (partial) | Warn | Rust duplication tool selection: cargo-dupes is required if a Rust duplication step exists; jscpd alone is wrong, but jscpd may coexist if cargo-dupes is also present. | Implemented |
| HOOK-RS-08 | — | Warn | `guardrail3 rs validate --staged` or `guardrail3 validate --staged` step present | Implemented |
| HOOK-RS-09 | — | Warn | cargo clippy step includes `-D warnings` or equivalent deny-warnings flag | Implemented |
| HOOK-RS-10 | — | Info | cargo test step uses `--workspace` for workspace projects | Implemented |
| HOOK-RS-11 | — | Warn | gitleaks step present in the hook, not just installed on PATH | Implemented |
| HOOK-RS-12 | — | Warn | cargo-dupes step present for Rust projects | Implemented |
| HOOK-RS-13 | — | Info | cargo-dupes invocation uses `--exclude-tests` | Implemented |
| HOOK-RS-14 | — | Error | `guardrail3` binary available when Rust guardrail validation is required | Implemented |
| HOOK-RS-15 | — | Error | `cargo-dupes` installed when Rust duplication checking is required | Implemented |
| HOOK-RS-16 | — | Warn | Rust guardrail config changes must trigger Rust hook validation | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| HOOK-RS-08 | Warn | guardrail3 validate step present. The pre-commit hook should run `guardrail3 rs validate --staged` or top-level `guardrail3 validate --staged` so AST-based tamper detection runs before commit. This is the hook-side enforcement point for RS-SOURCE, RS-GARDE, RS-HEXARCH, dependency allowlists, and other non-tool checks. | Implemented |
| HOOK-RS-09 | Warn | cargo clippy must fail on warnings. Presence of `cargo clippy` is not enough; the invocation must include `-D warnings` or an equivalent deny-warnings form, otherwise warnings do not block commits. | Implemented |
| HOOK-RS-10 | Info | cargo test should use `--workspace`. In workspaces, plain `cargo test` can silently skip member crates. This is advisory because single-crate repos do not need it. | Implemented |
| HOOK-RS-11 | Warn | gitleaks step present. CLAUDE.md treats secret scanning as a pre-commit requirement. Tool installation is covered by HOOK-RS-06; this rule verifies the hook actually runs it. | Implemented |
| HOOK-RS-12 | Warn | cargo-dupes step present. For Rust projects the duplication check should be cargo-dupes, not jscpd. HOOK-RS-07 checks tool choice at a high level; this rule verifies the hook actually invokes the right tool. | Implemented |
| HOOK-RS-13 | Info | cargo-dupes uses `--exclude-tests`. CLAUDE.md calls out `cargo-dupes --exclude-tests` specifically. This avoids noise from deliberate duplication in tests while still checking production dependency duplication. | Implemented |
| HOOK-RS-14 | Error | `guardrail3` installed when Rust validation is expected to run. The generated hook currently treats missing `guardrail3` as a warning and skips AST-based validation, but that is a fail-open path for the meta-guardrail. This rule makes the contract explicit on the validation side. | Implemented |
| HOOK-RS-15 | Error | `cargo-dupes` installed when Rust duplication checks are required. The current tool-install check does not cover cargo-dupes even though the Rust hook plan now requires a cargo-dupes step. | Implemented |
| HOOK-RS-16 | Warn | Rust guardrail config changes trigger Rust hook validation. Changes to `clippy.toml`, `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml`, `guardrail3-rs.toml`, and similar Rust guardrail config files must cause the Rust hook path to run, even if no `.rs` file changed. Otherwise config-only weakening can bypass hook enforcement. | Implemented |

## Hook-to-Rust-checker mapping

| Hook rule | Protects / backs up |
|-----------|---------------------|
| HOOK-RS-01 | RS-FMT |
| HOOK-RS-02, HOOK-RS-09 | RS-CARGO, RS-CLIPPY |
| HOOK-RS-03 | RS-DENY |
| HOOK-RS-04, HOOK-RS-10 | RS-TEST |
| HOOK-RS-05 | RS-DEPS dependency hygiene |
| HOOK-RS-08 | RS-SOURCE, RS-GARDE, RS-HEXARCH, RS-DEPS allowlists, other AST/meta checks |
| HOOK-RS-11 | Secret scanning guardrail from CLAUDE.md |
| HOOK-RS-12, HOOK-RS-13 | Duplicate dependency/version hygiene guardrail from CLAUDE.md |
| HOOK-RS-14 | AST/meta-validation contract from CLAUDE.md and pre-commit template |
| HOOK-RS-15 | Rust duplication enforcement contract |
| HOOK-RS-16 | Config-completeness and config-enforcement families when only config files changed |

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| cargo fmt scope (`--all` vs staged-only wrapper) | Both patterns are acceptable. Presence of the formatting step matters more than the exact scope. |
| Enforcing exact cargo-deny subcommand spelling | `cargo deny check`, `cargo-deny check`, or wrapped scripts are equivalent if they run deny validation. |
| Enforcing exact cargo-machete flags | Presence of the check matters; exact flags are secondary unless a concrete bypass is identified. |
| Minimum tool version checks | Useful in principle, but they require ongoing version policy and comparison semantics. Not included until there is a clear compatibility floor to enforce. |
| CI/server-side bypass mitigation checks | The risk from `--no-verify` is real, but local static files do not reliably prove whether CI or server-side validation exists. Keep hook-side checks focused on local artifacts we can validate directly. |
