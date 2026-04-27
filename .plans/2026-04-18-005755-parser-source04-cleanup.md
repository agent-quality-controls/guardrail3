Goal

Remove the last remaining `g3rs-code/ast-04-item-level-allow-with-reason` warnings from the five parser package roots so the full package-root validate sweep returns `No findings.` everywhere.

Approach

- Update the centralized `fs.rs` and `parser.rs` allow attributes in:
  - `packages/parsers/mutants-toml-parser`
  - `packages/parsers/nextest-toml-parser`
  - `packages/parsers/release-plz-toml-parser`
  - `packages/parsers/rust-toolchain-toml-parser`
  - `packages/parsers/rustfmt-toml-parser`
- Normalize those attributes from comment-style `// reason:` to the already-accepted attribute form `#[allow(..., reason = "...")]`.
- Keep the code behavior unchanged. This is not a parser refactor; it is a source-shape cleanup that matches the already-clean parser packages.
- Revalidate each touched parser package, then rerun the full package-root sweep.

Key decisions

- Do not change `g3rs-code/ast-04-item-level-allow-with-reason`. The rule is consistent with the repo: the clean parser packages already avoid the warning by using the attribute `reason =` form.
- Do not add new waivers. The warning comes from an avoidable local source shape, so the architecturally correct fix is to normalize the source shape.

Files to modify

- `packages/parsers/mutants-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/mutants-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/nextest-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/release-plz-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/rust-toolchain-toml-parser/crates/runtime/src/parser.rs`
- `packages/parsers/rustfmt-toml-parser/crates/runtime/src/fs.rs`
- `packages/parsers/rustfmt-toml-parser/crates/runtime/src/parser.rs`
