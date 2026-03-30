# RS-FMT

Status: current, implemented, self-hosted, repository-root formatting policy family.

Implementation root:

- `apps/guardrail3/crates/app/rs/families/fmt/`

Current source of truth:

- this file for family planning/status
- `apps/guardrail3/crates/app/rs/families/fmt/README.md` for family-local behavior

Current state:

- self-hosted with `crates/runtime`, `crates/assertions`, and `test_support`
- root-level family, not multi-root policy discovery
- nested `rustfmt.toml` files are treated as override/shadowing behavior, not additional legitimate policy roots

Scope model:

- repo-global by contract
- subtree validation should not localize this family unless the family contract
  changes first

Agent handoff focus:

- audit production path first:
  - `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`
  - `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/fmt/crates/runtime/src/facts.rs`
- prove repo-global behavior is explicit rather than accidental
- if product expectations change, treat that as a family-contract change, not a
  hidden mapper tweak

Known current risk:

- this family ignores routed scope entirely by design
- the risk is contract confusion, not a hidden production bug

Done means:

- docs and tests clearly state that `fmt` is repo-global
- subtree invocation tests prove current intended behavior
- no one mistakes missing subtree narrowing for a mapper regression

Historical/supplemental references:

- `.plans/todo/checks/rs/fmt.md`
- `.plans/by_file/rs/rustfmt-toml.md`
- `.plans/by_file/tools/rustfmt.md`

Next planning focus:

- keep broadening exact-result attack coverage for nested override and dual-file-conflict discovery
- keep older support docs aligned with the live `families/fmt` path and already-completed `*_tests/` directory migration
- if TypeScript formatting planning is revived, keep it clearly separate from this root-level Rust family
