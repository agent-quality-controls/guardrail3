# Fail-Closed And Input Integrity

## hexarch

- `rs/hexarch` still fails open on unreadable/unparsable Rust sources for `RS-HEXARCH-22/23`.
- `rs/hexarch` also fails open on malformed `guardrail3.toml` for boundary-config / `allowed_deps` checks.

## cargo

- `RS-CARGO` still has an unrecorded fail-open on malformed `guardrail3.toml`:
  - profile-sensitive rules silently lose `profile_name` when config parsing fails
  - add explicit profile-input failure handling or stop depending on silently parsed config for those rules
- `g3rs-cargo/priority-order` implementation is broader than the plan text:
  - reconcile plan text and actual warning surface instead of allowing silent drift

## fmt

- `RS-FMT` still has an unrecorded fail-open/defaulting bug:
  - malformed `Cargo.toml` / `rust-toolchain.toml` gets silently dropped
  - some fmt rules then default or skip instead of surfacing input-integrity failure
- concrete under-implementation notes still matter:
  - `g3rs-fmt/nightly-keys-on-stable` currently depends only on repo-root toolchain facts
  - `g3rs-fmt/edition-mismatch` currently compares against repo-root Cargo metadata only

## toolchain

- `RS-TOOLCHAIN` still fails open on malformed `Cargo.toml` for MSRV checks.
- Add family-scoped input-integrity handling instead of treating parse failure as “no rust-version declared”.
- explicit channel-shape hardening is still needed:
  - handle `beta`
  - handle pinned `nightly-*`
- `g3rs-toolchain/msrv-consistency` still lacks profile context for the library-specific behavior the plan describes.

## deps

- `rs/deps` still misses target-specific dependency tables:
  - `target.*.dependencies`
  - `target.*.build-dependencies`
  - `target.*.dev-dependencies`

## release

- `RS-RELEASE-12` is only partially fail-closed:
  - discovery-critical failures emit an error
  - dependent repo/crate coverage rules can still run from incomplete facts
- Gate dependent release rules when discovery-critical inputs fail.

## general migration hardening

- Add explicit family-level input-failure handling wherever prerequisite config/source parsing currently defaults or skips.
