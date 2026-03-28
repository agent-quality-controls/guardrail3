# RS-DENY

Rust `cargo-deny` policy family.

This family owns allowed deny config placement plus the generated `deny.toml` contract: graph settings, ban baselines, feature bans, license allowlists, advisory policy, source restrictions, and inventory-style visibility for local exceptions.

## What This Family Owns

`RS-DENY` owns:

- allowed root placement for `deny.toml`, `.deny.toml`, and `.cargo/deny.toml`
- local shadowing detection for nested deny configs
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

## Current Workspace Shape

```text
apps/guardrail3/crates/app/rs/families/deny/
  Cargo.toml
  README.md
  deny.toml
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

- workspace root plus `crates/runtime`, `crates/assertions`, and `test_support`
- one production rule file per `RS-DENY-*`
- one rule-specific sidecar directory per rule
- owned assertions modules for reusable result-shape proofs
- clean `RS-TEST` validation for the internal sidecar/assertions boundary
- self-hosted family-root `deny.toml`, `rustfmt.toml`, and `rust-toolchain.toml`

The family-level test suite is currently green from the nested workspace:

- `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/deny/Cargo.toml --workspace`

Recent hardening that is now part of the real contract:

- `RS-DENY-19` is intentionally strict: `[sources].allow-registry` must allow only the accepted crates.io forms, and extra registries are errors
- `RS-DENY-28` now warns on unsupported schema in critical deny sections instead of silently skipping wrong-type containers

So `deny` is no longer missing its rule corpus or family structure. The remaining work is semantic hardening and fail-closed cleanup, not basic migration.

## Known Issues

This README should track real current defects, not the target state.

Known remaining problems:

- malformed `guardrail3.toml` still fails open for deny profile selection
  - `facts.rs` falls back to the default/empty profile map when `guardrail3.toml` cannot be parsed
  - profile-sensitive rules then degrade to service defaults instead of surfacing a deny-family input failure
  - this can silently skip library-only deny expectations such as library ban/wrapper policy
- the deny hardening matrix is not closed yet
  - several rules still have explicit adversarial backlog in `.plans/todo/check_review/test_hardening/14-clippy-deny-coverage-matrix.md`
  - the open work is mostly broader mixed-root/profile attacks, parity framing, and false-positive controls rather than missing rule files
- generator/checker end-to-end parity is still weaker than the ideal contract
  - the family has strong rule-local parity tests against generator-derived baselines
  - but it still lacks a fuller end-to-end generator-root exactness / generate-then-validate closure story inside the family itself

Resolved recent drift:

- the old `RS-DENY-19` plan/code mismatch is gone; the plan now matches the stricter runtime behavior
- the old `RS-DENY-28` unsupported-schema gap is closed for critical section/container shapes

## Next Work

The next deny work should stay narrow:

1. fix the malformed-`guardrail3.toml` profile fail-open path
2. add the missing adversarial coverage from the deny hardening matrix
3. improve end-to-end generator/root parity evidence

Do not broaden the deny policy surface until those are closed.
