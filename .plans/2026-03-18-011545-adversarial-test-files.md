# Create 2 adversarial test files for guardrail3

**Date:** 2026-03-18 01:15
**Task:** Create adversarial_deep_nesting.rs and adversarial_broken_configs.rs test files

## Goal
Two comprehensive test files exercising deeply nested workspace structures and corrupted/weird configs.

## Approach
Copy exact header pattern from adversarial_generate.rs, implement fixtures and tests per spec.

## Key decisions
- Use `rs diff` for dry-run checks (not `--dry-run` flag on generate)
- Use `rs generate` for actual file generation
- `is_pure=false` for service profile means global-state types (LazyLock/OnceLock) are NOT banned in clippy.toml
- `is_pure=true` for library profile means global-state types ARE banned

## Files to Modify
- `apps/guardrail3/tests/adversarial_deep_nesting.rs` — new file
- `apps/guardrail3/tests/adversarial_broken_configs.rs` — new file
