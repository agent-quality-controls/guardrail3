# RS-CODE

Status: current, implemented, self-hosted, inventory-complete, still the main source-rule policy lane.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/code/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/code/README.md` for family-local behavior

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- current rule inventory is live through `RS-CODE-36`
- documented local bypasses stay visible in normal output:
  - `RS-CODE-04` for item-level `#[allow(...)]` / `#[expect(...)]`
  - `RS-CODE-06` for documented non-exempt `#[garde(skip)]`
- detailed semantics were historically tracked in `.plans/todo/checks/rs/code.md`
- two companion docs remain current supplements, not primary contract:
  - `apps/guardrail3/crates/app/rs/families/code/FIXES.md`
  - `apps/guardrail3/crates/app/rs/families/code/EXPANSION.md`
- `code-family-stabilization.md` is tactical history, not current authority

Scope model:

- global family over all non-excluded Rust source files
- subtree invocation must not narrow the owned source universe to one routed
  workspace

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`
- migrate the family from routed-root thinking to repo-global owned-file
  routing
- verify file ownership and exception-comment discovery both
  respect the shared owned source surface rather than routed roots

Known current risk:

- current docs/code still describe this family as routed-root scoped even
  though the target contract is repo-global over all non-excluded Rust files

Done means:

- repo-global route tests prove this family sees every owned Rust source file
  outside exclusions
- illegal or out-of-workspace Rust files do not disappear from `RS-CODE`
- production facts stay route-driven instead of path-glob driven

Historical/supplemental references:

- `.plans/todo/checks/rs/code.md`
- `.plans/todo/checks/rs/code-family-stabilization.md`

Next planning focus:

- move any still-live rule inventory details from the old ledger into the README over time
- keep `FIXES.md` for correctness backlog and `EXPANSION.md` for policy ideas; do not let either become an unowned shadow spec
