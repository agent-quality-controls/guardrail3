# RS-TEST — Rust test quality checker (18 rules)

**Input:** Cargo.toml + .cargo/mutants.toml + *.rs files + .claude/ configs + .config/nextest.toml
**Parser:** TOML + syn AST + string matching (hooks)
**Current code:** `test_checks.rs`, `test_quality_checks.rs`

## Rules

| New ID | Old ID | Severity | What | Status |
|--------|--------|----------|------|--------|
| RS-TEST-01 | R-TEST-01 | Warn | cargo-mutants installed on PATH | Implemented |
| RS-TEST-02 | R-TEST-02 | Warn | .cargo/mutants.toml config exists | Implemented |
| RS-TEST-03 | R-TEST-03 | Warn | [profile.mutants] in Cargo.toml (optimized mutation build) | Implemented |
| RS-TEST-04 | R-TEST-04 | Error/Info | At least one #[test] or #[tokio::test] exists. AST-based. | Implemented |
| RS-TEST-05 | R-TEST-05 | Info | Test coverage inventory: public fn count vs test fn count, ratio | Implemented |
| RS-TEST-06 | R-TEST-06 | Info | Integration tests/ dir exists with .rs files | Implemented |
| RS-TEST-07 | R-TEST-07 | Warn | #[ignore] without `// reason:` comment. AST-based. | Implemented (needs reason: prefix fix — see code_fixes.md) |
| RS-TEST-08 | R-TEST-08 | Warn | Mutation testing hook in .claude/ or pre-commit | Implemented |
| RS-TEST-09 | R-TEST-09 | Error | Inline test code in src/ files (#[cfg(test)] mod tests with body) | Implemented (zero tests — see code_fixes.md) |

## New rules from audit round 1

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TEST-10 | Warn | Test function naming: warn on test fns with names <10 chars or purely numeric suffixes (`test_1`, `test_2`). Lazy names make test suites unnavigable. AST-based. | Planned |
| RS-TEST-11 | Warn | `#[cfg(test)]` module naming: warn on modules not named `tests`. Modules named `test`, `testing`, `test_utils` etc. break convention. AST-based. | Planned |
| RS-TEST-12 | Warn | Test timeout configuration: if tokio is a dependency, check that `.config/nextest.toml` exists with `slow-timeout` and `leak-timeout` set. Tests without timeouts hang CI forever. | Planned |

## New rules from audit round 2

| New ID | Severity | What | Status |
|--------|----------|------|--------|
| RS-TEST-13 | Warn | `#[should_panic]` without `expected` string. Matches ANY panic — fragile test. Same philosophy as RS-TEST-07. No tool covers this. AST-based. | Planned |
| RS-TEST-14 | Warn | Tautological `assert_eq!(lit, lit)` / `assert_ne!(lit, lit)`. Both arguments are `syn::Expr::Lit` — assertion proves nothing. Clippy covers `assert!(true)` but not literal-vs-literal in assert_eq/ne. | Planned |
| RS-TEST-15 | Warn | Test function with zero assertion macros (assert!, assert_eq!, assert_ne!, assert_matches!, debug_assert*). Tests that never assert are dead weight. Root cause of 2:1 happy-path ratio (audit 14). Exception: functions returning Result (? is the assertion) or calling fns containing "assert"/"verify"/"expect" in name. | Planned |
| RS-TEST-16 | Warn | Test file >500 effective lines. Same threshold as production code (RS-SOURCE-09). Currently test files are completely exempt from R38 (`if is_test { return; }`). Tests aren't special — split into modules. | Planned |
| RS-TEST-17 | Warn | `assert!(matches!(...))` with `_` wildcards in data positions. Proves the variant but not the payload. A mutation changing the payload survives. Always possible to match something specific instead. | Planned |
| RS-TEST-18 | Warn | Mutation config content validation. `.cargo/mutants.toml` with `exclude_re = [".*"]` makes mutation testing useless (everything excluded). Also flag `timeout_multiplier < 1.0` (fake 100% score via timeouts). RS-TEST-02 checks existence but not content. | Planned |

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
