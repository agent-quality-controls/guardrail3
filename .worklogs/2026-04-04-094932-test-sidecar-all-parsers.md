# Move inline tests to sidecars in all 5 parser crates

**Date:** 2026-04-04 09:49

## Summary
Fixed RS-TEST-01 in all 5 parser crates. Converted config.rs to
config/ directory module with sidecar tests: config/mod.rs (facade),
config/types.rs (production), config/tests/mod.rs + parsing.rs (tests).
Enabled test family for packages in guardrail3.toml. 49 tests pass.
Zero fixable guardrail warnings remaining.
