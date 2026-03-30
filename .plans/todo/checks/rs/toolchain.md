# RS-TOOLCHAIN — rust-toolchain.toml checker (4 rules)

> Superseded as the primary family plan by [`.plans/by_family/rs/toolchain.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/toolchain.md).
> Keep this file as a detailed rule ledger and migration/history reference.

**Input:** local `rust-toolchain.toml` / `rust-toolchain` at each routed Rust
policy root
**Parser:** TOML
**Current code:** `apps/guardrail3/crates/app/rs/families/toolchain/**` (old `config_files.rs` / `toolchain_check.rs` are legacy seed material only)

## Implementation mapping contract

- exactly one `RS-TOOLCHAIN-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `discover.rs`, `facts.rs`, and `inputs.rs` may contain shared discovery, typed inputs, and normalization helpers only

Forbidden:

- grouped family test files such as `toolchain_tests.rs`
- helper files that hide multiple rule predicates behind one API

## Scope decision

`RS-TOOLCHAIN` is a routed policy-root family.

It validates one local toolchain/MSRV contract for each owned Rust policy root:
- workspace roots
- standalone package roots not claimed as workspace members

The family should not silently drift back to “validation-root means policy
root.” Root ownership comes from placement + family mapping.

## Discovery / ownership model

- each owned policy root must carry its own local `rust-toolchain.toml`
- local legacy `rust-toolchain` is a compatibility surface, not the preferred contract
- if both local files exist, the ambiguity is owned by `RS-TOOLCHAIN-04`
- `RS-TOOLCHAIN` reads MSRV from the owned root `Cargo.toml`
- parent/repo-root toolchain files do not satisfy governed app/package roots

In this repo, that means repo-root validation must still enforce
`apps/guardrail3/rust-toolchain.toml`, because Arch forbids a repo-root Rust
workspace manifest.

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-TOOLCHAIN-01 | R24 | Error | rust-toolchain.toml exists at each owned policy root | Implemented |
| RS-TOOLCHAIN-02 | R25 | Error/Warn/Info | Channel + components policy. `stable` is clean inventory; pinned stable versions are tolerated inventory; nightly, pinned-nightly, and beta are errors; missing channel/components are warnings. Components must include `clippy` + `rustfmt`. | Implemented |

## New rules from audit

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TOOLCHAIN-03 | Warn/Info | MSRV consistency. If `rust-version` in Cargo.toml AND toolchain pins specific stable version, warn if pinned < MSRV. If `rust-version` is missing, inventory that MSRV consistency cannot be checked. | Implemented |
| RS-TOOLCHAIN-04 | Warn | Legacy `rust-toolchain` file (no .toml extension) cannot specify components. Warn to migrate. Also warn if both `rust-toolchain` and `rust-toolchain.toml` coexist (ambiguous). | Implemented |

## Input integrity / fail-closed expectations

The family depends on:
- local owned-root `rust-toolchain.toml` when present
- local owned-root `Cargo.toml` for MSRV comparison

Malformed inputs required for the rule should not silently weaken enforcement:
- malformed `rust-toolchain.toml` must surface explicitly
- malformed owned-root `Cargo.toml` must not silently disable `RS-TOOLCHAIN-03`

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
- `RS-TOOLCHAIN-03` checks whether the chosen local policy-root toolchain is
  compatible with that metadata

That overlap is intentional and should stay explicit in the plan.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| `profile` field (minimal/default/complete) | Explicit components check (RS-TOOLCHAIN-02) is stronger than implicit profile defaults. |
| Edition vs toolchain version compatibility | cargo catches at build time ("edition 2024 requires Rust 1.85+"). |
| Toolchain file gitignored | Failure is obvious and immediate (wrong toolchain). |
| Unknown/typo'd keys | Consequences caught by existing checks (missing channel, missing components). |
