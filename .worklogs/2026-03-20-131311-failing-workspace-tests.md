# Add failing tests for workspace enforcement (rules 7-11)

**Date:** 2026-03-20 13:13

## Summary
Added 5 tests for R-ARCH-01 rules 7-11 that intentionally fail — proving the workspace enforcement checks don't exist yet. Test-first: implement the checks to make them pass.

## Tests added
- **rule_07**: Crate subdir exists but is not in workspace members
- **rule_08**: App Cargo.toml is `[package]` not `[workspace]`
- **rule_09**: Workspace lists a member that doesn't exist on disk
- **rule_10**: Workspace member points outside app directory (`../../packages/...`)
- **rule_11**: Root workspace includes an app as a direct member

## Current state
13 passing (rules 1-6, 12), 5 failing (rules 7-11). Total 18 tests.

## Next steps
Implement rules 7-11 in hex_arch_structure.rs to make the failing tests pass.
