# Rename G3RS And G3TS TOML Parsers

## Summary

Renamed the active TOML parser packages from `guardrail3-rs-toml-parser` and `guardrail3-ts-toml-parser` to `g3rs-toml-parser` and `g3ts-toml-parser`.

## Decisions Made

- Kept two parser packages because they parse two different config files: `guardrail3-rs.toml` and `guardrail3-ts.toml`.
- Renamed package and crate identities to match the existing `g3rs-*` and `g3ts-*` package convention.
- Kept schema type names such as `Guardrail3RsToml` and `Guardrail3TsToml` because those name the parsed file format, not the package.
- Updated the shared-infra verifier to load the parser rename manifest and fail on old active package names.

## Key Files

- `.plans/2026-05-11-114847-rename-g3rs-g3ts-toml-parsers.md`
- `.plans/2026-05-11-114847-rename-g3rs-g3ts-toml-parsers.manifest.toml`
- `scripts/verify-shared-infra-boundaries.py`
- `packages/parsers/g3rs-toml-parser/Cargo.toml`
- `packages/parsers/g3ts-toml-parser/Cargo.toml`

## Verification

- `cargo test --manifest-path packages/parsers/g3rs-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/parsers/g3ts-toml-parser/Cargo.toml --workspace`
- `cargo test --manifest-path packages/ts/astro/content/g3ts-astro-content-ingestion/Cargo.toml --workspace`
- `python3 scripts/verify-shared-infra-boundaries.py`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/parsers/g3rs-toml-parser`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/parsers/g3ts-toml-parser`
- `apps/guardrail3-rs/target/release/g3rs validate --path packages/ts/astro/content/g3ts-astro-content-ingestion`

## Next Steps

- Commit the shared-infra migration and parser rename together.
