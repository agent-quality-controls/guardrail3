# Rename G3RS and G3TS TOML Parsers

## Goal

The Rust and TypeScript TOML parser packages use the same naming scheme as the rest of the repository:

- `packages/parsers/g3rs-toml-parser`
- `packages/parsers/g3ts-toml-parser`

The package names and Rust crate import names must match those directory names.

## Approach

- Rename `packages/parsers/guardrail3-rs-toml-parser` to `packages/parsers/g3rs-toml-parser`.
- Rename `packages/parsers/guardrail3-ts-toml-parser` to `packages/parsers/g3ts-toml-parser`.
- Rename parser crate packages from `guardrail3-rs-*` and `guardrail3-ts-*` to `g3rs-*` and `g3ts-*`.
- Rename Rust crate import paths from `guardrail3_rs_*` and `guardrail3_ts_*` to `g3rs_*` and `g3ts_*`.
- Keep config-file type names such as `Guardrail3RsToml` and `Guardrail3TsToml`; those describe the parsed file format, not the package identity.
- Update the shared-infra manifest and verifier so old parser package names fail in active code.

## Files To Modify

- `.plans/2026-05-11-105304-clean-shared-infra-boundaries.manifest.toml`
- `scripts/verify-shared-infra-boundaries.py`
- `packages/parsers/g3rs-toml-parser/**`
- `packages/parsers/g3ts-toml-parser/**`
- active Cargo manifests and Rust files that import either parser
- guardrail allowlists that name either parser package

## Verification

- `python3 scripts/verify-shared-infra-boundaries.py`
- `cargo test --manifest-path packages/parsers/g3rs-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/parsers/g3ts-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/ts/astro/content/g3ts-astro-content-ingestion/Cargo.toml --workspace`
