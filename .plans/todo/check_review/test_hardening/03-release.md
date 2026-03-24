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
- `RS-RELEASE-01` now only inventories allowed root license filenames instead of any arbitrary `license-file` path
- `RS-BIN-01` now requires a coherent binary release path:
  - same-job `cargo build --release` plus GitHub release action, or
  - split build/publish jobs linked by `needs:`
- `RS-BIN-02` now accepts Linux presence from the actual linked release path, including:
  - split `needs:` build jobs
  - matrix-driven `runs-on: ${{ matrix.os }}` with Linux values
- the binary helper layer has now been hardened further under the strict four-agent rule loop:
  - crate-targeted build matching now respects `-p`, `--package=`, `--bin`, `--bin=`, and `--manifest-path`
  - `src/bin/*.rs` and `src/bin/*/main.rs` autodiscovery now count as binary scope unless `autobins = false`
  - explicit `[[bin]]` still stays in scope when `autobins = false`
  - generic `cargo build --release` no longer credits every binary crate in multi-binary repos
  - GitHub release action matching now uses exact action identity instead of substring hits, and accepts both `action-gh-release` and `release-action`
  - Linux detection now ignores echo/prose/`--target-dir ...linux...` noise, supports matrix `include`, and ties Linux-attribution to the current crate’s build step rather than any other step in the same job
- `RS-PUB-13` now correctly recognizes nested TOML shape from `[package.metadata.docs.rs]`, not just a flat `metadata["docs.rs"]` lookup
- manifest-backed positive coverage was added for:
  - inherited `license-file.workspace = true` under `RS-PUB-02`
  - non-empty `[package.metadata.docs.rs]` under `RS-PUB-13`
  - explicit `include = [...]` under `RS-PUB-14`
  - real `[package.metadata.binstall]` under `RS-BIN-03`
  - workspace-inherited unreadable README fail-closed under `RS-RELEASE-12`

## Remaining gaps

- release-family rule migration to rule-specific `*_tests/` directories is complete
- workflow rules now evaluate preserved parsed workflow structure, but the semantic matching still relies on release-family helpers rather than a fuller Actions execution model
- the strict four-agent adversarial loop has now been run through every `RS-RELEASE-*`, `RS-PUB-*`, and `RS-BIN-*` rule; remaining suggestions have converged to lower-value combinatorial variants rather than new concrete checker bugs

## Adversarial findings queued

- the remaining workflow limitation is semantic depth, not data loss:
  - parsed jobs/steps are preserved now
  - helper matching is still specific to current release-family semantics rather than a fuller Actions execution model

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

## Verification boundary

- `cargo check -p guardrail3 --lib` passes after the current release-family changes
- `cargo check -p guardrail3 --tests` now passes again
- `cargo test -p guardrail3 --lib --no-run` passes
- targeted release-family tests are green for:
  - `RS-RELEASE-01`
  - `RS-RELEASE-03`
  - `RS-RELEASE-04`
  - `RS-RELEASE-05`
  - `RS-RELEASE-06`
  - `RS-RELEASE-07`
  - `RS-PUB-02`
  - `RS-PUB-13`
  - `RS-PUB-14`
  - `RS-RELEASE-12`
  - `RS-BIN-01`
  - `RS-BIN-02`
  - `RS-BIN-03`

## Success condition

Release tests prove:
- the rule hits only real release-wiring failures
- prose/comments do not satisfy the workflow checks
- inherited path-edge cases are caught exactly
