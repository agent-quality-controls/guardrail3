# Audit 14: Tests, Self-Validation & Test Quality

## Self-Validation Status

**guardrail3 PASSES its own validation.** Running `guardrail3 rs validate . --format json` produces 0 errors, 0 warnings. Exit code 0. This is the most basic eat-your-own-dogfood test and it passes.

**All 292 tests pass.** Zero failures, zero ignored.

---

## Test Infrastructure Overview

| Category | Test count | Files |
|---|---|---|
| Unit tests | 303 | 26 files in `tests/unit/` |
| Adversarial integration tests | 267 | 18 `adversarial_*.rs` files |
| CLI integration tests | 34 | `cli_tests.rs` |
| Property-based tests | 11 | `property_tests.rs` |
| **Total** | **615** | |

Additional infrastructure:
- 95 fixture files across `tests/fixtures/adversarial/`, `tests/fixtures/adversarial-configs/`, `tests/fixtures/grep-attacks/`
- 5 golden snapshots (self-validate, 4 external projects)
- Golden test scripts (`run-golden.sh`, `compare.sh`, `normalize.sh`)
- `cargo-mutants` config exists at both workspace and crate level
- `proptest-regressions` file present (regressions have been captured)

---

## FINDING 1: 48 Check IDs Have ZERO Targeted Tests

The following check IDs exist in source code but have NO test that specifically asserts on them. Some may fire incidentally in integration tests, but no test says "this ID should appear with this severity."

### Rust checks untested (10 IDs)

| ID | What it checks | Source file |
|---|---|---|
| **R6** | Clippy type ban coverage (subset 1) | `clippy_coverage.rs` |
| **R7** | Clippy type ban coverage (subset 2) | `clippy_coverage.rs` |
| **R13** | deny.toml wildcard ban section | `deny_bans.rs` |
| **R15** | deny.toml license confidence threshold | `deny_licenses.rs` |
| **R18** | deny.toml feature-ban section | `deny_bans.rs` |
| **R20** | deny.toml advisory-ignore inventory | `deny_inventory.rs` |
| **R22** | rustfmt.toml settings completeness | `rustfmt_check.rs` |
| **R23** | rustfmt.toml edition match | `rustfmt_check.rs` |
| **R-ARCH-04** | Hex arch composition-root layer violations | `hex_arch_checks.rs` |
| **R-TEST-09** | Inline `#[cfg(test)] mod tests` in src/ | `test_checks.rs` |

### TypeScript checks untested (28 IDs)

| ID | What it checks | Source file |
|---|---|---|
| **T10** | tsconfig noEmit | `tsconfig_check.rs` |
| **T12-T14** | npmrc settings (3 IDs) | `npmrc_check.rs` |
| **T15-T17** | package.json override/resolution (3 IDs) | `package_check.rs` |
| **T59** | Banned packages in node_modules | `package_deps.rs` |
| **T-ESLP-02 through T-ESLP-05** | ESLint preset configs (4 IDs) | `eslint_plugin_checks.rs` |
| **T-ESLP-08** | jsx-a11y strict mode | `eslint_plugin_checks.rs` |
| **T-ESLP-09** | React rules in ESLint | `eslint_plugin_checks.rs` |
| **T-ESLP-11** | Test file relaxations | `eslint_plugin_checks.rs` |
| **T-ESLP-13, T-ESLP-14** | Additional ESLint presets (2 IDs) | `eslint_plugin_checks.rs` |
| **T-JSCPD-01 through T-JSCPD-04** | jscpd config (4 IDs) | `jscpd_check.rs` |
| **T-PKG-01 through T-PKG-04** | Package.json fields (4 IDs) | `package_check.rs` |
| **T-TOOL-02, T-TOOL-03** | Tool packages (2 IDs) | `package_check.rs` |
| **T-TOOL-09, T-TOOL-10** | Script checks (2 IDs) | `tool_config_checks.rs` |

### Hook & Deploy checks untested (10 IDs)

| ID | What it checks | Source file |
|---|---|---|
| **H9** | Hook script inventory | `hook_checks.rs` |
| **H-CSS-01** | Stylelint in pre-commit | `hook_script_checks.rs` |
| **H-TOOL-01 through H-TOOL-05** | Tool hook steps (5 IDs) | `hook_script_checks.rs` |
| **ALL D1-D5** (but see note) | Deploy checks | `deploy_checks.rs` |

**Note on D1-D5:** These ARE referenced in golden snapshots but no test specifically creates a deployment scenario and asserts the check fires. The golden test only verifies the output hasn't changed, not that the check logic is correct.

**Note on H1-H12:** H1-H8, H10-H12 appear in golden snapshots. But golden snapshots are regression tests (output stability), NOT logic tests. No test creates a broken hook and asserts H3 fires as error.

---

## FINDING 2: 24 Source Modules Have ZERO Unit Tests

These modules have no corresponding unit test file AND are not directly referenced in any adversarial test:

### Rust validate modules (6 with zero coverage anywhere)

