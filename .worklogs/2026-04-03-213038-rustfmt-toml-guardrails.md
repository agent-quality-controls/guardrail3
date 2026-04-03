# Fix rustfmt-toml guardrail violations

**Date:** 2026-04-03 21:30

## Summary
Fixed ~64 guardrail violations on packages/rustfmt-toml/. Added full lint
policy, rust-toolchain.toml, clippy.toml, deny.toml, publish metadata,
README, test structure (inline tests), fs boundary module, allow reasons.
Configured [rust.packages] in guardrail3.toml to disable hexarch/garde/test/
release/hooks for library packages. 1 remaining: CODE-31 (80 pub fields,
intentional for config struct pattern).
