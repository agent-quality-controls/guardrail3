# Update plan: standard parsers + rule ownership audit

**Date:** 2026-04-03 20:43

## Summary
Updated extraction plan with standard parser strategy (cargo_toml crate
for Cargo.toml, rust-toolchain-file for toolchain config, raw strings for
family-owned configs). Added rule ownership audit — all rules stay in
current families, cross-domain file reads resolved by pre-extraction.
CODE-26/27 confirmed dead, CODE-32 moves to test family.
