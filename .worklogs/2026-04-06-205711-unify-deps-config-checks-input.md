# Unify deps config checks into one input type and one check function

**Date:** 2026-04-06 20:57
**Scope:** g3rs-deps-config-checks, apps/guardrail3 deps family

## Summary
Replaced `G3RsDepsConfigPolicyChecksInput` + `G3RsDepsConfigDirectDependencyCapInput` + `check_policy` + `check_direct_dependency_cap` with one unified `G3RsDepsConfigChecksInput` and one `check` function. All fields are required — guardrail3.toml must be provided. The dep cap check ignores the guardrail fields but they're present in the type to prevent silent skips.

## Key Files
- `packages/g3rs-deps-config-checks/crates/types/src/input.rs` — unified input type
- `packages/g3rs-deps-config-checks/crates/runtime/src/run.rs` — single check function
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/run.rs` — updated app caller