| Module | Lines | Check IDs |
|---|---|---|
| `clippy_coverage.rs` | ~140 | R4-R7 |
| `deny_audit.rs` | ~270 | R8-R11 |
| `dependency_scan.rs` | ~20 | R45-R48 |
| `rustfmt_check.rs` | ~210 | R21-R23 |
| `toolchain_check.rs` | ~125 | R24-R25 |
| `workspace_metadata.rs` | ~95 | R55-R57 |

### TypeScript validate modules (13 with zero unit tests, zero adversarial references)

| Module | Lines | Check IDs |
|---|---|---|
| `eslint_check.rs` | unknown | T1-T8, T36-T51 |
| `eslint_plugin_checks.rs` | unknown | T-PLUG-*, T-ESLP-* |
| `eslint_audit.rs` | unknown | various |
| `eslint_rule_infra.rs` | unknown | infrastructure |
| `stylelint_check.rs` | unknown | T-STYL-* |
| `jscpd_check.rs` | unknown | T-JSCPD-*, T19-T22 |
| `npmrc_check.rs` | unknown | T11-T14 |
| `package_check.rs` | unknown | T15-T18, T55-T58 |
| `package_deps.rs` | unknown | T59 |
| `tsconfig_check.rs` | unknown | T9-T10, T52-T54 |
| `i18n_check.rs` | unknown | T-TOOL-12 |
| `tool_config_checks.rs` | unknown | T-TOOL-07..11 |
| `config_files.rs` (TS) | unknown | various |

### Hooks module

| Module | Lines |
|---|---|
| `hook_script_checks.rs` | unknown | H-TOOL-*, H-CSS-* |

**Critical:** The TS validate modules are tested ONLY through their corresponding adversarial integration tests (`adversarial_ts_eslint.rs`, `adversarial_ts_plugins.rs`, etc.) which exercise the BINARY not the FUNCTIONS. This means:
- Individual check functions cannot be tested in isolation
- A bug in one check can be masked by another check producing the same ID
- Edge cases within a check function are invisible to integration tests

---

## FINDING 3: Coverage Map Engine & Crawler Have ZERO Tests

**2,431 lines of code with zero test coverage:**

| Module | Lines | What it does |
|---|---|---|
| `commands/coverage/engine.rs` | 340 | Core coverage analysis engine |
| `commands/coverage/clippy.rs` | 149 | Clippy coverage mapping |
| `commands/coverage/deny.rs` | 97 | Deny coverage mapping |
| `commands/coverage/eslint.rs` | 50 | ESLint coverage mapping |
| `commands/coverage/rustfmt.rs` | 111 | Rustfmt coverage mapping |
| `commands/coverage/rust_toolchain.rs` | 115 | Toolchain coverage mapping |
| `commands/coverage/tsconfig.rs` | 61 | Tsconfig coverage mapping |
| `commands/coverage/npmrc.rs` | 58 | Npmrc coverage mapping |
| `commands/coverage/jscpd.rs` | 63 | Jscpd coverage mapping |
| `commands/coverage/cspell.rs` | 65 | Cspell coverage mapping |
| `commands/coverage/prettier.rs` | 51 | Prettier coverage mapping |
| `commands/coverage/stylelint.rs` | 51 | Stylelint coverage mapping |
| `app/crawl.rs` | 275 | Project crawler |
| `app/project_map.rs` | 578 | Project map builder |
| `commands/map.rs` | 350 | Map command |

No CLI test covers the `map` command. No unit test covers any coverage function. No integration test creates a project and runs `guardrail3 map` on it.

---

## FINDING 4: Golden Tests Are Not Automated in CI

Golden tests exist as shell scripts (`run-golden.sh`, `compare.sh`) but:
1. They are NOT run by `cargo test` -- they require manual invocation
2. No test asserts `compare.sh` succeeds
3. Golden snapshots reference 4 external projects (pipelin3r, schedulr, steady-parent, websmasher) -- these may not be available in CI
4. There is no mechanism to detect when golden snapshots are stale (i.e., a code change invalidated them but nobody re-ran the scripts)
5. The self-validate golden snapshot is the ONLY reliable one (runs against own repo)

**Consequence:** Golden snapshots serve as documentation, not as automated regression tests.

---

## FINDING 5: Property Tests Are Shallow

11 property tests exist, covering:
1. TOML parse never panics (50 cases)
2. Config round-trip (30 cases)
3. Init never panics (9 cases)
4. Validate never panics on random source (30 cases)
5. Check results have non-empty ID (20 cases)
6. Severity always valid (20 cases)
7. Results are deterministic (15 cases)
8. Allow without reason detected (15 cases)
9. Crate-wide allow detected (10 cases)
10. Empty project never panics
11. Deeply nested paths no overflow

**Gaps:**
- No property tests for TypeScript validation at all
- No property test for hooks/deploy checks
- Property tests for TOML parsing use very constrained input (`[a-z_]{1,10} = [a-z0-9...]{0,20}`) -- no nested tables, no arrays, no unicode
- No property test that generates adversarial file structures (symlinks, circular paths, permission-denied files)
- No property test for the `generate` command (could generate with random overrides)
- No property test verifying that error/info counts are monotonic (adding a violation never decreases error count)

