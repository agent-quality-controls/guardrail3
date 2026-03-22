# Fix all verified critical findings

**Date:** 2026-03-19 18:25
**Task:** Implement 12 verified fixes from adversarial audit

## Fixes grouped by file

1. engine.rs: Path::starts_with for shadows + max_by_key for non-walk-up
2. cargo_lints.rs: Add missing_docs + missing_debug_implementations to EXPECTED_RUST_LINTS
3. structure_checks.rs + source_scan.rs: Remove dead check_unsafe, wire check_unsafe_code_forbid
4. npmrc_check.rs: rfind + duplicate key detection
5. validate.rs: ACMR diff filter + git ls-files --others for untracked
6. discover.rs: Check tsconfig/typescript, not just package.json
7. ts_arch_checks.rs: Add ports + application layer checks
8. hook_checks.rs: Add set -e / pipefail check
9. cli.rs: Add clap ArgGroup for scope flags
