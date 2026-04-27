## Goal

Decouple the remaining weak `code` and `hooks` package boundaries:
- `code` config checks should receive config files, not precomputed lint/comment facts
- `hooks` packages should receive parsed hook scripts from ingestion instead of parsing raw strings inside rule runtimes

## Approach

1. Refactor `code` config input to be file-based.
   - Replace fact vectors with family-owned config file inputs.
   - Carry parsed `Cargo.toml` where available.
   - Carry raw config file content for comment inventory.
   - Move comment extraction and workspace lint lookup into the config rule runtime.
2. Refactor `hook-shell-parser` to own its AST.
   - Remove borrowed parser output so parsed scripts can cross package boundaries safely.
3. Refactor `hooks` family types and ingestion.
   - `config` input gets parsed selected-hook facts.
   - `source` input gets parsed script plus minimal lexical source data.
   - ingestion parses once and packages consume typed parsed input.
4. Remove production-time parser calls from hooks rule runtimes.
   - tests may still use `parse_script(...)` to build fixtures, but package runtime code should not.
5. Verify affected package workspaces and parser workspaces.

## Key Decisions

- Keep `g3rs-code/exception-comment-inventory` in config.
  - Reason: it inventories comments in config files, so the lane is correct; the old flaw was the input contract, not the lane.
- Use file-based `code` config inputs instead of tiny derived facts.
  - Reason: the user asked for config-like inputs rather than precomputed extraction.
- Use the real `hook-shell-parser` as the boundary primitive for hooks.
  - Reason: there is already a dedicated parser package, and ingestion should own parse-once work.
- Keep minimal lexical source data for hooks where rules truly need raw lines.
  - Reason: some rules inspect comments and branch layout that are not yet fully encoded in the parser AST.

## Files To Modify

- `.plans/2026-04-13-163713-code-hooks-boundary-decoupling.md`
- `packages/rs/code/g3rs-code-types/src/lib.rs`
- `packages/rs/code/g3rs-code-config-checks/**`
- `packages/rs/code/g3rs-code-ingestion/**`
- `packages/parsers/hook-shell-parser/**`
- `packages/rs/hooks/g3rs-hooks-types/src/lib.rs`
- `packages/rs/hooks/g3rs-hooks-config-checks/**`
- `packages/rs/hooks/g3rs-hooks-source-checks/**`
- `packages/rs/hooks/g3rs-hooks-ingestion/**`
