# CARGO-03: skip required allows, remove escape hatches

**Date:** 2026-04-04 10:15

## Summary
CARGO-03 now skips lints in EXPECTED_CLIPPY_REQUIRED_ALLOW — if CARGO-01
requires a lint to be allowed, CARGO-03 doesn't warn about it. Removed
all 7 redundant_pub_crate escape hatches from guardrail3.toml. Zero
warnings for the required allow across all crates.
