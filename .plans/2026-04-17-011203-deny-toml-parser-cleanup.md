Goal
- Normalize `packages/parsers/deny-toml-parser` to the clean parser package shape used by the already-clean parser packages.
- Keep the root facade strict: root parse API only, schema types under `types`.
- Remove old parser-nested crate layout and old test/proof shape.

Approach
- Read the current `deny-toml-parser` package layout, manifests, parser/tests, and type modules.
- Move `crates/parser/{runtime,assertions,types}` to sibling `crates/{runtime,assertions,types}` and update workspace manifests.
- Rewrite root and runtime facades to the strict parser shape.
- Move sidecar proof into shared assertions and remove local type/module escapes from parser tests.
- Add package root policy files and package `guardrail3-rs.toml`.
- Mark internal crates unpublished unless a package-specific reason says otherwise.
- Run package tests and validate the package, then stop if the next issue is a real rule contradiction.

Key decisions
- No root type aliases.
- No local lint escape hatches unless there is a narrow package waiver path.
- Shared assertions must prove parser results; sidecars do not inspect sibling types directly.

Files to modify
- packages/parsers/deny-toml-parser/**
- direct fallout files elsewhere only if the parser public API path changes require it
