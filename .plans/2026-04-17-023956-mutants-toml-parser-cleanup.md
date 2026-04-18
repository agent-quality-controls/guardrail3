Goal

Make `packages/parsers/mutants-toml-parser` validate clean under `g3` without adding compatibility shims or rule-specific hacks.

Approach

- Normalize the package from the old `crates/parser/...` layout to the current sibling-crate layout under `crates/`.
- Keep the root facade clean: behavior at the root, types under `types`.
- Add the package root policy files and explicit publish intent.
- Move parser tests onto owned sidecars and shared assertions.
- Keep the types crate passive and move any behavior into runtime or assertions.
- Re-run package tests and full `g3` validation after each structural step so the next remaining issue is either package debt or a real rule problem.

Key decisions

- No root type aliases for backward compatibility. Callers should use `mutants_toml_parser::types::...`.
- No getter shims on passive types. Downstream callers will be updated to fields directly if needed.
- If a rule failure remains after the package matches the established parser pattern, stop and surface it instead of patching rules silently.

Files to modify

- `packages/parsers/mutants-toml-parser/**`
- any direct downstream callers that still rely on removed root type exports or removed type methods
- worklog for this slice
