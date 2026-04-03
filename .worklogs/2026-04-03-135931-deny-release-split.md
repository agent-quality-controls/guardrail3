# Split deny + release unsplit rules into rule.rs + helpers

**Date:** 2026-04-03 13:59

## Summary
Split 56 previously-unsplit rules (27 deny, 29 release) into the
rule.rs + tests/helpers.rs facade pattern. Production code moved to
rule.rs, test helpers to tests/helpers.rs, mod.rs cleaned to facade.
All 412 tests pass (146 deny + 266 release).
