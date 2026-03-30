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

- routed roots plus routed scoped files
- subtree validation should narrow both source-file scans and config-adjacent
  reporting to the active routed root set

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/scoped_files.rs`
- expand the current subtree/runtime proof set beyond the existing narrow
  config-result tests
- verify file ownership, structural caps, and exception-comment discovery all
  respect routed `scoped_files`

Known current risk:

- this family already has the best subtree coverage, but the shared runtime
  subtree test harness is stale and needs repair

Done means:

- runtime subtree tests compile again
- subtree tests prove file scans and config-side findings do not bleed across
  sibling roots
- production facts stay route-driven instead of path-glob driven

Historical/supplemental references:

- `.plans/todo/checks/rs/code.md`
- `.plans/todo/checks/rs/code-family-stabilization.md`

Next planning focus:

- move any still-live rule inventory details from the old ledger into the README over time
- keep `FIXES.md` for correctness backlog and `EXPANSION.md` for policy ideas; do not let either become an unowned shadow spec
