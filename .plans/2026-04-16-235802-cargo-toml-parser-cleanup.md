Goal
- Clean `packages/parsers/cargo-toml-parser` under the active g3 families without changing rules unless a real contradiction appears.

Approach
- Read the current package root, crate layout, and the files named by validation.
- Fix package-local debt in the intended direction: package-style sibling crates, owned sidecars, shared assertions proof, typed public errors, root policy files, and release metadata.
- Re-run package validation after each batch until either the package is clean or the next failure is a real rule problem.

Key decisions
- Treat this as package cleanup first, not a rule exercise.
- Reuse the parser-package shape already established in `cargo-config-toml-parser` where it fits.
- If the remaining findings collapse to schema-mirror large-type warnings again, stop and report that instead of forcing fake wrapper structs.

Files to modify
- packages/parsers/cargo-toml-parser/Cargo.toml
- packages/parsers/cargo-toml-parser/guardrail3-rs.toml
- packages/parsers/cargo-toml-parser/{clippy,deny,rustfmt,release-plz,cliff}.toml
- packages/parsers/cargo-toml-parser/crates/parser/** or flattened equivalents
- packages/parsers/cargo-toml-parser/src/**
