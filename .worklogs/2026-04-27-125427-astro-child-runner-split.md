Summary

Split the Astro TS runner into explicit child entrypoints for setup, content, MDX, SEO, and file-tree state. Child-facing findings now use child-prefixed IDs, while the aggregate runner composes child entrypoints instead of directly calling the flat rule bucket.

Decisions made

- Kept the shared Astro ingestion snapshot for this slice. Creating separate Cargo packages before proving child ownership would add churn without changing behavior.
- Removed the config-side state child entrypoint. State remains file-tree owned in this runner split.
- Moved child ID remapping to the child runner boundary. Rule files can remain focused on their local checks while public child output gets stable child IDs.
- Kept existing family-wide rule behavior tests in place. Added focused child composition tests instead of refactoring the whole historical test layout in this commit.
- Treated MDX/SEO helper presence as MDX/SEO-owned, and content-root overlap as content-owned.

Key files for context

- apps/guardrail3-ts/crates/logic/family-runner-structure/src/run.rs
- apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/src/run_tests/cases.rs
- packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run.rs
- packages/ts/astro/g3ts-astro-config-checks/crates/runtime/src/run_tests/cases.rs
- packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/run.rs
- packages/ts/astro/g3ts-astro-file-tree-checks/crates/runtime/src/run_tests/cases.rs
- .plans/2026-04-27-121601-astro-child-family-runner-split.md

Verification

- cargo test --workspace in packages/ts/astro/g3ts-astro-config-checks
- cargo test --workspace in packages/ts/astro/g3ts-astro-file-tree-checks
- cargo test --workspace in apps/guardrail3-ts
- g3rs validate --path packages/ts/astro/g3ts-astro-config-checks
- g3rs validate --path packages/ts/astro/g3ts-astro-file-tree-checks
- g3rs validate --path apps/guardrail3-ts
- cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force
- g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory
- adversarial review loop converged with no remaining MUST FIX items for this slice

Next steps

- Extract child runners into separate packages only if package boundaries are needed next.
- Continue with new Astro child-family rules from the larger Astro/content plan.
