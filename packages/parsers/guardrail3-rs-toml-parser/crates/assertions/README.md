# guardrail3-rs-toml-parser-runtime-assertions

Shared proof helpers for `guardrail3-rs.toml` parser tests.

Runtime sidecar tests call these helpers instead of asserting parser result
shape directly, so internal and external tests use the same proof surface.
