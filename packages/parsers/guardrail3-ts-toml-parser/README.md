# guardrail3-ts-toml-parser

Facade crate that re-exports the typed parser for `guardrail3-ts.toml` files.

The schema mirrors the per-package adoption marker for TS workspaces and exposes
a `[checks]` table where each TS family can be disabled by setting the field to
`false`.
