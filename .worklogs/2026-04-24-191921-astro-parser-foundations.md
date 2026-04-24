# Astro Parser Foundation Hardening

## Summary

Implemented the first Astro content-pipeline hardening slice in shared parser packages. This adds typed ESLint directive and package-script command parsers, and hardens existing ESLint config, package.json, and guardrail3-rs TOML parsers for the facts future Astro checks need.

## Decisions Made

- Kept parsing in packages/parsers and left Astro-family policy decisions for later ingestion/check packages.
- Removed canonical waiver selector construction from eslint-directive-parser; selectors belong in Astro policy matching, not shared parsing.
- Made eslint-directive-parser fail closed or parse typed facts for directive comments, inline ESLint config comments, Astro frontmatter/template HTML comments, template-expression comments, and malformed rule lists.
- Made package-script-command-parser expose typed command and ESLint invocation facts, including supported wrappers and fail-closed unsupported shell syntax. It captures command index/raw invocation for precise future findings.
- Made package-json-parser range facts distinguish safe lower bounds from unknown-below-minimum ranges and implemented SemVer prerelease ordering without integer overflow.
- Fixed eslint-config-parser ignored probe handling for the real ESLint API shape where ignored files can return undefined config.

## Verification

- cargo test -q --manifest-path packages/parsers/eslint-config-parser/Cargo.toml --workspace
- cargo test -q --manifest-path packages/parsers/package-json-parser/Cargo.toml --workspace
- cargo test -q --manifest-path packages/parsers/guardrail3-rs-toml-parser/Cargo.toml --workspace
- cargo test -q --manifest-path packages/parsers/eslint-directive-parser/Cargo.toml --workspace
- cargo test -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace
- cargo clippy -q --manifest-path packages/parsers/eslint-directive-parser/Cargo.toml --workspace --all-targets -- -D warnings
- cargo clippy -q --manifest-path packages/parsers/package-script-command-parser/Cargo.toml --workspace --all-targets -- -D warnings
- Adversarial review convergence completed; final narrow pass returned NO BLOCKING FINDINGS.

## Key Files For Context

- .plans/2026-04-24-173946-astro-content-pipeline-hardening.md
- packages/parsers/eslint-directive-parser
- packages/parsers/package-script-command-parser
- packages/parsers/package-json-parser/crates/runtime/src/parser.rs
- packages/parsers/package-json-parser/crates/types/src/document.rs
- packages/parsers/eslint-config-parser/crates/runtime/src/parser.rs
- packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs

## Next Steps

- Wire these parser outputs into packages/ts/astro ingestion.
- Add Astro family checks that consume these typed parser facts instead of parsing files directly.
