# Rule message clarity — TOOLCHAIN + FMT families

Audited RS-TOOLCHAIN (4 rules) and RS-FMT (8 rules) error messages. Applied copy fixes and bug fixes.

## Changes

### RS-TOOLCHAIN
- **01**: Added fix action (create rust-toolchain.toml with [toolchain] section)
- **02**: Differentiated nightly/beta/unsupported messages (were identical)
- **03**: Disambiguated duplicate title ("unparseable" vs "invalid"); added fix action to MSRV mismatch
- **04**: Fixed double-emit bug (Warn + Error when both files exist → now only Error); replaced "shadowed" jargon

### RS-FMT
- **01**: Fixed empty file path bug (was `Some("")`); added fix action; dropped `.rustfmt.toml` mention
- **02**: Added fix action to wrong-value message
- **03**: Made generic message specific (includes key name and file)
- **04**: Added fix action; replaced "prove the channel" jargon
- **05**: Added fix action (delete nested config)
- **06**: Added fix action (update edition to match Cargo)
- **07**: Added fix actions to missing-reason and weak-reason; fixed `file: None` on count result
- **08**: Added fix action; included directory name in message

## Bugs fixed
- TOOLCHAIN-04: double-emit when both rust-toolchain files present
- FMT-01: empty string file path
- FMT-07: count result had `file: None`

## Key files
- All rule.rs under toolchain/crates/runtime/src/ and fmt/crates/runtime/src/

## Next steps
- Continue auditing remaining families (deps, cargo, garde, hexarch, code, clippy, test, release)