---

## FINDING 6: Mutation Testing Not Actually Run

`cargo-mutants` config exists at `.cargo/mutants.toml` and `apps/guardrail3/.cargo/mutants.toml`, but:
1. No CI step runs mutation testing
2. No record of mutation testing results in worklogs
3. The config excludes `tests/**`, `fuzz/**`, `src/main.rs`, `src/report/**` -- reasonable, but this was never validated
4. **Key question unanswered:** How many of the 292 passing tests would catch a mutation? Given that 101 assertions use `is_empty()` (testing for absence of findings) vs. only 52 using `!is_empty()`, many tests may not kill mutants because they only verify nothing happens.

**High mutation survival risk areas:**
- Unit tests that only check `results.is_empty()` -- a mutation that removes a check entirely would make the test STILL pass (no results = still empty)
- Integration tests that only check exit code 0 or 1 -- any behavior that doesn't crash passes
- Tests that check `any(|r| r.id == "X")` but don't check severity -- a mutation changing severity from Error to Info would survive

---

## FINDING 7: No Negative Self-Validation Test

guardrail3 passes its own validation, but there's no test that:
1. Intentionally breaks a guardrail in the guardrail3 repo
2. Verifies that `guardrail3 rs validate .` catches it
3. Restores the fix

This would prove that self-validation is meaningful, not just "the codebase happens to be clean."

---

## FINDING 8: Missing Edge Case Fixtures

Fixture files exist for:
- Adversarial allow/grep attacks (good)
- Edge cases: empty file, CRLF, BOM, syntax errors, very long lines (good)
- Config attack: missing/incomplete configs (good)

**Missing:**
- Huge files (>10,000 lines) -- does the 500-line check handle files with no newlines?
- Binary files in src/ -- does the AST parser crash on non-UTF8?
- Symlink loops -- does the crawler infinite-loop?
- Files with null bytes -- does parsing crash?
- Cargo.toml with duplicate keys -- does TOML parsing handle it?
- `.gitignore`d files -- are they correctly skipped?
- Read-only files -- does generate fail gracefully?
- Workspace with path dependencies pointing outside the repo

---

## FINDING 9: Test Quality Issues

### Tests that only verify happy paths

Many unit tests create a "good" fixture and assert no errors. They don't create a "bad" fixture and assert the specific error fires:

- `test_release_checks.rs` (39 lines) -- only tests that R-PUB-12 emits as Info, never tests that it emits as Error when metadata is missing
- `ast_visitors_test.rs` (24 lines) -- single test, only tests the "not flagged" case for `#[ignore]`
- `help_gen_test.rs` (60 lines) -- only tests that output is non-empty

### Integration tests that mask failures

Many adversarial integration tests run the binary and check exit codes or JSON structure, but don't verify that SPECIFIC checks fire for SPECIFIC scenarios. A bug could cause a check to silently stop working and the test would still pass because other checks produce output.

### `is_empty()` dominance

101 assertions check `results.is_empty()` vs. 52 checking `!is_empty()`. This means nearly 2:1 ratio of "nothing happened" tests vs "something was detected" tests. For a tool whose purpose is to DETECT violations, the ratio should be inverted.

---

## FINDING 10: Missing Check IDs in Source (Gaps in Numbering)

Check IDs that appear in the help text / CLAUDE.md but NOT in source code:

### Rust
- **R39** -- Referenced in golden snapshot as info but missing from source ID list (may be emitted by structure_checks.rs without an explicit string)
- **R50** -- Banned crates in Cargo.lock -- listed in CLAUDE.md but not found in source grep
- **R51, R52** -- Dependency direction -- listed in CLAUDE.md but not in source code
- **R54** -- Gap in numbering (R53 exists, R55 exists)

### TypeScript
- **T33** -- Gap: T32 and T34 exist but T33 is not in source code

### Hooks
- **H9** through **H12** exist in source but H9 has zero test coverage

---

## Summary of Severity

| Finding | Severity | Impact |
|---|---|---|
| 48 check IDs untested | **CRITICAL** | Checks may be broken with no detection |
| 24 modules with zero unit tests | **CRITICAL** | No isolation testing for check logic |
| Coverage engine/crawler untested (2431 LOC) | **HIGH** | Entire subsystem is a black box |
| Golden tests not in CI | **HIGH** | Regression snapshots provide no automated protection |
| Property tests miss TS/hooks entirely | **MEDIUM** | Random input testing only covers Rust path |
| Mutation testing never run | **HIGH** | Unknown how many tests actually catch bugs |
| No negative self-validation test | **MEDIUM** | Self-compliance is coincidental not proven |
| Missing edge case fixtures | **MEDIUM** | Binary files, symlinks, null bytes untested |
| 2:1 happy-path to error-path ratio | **HIGH** | Tests prove absence of crashes, not presence of detection |
| Check ID numbering gaps | **LOW** | Cosmetic but confusing for auditors |
