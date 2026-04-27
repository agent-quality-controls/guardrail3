Summary

Normalized `packages/parsers/nextest-toml-parser` to the current sibling-crate package shape and removed all package-debt errors. The only remaining signals are the 2 intentional `g3rs-code/ast-04-item-level-allow-with-reason` inventory warnings on the centralized fs and parser boundary allows.

Decisions made

- Moved the package from `crates/parser/...` to `crates/{runtime,assertions,types}` to match the clean parser package shape.
- Removed the old root type dump. The root facade now exposes behavior at the root and `NextestToml` through `types`, while the detailed schema stays under `types::nextest_toml::{basics,execution,profile,scripts}`.
- Moved parser-side type-shape proof into the shared assertions crate so the parser sidecar no longer imports sibling local modules.
- Split the oversized `nextest_toml` schema mirror into section modules and made `nextest_toml/mod.rs` a real facade.
- Kept the centralized `toml::from_str` and centralized fs boundary allows visible as warnings instead of hiding them behind compatibility code or waiver support.

Key files for context

- `packages/parsers/nextest-toml-parser/Cargo.toml`
- `packages/parsers/nextest-toml-parser/src/lib.rs`
- `packages/parsers/nextest-toml-parser/src/types.rs`
- `packages/parsers/nextest-toml-parser/guardrail3-rs.toml`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/parser_tests/parsing.rs`
- `packages/parsers/nextest-toml-parser/crates/assertions/src/parser.rs`
- `packages/parsers/nextest-toml-parser/crates/types/src/nextest_toml/mod.rs`
- `packages/parsers/nextest-toml-parser/crates/types/src/nextest_toml/document.rs`
- `packages/parsers/nextest-toml-parser/crates/types/src/nextest_toml/execution.rs`
- `packages/parsers/nextest-toml-parser/crates/types/src/nextest_toml/profile.rs`
- `packages/parsers/nextest-toml-parser/crates/types/src/nextest_toml/scripts.rs`

Next steps

- Commit this slice only.
- Continue to the next parser package root.
- Stop only if the next remaining issue is a real rule defect rather than package debt.
