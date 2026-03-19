# Complete coverage maps + crawler Vec fix

**Date:** 2026-03-19 12:02
**Scope:** crawl.rs, project_map.rs, map.rs, coverage/*.rs, cli.rs, main.rs

## Summary
- Fixed crawler: all config file fields changed from Option<PathBuf> to Vec<PathBuf> so no instances are silently dropped in monorepos
- Added 4 new coverage maps: jscpd, tsconfig, rust-toolchain, npmrc
- Total: 11 coverage maps (clippy, deny, rustfmt, eslint, stylelint, prettier, cspell, jscpd, tsconfig, rust-toolchain, npmrc)

## Key Findings on steady-parent
- rust-toolchain: only validator-rust has one, 8 dirs uncovered
- tsconfig: all 206 dirs covered, mix of extends-base and standalone-strict
- jscpd: CWD-only, depends on invocation location
- npmrc: root .npmrc covers everything via walk-up
