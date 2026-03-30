# TS-LIBARCH

Status: planned family contract, no cohesive family runtime yet.

Implementation roots:

- library/package concerns are currently scattered across:
  - `apps/guardrail3/crates/app/ts/validate/package_check.rs`
  - `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - app-type handling in `apps/guardrail3/crates/app/ts/validate/mod.rs`

Current source of truth:

- this file for family planning/status
- `.plans/todo/checks/ts/libarch.md` as the detailed family ledger until the cutover is complete

Current state:

- library/package architecture is still a design lane, not a distinct runtime family

Rule inventory:

- `TS-LIBARCH-01` — library/package roots are correctly identified.
  - What it should do: classify TS library/package roots distinctly from service/content roots.
  - What it is for: package architecture cannot be enforced if libraries are not identified cleanly first.
- `TS-LIBARCH-02` — canonical public entrypoints exist.
  - What it should do: require approved public entrypoints for library packages.
  - What it is for: this prevents ad hoc import surfaces from leaking internal modules.
- `TS-LIBARCH-03` — internal vs exported module layout is coherent.
  - What it should do: enforce the internal/public structure contract of a library package.
  - What it is for: this keeps packages evolvable without coupling consumers to internals.
- `TS-LIBARCH-04` — package dependency shape matches the library contract.
  - What it should do: enforce architectural dependency-shape rules such as `peerDependencies` vs `dependencies` where relevant.
  - What it is for: library boundary design is partly expressed through manifest dependency shape.
- `TS-LIBARCH-05` — internal library layering is respected.
  - What it should do: reject forbidden internal import directions inside structured library packages.
  - What it is for: this gives library packages their own architecture pressure instead of borrowing service-app rules.

Current code mapping:

- no dedicated runtime yet
- current partial concerns are scattered across:
  - `apps/guardrail3/crates/app/ts/validate/package_check.rs`
  - `apps/guardrail3/crates/app/ts/validate/package_deps.rs`
  - `apps/guardrail3/crates/app/ts/validate/mod.rs`

Current doc/code reconciliation notes:

- this family is still planning-led
- the old ledger is the only detailed source today, but it still needs conversion from broad design bullets into a concrete rule set like the one above

Historical/supplemental references:

- `.plans/todo/checks/ts/libarch.md`

Next planning focus:

- define concrete library root detection and package-boundary architecture inputs
- decide what the minimum viable first runtime for `ts/libarch` is, instead of letting package architecture stay scattered
