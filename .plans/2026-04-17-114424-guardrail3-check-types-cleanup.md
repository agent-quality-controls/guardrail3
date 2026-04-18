Goal
- Clean `packages/shared/guardrail3-check-types` under current guardrail families with the minimal package-local changes required.

Approach
- Read the root manifest and the single member crate manifest to determine whether this package should publish and whether it should stay single-crate or adopt a cleaner package shape.
- Add the missing package policy files and normalize `deny.toml` to the current baseline.
- Add `guardrail3-rs.toml` and make publish intent explicit at the member crate.
- Verify with package tests and `guardrail3-rs validate --path packages/shared/guardrail3-check-types`.

Key decisions
- No API compatibility work. If the crate shape is already coherent as a one-member package, keep it that way.
- Stop and surface the issue only if a non-package rule contradiction appears after the baseline cleanup.

Files to modify
- `packages/shared/guardrail3-check-types/**`
- `.worklogs/...guardrail3-check-types-cleanup.md`
