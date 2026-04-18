Goal
- Normalize `packages/parsers/rustfmt-toml-parser` to the clean parser package shape and get it clean under `guardrail3-rs validate`, except for intentional escape-hatch warnings if that is all that remains.

Approach
- Read the current package manifest, root facade, and member crates to identify old `crates/parser/...` layout, old root exports, and stale sidecar/assertions wiring.
- Reshape the package to sibling crates under `crates/{runtime,assertions,types}` and update the root crate manifest and member paths.
- Move schema types under `rustfmt_toml_parser::types`, keep root API limited to parse behavior, and rewrite downstream callers accordingly.
- Move parser test proof fully into the shared assertions crate, keeping file-owned sidecars and removing local helper leakage.
- Add or normalize package policy files (`guardrail3-rs.toml`, `rustfmt.toml`, release metadata, publish intent) only as required by current findings.
- Verify with package tests, downstream compile checks if imports change, and `guardrail3-rs validate --path packages/parsers/rustfmt-toml-parser`.

Key decisions
- No backward-compat root type aliases. Downstream callers move to `rustfmt_toml_parser::types::...`.
- Treat any remaining `RS-CODE-SOURCE-04` centralized parser/fs warnings as intentional escape-hatch inventory, not package debt.
- Stop and surface the issue if the remaining blocker is a rule contradiction rather than package cleanup.

Files to modify
- `packages/parsers/rustfmt-toml-parser/**`
- Downstream callers of `rustfmt_toml_parser` types, if any
- `.worklogs/...rustfmt-toml-parser-cleanup.md`
