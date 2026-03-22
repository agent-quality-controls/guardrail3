# HOOK-RS — Rust-specific hook step checker (16 rules)

**Input:** .githooks/pre-commit script content (pattern matching in executable lines)
**Current code:** `hook_checks.rs` (H5 patterns), `tool_checks.rs` (H8, H12)

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| HOOK-RS-01 | H5 (partial) | Warn/Info | cargo fmt --check step present | Implemented |
| HOOK-RS-02 | H5 (partial) | Warn/Info | cargo clippy -D warnings step present | Implemented |
| HOOK-RS-03 | H5 (partial) | Warn/Info | cargo-deny check step present | Implemented |
| HOOK-RS-04 | H5 (partial) | Warn/Info | cargo test step present | Implemented |
| HOOK-RS-05 | H5 (partial) | Warn/Info | cargo-machete step present | Implemented |
| HOOK-RS-06 | H8 | Error | Required tools installed (gitleaks, cargo-deny, cargo-machete) | Implemented |
| HOOK-RS-07 | H12 (partial) | Warn | Duplication tool: cargo-dupes for Rust projects (not jscpd) | Implemented |
| HOOK-RS-08 | — | Warn | `guardrail3 rs validate --staged` or `guardrail3 validate --staged` step present | Planned |
| HOOK-RS-09 | — | Warn | cargo clippy step includes `-D warnings` or equivalent deny-warnings flag | Planned |
| HOOK-RS-10 | — | Info | cargo test step uses `--workspace` for workspace projects | Planned |
| HOOK-RS-11 | — | Warn | gitleaks step present in the hook, not just installed on PATH | Planned |
| HOOK-RS-12 | — | Warn | cargo-dupes step present for Rust projects | Planned |
| HOOK-RS-13 | — | Info | cargo-dupes invocation uses `--exclude-tests` | Planned |
| HOOK-RS-14 | — | Error | `guardrail3` binary available when Rust guardrail validation is required | Planned |
| HOOK-RS-15 | — | Error | `cargo-dupes` installed when Rust duplication checking is required | Planned |
| HOOK-RS-16 | — | Warn | Rust guardrail config changes must trigger Rust hook validation | Planned |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| HOOK-RS-08 | Warn | guardrail3 validate step present. The pre-commit hook should run `guardrail3 rs validate --staged` or top-level `guardrail3 validate --staged` so AST-based tamper detection runs before commit. This is the hook-side enforcement point for RS-SOURCE, RS-GARDE, RS-HEXARCH, dependency allowlists, and other non-tool checks. | Planned |
| HOOK-RS-09 | Warn | cargo clippy must fail on warnings. Presence of `cargo clippy` is not enough; the invocation must include `-D warnings` or an equivalent deny-warnings form, otherwise warnings do not block commits. | Planned |
| HOOK-RS-10 | Info | cargo test should use `--workspace`. In workspaces, plain `cargo test` can silently skip member crates. This is advisory because single-crate repos do not need it. | Planned |
| HOOK-RS-11 | Warn | gitleaks step present. CLAUDE.md treats secret scanning as a pre-commit requirement. Tool installation is covered by HOOK-RS-06; this rule verifies the hook actually runs it. | Planned |
| HOOK-RS-12 | Warn | cargo-dupes step present. For Rust projects the duplication check should be cargo-dupes, not jscpd. HOOK-RS-07 checks tool choice at a high level; this rule verifies the hook actually invokes the right tool. | Planned |
| HOOK-RS-13 | Info | cargo-dupes uses `--exclude-tests`. CLAUDE.md calls out `cargo-dupes --exclude-tests` specifically. This avoids noise from deliberate duplication in tests while still checking production dependency duplication. | Planned |
| HOOK-RS-14 | Error | `guardrail3` installed when Rust validation is expected to run. The generated hook currently treats missing `guardrail3` as a warning and skips AST-based validation, but that is a fail-open path for the meta-guardrail. This rule makes the contract explicit on the validation side. | Planned |
| HOOK-RS-15 | Error | `cargo-dupes` installed when Rust duplication checks are required. The current tool-install check does not cover cargo-dupes even though the Rust hook plan now requires a cargo-dupes step. | Planned |
| HOOK-RS-16 | Warn | Rust guardrail config changes trigger Rust hook validation. Changes to `clippy.toml`, `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml`, `guardrail3.toml`, and similar Rust guardrail config files must cause the Rust hook path to run, even if no `.rs` file changed. Otherwise config-only weakening can bypass hook enforcement. | Planned |

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
