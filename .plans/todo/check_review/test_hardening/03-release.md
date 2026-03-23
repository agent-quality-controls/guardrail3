# Release Hardening Lane

## Focus

Tighten `release` from “broad strings found” toward actual semantic release wiring checks.

## Main attack classes

- fake workflow hits via comments/prose
- inherited path dependency edges
- publishability inference bugs
- `readme = false`
- malformed release config / partial facts

## Priority rule groups

### Repo/workflow
- `RS-RELEASE-01..12`

### Publishable crate metadata and deps
- `RS-PUB-01..14`

### Binary release workflow
- `RS-BIN-01..03`

## Explicit gaps to close

- `readme = false`
- `workspace = true` inherited local path edges for `RS-PUB-10/11`
- workflow command-context detection
- semantic `release-plz.toml` / `cliff.toml` baseline promotion
- rule inputs still too aggregate-heavy in parts of the family

## Success condition

Release tests prove:
- the rule hits only real release-wiring failures
- prose/comments do not satisfy the workflow checks
- inherited path-edge cases are caught exactly
