# Worklog - test package plan boundary fix

## Summary

Updated the new `test` package plan so config inputs carry parsed
`CargoToml` / `NextestToml` / `MutantsToml` types instead of custom normalized
mini-schemas. The plan now matches the parser-boundary pattern already used in
the other config families.

## Decisions made

- Keep parsed config files at the package boundary.
  - Why: `nextest.toml` and `mutants.toml` are first-class config files with
    real parser crates, so inventing custom summarized structs would be the
    wrong boundary.

- Keep only orchestration-level facts normalized.
  - Examples: `has_tests`, `has_tokio_tests`, tool presence, mutation-hook
    activation.

## Key files for context

- `.plans/2026-04-09-204323-test-config-and-ast-packages.md`
- `packages/parsers/nextest-toml-parser`
- `packages/parsers/mutants-toml-parser`

## Next steps

1. Scaffold `packages/rs/test/` package group.
2. Start with `g3rs-test-config-checks` and `g3rs-test-ingestion`.
