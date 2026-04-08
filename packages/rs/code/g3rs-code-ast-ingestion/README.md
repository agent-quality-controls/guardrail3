# g3rs-code-ast-ingestion

Builds `g3rs-code-ast-checks` input from a workspace crawl.

Current behavior:

- selects `.rs` files from the crawl
- skips fixture paths
- classifies `is_test`
- reads source content
- emits one `G3RsCodeAstChecksInput` per file

Current limitation:

- `profile_name` is left as `None`
