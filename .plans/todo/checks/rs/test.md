# RS-TEST — Rust test quality checker (19 rules)

**Input:** Cargo.toml + .cargo/mutants.toml + *.rs files + cached pre-commit hook files + .config/nextest.toml
**Parser:** TOML + syn AST + executable-line matching (hooks)
**Current code:** `crates/app/rs/checks/rs/test/**` (old `test_checks.rs` / `test_quality_checks.rs` are legacy seed material only)

## Implementation mapping contract

- exactly one `RS-TEST-*` rule ID per production file
- exactly one rule-specific `*_tests/` module directory per production rule file
- `mod.rs` orchestrates only
- `discover.rs`, `facts.rs`, `inputs.rs`, `parse.rs`, and `test_support.rs` may contain shared discovery, typed inputs, parsing, and fixture helpers only

Forbidden:

- grouped family test files such as `test_tests.rs`
- helper files that hide multiple rule predicates behind one API

## Discovery / ownership model

`RS-TEST` is a multi-root family.

Its owned Rust roots are:
- workspace roots
- standalone package roots that are not members of a workspace

Per owned root, the family determines:
- root Cargo test-related config facts
- root-local `.cargo/mutants.toml`
- root-local `.config/nextest.toml`
- Rust source/test files belonging to that root

The family must not collapse to repo-root-only behavior for test roots.

## Shared validation-root inputs

`RS-TEST` also depends on validation-root artifacts that are not root-local:
- validation-root `guardrail3.toml`
- active shared/Rust hook surfaces for mutation-hook presence

That split is intentional:
- root-owned rules evaluate package/workspace-local test facts
- hook-presence and root policy resolution depend on validation-root artifacts

## Hook resolution contract

`RS-TEST-08` does not own generic hook structure.

It only owns whether the active shared/Rust hook surfaces contain the expected mutation-testing step.
Missing hook files themselves are owned by the hook families.

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-TEST-01 | R-TEST-01 | Warn | cargo-mutants installed on PATH | Implemented |
| RS-TEST-02 | R-TEST-02 | Warn | .cargo/mutants.toml config exists | Implemented |
| RS-TEST-03 | R-TEST-03 | Warn | [profile.mutants] in Cargo.toml (optimized mutation build) | Implemented |
| RS-TEST-04 | R-TEST-04 | Error/Info | At least one `#[test]` or `#[tokio::test]` exists. AST-based. | Implemented |
| RS-TEST-05 | R-TEST-05 | Info | Test coverage inventory: public fn count vs test fn count, ratio | Implemented |
| RS-TEST-06 | R-TEST-06 | Info | Integration tests/ dir exists with .rs files | Implemented |
| RS-TEST-07 | R-TEST-07 | Warn | `#[ignore]` without documented reason. Accept inline `#[ignore = "..."]`, same-line `// reason: ...`, or previous-line `// reason: ...`. AST-based. | Implemented |
| RS-TEST-08 | R-TEST-08 | Warn | Mutation testing hook in the active shared/Rust hook surfaces | Implemented |
| RS-TEST-09 | R-TEST-09 | Error | Inline test code in `src/` files (`#[cfg(test)] mod tests { ... }` with body) | Implemented |

## New rules from audit round 1

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TEST-10 | Warn | Test function naming: warn on test fns with names <10 chars or purely numeric suffixes (`test_1`, `test_2`). Lazy names make test suites unnavigable. AST-based. | Implemented |
| RS-TEST-11 | Warn | `#[cfg(test)]` module naming: warn on modules not named `tests`. Modules named `test`, `testing`, `test_utils` etc. break convention. AST-based. | Implemented |
| RS-TEST-12 | Warn | Test timeout configuration: if tokio is a dependency, check that `.config/nextest.toml` exists with `slow-timeout` and `leak-timeout` set. Tests without timeouts hang CI forever. | Implemented |

