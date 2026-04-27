Summary

Removed the last five parser-package warning pairs by normalizing their centralized parser/fs allow attributes to the already-accepted `reason = "..."` form. After that change, the full app-and-package-root validate sweep returned `No findings.` everywhere.

Decisions made

- Did not change `g3rs-code/ast-04-item-level-allow-with-reason`. The clean parser packages already showed the intended shape: centralized parser/fs allows should use the attribute `reason =` form, not the comment-style `// reason:` form.
- Kept parser behavior unchanged. The only source edits were attribute normalization on the centralized boundary modules.
- Reused the existing full-sweep report path `.worklogs/2026-04-18-005459-full-validate-report.txt` and overwrote it with the final clean sweep so there is one canonical report for the completed state.

Key files for context

- `.plans/2026-04-18-005755-parser-source04-cleanup.md`
- `.worklogs/2026-04-18-005459-full-validate-report.txt`
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

Next steps

- None for Rust package-root cleanliness. The current full sweep is clean under all families.
