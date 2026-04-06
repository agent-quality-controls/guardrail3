# Add lint config to reason-policy

**Date:** 2026-04-06 16:16
**Scope:** packages/reason-policy/

## Summary
reason-policy had no workspace lint config at all. Added full lint tables matching the repo standard. Also added `[lints] workspace = true` to the member crate so it inherits the workspace lints.

## Remaining known finding
- apps/guardrail3 is missing `unreachable_pub = "deny"` — legitimate finding, but enabling it requires fixing 28 source violations across app crates first. Not a config-only fix.
