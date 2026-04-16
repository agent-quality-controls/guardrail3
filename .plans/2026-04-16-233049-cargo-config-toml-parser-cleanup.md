Goal
- Clean `packages/parsers/cargo-config-toml-parser` under the active g3 families without changing rules unless a real contradiction appears.

Approach
- Read the current package root, crate layout, and the files named by validation.
- Fix package-local debt in the intended direction: package-style sibling crates, owned sidecars, shared assertions proof, typed public errors, root policy files, and release metadata.
- Re-run package validation after each batch until either the package is clean or the next failure is a real rule problem.

Key decisions
- Treat this as package cleanup first, not a rule exercise.
- If any failure depends on the same apparch/arch-style overlap already fixed elsewhere, stop and report it instead of patching around it.
- Keep changes local to this parser package unless a broader bug is proven by tests first.

Files to modify
- packages/parsers/cargo-config-toml-parser/Cargo.toml
- packages/parsers/cargo-config-toml-parser/guardrail3-rs.toml
- packages/parsers/cargo-config-toml-parser/deny.toml
- packages/parsers/cargo-config-toml-parser/rustfmt.toml
- packages/parsers/cargo-config-toml-parser/crates/parser/**
- packages/parsers/cargo-config-toml-parser/crates/types/**
