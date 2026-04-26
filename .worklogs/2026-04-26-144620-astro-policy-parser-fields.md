# Summary

Added the missing finalized Astro policy keys to the shared `guardrail3-rs.toml` parser. This gives future Astro semantic checks typed access to generated-state, build-output, blog-route, metadata, JSON-LD, and Contentlayer policy globs.

# Decisions Made

- Kept parsing in the shared parser package, not in the Astro family.
- Added only schema fields and parser coverage in this slice; no Astro semantic checks were added here.
- Preserved unknown-field passthrough through `extra`.

# Key Files

- `packages/parsers/guardrail3-rs-toml-parser/crates/types/src/guardrail3_rs_toml.rs`
- `packages/parsers/guardrail3-rs-toml-parser/crates/runtime/src/parser_tests/parsing.rs`

# Verification

- `cargo test -q --manifest-path packages/parsers/guardrail3-rs-toml-parser/Cargo.toml --workspace`

# Next Steps

- Add Astro ingestion facts that select the nearest `guardrail3-rs.toml`.
- Add `TS-ASTRO-POLICY-01` and `TS-ASTRO-POLICY-02` over parsed policy facts.
