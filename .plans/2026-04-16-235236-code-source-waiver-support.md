Goal
- Make `guardrail3-rs.toml` waivers actually reach the `code` source family so package-local waivers can stand down `RS-CODE-SOURCE-19` without changing the rule meaning.

Approach
- Read the existing code ingestion and code source rule tests to find the smallest typed place to thread waivers.
- Add tests first that prove two cases: a matching `RS-CODE-SOURCE-19` waiver suppresses the large-type warning, and a non-matching waiver does not.
- Add a minimal code-family waiver type and carry parsed `guardrail3-rs.toml` waivers into `G3RsCodeSourceChecksInput`.
- Update `CodeSourceRuleInput` and `RS-CODE-SOURCE-19` to honor only an exact matching waiver on the file and selector.
- Re-run `code` workspace tests and parser package validation.

Key decisions
- Keep the waiver support narrow and typed inside the code family instead of teaching every code rule about raw parser structs.
- Use exact waiver matching by `rule`, `file`, and `selector` so this stays consistent with the rest of the repo.
- Do not weaken `RS-CODE-SOURCE-19` globally; only add the already-established package-local waiver mechanism.

Files to modify
- packages/rs/code/g3rs-code-types/src/types.rs
- packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs
- packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest.rs
- packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run_tests/*.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/support.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule.rs
- packages/rs/code/g3rs-code-source-checks/crates/runtime/src/rs_code_ast_19_large_type_inventory/rule_tests/*.rs
- packages/parsers/cargo-config-toml-parser/guardrail3-rs.toml
