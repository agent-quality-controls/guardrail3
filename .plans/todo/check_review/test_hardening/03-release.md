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

## Closed in current pass

- workflow command-context detection has been tightened enough to stop counting:
  - comment lines
  - display/prose text
  - `echo ...` fake command mentions
- `RS-RELEASE-05`, `RS-RELEASE-06`, `RS-RELEASE-07`, `RS-BIN-01`, and `RS-BIN-02` now use rule-specific `*_tests/` directories
- `readme = false` is now treated as explicit opt-out instead of silently falling back to default `README.md`
- `RS-PUB-04`, `RS-PUB-05`, and `RS-RELEASE-11` now use rule-specific `*_tests/` directories with non-publishable and opt-out attacks
- inherited `workspace = true` path edges are now surfaced into release-edge facts for `RS-PUB-10/11`
- `RS-PUB-10` and `RS-PUB-11` now use rule-specific `*_tests/` directories with inherited-edge attacks
- `RS-RELEASE-12` now uses a rule-specific `*_tests/` directory with malformed-config and partial-facts coverage over synthetic `ProjectTree` inputs
- `RS-PUB-09` now uses a rule-specific `*_tests/` directory with real richer-fixture `cargo publish --dry-run` pass/fail coverage
- unreadable README fail-closed coverage is now exercised with a real on-disk permission failure and exact `RS-RELEASE-12` ownership
- unreadable cached `Cargo.toml`, `release-plz.toml`, `cliff.toml`, and workflow YAML inputs now fail closed instead of disappearing from release-family discovery
- `RS-RELEASE-03` now enforces the canonical `release-plz.toml` workspace baseline in addition to package coverage
- `RS-RELEASE-04` now enforces the canonical `cliff.toml` git baseline, including commit-parser coverage
- `publish = []` is now treated as non-publishable, both directly and through `publish.workspace = true`

## Remaining gaps

- release-family rule migration to rule-specific `*_tests/` directories is complete
- workflow facts are less bypassable now, but they still collapse rich Actions structure into booleans before rule evaluation

## Adversarial findings queued

- `RS-BIN-01` and `RS-BIN-02` likely still overcount unrelated build/release jobs as a valid binary-release path

## Fixture note

The release pass now uses two fixture modes:

- folder structure plus config files only
- richer project fixture with real crate bodies under `tests/fixtures/r_arch_01/golden`

That is enough for:

- workflow YAML semantics
- manifest parsing
- release-plz/cliff malformed-config coverage
- workspace dependency inheritance
- publishability inference from config
- realistic `cargo publish --dry-run` behavior for `RS-PUB-09`

It is not enough for:

- richer Actions execution semantics beyond the current parsed-step model

## Success condition

Release tests prove:
- the rule hits only real release-wiring failures
- prose/comments do not satisfy the workflow checks
- inherited path-edge cases are caught exactly
