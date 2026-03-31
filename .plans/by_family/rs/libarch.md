# RS-LIBARCH

Status: current, implemented, self-hosted, newly live.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/libarch/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/libarch/README.md` for family-local behavior
- `.plans/todo/checks/rs/libarch.md` as the detailed rule ledger and design history

Current state:

- self-hosted with:
  - `crates/runtime`
  - `crates/assertions`
  - `test_support`
- runtime/model/config/reporting selection already know `libarch`
- the family now owns layered-library escalation, layered crate-set shape, layer dependency direction, and root facade export policy for package-owned library roots
- workspace-membership exactness now belongs to `RS-TOPOLOGY`, not `RS-LIBARCH`
- the old detailed design ledger remains useful as history, but it is no longer describing a hypothetical family

Scope model:

- routed package-root family
- subtree validation should narrow to the owning routed package roots, not to
  arbitrary repo-global package discovery

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/libarch/crates/runtime/src/facts.rs`
  - `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`
- prove only routed package roots reach layered-library checks after the
  whole-project walker change
- add subtree tests for sibling package non-bleed and package-zone ownership

Known current risk:

- no confirmed production bug yet, but this family is new enough that subtree
  proof coverage is still thin

Done means:

- nested-path tests prove only the owning routed package roots are active
- escalation and layer-shape findings remain package-local
- no family-local package-root rediscovery bypasses the route

Historical/supplemental references:

- `.plans/todo/checks/rs/libarch.md`
- `.plans/todo/family-implementation-handoffs/libarch.md`
- `topology` and `hexarch` docs where package/app ownership boundaries are already described

Next planning focus:

- keep package architecture separate from generic Cargo policy and from repo-global `topology`
- pressure fail-closed and dependency-direction edges as the package zone evolves
