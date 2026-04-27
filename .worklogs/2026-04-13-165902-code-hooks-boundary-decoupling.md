## Summary

Decoupled the `code` config lane from pre-derived fact vectors and moved it to explicit config-file inputs. Decoupled the `hooks` family from raw script strings by making `hook-shell-parser` return owned parsed scripts with physical source lines and moving root hook parsing into ingestion.

## Decisions made

- `code` config checks now consume `G3RsCodeConfigFile` values instead of `exception_comments` and `unsafe_code_lints` fact lists.
  - Why: the public contract should be config files plus parsed config semantics where available, not ingestion-specific derived vectors.
  - Rejected: keeping the old fact-vector contract. It leaked ingestion internals and made the rule inputs less honest.
- `g3rs-code/exception-comment-inventory` scans explicit config file contents itself.
  - Why: it is a config-file comment rule, so it should receive files and own the local assertion.
- `g3rs-code/unsafe-code-lint` now reads parsed `Cargo.toml` state from `G3RsCodeConfigFileKind::CargoToml`.
  - Why: it is a manifest rule and should consume manifest data directly.
- `hook-shell-parser` now returns owned `ParsedShellScript`, `ExecutableLine`, `FailOpenWrapper`, and `SourceLine`.
  - Why: the previous borrowed parser output forced hook package boundaries to smuggle raw file strings through public inputs.
  - Rejected: keeping raw `content: String` in hook family types. That kept parsing in rule packages.
- `hooks` ingestion now parses selected hook scripts once and stores parsed scripts in family inputs.
  - Why: ingestion is the correct parse boundary for root hook files.
- Physical `source_lines` are part of the parser output, while logical-line parsing stays internal.
  - Why: inert-text and comment rules must still see heredoc bodies and ordinary source lines.
  - Rejected: exposing only logical lines. That broke heredoc coverage in `g3rs-hooks/hook-shared-18-executable-command-context-only`.
- Deleted stale `code` ingestion collectors that still advertised the old fact-vector model.
  - Why: leaving dead collectors in place would keep the wrong contract visible.

## Key files for context

- `.plans/2026-04-13-163713-code-hooks-boundary-decoupling.md`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_07_exception_comment_inventory/rule.rs`
- `packages/rs/code/g3rs-code-config-checks/crates/runtime/src/rs_code_config_12_unsafe_code_lint/rule.rs`
- `packages/parsers/hook-shell-parser/crates/parser/runtime/src/lib.rs`
- `packages/parsers/hook-shell-parser/crates/parser/runtime/src/support.rs`
- `packages/rs/hooks/g3rs-hooks-types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/run.rs`

## Next steps

- Audit `topology` and `hexarch` against the same family-type contract if they are brought back into active scope.
- Normalize remaining package-root naming drift (`file-tree` vs `filetree`) when the user asks for contract cleanup beyond typed inputs.
