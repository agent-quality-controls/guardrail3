Goal
- Clean `packages/shared/reason-policy` under current guardrail families and normalize it to the clean root-crate shape.

Approach
- Collapse the one-member workspace into a root crate.
- Split `src/lib.rs` into a facade plus focused submodules so behavior and tests leave the facade.
- Add the missing workspace policy files and explicit publish intent.
- Verify with package tests, `cargo check`, and `guardrail3-rs validate --path packages/shared/reason-policy`.

Key decisions
- No backward-compat nested crate path. Downstream dependencies should point at the package root.
- Keep the package single-crate if the logic is small and cohesive.
- Stop and surface the issue only if a real rule contradiction appears after the package cleanup.

Files to modify
- `packages/shared/reason-policy/**`
- Downstream Cargo.toml path references if the nested member path is removed
- `.worklogs/...reason-policy-cleanup.md`
