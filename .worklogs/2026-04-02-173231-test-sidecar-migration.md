# RS-ARCH-09: Test sidecar migration (307 → 9)

**Date:** 2026-04-02 17:32

## Summary
Migrated ~277 test sidecars from alongside-rule pattern to inside-rule pattern.
Rule files became rule/mod.rs directories with tests/ inside. RS-ARCH-09
#[path] violations dropped from 307 to 9.

## Pattern
Before: rule.rs + rule_tests/ (sibling)
After: rule/mod.rs + rule/tests/ (inside)

## Remaining 9
Handled by follow-up agent: facts_tests in mod.rs files, hooks-rs nested
paths, lib_tests in hooks-rs and project-tree.
