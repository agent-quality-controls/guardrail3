# guardrail3 — Self-compliance (dogfooding)

**Date:** 2026-03-15 14:06
**Scope:** Cargo.toml, clippy.toml, deny.toml, rustfmt.toml, rust-toolchain.toml, CLAUDE.md, all source files

## Summary
Made guardrail3 pass its own validation: 0 errors, 2 acceptable warnings.

## What was done
- Added full workspace lint configuration to Cargo.toml (40 clippy rules, 5 rust lints)
- Generated clippy.toml, deny.toml, rustfmt.toml, rust-toolchain.toml via `guardrail3 generate`
- Installed pre-commit hooks via `guardrail3 hooks install`
- Split 4 oversized files into sub-modules (config_files, deny_audit, hooks/validate, ts/config_files)
- Fixed 21 lint errors (dead code, unused results)
- Fixed minimal profile to include env mutation bans
- Created CLAUDE.md
