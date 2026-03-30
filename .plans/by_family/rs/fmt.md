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

Historical/supplemental references:

- `.plans/todo/checks/rs/fmt.md`
- `.plans/by_file/rs/rustfmt-toml.md`
- `.plans/by_file/tools/rustfmt.md`

Next planning focus:

- keep broadening exact-result attack coverage for nested override and dual-file-conflict discovery
- keep older support docs aligned with the live `families/fmt` path and already-completed `*_tests/` directory migration
- if TypeScript formatting planning is revived, keep it clearly separate from this root-level Rust family
