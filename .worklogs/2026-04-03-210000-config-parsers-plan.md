# Plan: standalone config parser crates

**Date:** 2026-04-03 21:00

## Summary
Designed 5 standalone config parser crates: clippy-toml, deny-toml,
rustfmt-toml, nextest-toml, mutants-toml. Following cargo_toml crate
patterns (serde rename_all kebab-case, flatten catch-all, no
deny_unknown_fields, from_str/from_path API). Full type definitions
and guardrail3 usage mapping in .plans/2026-04-03-210000-config-parsers.md.
