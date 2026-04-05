# g3-deny-content-checks

Extracted content checks for `deny.toml`.

This package receives an already parsed `deny_toml_parser::DenyToml` and
validates only `deny.toml` content semantics. The app orchestrator keeps
ownership of file discovery, authoritative config selection, coverage,
shadowing, parse-failure routing, and profile resolution.

Current scaffold status:

- public input contract is defined
- runtime crate exists and compiles
- rule area modules are split into advisories, bans, licenses, and sources
- deny-specific extraction plan lives in
  `.plans/2026-04-05-deny-content-checks-extraction.md`

The package does not yet contain migrated deny rules.
