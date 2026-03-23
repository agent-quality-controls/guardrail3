# Hooks Coverage Matrix

This file is the migration map for the hook lane.

It captures:
- the canonical new-family location
- the current legacy ownership
- the old-to-new rule mapping
- the known semantic gaps that must be closed during migration

## Canonical migrated location

The migrated hook families should live under the new checker root:

- `apps/guardrail3/crates/app/rs/checks/hooks/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/hooks/shared/`
- `apps/guardrail3/crates/app/rs/checks/hooks/rs/`

Reason:
- this keeps migrated hooks under the same `ProjectTree -> orchestrator -> typed inputs -> pure rules` architecture as the Rust families
- it avoids treating legacy `app/hooks/` as a long-term home
- it avoids incorrectly placing `HOOK-SHARED` under `checks/rs/`

Legacy `apps/guardrail3/crates/app/hooks/` is now migration-source code, not the target architecture.

## Current legacy ownership

| Surface | Current file(s) | Current shape | Migration note |
|---|---|---|---|
| Hook report entrypoint | `apps/guardrail3/crates/app/hooks/validate.rs` | legacy report wrapper | replace with migrated hook-family entrypoint |
| Main hook orchestrator | `apps/guardrail3/crates/app/hooks/hook_checks.rs` | crawler/FS-driven | split into new `shared` and `rs` orchestrators |
| Hook command/pattern checks | `apps/guardrail3/crates/app/hooks/hook_script_checks.rs` | substring matching | replace with executable-command semantics |
| Hook tool checks | `apps/guardrail3/crates/app/hooks/tool_checks.rs` | coarse tool inventory | split shared vs Rust ownership |
| Mutation-hook overlap | `apps/guardrail3/crates/app/rs/validate/test_quality_checks.rs` | coarse non-comment detection | align `RS-TEST-08` with the hook parser |
| Hook generation | `apps/guardrail3/crates/adapters/inbound/cli/generate.rs` | generator writes hook content | must be reconciled with new checker semantics |
| Hook template | `apps/guardrail3/crates/domain/modules/pre_commit.rs` | concrete shell template | golden baseline and generator parity source |

## Routing impact

These integration points will need follow-up migration once the new families are real:

| Area | File | Current state | Planned change |
|---|---|---|---|
| checker root exposure | `apps/guardrail3/crates/app/rs/checks/mod.rs` | only `pub mod rs;` | add `pub mod hooks;` |
| Rust validate orchestration | `apps/guardrail3/crates/app/rs/validate/mod.rs` | no migrated hook section | add migrated hook-family execution |
| CLI validate routing | `apps/guardrail3/crates/adapters/inbound/cli/validate.rs` | appends legacy hook report | route through migrated hook families |
| CLI command dispatch | `apps/guardrail3/crates/main.rs` | hook validate commands call legacy path | repoint to migrated entrypoint |
| report category model | `apps/guardrail3/crates/domain/report/mod.rs` | no hook category | add explicit hook routing dimension |
| config category model | `apps/guardrail3/crates/domain/config/types.rs` | `RustChecksConfig` has no `hooks` field | add hook toggle if category-level routing remains configurable |

## Old-to-new rule mapping

### Shared structure and inventory

| New ID | Planned meaning | Legacy source | Status at start |
|---|---|---|---|
| `HOOK-SHARED-01` | `.githooks/pre-commit` exists | `H1` in `hook_checks.rs` | implemented only in legacy shape |
| `HOOK-SHARED-02` | `core.hooksPath = .githooks` | `H2` in `hook_checks.rs` | implemented only in legacy shape |
| `HOOK-SHARED-03` | `pre-commit.d/` inventory | `H3` in `hook_checks.rs` | implemented only in legacy shape |
| `HOOK-SHARED-04` | modular dispatcher present | `H4` in `hook_script_checks.rs` | weak token matching |
| `HOOK-SHARED-05` | top-level hook executable bit | `H7` in `hook_checks.rs` | implemented only for dispatcher |
| `HOOK-SHARED-06` | script stats inventory | `H6` in `hook_script_checks.rs` | implemented only in legacy shape |
| `HOOK-SHARED-07` | modular script inventory | `H9` in `hook_checks.rs` | implemented only in legacy shape |
| `HOOK-SHARED-08` | pre-commit file size inventory | `H10` in `hook_checks.rs` | implemented only in legacy shape |
| `HOOK-SHARED-09` | local override inventory | `H11` inventory in `hook_script_checks.rs` | implemented only in legacy shape |

