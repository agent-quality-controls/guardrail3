# Fix parser Value re-export and shared metadata

**Date:** 2026-04-06 22:42
**Scope:** release-plz-toml-parser, cliff-toml-parser, 4 older parsers

## Fixes
1. release-plz-toml-parser and cliff-toml-parser: added `pub use toml::Value` re-export through runtime and facade. Both parsers use `BTreeMap<String, Value>` in public `extra` fields — consumers need the `Value` type without a direct `toml` dependency.
2. Added `[package.metadata.guardrail3] shared = true` to: cargo-config-toml-parser, mutants-toml-parser, nextest-toml-parser, guardrail3-rs-toml-parser.
