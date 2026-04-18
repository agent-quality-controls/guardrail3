Goal

Normalize `packages/parsers/nextest-toml-parser` to the current parser package shape and remove all package-debt findings, leaving only any legitimate schema waivers if required.

Approach

- Move the package from `crates/parser/...` to sibling crates under `crates/`.
- Clean the root facade: behavior at the root, types under `types`.
- Add root policy files and explicit unpublished intent for internal crates.
- Keep the types crate passive and split or gate facades as required.
- Move parser test proof into the shared assertions crate and remove local sidecar escapes.
- Re-run package tests and `g3` validation after each structural step.

Key decisions

- No backward-compat root type aliases.
- No getter shims on types.
- If the remaining large schema types still trip inventory rules after cleanup, use the exact schema-waiver pattern already established in other parser packages rather than local lint hacks.

Files to modify

- `packages/parsers/nextest-toml-parser/**`
- any downstream callers that still use removed root type exports
- worklog for this slice
