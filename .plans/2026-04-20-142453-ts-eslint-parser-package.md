## Goal

Add a dedicated `eslint-config-parser` package and rewire `packages/ts/eslint` so `config-checks` receives a parsed document instead of discovery summaries.

End state:

- `packages/parsers/eslint-config-parser` exists as a normal parser package
- parser runtime shells out to a small Node helper that uses official ESLint loading/evaluation
- `g3ts-eslint-ingestion` selects one active root config by official precedence, parses it once, and returns parsed document state
- `g3ts-eslint-config-checks` consumes parsed document state only
- multi-root conflict inventory is removed from the config-content boundary

## Approach

1. Scaffold `packages/parsers/eslint-config-parser` from the existing parser package pattern.
2. Define parser-owned document/types:
   - selected config metadata
   - normalized effective-config probes
   - passive `EslintConfigDocument`
3. Implement runtime bridge:
   - Rust launches `node`
   - helper script imports `eslint`
   - helper normalizes selected config and effective config for probe files to JSON
   - Rust deserializes JSON into parser types
4. Redesign `g3ts-eslint-types` to carry:
   - `Missing`
   - `Unreadable`
   - `ParseError`
   - `Parsed { rel_path, document }`
5. Rework `g3ts-eslint-ingestion`:
   - select one active root config by official precedence
   - gather representative probe files from workspace crawl
   - call parser runtime once
6. Keep `g3ts-eslint-config-checks` minimal for now, but boundary-correct.

## Key Decisions

- Use ESLint's own Node-side loader instead of a pure Rust AST parser.
  - Reason: `eslint.config.*` is executable JS/TS module code, not a declarative format.
- Keep root-config precedence handling in ingestion.
  - Reason: `config-checks` should not receive discovery mechanics.
- Start with a normalized effective-config snapshot instead of trying to preserve raw JS AST.
  - Reason: the family cares about actual lint semantics, not faithful source reconstruction.

## Alternatives Considered

- Pure Rust tree-sitter parser over JS/TS config files.
  - Rejected: it cannot reliably evaluate imports, spreads, presets, and plugin-provided config objects.
- Leave the current summary-based scaffold in place and add rules later.
  - Rejected: it breaks the file-boundary contract we already agreed on.

## Files To Modify

- `packages/parsers/eslint-config-parser/**`
- `packages/ts/eslint/g3ts-eslint-types/**`
- `packages/ts/eslint/g3ts-eslint-ingestion/**`
- `packages/ts/eslint/g3ts-eslint-config-checks/**`
