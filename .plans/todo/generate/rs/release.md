# Rust Generator — `release`

## Generated artifacts

At the validation root, when the validated Rust release domain is service-profile:
- `release-plz.toml`
- `cliff.toml`
- `.github/workflows/release.yml`

When the validation root contains publishable binary crates:
- `.github/workflows/binary-release.yml`

## Ownership mode

- exact-owned

## Root selection

`release` is a validation-root family.

The generator owns release artifacts only at the validation root.
That release domain remains validation-root-owned even when the repository also contains nested app workspaces or standalone package roots.

Release-domain profile is resolved from the validation root's Rust config contract:
- the validation root `guardrail3.toml`
- specifically the root Rust profile / release-enablement settings that drive `RS-RELEASE`

It must never generate release artifacts at:
- nested app workspaces
- standalone package roots
- workspace member roots
- inner hex structural roots

## Required generator contract

- service-profile validation roots receive the full release artifact set
- non-service validation roots receive no release artifacts
- validation roots with Rust release checks disabled receive no release artifacts
- `cliff.toml` is the canonical changelog config for the validation root
- `release-plz.toml` is derived from actual publishable package facts governed by the validation root:
  - real package names
  - real package inventory
  - no placeholder entries
- `.github/workflows/release.yml` contains a real release-plz execution path, real dry-run publishing step, and real token wiring that satisfy the release checker contract
- `.github/workflows/binary-release.yml` exists exactly when the validation root owns publishable binary crates and satisfies the binary release workflow contract

Nested Rust roots do not get independent release domains under this contract.

The generator does not own:
- package README content
- `cargo-semver-checks` installation state

## Checker target

- `.plans/todo/checks/rs/release.md`
- checker family: `RS-RELEASE`

The generated result must satisfy the generator-ownable release surfaces for:
- `RS-RELEASE-02`
- `RS-RELEASE-03`
- `RS-RELEASE-04`
- `RS-RELEASE-05`
- `RS-RELEASE-06`
- `RS-RELEASE-07`
- `RS-BIN-01`
- `RS-BIN-02`

## Parity contract

1. `generate -> validate`
- generate root release artifacts
- `RS-RELEASE` passes for the generator-owned release surfaces

2. `generate twice`
- second generation is byte-identical when publishable package facts are unchanged

3. negative mutation
- mutating one generated release config or workflow surface produces the exact `RS-RELEASE-*` or `RS-BIN-*` finding for that surface

4. inventory exactness
- generated package inventory matches the actual publishable package set owned by the validation root
- no placeholder entries remain