### Shared shell-safety and semantic-command rules

| New ID | Planned meaning | Legacy source | Status at start |
|---|---|---|---|
| `HOOK-SHARED-10` | shell error handling | `H-SAFE-01` in `hook_script_checks.rs` | overclaimed, still substring-based |
| `HOOK-SHARED-11` | valid shebangs | none | missing |
| `HOOK-SHARED-12` | modular scripts executable | none | missing |
| `HOOK-SHARED-13` | no unconditional `exit 0` bypass | none | missing |
| `HOOK-SHARED-14` | no `--no-verify` guidance | none | missing |
| `HOOK-SHARED-15` | merge-conflict step present | `H-TOOL-02` in `hook_script_checks.rs` | substring-based |
| `HOOK-SHARED-16` | file-size step present | template only | checker missing |
| `HOOK-SHARED-17` | hook execution trust / shadowing risk | none | missing |
| `HOOK-SHARED-18` | executable-command context only | none | missing core semantic rule |
| `HOOK-SHARED-19` | real dispatcher syntax only | partial `H4` | weak token matching |
| `HOOK-SHARED-20` | concrete lockfile command | `H-TOOL-03` in `hook_script_checks.rs` | substring-based |
| `HOOK-SHARED-21` | no fail-open wrappers on critical commands | none | missing |

### Rust-specific hook rules

| New ID | Planned meaning | Legacy source | Status at start |
|---|---|---|---|
| `HOOK-RS-01` | `cargo fmt --check` step present | `H5` pattern checks | substring-based |
| `HOOK-RS-02` | `cargo clippy` step present | `H5` pattern checks | substring-based |
| `HOOK-RS-03` | `cargo deny` step present | `H5` pattern checks | substring-based |
| `HOOK-RS-04` | `cargo test` step present | `H5` pattern checks | substring-based |
| `HOOK-RS-05` | `cargo machete` step present | `H5` pattern checks | substring-based |
| `HOOK-RS-06` | required tools installed | `H8` in `tool_checks.rs` | incomplete tool surface |
| `HOOK-RS-07` | duplication tool is `cargo-dupes` | `H12` in `tool_checks.rs` | coarse content matching |
| `HOOK-RS-08` | `guardrail3 ... validate --staged` present | template only | checker missing |
| `HOOK-RS-09` | clippy denies warnings | partial template coverage | checker missing |
| `HOOK-RS-10` | workspace-aware `cargo test --workspace` | template only | checker missing |
| `HOOK-RS-11` | `gitleaks` step present | broad shared pattern only | Rust hook checker missing |
| `HOOK-RS-12` | `cargo-dupes` step present | `H12` partial | weak legacy shape |
| `HOOK-RS-13` | `cargo-dupes --exclude-tests` | template only | checker missing |
| `HOOK-RS-14` | `guardrail3` availability fail-closed | template warns and skips | fail-open bug |
| `HOOK-RS-15` | `cargo-dupes` installed when required | none | missing |
| `HOOK-RS-16` | config-only Rust guardrail changes trigger validation | template only | checker missing |

## Required parser-owned semantics

The migrated families must not keep these as substring checks:

- executable command presence
- dispatcher detection
- lockfile command validation
- fail-open wrapper detection
- `guardrail3` step detection
- `cargo clippy -D warnings` semantics
- `cargo test --workspace` semantics
- `cargo-dupes --exclude-tests` semantics
- config-change trigger logic

These belong to the new hook parser / command model, not to rule-local ad hoc string matching.

## Immediate implementation implications

The next concrete code steps are:

1. add the new module scaffold under `apps/guardrail3/crates/app/rs/checks/hooks/`
2. add a parser support module under that scaffold
3. implement parser tests first
4. migrate `HOOK-SHARED-18`, `HOOK-SHARED-19`, and `HOOK-SHARED-21` before lower-risk inventory rules

