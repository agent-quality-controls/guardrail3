# RS-TOOLCHAIN — rust-toolchain.toml checker (4 rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/toolchain.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/toolchain.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** rust-toolchain.toml / rust-toolchain at repository root
**Parser:** TOML
**Current code:** `crates/app/rs/checks/rs/toolchain/**` (old `config_files.rs` / `toolchain_check.rs` are legacy seed material only)

## Implementation mapping contract

- exactly one `RS-TOOLCHAIN-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `discover.rs`, `facts.rs`, and `inputs.rs` may contain shared discovery, typed inputs, and normalization helpers only

Forbidden:

- grouped family test files such as `toolchain_tests.rs`
- helper files that hide multiple rule predicates behind one API

## Scope decision

`RS-TOOLCHAIN` is currently intentionally root-level, not a multi-root family.

It validates the top-level Rust toolchain contract for the repository:
- one effective root toolchain
- one effective root MSRV/toolchain relationship

This plan should not silently drift into per-workspace/per-package toolchain discovery without an explicit architecture decision.

## Discovery / ownership model

- `rust-toolchain.toml` at repo root is the primary owned input
- legacy `rust-toolchain` at repo root is a compatibility surface, not the preferred contract
- if both files exist, the ambiguity is owned by `RS-TOOLCHAIN-04`
- `RS-TOOLCHAIN` currently reads MSRV only from the root `Cargo.toml`

The family is about one repository toolchain contract, not many local toolchain contracts.

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-TOOLCHAIN-01 | R24 | Error | rust-toolchain.toml exists at repository root | Implemented |
| RS-TOOLCHAIN-02 | R25 | Error/Warn/Info | Channel + components policy. `stable` is clean inventory; pinned stable versions are tolerated inventory; nightly, pinned-nightly, and beta are errors; missing channel/components are warnings. Components must include `clippy` + `rustfmt`. | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TOOLCHAIN-03 | Warn/Info | MSRV consistency. If `rust-version` in Cargo.toml AND toolchain pins specific stable version, warn if pinned < MSRV. If `rust-version` is missing, inventory that MSRV consistency cannot be checked. | Implemented |
| RS-TOOLCHAIN-04 | Warn | Legacy `rust-toolchain` file (no .toml extension) cannot specify components. Warn to migrate. Also warn if both `rust-toolchain` and `rust-toolchain.toml` coexist (ambiguous). | Implemented |

## Input integrity / fail-closed expectations

The family depends on:
- root `rust-toolchain.toml` when present
- root `Cargo.toml` for MSRV comparison

Malformed inputs required for the rule should not silently weaken enforcement:
- malformed `rust-toolchain.toml` must surface explicitly
- malformed root `Cargo.toml` must not silently disable `RS-TOOLCHAIN-03`

## Channel policy details

The stable contract is:
- plain `stable` is accepted
- pinned stable versions are informationally tolerated
- `beta` is an error
- `nightly` is an error
- pinned-nightly forms are treated as nightly and are errors

## Cross-family dependency

`RS-TOOLCHAIN-03` and `RS-CARGO-15` deliberately touch the same MSRV space from different sides:
- `RS-CARGO-15` checks whether the manifest declares the metadata
- `RS-TOOLCHAIN-03` checks whether the chosen toolchain is compatible with that metadata

That overlap is intentional and should stay explicit in the plan.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| `profile` field (minimal/default/complete) | Explicit components check (RS-TOOLCHAIN-02) is stronger than implicit profile defaults. |
| Edition vs toolchain version compatibility | cargo catches at build time ("edition 2024 requires Rust 1.85+"). |
| Toolchain file gitignored | Failure is obvious and immediate (wrong toolchain). |
| Unknown/typo'd keys | Consequences caught by existing checks (missing channel, missing components). |
