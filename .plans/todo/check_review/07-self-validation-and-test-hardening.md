# Self Validation And Test Hardening

## Meta-quality backlog

- guardrail3 still lacks explicit negative/self-validation coverage goals.
- Mutation-resistance hardening of per-rule tests should be an intentional later phase, not accidental.

## Rule-specific semantic drifts

- `RS-TEST-16` is narrower than the current plan:
  - current code only checks integration/sidecar test files, not all test-bearing files
  - current “effective line” counting is weaker than the production-code model described in the plan

## Why this file exists

These items were getting lost when grouped only under generic migration or family files. They are Rust-side test-hardening work and should remain explicit in the review backlog.
