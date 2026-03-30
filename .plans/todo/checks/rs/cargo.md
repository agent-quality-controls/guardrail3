# RS-CARGO — Cargo.toml lint policy checker

> Superseded as the primary family plan by [`.plans/by_family/rs/cargo.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/cargo.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** Cargo.toml at owned Rust policy roots plus member Cargo.toml files
**Parser:** TOML
**Current code:** `crates/app/rs/checks/rs/cargo/**` (multi-root policy-root model implemented)

## Implementation mapping contract

- exactly one `RS-CARGO-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `facts.rs`, `inputs.rs`, `discover.rs`, and `lint_support.rs` may contain shared facts, typed inputs, discovery, canonical baseline data, and normalization helpers only

Forbidden:

- production files that bundle multiple independent rule surfaces
- grouped family test files such as `cargo_tests.rs`
- helper files that hide multiple rule predicates behind one API

## Root discovery / ownership model

`RS-CARGO` must be a multi-root family, not a repo-root-only family.

Its owned Rust policy roots are:
- workspace roots
- standalone package roots that are not members of a workspace

For workspace roots:
- the root `Cargo.toml` owns workspace lint policy
- member manifests are checked relative to that workspace root

For standalone package roots:
- the package `Cargo.toml` itself owns the lint policy
- there is no separate workspace/member split

This family must not assume the repository root Cargo manifest is the only relevant one.

## Nested root / ownership semantics

Owned roots are derived from all discovered `Cargo.toml` roots, then classified as:
- workspace roots: roots whose `Cargo.toml` contains `[workspace]`
- standalone package roots: roots whose `Cargo.toml` contains `[package]` and are not members of any discovered workspace root

Membership is determined from declared workspace members after glob expansion.

Implications:
- a package root that is a member of a discovered workspace is not also treated as a standalone policy root
- workspace-member rules attach to the owning workspace root, not to arbitrary ancestor roots
- nested or sibling Rust roots must not collapse into one repo-global Cargo policy surface

## Rule applicability by root kind

### Policy-root rules

These apply to:
- workspace roots
- standalone package roots

They validate the manifest that owns the lint policy for that root:
- `RS-CARGO-01`
- `RS-CARGO-02`
- `RS-CARGO-03`
- `RS-CARGO-05`
- `RS-CARGO-15`
- `RS-CARGO-07`

### Workspace-only rules

These apply only when the owned root is a workspace:
- `RS-CARGO-04`
- `RS-CARGO-06`
- `RS-CARGO-08`
- `RS-CARGO-09`
- `RS-CARGO-10`

Standalone packages must not be forced through workspace-member rules they do not conceptually have.

## Root-local policy inputs

`guardrail3.toml` matters only when profile-sensitive lint expectations are relevant.

For `RS-CARGO`, profile policy must resolve from the owned policy root itself:
- workspace root uses the `guardrail3.toml` attached to that workspace root
- standalone package root uses the `guardrail3.toml` attached to that package root

The family must not silently fall back to an unrelated repo-root `guardrail3.toml` when evaluating a non-root owned policy root.

## Policy semantics

The cargo family owns Cargo lint policy, not arbitrary manifest content.

That means:
- the canonical Rust/clippy lint baseline belongs here
- inheritance from workspace lint policy belongs here
- member-level weakening or bypass of workspace lint policy belongs here
- workspace metadata used as part of lint/toolchain compatibility belongs here

It does not own:
- dependency direction (`RS-HEXARCH`, `RS-LIBARCH`)
- dependency allowlists (`RS-DEPS`)
- release metadata (`RS-RELEASE`)
- toolchain file settings (`RS-TOOLCHAIN`)

## Input integrity / fail-closed expectations

The family depends on:
- each owned policy-root `Cargo.toml`
- member `Cargo.toml` files for workspace-owned member checks
- `guardrail3.toml` only when profile-specific lint policy is relevant

Malformed required inputs must surface explicitly and must not silently collapse the family to “no workspace found”.

That includes:
- malformed owned-root `Cargo.toml`
- malformed member `Cargo.toml` for a member rule
- malformed `guardrail3.toml` when profile-sensitive lint expectations are needed

The current code fails open here. The plan must not.

## Exact standalone-package shape

For standalone package roots, the package-owned equivalents are:
- `[lints.rust]`
- `[lints.clippy]`

So:
- `RS-CARGO-01` checks completeness in `[workspace.lints.*]` for workspace roots and `[lints.*]` for standalone package roots
- `RS-CARGO-02` checks level correctness in `[workspace.lints.*]` for workspace roots and `[lints.*]` for standalone package roots
- `RS-CARGO-03` inventories approved `allow` deviations from `[workspace.lints.*]` for workspace roots and `[lints.*]` for standalone package roots

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-CARGO-01 | R26 | Error | Owned policy-root lint completeness: required `[workspace.lints.*]` for workspace roots or equivalent package-owned lint tables for standalone package roots. Includes Rust lint baseline, clippy deny baseline, and clippy groups. **Profile-aware:** library profile adds `unreachable_pub = "deny"`. `missing_docs` is intentionally not enforced. | Implemented |
| RS-CARGO-02 | R27 | Error | Lint levels correct (deny/warn/forbid/allow match expected). Weaker-than-baseline is Error. Stricter-than-baseline is allowed silently. | Implemented |
| RS-CARGO-03 | R28 | Info | Approved allow deviations inventory (9 lints: missing_docs_in_private_items, module_name_repetitions, etc.) | Implemented |
| RS-CARGO-04 | R29 | Error | Workspace-member lint inheritance: `[lints] workspace = true` in each member Cargo.toml | Implemented |
| RS-CARGO-05 | R55 | Error | Policy-root edition policy. Missing edition or edition below 2021 is Error. | Implemented |
| RS-CARGO-14 | — | Error | Cargo-family input failures: malformed owned-root `Cargo.toml`, malformed member `Cargo.toml` for workspace-member rules, or malformed root-local `guardrail3.toml` when profile-sensitive expectations are needed. | Implemented |
| RS-CARGO-15 | — | Error/Info | Policy-root `rust-version` / MSRV policy. Library profile missing `rust-version` is Error because MSRV is a compatibility contract; service profile may inventory only. | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-CARGO-06 | Error | Per-crate lint overrides weaken workspace. After confirming `workspace = true`, scan each member's `[lints.rust]` and `[lints.clippy]` for entries whose level is WEAKER than the workspace level. Weakening is ordinal: `forbid > deny > warn > allow`. So `forbid→deny` IS a weakening (`deny` can be overridden with `#[allow]`, `forbid` cannot). Each weakening is Error. | Implemented |
| RS-CARGO-07 | Warn | Lint group priority ordering. Specific lints with `priority < 0` get overridden by groups at `priority = -1`. Verify all specific deny lints have priority >= 0 (or no priority, default 0) so they take precedence over groups. Flag any specific lint with `priority < 0`. | Implemented |
| RS-CARGO-08 | Error | Workspace resolver enforcement. Every workspace root must set explicit `resolver = "2"` or `"3"`. Standalone packages are out of scope for this rule. | Implemented |
| RS-CARGO-09 | Warn | Workspace-member edition override detection. A member crate setting `edition = "2018"` while workspace uses `edition = "2024"` silently downgrades safety. | Implemented |
| RS-CARGO-10 | Warn | Declared workspace member missing `Cargo.toml`. If `[workspace].members` declares a directory but no member manifest is discovered there, warn explicitly rather than silently skipping it. | Implemented |

## Additional clean hardening already identified

These are enforceable and belong in this family even if not yet implemented:

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-CARGO-11 | Error | Canonical lint baseline must include `clippy::disallowed_macros = "deny"` so `RS-CLIPPY-20` is not toothless. | Implemented |
| RS-CARGO-12 | Error | Any explicit `allow` outside the approved allowlist should be surfaced; today `RS-CARGO-03` inventories only the known approved set. | Implemented |
| RS-CARGO-13 | Error | When a member uses `[lints] workspace = true`, member-local `allow` entries are forbidden even if they are not weakening an already-declared workspace lint. | Implemented |

## Current implementation notes

- discovery now classifies owned policy roots across:
  - workspace roots
  - standalone package roots
- profile-sensitive expectations resolve from root-local `guardrail3.toml`
- malformed owned-root `Cargo.toml`, member `Cargo.toml`, and root-local `guardrail3.toml` surface through `RS-CARGO-14`
- rule tests now use rule-specific `*_tests/` directories instead of legacy `*_tests.rs` sidecars

## Cross-family dependencies

- `RS-CARGO` owns the manifest-side switch that makes many `RS-CLIPPY` config bans actually enforceable
- `RS-CARGO-15` and `RS-TOOLCHAIN-03` intentionally overlap around MSRV/toolchain compatibility
- `RS-CARGO` should not absorb dependency allowlist or release policy concerns just because they also live in `Cargo.toml`

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| `[profile.dev/test]` settings | Subjective. Release profile already covered by RS-CARGO-05. |
| `[workspace.metadata]` custom sections | Extension point for third-party tools. guardrail3 can't validate arbitrary tool config. |
| `default-members` vs `members` | Pre-commit hook uses `cargo test --workspace` which ignores default-members. |
| Workspace deps with dangerous features | deny.toml feature bans (RS-DENY) handle this. Duplicate check. |
| Lint name typos | `warnings = "deny"` (RS-CARGO-01) + cargo clippy catches unknown lint names at compile time. |
| Keep `rust-version` folded into RS-CARGO-05 | Rejected. Edition policy and MSRV policy are now split across `RS-CARGO-05` and `RS-CARGO-15`. |