## New rules from audit round 2

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TEST-13 | Warn | `#[should_panic]` without `expected` string. Matches ANY panic — fragile test. Same philosophy as RS-TEST-07. No tool covers this. AST-based. | Implemented |
| RS-TEST-14 | Warn | Tautological `assert_eq!(lit, lit)` / `assert_ne!(lit, lit)`. Both arguments are `syn::Expr::Lit` — assertion proves nothing. Clippy covers `assert!(true)` but not literal-vs-literal in assert_eq/ne. | Implemented |
| RS-TEST-15 | Warn | Test function with zero assertion macros (assert!, assert_eq!, assert_ne!, assert_matches!, debug_assert*). Tests that never assert are dead weight. Root cause of 2:1 happy-path ratio (audit 14). Exception: functions returning Result (? is the assertion) or calling fns containing "assert"/"verify"/"expect" in name. | Implemented |
| RS-TEST-16 | Warn | Test file >500 effective lines. Same threshold as production code (RS-CODE-09). Tests are not exempt from structural pressure. | Implemented |
| RS-TEST-17 | Warn | `assert!(matches!(...))` with `_` wildcards in data positions. Proves the variant but not the payload. A mutation changing the payload survives. Always possible to match something specific instead. | Implemented |
| RS-TEST-18 | Warn | Mutation config content validation. `.cargo/mutants.toml` with `exclude_re = [".*"]` makes mutation testing useless (everything excluded). Also flag `timeout_multiplier < 1.0` (fake 100% score via timeouts). RS-TEST-02 checks existence but not content. | Implemented |
| RS-TEST-19 | Error | Input failures for the test family: unreadable/unparsable Rust source, Cargo.toml, `.cargo/mutants.toml`, `.config/nextest.toml`, or `guardrail3.toml` required to evaluate test rules. Fail closed instead of silently skipping. | Implemented |

## Input integrity / fail-closed expectations

The family depends on:
- readable/parsible owned-root `Cargo.toml`
- readable/parsible owned-root `.cargo/mutants.toml` when present
- readable/parsible owned-root `.config/nextest.toml` when present
- readable Rust source files for analyzed tests/source modules
- readable validation-root `guardrail3.toml` when policy resolution needs it

Malformed required inputs must surface through `RS-TEST-19`.

## Explicitly rejected

| Finding | Why rejected |
|---------|-------------|
| `#[allow(unused/dead_code)]` in test modules | R32-R33 already require reason on all #[allow]. Stale reasons are human judgment. |
| Mixing #[test] and #[tokio::test] | Too many legitimate cases (sync domain + async adapter in same crate). |
| Test helpers that panic instead of returning Result | Community consensus: unwrap in tests is fine. |
| Mutation config content validation beyond basics | RS-TEST-18 covers the dangerous cases (exclude-all, fake timeouts). Specific settings like debug=none are guidance, not enforcement. |
| Snapshot test config (insta) | Too tool-specific. |
| Property test config (proptest) | Too tool-specific. |
| Benchmark setup (criterion/divan) | Too niche. |
| `#[cfg(test)]` on individual items (not modules) | Rare, not harmful. RS-TEST-09 catches the important case. |

## Notes for new implementation

- Build `rs/test` as a new-architecture family; do not migrate the old `WalkDir` orchestration directly.
- Old unit tests are useful seeds for:
  - `RS-TEST-01..09`
  - especially `R-TEST-07` ignore-reason parsing
  - `R-TEST-09` cfg(test) inline-module detection
- Old validator gaps to avoid copying:
  - `has_mutants_profile()` is line-based instead of TOML-based
  - `RS-TEST-04` treats parse failures as “no tests” instead of explicit input failures
  - older implementations of `RS-TEST-08` only checked legacy hook paths instead of the active shared/Rust hook surfaces
  - `RS-TEST-09` has no new-family typed-input coverage yet
- New family should include an explicit input-failure rule (`RS-TEST-19`) if `.cargo/mutants.toml`, `Cargo.toml`, `.config/nextest.toml`, `guardrail3.toml`, or relevant Rust source files are unreadable/unparsable for rule execution.

## Legacy carry-forward from archived parsing migration

The old top-level AST migration note is being archived, but these Rust-only hardening items remain live for `rs/test`:

- `RS-TEST-13` still depends partly on token-string inspection for `#[should_panic(expected = ...)]` shape detection
- `RS-TEST-15` still uses heuristic name matching (`assert` / `verify` / `expect`) as one escape hatch for “test without assertions”
- any remaining parser logic that infers semantics from token text rather than AST node shape should be treated as future hardening work, even though the family is breadth-first complete now
