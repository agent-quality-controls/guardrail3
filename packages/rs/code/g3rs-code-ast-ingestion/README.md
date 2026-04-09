# g3rs-code-ast-ingestion

Builds `g3rs-code-ast-checks` input from a workspace crawl.

Current behavior:

- selects `.rs` files from the crawl
- skips fixture paths
- classifies `is_test`
- resolves `profile_name` as `library` or `binary` when Cargo target ownership is clear
- marks the exact library root file with `is_library_root`
- reads source content
- emits one `G3RsCodeAstChecksInput` per file
