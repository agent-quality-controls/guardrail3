# Update rules 01-03 tests to break portal

## Summary
Updated 6 tests across rules 01-03 to also break the new portal app, mirroring the RS pattern of breaking ALL apps in "everywhere"/"all" tests.

## Changes
- rule_01 `all_ts_apps_missing_modules`: also removes portal's modules/, count 2→3
- rule_02 `missing_all_four`: breaks both admin + portal (4→8 errors)
- rule_02 `all_three_violations`: injects 3 violation types into both apps (3→6)
- rule_03 `missing_all_four`: breaks both admin + portal io dirs (4→8)
- rule_03 `all_three_violations`: injects 3 violation types into both apps (3→6)
- rule_03 `violations_in_both_adapters_and_ports`: breaks both apps (2→4)

## Key decisions
- For portal mutations that touch adapters/, inner hexes (payments, ai-chat) become unreachable — no cascade errors
- For "all_three_violations" in rule_03, mutate portal's ports/ instead of adapters/ to avoid inner hex cascade
- Replaced `assert_standard` (which calls `assert_all_mention_admin`) with explicit per-app assertions + shared helpers in multi-app tests
