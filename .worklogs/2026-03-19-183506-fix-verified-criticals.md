# Fix 12 verified critical findings from adversarial audit

**Date:** 2026-03-19 18:35
**Scope:** engine.rs, cargo_lints.rs, structure_checks.rs, source_scan.rs, npmrc_check.rs, validate.rs, discover.rs, ts_arch_checks.rs, hook_script_checks.rs, hook_checks.rs, cli.rs + 4 test files

## Summary
Fixed 12 verified findings from the 14-agent adversarial audit. 4 findings were rejected (auditor wrong or not fixable).

## Fixes

### Coverage engine (engine.rs)
- Shadow detection: `String::starts_with` → `Path::starts_with` (fixes false sibling shadows)
- Non-walk-up resolution: `.find()` → `.max_by_key(components)` (selects nearest ancestor)

### Cargo lints (cargo_lints.rs)
- Added `missing_docs` (deny) and `missing_debug_implementations` (warn) to EXPECTED_RUST_LINTS

### Dead code (structure_checks.rs, source_scan.rs)
- Removed dead `check_unsafe` (R42) — redundant with clippy forbid
- Wired `check_unsafe_code_forbid` (R53) into source_scan orchestrator

### npmrc (npmrc_check.rs)
- Changed `.find()` to `.rev().find()` — last-wins matching (aligns with pnpm)
- Added T-NPMRC-01: duplicate key detection

### Scope flags (validate.rs)
- `--staged`: ACMR diff filter (was ACM, missing renames)
- `--dirty`: added `git ls-files --others --exclude-standard` for untracked files

### Discovery (discover.rs)
- TS detection now requires tsconfig.json OR typescript in deps (was: any package.json)

### Hex arch (ts_arch_checks.rs)
- Added `application` layer check (was: only domain + adapters)

### Hooks (hook_script_checks.rs, hook_checks.rs)
- Added H-SAFE-01: check for `set -e` or `set -euo pipefail`

### CLI (cli.rs)
- Scope flags (`--staged`, `--dirty`, `--commits`, `--files`) now mutually exclusive via clap ArgGroup

## Rejected findings
- CRIT-02 (_profile unused): intentional, code comment explains
- F04 (max-lines 400): 400 is the correct baseline
- CLI-03 (--code zero checks): false — code checks run unconditionally
- FINDING-H-01 (--no-verify): not detectable from static analysis
