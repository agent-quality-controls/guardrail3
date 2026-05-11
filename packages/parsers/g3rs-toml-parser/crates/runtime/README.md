# g3rs-toml-parser-runtime

Runtime crate for parsing `guardrail3-rs.toml`.

This crate owns:
- the typed public error
- the file-reading boundary
- the `parse` and `from_path` entry points

Schema types are exposed under `g3rs_toml_parser_runtime::types`.
