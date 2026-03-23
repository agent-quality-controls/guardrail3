# Code Hardening Lane

## Focus

Attack source-level rules as bypass surfaces, not stylistic hints.

## Main attack classes

- suppression tricks
- attribute placement tricks
- aliasing/import tricks
- grouped `use` forms
- nested-module placement
- test-vs-prod path confusion
- parse/read failures
- public-API leakage edge cases

## Priority rule groups

### Escape hatches / bypasses
- `RS-CODE-01..08`
- `RS-CODE-17..24`

### Public API and organization
- `RS-CODE-25..29`

## Explicit gaps to close

- retire legacy `ast_helpers`
- whole-type `#[garde(skip)]` ownership
- grouped/aliased attribute edge cases
- fail-closed parsing already added in `RS-CODE-30`; deepen adversarial coverage

## Success condition

Each rule has:
- golden coverage
- one attack vector applied across all relevant source files in the golden tree
- exact file hit sets
- false-positive controls for similar legal syntax
