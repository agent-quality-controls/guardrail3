# Test infrastructure unification + TS-ARCH-01 suite

**Date:** 2026-03-20 19:24

## Summary
Shared test_support/ module extracted. RS-ARCH-01 helpers refactored to use it. TS-ARCH-01 test suite bootstrapped using same golden fixture. All 202 RS-ARCH-01 passing tests still pass. 7 intentional failures unchanged (rules 07-11 unimplemented, rule 06 .gitkeep gap, rule 12 empty src gap).

## Changes
- New: tests/unit/test_support/{mod,fixture,fs_ops,assertions}.rs
- New: tests/unit/ts_arch_01/{mod,helpers,golden,rule_01..rule_07}.rs
- Refactored: rs_arch_01/helpers.rs re-exports from test_support
- Refactored: all rs_arch_01/rule_*.rs updated imports (copy_fixture, arch_errors)
- Deleted: legacy test_r_arch_01.rs
- Updated: unit.rs with test_support + ts_arch_01 modules
