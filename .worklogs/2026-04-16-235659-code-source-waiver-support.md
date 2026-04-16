Summary
- Added real code-family waiver support for source checks and used it to stand down `RS-CODE-SOURCE-19` in `packages/parsers/cargo-config-toml-parser`.
- The parser package now validates clean, and the code-family tests cover both direct package waivers and package-root waivers for member crates.

Decisions made
- Added a typed code-family waiver model instead of passing raw parser waiver structs into rules.
  Why: rules should see a small typed input, not parser-specific config objects.
  Rejected: reading `guardrail3-rs.toml` directly inside `RS-CODE-SOURCE-19`.
- Threaded waivers through source ingestion, not config checks.
  Why: the bug was that source rules had no waiver channel at all.
  Rejected: adding a special one-off escape hatch only for `RS-CODE-SOURCE-19`.
- Resolved waivers from the nearest ancestor `guardrail3-rs.toml`, not only the member crate directory.
  Why: package roots own policy for sibling member crates in the current package layout.
  Rejected: looking only beside each member `Cargo.toml`, which failed for package-root policy files like `packages/parsers/cargo-config-toml-parser/guardrail3-rs.toml`.
- Used exact selector matching with `struct:<Name>` / `enum:<Name>`.
  Why: it keeps waiver scope narrow and explicit.
  Rejected: broad file-level suppression of all `RS-CODE-SOURCE-19` findings.

Key files for context
- `packages/rs/code/g3rs-code-types/src/types.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/classify.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/select.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/basic.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule.rs`
- `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule_tests/direct.rs`
- `packages/parsers/cargo-config-toml-parser/guardrail3-rs.toml`

Next steps
- Continue package-by-package through `packages/parsers/*` and `packages/shared/*`.
- If the next parser package hits the same old `crates/parser/...` layout debt, apply the same package-local cleanup pattern first.
- Stop again only when the next remaining finding is not clearly package debt.
