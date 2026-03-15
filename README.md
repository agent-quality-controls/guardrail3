# guardrail3

Composable code guardrails for Rust and TypeScript projects.

guardrail3 is a single Rust binary that validates any project against a canonical rule set, generates config files (clippy.toml, deny.toml, rustfmt.toml, pre-commit hooks) from composable modules, and reports violations in text, JSON, or markdown. It auto-detects your stack -- Rust, TypeScript, or both -- and runs only the relevant checks. No configuration required for validation.

## Installation

```sh
cargo install guardrail3
```

Or with [cargo-binstall](https://github.com/cargo-bins/cargo-binstall) for prebuilt binaries:

```sh
cargo binstall guardrail3
```

## Quick Start

```sh
# Validate the current project (auto-detects Rust/TypeScript)
guardrail3 validate .

# Rust checks only
guardrail3 rs validate .

# TypeScript checks only
guardrail3 ts validate .

# JSON output for CI
guardrail3 validate . --format json
```

## Commands

| Command | Config required | Description |
|---|---|---|
| `guardrail3 validate [path]` | No | Auto-detect stacks, run all checks |
| `guardrail3 init --profile <name>` | No (creates it) | Scaffold guardrail3.toml + local/ |
| `guardrail3 generate` | Yes | Produce config files from modules + profile |
| `guardrail3 check` | Yes | Verify generated files are current (for CI) |
| `guardrail3 diff` | Yes | Dry run of generate, showing diffs |
| `guardrail3 list-modules` | No | List all embedded modules |
| `guardrail3 show-module <name>` | No | Print contents of a module |
| `guardrail3 rs validate [path]` | No | Rust checks only |
| `guardrail3 rs generate` | Yes | Generate Rust configs only |
| `guardrail3 ts validate [path]` | No | TypeScript checks only |
| `guardrail3 ts generate` | Yes | Generate TypeScript configs only |
| `guardrail3 hooks validate [path]` | No | Validate pre-commit hook setup |
| `guardrail3 hooks install` | Yes | Install pre-commit hook |

### Scope Flags

Narrow `validate` to specific files:

```
--staged           only staged files (git diff --cached)
--dirty            staged + unstaged changes
--commits N        files changed in last N commits
--files a.rs b.rs  explicit file list
--format text|json|md
```

### Domain Flags

Focus `validate` on a specific concern:

| Flag | What it checks |
|---|---|
| `--code` | Code quality: lints, bans, suppressions, structural health |
| `--architecture` | Architecture: dependency direction, layer violations, centralized I/O |
| `--release` | Release readiness: metadata, publish checks, version hygiene |
| `--tests` | Test quality: coverage config, test structure, mutation testing |
| `--thorough` | Slow checks: cargo publish --dry-run, full dependency audit |

Without any domain flag, all checks run.

## Profiles

Profiles control which bans and rules are active during `generate`.

| Profile | Description |
|---|---|
| `service` | Full guardrails for Axum/tokio HTTP services. All clippy bans, all deny bans, tokio feature gating. |
| `library` | Service + ban all I/O crates (axum, tokio, reqwest, sqlx) + global-state bans on every crate. |
| `monorepo` | Service for the Rust side, plus TypeScript config generation. |

Initialize a project with a profile:

```sh
guardrail3 init --profile service
```

This creates `guardrail3.toml` and a `local/` directory for project-specific overrides.

## What Gets Checked

175+ checks across four domains:

**Rust (R1-R58, R-GARDE, R-PUB, R-REL, R-BIN, R-TEST)** -- Config completeness (clippy.toml, deny.toml, rustfmt, toolchain, Cargo.toml workspace lints), source scan (allow/garde/exception auditing, file length, use count, unsafe, todo, unwrap), tool installation (cargo-deny, cargo-machete, cargo-dupes, gitleaks), architecture (dependency direction, centralized I/O enforcement, workspace metadata).

**TypeScript (T1-T83, T-TEST)** -- ESLint config and 35+ individual rule checks, tsconfig strict settings, npmrc enforcement, package.json overrides and banned deps, jscpd config, source scan (eslint-disable, ts-ignore, process.env, any, file length).

**Hooks (H1-H12)** -- Pre-commit hook existence, structure, required tool checks, script inventory, language-appropriate duplication tool.

**Deployment (D1-D5)** -- Railpack configs, Next.js standalone output, Tailwind dependency placement.

## Embedded Modules

guardrail3 ships with all guardrail content embedded in the binary. No external files needed.

- **Clippy method bans (6):** env-vars, env-mutation, process-control, blocking-sleep, filesystem, http-client
- **Clippy type bans (4):** collections, sync, filesystem, global-state
- **Deny sections (16):** graph, bans-base, 10 ban categories, feature-bans, licenses, advisories, sources
- **Canonical configs (7):** rustfmt.toml, rust-toolchain.toml, cargo-lints, npmrc, tsconfig-base, jscpd, eslint-starter
- **Pre-commit hook:** Profile-aware builder (cargo-dupes for Rust, jscpd for TypeScript, both for mixed)

List them with `guardrail3 list-modules`, inspect with `guardrail3 show-module <name>`.

## Configuration

`guardrail3.toml` is only needed for `generate`, `check`, and `diff`. Validation works without it.

```toml
version = "0.1"

[profile]
name = "service"

[rust]
workspace_root = "."

[rust.crates.api]
layer = "composition-root"

[rust.crates.domain-types]
layer = "pure"

[local]
clippy_methods = "local/clippy-methods.toml"
clippy_types = "local/clippy-types.toml"
deny_bans = "local/deny-bans.toml"
deny_skip = "local/deny-skip.toml"
deny_feature_bans = "local/deny-feature-bans.toml"
```

The `local/` directory holds project-specific overrides that get merged with the canonical modules during generation.

## Self-Validation

guardrail3 enforces the same rules on itself that it enforces on other projects. Every commit passes through the full pre-commit hook (gitleaks, cargo fmt, cargo clippy, cargo-deny, structural health, cargo-machete, cargo test, cargo-dupes), and `guardrail3 validate .` reports zero errors against its own codebase.

## License

MIT OR Apache-2.0
