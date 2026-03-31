# RS-DENY

Rust `cargo-deny` policy family.

This family owns the generated `deny.toml` contract: graph settings, ban
baselines, feature bans, license allowlists, advisory policy, source
restrictions, and inventory-style visibility for local exceptions. It is
workspace-local over legal workspaces plus deny-relevant files.

## What This Family Owns

`RS-DENY` owns:

- generated service/library deny baseline parity
- graph, bans, feature-ban, license, advisory, and source policy checks
- inventory rules for stricter local policy and documented local exceptions
- self-hosted runtime/assertions/test-support workspace structure

It does not own:

- Cargo workspace structure
- general Rust source architecture
- formatting or toolchain policy

Those belong to:

- `RS-ARCH`
- `RS-CARGO`
- `RS-CODE`
- `RS-FMT`
- `RS-TOOLCHAIN`

## Shared Placement And Routing

This family must not decide which Rust roots or deny-owned files are live.

It consumes:

- shared topology facts from `placement`
- legal workspaces plus deny-relevant files from `FamilyMapper::map_rs_deny()`

Inside that owned surface, the family may then do family-local work:

- deny config discovery and parsing
- profile-map and policy-context resolution
- workspace-local coverage and same-root conflict analysis
- fail-closed input collection
- per-rule fan-out

That split is intentional:

- `placement` decides what Rust roots exist
- `arch` decides legality for workspace placement and illegal family-file shapes
- `FamilyMapper` decides which legal workspaces and deny-relevant files reach `deny`
- `deny` decides deny-policy facts over that workspace-local owned surface

## Current Workspace Shape

```text
apps/guardrail3/crates/app/rs/families/deny/
  README.md
  rustfmt.toml
  rust-toolchain.toml
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_deny_01_*.rs
        ...
        rs_deny_30_*.rs
        rs_deny_01_*_tests/
          mod.rs
        ...
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_deny_01_*.rs
        ...
        rs_deny_30_*.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

## Current Status

This family is now in the same stabilized shape as `RS-FMT`:

- split family root plus `crates/runtime`, `crates/assertions`, and `test_support`
- one production rule file per `RS-DENY-*`
- one rule-specific sidecar directory per rule
- owned assertions modules for reusable result-shape proofs
- clean `RS-TEST` validation for the internal sidecar/assertions boundary
- family-local `rustfmt.toml` and `rust-toolchain.toml`
- no live family-root `deny.toml`; app-root deny policy remains the single allowed live config for this repo shape

The family-level test suite is currently green from the app workspace:

- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny --lib`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny-assertions --lib`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny-test-support --lib`

Recent hardening that is now part of the real contract:

- `RS-DENY-19` is intentionally strict: `[sources].allow-registry` must contain exactly one canonical crates.io entry, `sparse+https://index.crates.io/`
- `RS-DENY-28` now warns on unsupported schema in critical deny sections instead of silently skipping wrong-type containers

So `deny` is no longer missing its rule corpus or family structure. The remaining work is semantic hardening and fail-closed cleanup, not basic migration.

## Known Issues

This README should track real current defects, not the target state.

Known remaining problems:

- the deny hardening matrix is not closed yet
  - several rules still have explicit adversarial backlog in `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`
  - the open work is mostly broader mixed-root/profile attacks, parity framing, and false-positive controls rather than missing rule files
- generator/checker end-to-end parity is still weaker than the ideal contract
  - the family has strong rule-local parity tests against generator-derived baselines
  - but it still lacks a fuller end-to-end generator-root exactness / generate-then-validate closure story inside the family itself

Resolved recent drift:

- malformed `guardrail3.toml` no longer fails open for deny profile selection
  - deny now emits an explicit `guardrail3.toml` policy-context error instead of silently degrading profile-sensitive rules to service defaults
  - profile-sensitive rules (`RS-DENY-09`, `RS-DENY-25`, `RS-DENY-30`) now stand down when deny cannot trust the active profile context
- the old `RS-DENY-19` plan/code mismatch is gone; the plan now matches the stricter runtime behavior
- the old `RS-DENY-28` unsupported-schema gap is closed for critical section/container shapes

## Next Work

The next deny work should stay narrow:

1. add the missing adversarial coverage from the deny hardening matrix
2. improve end-to-end generator/root parity evidence

Do not broaden the deny policy surface until those are closed. Placement
legality is already owned by `arch`.
