# Consolidated Audit Findings

**Date:** 2026-03-19
**Source:** 14 adversarial audit reports
**Scope:** All guardrail3 validation, generation, CLI, reporting, testing, and architecture
**Last updated:** 2026-03-19 (post-fix status annotations added)

---

## Totals

| Severity | Count | Fixed | Rejected | Open |
|----------|-------|-------|----------|------|
| CRITICAL | 22 | 12 | 4 | 6 |
| HIGH | 59 | 4 | 0 | 55 |
| MEDIUM | 74 | 2 | 0 | 72 |
| LOW | 48 | 3 | 0 | 45 |
| **TOTAL** | **203** | **21** | **4** | **178** |

### By Report

| # | Report | CRIT | HIGH | MED | LOW | Total |
|---|--------|------|------|-----|-----|-------|
| 01 | Rust Config Checks | 3 | 7 | 7 | 7 | 24 |
| 02 | deny.toml Checks | 0 | 3 | 9 | 4 | 16 |
| 03 | Rust Source Scan | 2 | 4 | 9 | 5 | 20 |
| 04 | Rust Deps & Arch | 0 | 2 | 9 | 3 | 14 |
| 05 | ESLint Validation | 3 | 14 | 8 | 1 | 26 |
| 06 | TS Source Scan | 0 | 2 | 5 | 5 | 12 |
| 07 | tsconfig/npmrc/jscpd | 1 | 6 | 5 | 4 | 16 |
| 08 | package.json | 0 | 0 | 0 | 17 | 17 |
| 09 | Hooks & Deploy | 2 | 4 | 10 | 6 | 22 |
| 10 | Coverage Engine | 2 | 5 | 5 | 5 | 17 |
| 11 | Generate & Modules | 0 | 0 | 6 | 13 | 19 |
| 12 | CLI/Reporting/FS | 5 | 0 | 0 | 28 | 33 |
| 13 | TS Architecture | 2 | 4 | 7 | 3 | 16 |
| 14 | Tests & Self-Validation | 2 | 4 | 3 | 1 | 10 |

**Note on Report 08 and 12 severity mapping:** Report 08 uses "GAP" severity (mapped to LOW). Report 12 uses "ERROR" (mapped to CRITICAL) and "WARN" (mapped to HIGH for actionable items, MEDIUM for moderate-impact items, LOW for cosmetic/informational items) per the descriptions in each finding.

---

## CRITICAL Findings (22)

| # | Report | ID | Description | Affected File(s) | Fix | Status |
|---|--------|----|-------------|-------------------|-----|--------|
| 1 | 01 | CRIT-01 | `missing_docs` and `missing_debug_implementations` lints not in EXPECTED_RUST_LINTS -- R26 silently passes on incomplete lint config | `cargo_lints.rs` vs `canonical.rs` | Add both to `EXPECTED_RUST_LINTS` | **FIXED** -- `missing_docs` removed from guardrails entirely (user decision: agent-managed code, docs not needed). `missing_debug_implementations` added to EXPECTED_RUST_LINTS. Canonical module updated to match. |
| 2 | 01 | CRIT-02 | `_profile` param in `clippy_coverage::check` ignored -- library global-state type bans (LazyLock, OnceLock, once_cell) never validated | `clippy_coverage.rs` | When profile=="library", add TYPE_GLOBAL_STATE to expected bans | **REJECTED** -- `_profile` is intentionally unused. Code comment at line 111-114 explains: all profiles use the same expected bans. The parameter exists as a forward-looking hook for when profiles diverge. Both profiles use the strictest ban set. |
| 3 | 01 | CRIT-03 | R1/R21/R24 use `path.exists()` directly, bypassing FileSystem trait -- untestable and inconsistent | `config_files.rs`, `clippy_coverage.rs`, `cargo_lints.rs` | Replace with `fs.read_file()` or `fs.metadata()` checks | **OPEN** -- 21 call sites across 12 files. FileSystem trait lacks `exists()` method. Large refactor, low practical impact (real FS is the only production implementation). |
| 4 | 03 | CRIT-R42 | `check_unsafe()` (R42) is DEAD CODE -- never called from source_scan or orchestrator. Unsafe blocks/fns completely undetected | `structure_checks.rs`, `source_scan.rs` | Wire `check_unsafe` into `source_scan::check()` per-file loop | **FIXED** -- Function removed entirely. Redundant with clippy `unsafe_code = "forbid"` which cannot be overridden with `#[allow]`. |
| 5 | 03 | CRIT-R53 | `check_unsafe_code_forbid()` (R53) is DEAD CODE -- never called. Workspace lint level for unsafe_code not checked | `structure_checks.rs`, `source_scan.rs` | Wire `check_unsafe_code_forbid` into `source_scan::check()` outside per-file loop | **FIXED** -- Wired into source_scan orchestrator. Now validates `unsafe_code = "forbid"` in workspace lints. |
| 6 | 05 | F01 | ESLint flat config override blindness -- later config blocks can disable required rules. ALL ESLint checks vulnerable because they use `content.contains()` on whole file | `eslint_check.rs`, `eslint_plugin_checks.rs`, `eslint_rule_infra.rs` | Parse last effective config block per rule, or strip overrides | **OPEN** -- Requires tree-sitter-javascript to properly parse ESLint flat config structure. T7 inventory check provides partial safety net by flagging `"off"` lines. |
| 7 | 05 | F04 | `max-lines` expected value mismatch -- checker expects 400, actual config is 300. Wrong threshold in validation | `eslint_check.rs:62` | Change expected value to 300 (or to match canonical) | **REJECTED** -- 400 is the intentional baseline (ceiling, not target). `check_rule_value` uses stricter-or-equal comparison: project with `max: 300` passes (300 <= 400). The canonical template also specifies 400. |
| 8 | 05 | F09 | T36 zone definition check: generic word matching ("domain", "adapters") + comment bypass. Any ESLint config mentioning these words in comments passes | `eslint_audit.rs:29-31` | Parse actual boundaries config structure, not substring match | **OPEN** -- Requires tree-sitter-javascript for proper parsing. False positives only produce Info inventory items, not suppress errors. |
| 9 | 07 | NPM-01 | `.npmrc` parser uses `find()` (first match) but pnpm uses last match for duplicate keys. Attacker can shadow settings with duplicates | `npmrc_check.rs:93` | Use `rfind()` and flag duplicate keys as error | **FIXED** -- Changed to `.rev().find()` for last-wins semantics. Added T-NPMRC-01 duplicate key detection check. |
| 10 | 09 | FINDING-H-01 | No `--no-verify` bypass detection -- `git commit --no-verify` bypasses every hook check with zero detection | `hook_checks.rs`, `validate.rs` | Add check for commits without hook artifacts, suggest CI re-validation | **REJECTED** -- `--no-verify` is a runtime git CLI flag. guardrail3 is a static analysis tool and cannot intercept or detect past git runtime flags. The intended mitigation is running `guardrail3 validate` in CI, which re-validates everything regardless of `--no-verify`. |
| 11 | 09 | FINDING-H-02 | No `set -e` / shell safety validation in pre-commit hooks -- future edits without explicit error handling silently pass | `hook_checks.rs`, `pre_commit.rs` | Check for `set -e` or `set -uo pipefail` in hook content | **FIXED** -- Added H-SAFE-01 check for `set -e` or `set -euo pipefail` in hook scripts. |
| 12 | 10 | COV-CRIT-01 | `String::starts_with` in shadow detection (engine.rs:195) -- falsely reports `apps/web-admin` as shadow of `apps/web` | `engine.rs:195` | Use `Path::starts_with` or append `/` before comparing | **FIXED** -- Changed to `Path::starts_with` for component-level matching. |
| 13 | 10 | COV-CRIT-02 | Non-walk-up resolution finds FIRST match not NEAREST ancestor. Alphabetically-first config wins over nearer config | `engine.rs:106-110` | Sort candidates by path depth descending, use max_by_key | **FIXED** -- Replaced `.find()` with `.max_by_key(|d| d.components().count())` to select nearest ancestor. |
| 14 | 12 | CLI-01 | No mutual exclusion on scope flags -- `--staged --dirty --commits --files` can all be passed simultaneously, conflicting flags silently ignored | `cli.rs:149-167` | Add clap group conflict or emit warning when multiple scope flags used | **FIXED** -- Scope flags (`--staged`, `--dirty`, `--commits`, `--files`) now mutually exclusive via clap ArgGroup. |
| 15 | 12 | CLI-03 | `--code` flag runs ZERO Rust checks -- maps to nothing in `RustCheckCategories`, silently disables all categories | `main.rs:393-408`, `commands/validate.rs:243-258` | Map `--code` to appropriate Rust categories or remove flag for Rust | **REJECTED** -- Auditor is wrong. Code checks run unconditionally in `rs/validate/mod.rs` `run_code_checks()` -- they are not gated by category flags. `--code` correctly runs all code checks (config, clippy, deny, lints, source scan, deps) while disabling architecture/garde/tests/release. |
| 16 | 12 | SCOPE-01 | `--staged` misses renamed files -- `--diff-filter=ACM` excludes R (Renamed) | `commands/validate.rs:138-155` | Add R to diff filter: `--diff-filter=ACMR` | **FIXED** -- Diff filter changed to `--diff-filter=ACMR`. |
| 17 | 12 | SCOPE-02 | `--dirty` misses untracked new files -- only runs `git diff`, not `git ls-files --others` | `commands/validate.rs:158-190` | Add `git ls-files --others --exclude-standard` to dirty file collection | **FIXED** -- Added `git ls-files --others --exclude-standard` to dirty file collection. |
| 18 | 12 | DISC-01 | Any `package.json` triggers TypeScript detection regardless of actual TS usage -- false positives on pure JS projects | `discover.rs:353-373` | Check for `tsconfig.json` or `typescript` in dependencies, not just `package.json` existence | **FIXED** -- TS detection now requires `tsconfig.json` OR `typescript` in dependencies. |
| 19 | 13 | GAP-TS-ARCH-01 | TS import boundary checks use string matching, not AST -- dynamic `import()`, template literals, multi-line imports all invisible | `ts_arch_checks.rs:370-414` | Use tree-sitter AST to extract all import sources | **OPEN** -- Requires tree-sitter-javascript integration for import extraction. Violates project's own stated principle (CLAUDE.md principle 7). |
| 20 | 13 | GAP-TS-ARCH-06 | Hex arch structure check only verifies `domain/` and `adapters/` exist -- `ports/` and `application/` layers not checked | `ts_arch_checks.rs:100-155` | Add ports and application to structure check | **FIXED** -- Added `application` layer to structure check. `ports/` remains optional (not all apps need an explicit ports layer). |
| 21 | 14 | FIND-14-01 | 48 check IDs have ZERO targeted tests -- checks may be broken with no detection | 26+ test files | Add targeted tests for all untested check IDs | **OPEN** -- Many TS check IDs (T-PKG-*, T-JSCPD-*, T-NPMRC-*, T-TSCONFIG-*) still lack targeted tests. Some Rust checks have indirect golden test coverage. |
| 22 | 14 | FIND-14-02 | 24 source modules have ZERO unit tests -- no isolation testing for check logic (especially all TS validate modules) | 24 source modules | Add unit tests for each module | **OPEN** -- Most TS validate modules still have zero inline or external unit tests. Only 3 files have `#[cfg(test)]` blocks. Some modules have external test files. |

---

## HIGH Findings (59)

| # | Report | ID | Description | Affected File(s) | Fix | Status |
|---|--------|----|-------------|-------------------|-----|--------|
| 1 | 01 | HIGH-01 | Per-crate clippy.toml TOML parse error silently swallowed -- user sees passing check for broken file | `config_files.rs:156` | Emit Error result on parse failure | OPEN |
| 2 | 01 | HIGH-02 | Per-crate clippy.toml read failure silently swallowed -- file confirmed to exist but unreadable, no diagnostic | `config_files.rs:150` | Emit Error result on read failure | OPEN |
| 3 | 01 | HIGH-03 | R22 rustfmt.toml parse/read errors are Warn, should be Error | `rustfmt_check.rs:17,34` | Change severity to Error | OPEN |
| 4 | 01 | HIGH-04 | R25 toolchain parse/read errors are Warn, should be Error | `toolchain_check.rs:10,29` | Change severity to Error | OPEN |
| 5 | 01 | HIGH-05 | R25 treats "nightly" and pinned version (e.g. "1.75.0") the same -- both get Warn. Should distinguish | `toolchain_check.rs:61-69` | Error for nightly, Info for pinned version | OPEN |
| 6 | 01 | HIGH-06 | Ban entries without `reason` field not flagged -- bans pass R4/R5 with no developer guidance | `clippy_coverage.rs:169-175` | Check for `reason` field presence in ban entries | OPEN |
| 7 | 01 | HIGH-07 | `check_rustfmt_str` emits no result on success -- inconsistent with all other checks that emit Info on success | `rustfmt_check.rs:89` | Emit Info/inventory result on success | OPEN |
| 8 | 02 | FIND-02-01 | `_profile` ignored in deny_bans.rs -- library IO bans (13 crates: axum, tokio, reqwest, sqlx, hyper, etc.) never enforced for library profile | `deny_bans.rs:9` | When profile=="library", add DENY_BANS_LIBRARY_IO to expected set | OPEN |
| 9 | 02 | FIND-02-04 | Advisory ignore entries don't require `reason` field -- silent vulnerability suppression with no documentation | `deny_inventory.rs:49-72` | Emit Warn/Error when advisory ignore has no reason | OPEN |
| 10 | 02 | FIND-02-06 | Skip entries only inventoried at Info, never validated -- unlimited skips effectively disable multiple-versions check | `deny_inventory.rs:5-47` | Require reason field on skip entries, warn on excessive count | OPEN |
| 11 | 03 | HIGH-R32 | R32-R33 comment detection accepts ANY `//` comment as "reason" -- `// lol` passes. Should require `// reason:` prefix | `allow_checks.rs:97` | Check for `// reason:` prefix, not just `//` | OPEN |
| 12 | 03 | HIGH-R30/R37 | `cfg_attr(all(), allow(...))` gets Info (R37) instead of Error (R30) -- unconditional suppression disguised as conditional | `allow_checks.rs` | Detect always-true cfg_attr conditions and escalate severity | OPEN |
| 13 | 03 | HIGH-R58-glob | `use std::fs::*;` (glob import) is NOT detected by R58 -- complete bypass | `code_quality_checks.rs` | Handle `UseTree::Glob` in `use_subtree_is_fs` | OPEN |
| 14 | 03 | HIGH-R30-mod | Module-level `#![allow(...)]` inside inline `mod foo { #![allow(...)] }` undetected by both R30 and R32 | `allow_checks.rs` | Visit inner attributes on inline modules | OPEN |
| 15 | 04 | F-04-02 | R50 removed with no replacement -- no independent banned-crate verification. Transitive trust gap: guardrail3 validates config but not outcome | `dependency_scan.rs:19` | Either re-add R50 lockfile scanning or add check that cargo-deny actually runs | OPEN |
| 16 | 04 | F-04-03 | R-ARCH-02 only reads `[dependencies]`, not `[dev-dependencies]` or `[build-dependencies]` -- architecture violations in build scripts invisible | `hex_arch_checks.rs:169` | Also check dev-dependencies and build-dependencies | OPEN |
| 17 | 05 | F02 | Comments containing ESLint rule names produce false passes -- `// TODO: add no-floating-promises` passes presence check | All ESLint check files | Strip comments before matching | OPEN |
| 18 | 05 | F05 | `check_rule_value` extracts FIRST number near rule name -- wrong for object configs, fragile for multi-number lines | `eslint_rule_infra.rs:256-278` | Parse rule config structure, extract specific option value | OPEN |
| 19 | 05 | F06 | Rule value check only checks first occurrence -- later override blocks invisible | `eslint_rule_infra.rs:256-278` | Check last occurrence or parse all config blocks | OPEN |
| 20 | 05 | F08 | T6 false positive on word "boundaries" in any context (comments, strings) | `eslint_check.rs:97` | Require structural context, not bare string match | OPEN |
| 21 | 05 | F11 | T6/T36-T39 boundaries checks don't verify rules are set to "error" -- `"off"` passes | `eslint_check.rs`, `eslint_audit.rs` | Verify rule severity is "error" | OPEN |
| 22 | 05 | F12 | Comment bypass for ESLint preset checks (strictTypeChecked, stylisticTypeChecked) | `eslint_check.rs:128,159` | Strip comments before matching | OPEN |
| 23 | 05 | F14 | T50 route wrapper check matches MESSAGE text, not rule structure -- `withBody` appears in error message string, not in selector | `eslint_check.rs:342` | Verify no-restricted-syntax AST selector structure | OPEN |
| 24 | 05 | F17 | T51 process.env ban ALWAYS passes because "process.env" appears in the message text of the rule, not just config | `eslint_check.rs:376` | Check for AST selector structure, not message content | OPEN |
| 25 | 05 | F19 | All plugin group checks (T-ESLP-02..10) use `content.contains` -- same comment/override blindness as all ESLint checks | `eslint_plugin_checks.rs:112` | Strip comments, check last effective config | OPEN |
| 26 | 05 | F20 | UNICORN_DISABLED rules checked for presence, not "off" state -- `"unicorn/no-null": "error"` passes T-ESLP-02 | `eslint_plugin_checks.rs` | Verify rules are set to "off" | OPEN |
| 27 | 05 | F22 | T-ESLP-07 jsx-a11y check: "strict" is too generic -- matches `strictTypeChecked` and `strict-boolean-expressions` | `eslint_plugin_checks.rs:414` | Use more specific marker string | OPEN |
| 28 | 05 | F26 | No severity verification for T40-T48, T60-T83 -- rules set to "warn" pass all checks | All ESLint presence checks | Verify rule severity | OPEN |
| 29 | 05 | F28 | `no-restricted-syntax` rules not structurally validated -- only string presence checked | `eslint_check.rs` | Parse and validate AST selectors | OPEN |
| 30 | 05 | F37 | ESLint tests don't cover comment bypass scenarios | `eslint_plugin_checks_tests.rs` | Add adversarial tests with commented-out rules | OPEN |
| 31 | 05 | F38 | ESLint tests don't cover value checking logic | Test files | Add tests for check_rule_value, extract_number_from_line | OPEN |
| 32 | 06 | F-01 | T32/T33 file length threshold mismatch -- three conflicting sources: help text (>300), test comment (>300), actual code (>400) | `source_scan.rs`, `help_gen.rs` | Pick one source of truth and align all three | OPEN |
| 33 | 06 | F-02 | T30 process.env: `process["env"]` bracket notation and destructuring both bypass AST detection | `ts_code_analysis.rs` | Add subscript_expression base case detection | OPEN |
| 34 | 07 | TSC-01 | tsconfig.json JSONC not handled -- single `//` comment silently bypasses ALL 20+ tsconfig checks (serde_json rejects comments) | `tsconfig_check.rs:108-126` | Use JSONC-aware parser or strip comments before parsing | OPEN |
| 35 | 07 | TSC-02 | No validation of child tsconfig files that don't extend base -- per-app `tsconfig.json` with `strict: false` invisible | `tsconfig_check.rs:63-86` | Discover and validate child tsconfig files | OPEN |
| 36 | 07 | TSC-03 | No detection of setting overrides via `extends` chain -- child tsconfig can override any base setting undetected | `tsconfig_check.rs` | Parse extends chain and validate effective settings | OPEN |
| 37 | 07 | NPM-02 | Inline comments not stripped -- `strict-peer-dependencies=true # comment` parsed as value `"true # comment"`, fails match | `npmrc_check.rs:56-66` | Strip inline comments after `#` or `;` | OPEN |
| 38 | 07 | NPM-04 | No duplicate key detection in .npmrc -- enables the NPM-01 exploit | `npmrc_check.rs:53-68` | Detect and flag duplicate keys | **FIXED** -- T-NPMRC-01 duplicate key detection added as part of NPM-01 fix. |
| 39 | 07 | JSCPD-02 | `reporters` field not validated -- `"reporters": []` silently disables all jscpd console output | `jscpd_check.rs:43-217` | Validate reporters includes "consoleFull" | OPEN |
| 40 | 07 | JSCPD-03 | .jscpd.json parse error silently swallowed -- all checks bypassed with zero error output | `jscpd_check.rs:43-46` | Emit Error result before returning on parse failure | OPEN |
| 41 | 07 | CROSS-01 | Check ID collisions: T60 used for both `noPropertyAccessFromIndexSignature` AND content import restriction; T61 used for both `noImplicitOverride` AND Velite config | `tsconfig_check.rs:141-142`, `jscpd_check.rs:241,269` | Assign unique check IDs | OPEN |
| 42 | 09 | FINDING-H-03 | H5 pattern matching fooled by comments -- `# gitleaks protect --staged` (commented out) passes | `hook_checks.rs` | Verify tool strings appear in executable context, not comments | OPEN |
| 43 | 09 | FINDING-H-04 | H-TOOL-02 conflict marker check matches `<<<` from bash heredocs -- both actual hooks produce false pass | `hook_script_checks.rs` | Use more specific pattern for conflict marker detection | OPEN |
| 44 | 09 | FINDING-H-06 | H8 tool check list incomplete -- missing pnpm, cargo, tsc, eslint, jscpd from required tool list | `tool_checks.rs` | Add core toolchain to required tools list | OPEN |
| 45 | 09 | FINDING-H-10 | 6+ actual hook steps have no corresponding guardrail3 check (file size, migration, tamper detection, structural health, shell safety) | `hook_checks.rs` | Add checks for missing hook step patterns | OPEN |
| 46 | 10 | COV-HIGH-01 | Crawler misses `cspell.json` (bare filename) -- pattern only matches `cspell.config.*` and `.cspell*` | `crawl.rs:192-195` | Add exact match for `"cspell.json"` | OPEN |
| 47 | 10 | COV-HIGH-02 | `.cargo/deny.toml` collected but never resolves as covering any source dir -- cargo-deny uses it as if at project root | `crawl.rs:99`, `engine.rs` | Treat `.cargo/deny.toml` parent as project root for resolution | OPEN |
| 48 | 10 | COV-HIGH-03 | Crawler misses `rust-toolchain` (no `.toml` extension, legacy format) | `crawl.rs:101` | Add match for extensionless `rust-toolchain` | OPEN |
| 49 | 10 | COV-HIGH-04 | Prettier `source_dirs` only covers TS, misses CSS-only directories | `prettier.rs:34` | Return union of dirs_with_ts and dirs_with_css | OPEN |
| 50 | 10 | COV-HIGH-05 | cspell `source_dirs` only covers TS, misses Rust-only directories | `cspell.rs:34-35` | Return union of dirs_with_ts and dirs_with_rs | OPEN |
| 51 | 10 | COV-HIGH-06 | jscpd `source_dirs` only covers TS, misses Rust-only directories | `jscpd.rs:34-35` | Return union of dirs_with_ts and dirs_with_rs | OPEN |
| 52 | 13 | GAP-TS-ARCH-09 | ESLint audit zone definition check is extremely loose -- any occurrence of "element-types" or "domain"+"adapters" anywhere passes | `eslint_audit.rs:29-31` | Parse actual boundaries config, not substring match | OPEN |
| 53 | 13 | GAP-TS-ARCH-13 | ESLint rule presence check is substring match -- comments, "off" state, prefix overlap all produce false results | `eslint_rule_infra.rs` | Use structural parsing for rule presence and severity | OPEN |
| 54 | 13 | GAP-TS-ARCH-15 | No check that ESLint rules are set to "error" severity (duplicates F26 from report 05) | `eslint_rule_infra.rs`, `eslint_check.rs` | Add severity verification | OPEN |
| 55 | 13 | GAP-TS-ARCH-16 | Windows path separators break layer detection -- splits on `/` only | `ts_arch_checks.rs:191` | Use `std::path::MAIN_SEPARATOR` or normalize separators | OPEN |
| 56 | 14 | FIND-14-03 | Coverage map engine & crawler have ZERO tests -- 2,431 lines of untested code | `engine.rs`, `crawl.rs`, 11 coverage modules | Add unit tests for engine, crawler, and all coverage modules | OPEN |
| 57 | 14 | FIND-14-04 | Golden tests not automated in CI -- exist as manual shell scripts, not run by `cargo test` | `run-golden.sh`, `compare.sh` | Integrate golden tests into cargo test or CI pipeline | OPEN |
| 58 | 14 | FIND-14-06 | Mutation testing never actually run -- config exists but no CI step, no results, unknown mutant survival rate | `.cargo/mutants.toml` | Add mutation testing to CI | OPEN |
| 59 | 14 | FIND-14-09 | Test quality: 2:1 ratio of `is_empty()` (happy path) to `!is_empty()` (detection) assertions -- tests prove absence of crashes, not presence of detection | All test files | Add error-path tests for all checks, invert ratio | OPEN |

---

## MEDIUM Findings (74)

| # | Report | ID | Description | Affected File(s) | Fix | Status |
|---|--------|----|-------------|-------------------|-----|--------|
| 1 | 01 | MED-01 | R3 threshold values hardcoded, not sourced from canonical module -- manual sync required | `config_files.rs:245-251` | Extract shared const or test consistency | OPEN |
| 2 | 01 | MED-02 | R22/R23 expected rustfmt settings hardcoded, not linked to canonical module | `rustfmt_check.rs:54-61` | Link to canonical or test consistency | OPEN |
| 3 | 01 | MED-03 | R25 toolchain components check only verifies clippy/rustfmt -- no rust-src or llvm-tools-preview | `toolchain_check.rs:96` | Add optional component validation | OPEN |
| 4 | 01 | MED-04 | R27 priority error masked when lint level is also wrong -- user needs two passes | `cargo_lints.rs:441-484` | Report both level and priority issues simultaneously | OPEN |
| 5 | 01 | MED-06 | R29 silently skips crate Cargo.toml on read/parse failure | `cargo_lints.rs:313-319` | Emit error on read/parse failure | OPEN |
| 6 | 01 | MED-07 | R23 extra settings check only works for top-level keys | `rustfmt_check.rs:200-218` | Document limitation or add section checking | OPEN |
| 7 | 01 | MED-08 | R2 per-crate clippy.toml global-state check uses `contains()` for substring matching -- fragile (`NotLazyLock` matches) | `config_files.rs:173-178` | Match against full canonical paths or check segment boundaries | OPEN |
| 8 | 02 | FIND-02-02 | No check for `[graph]` section settings (`all-features = true`) -- feature-gated deps can skip checking | `deny_audit.rs` | Add [graph].all-features validation | OPEN |
| 9 | 02 | FIND-02-03 | No check for `[bans].wildcards` or `allow-wildcard-paths` existence | `deny_audit.rs` | Add existence/value checks | OPEN |
| 10 | 02 | FIND-02-05 | Advisory ignore table format (`{id = "...", reason = "..."}`) not parsed -- ID reported as "unknown" | `deny_inventory.rs:60` | Handle both string and table formats | OPEN |
| 11 | 02 | FIND-02-07 | Wrong-type advisory values report misleading error ("missing" instead of "wrong type") | `deny_audit.rs:167-223` | Distinguish missing from wrong-type in error message | OPEN |
| 12 | 02 | FIND-02-09 | `allow-registry` existence not verified -- missing means possible open registry | `deny_licenses.rs:204-220` | Error when allow-registry missing or doesn't contain crates.io | OPEN |
| 13 | 02 | FIND-02-11 | License allow list contents never validated -- copyleft licenses (GPL, AGPL, SSPL) pass silently | `deny_licenses.rs` | Maintain denied-license list, error if copyleft appears in allow | OPEN |
| 14 | 02 | FIND-02-14 | Tokio feature ban allow list not validated -- dangerous features addable | `deny_bans.rs:148-228` | Warn if allow list contains features not in expected set | OPEN |
| 15 | 02 | FIND-02-16 | Registry URL checked by substring (`contains("crates.io")`) -- typosquat possible | `deny_licenses.rs:207` | Use exact URL match | OPEN |
| 16 | 02 | FIND-02-18 | Ban list doesn't try `crate` key (only `name`) -- cargo-deny 0.19+ format bypasses detection | `deny_bans.rs:106` | Try `entry.get("crate")` as fallback | OPEN |
| 17 | 03 | MED-R58-fs | R58 `path.ends_with("fs.rs")` skip is too broad -- any file named `fs.rs` is exempt | `code_quality_checks.rs:137` | Check full path pattern, not just filename | OPEN |
| 18 | 03 | MED-R36-case | R36 EXCEPTION comment detection is case-sensitive -- `exception:` or `Exception:` not detected | `allow_checks.rs` | Use case-insensitive matching | OPEN |
| 19 | 03 | MED-R36-files | R36 only checks 4 config files -- misses `rust-toolchain.toml`, `.guardrail3/overrides/*.toml` | `allow_checks.rs` | Expand config file list | OPEN |
| 20 | 03 | MED-R40 | Grouped imports `use {a, b, c}` count as 1 use statement, not 3 -- exploitable by design | `structure_checks.rs` | Document or change to count individual imports | OPEN |
| 21 | 03 | MED-R40-mod | Only top-level use statements counted -- imports inside inline modules not counted | `structure_checks.rs` | Count use statements in inline modules | OPEN |
| 22 | 03 | MED-R34 | Struct-level `#[garde(skip)]` bypasses per-field type checking in typed visitor | `allow_checks.rs` | Check struct-level garde(skip) attributes | OPEN |
| 23 | 03 | MED-R53-override | Per-crate Cargo.toml can override workspace `unsafe_code` level if lint inheritance missing | `structure_checks.rs` | Cross-reference with R29 inheritance check | OPEN |
| 24 | 03 | MED-R53-single | Non-workspace (single-crate) projects silently skip R53 check | `structure_checks.rs` | Handle single-crate projects | OPEN |
| 25 | 03 | MED-R42-ffi | `unsafe extern "C" { }` (FFI blocks) not visited by UnsafeVisitor | `structure_checks.rs` | Add `visit_item_foreign_mod` to UnsafeVisitor | OPEN |
| 26 | 04 | F-04-01 | R45-R48 tool installation checks have no version validation -- ancient cargo-deny passes | `dependency_scan.rs:33`, `tool_runner.rs:13-19` | Add minimum version checks for critical tools | OPEN |
| 27 | 04 | F-04-04 | Crate renaming (`package = "..."`) bypasses dependency direction check | `hex_arch_checks.rs:173-178` | Read `package` field from dependency value | OPEN |
| 28 | 04 | F-04-05 | `layer_from_path` relies on directory naming conventions -- fragile | `hex_arch_checks.rs:47-59` | Document conventions or make layer detection configurable | OPEN |
| 29 | 04 | F-04-08 | `workspace = true` dependencies bypass allowlist entirely -- treated as internal path deps | `dependency_allowlist.rs:58-70` | Resolve workspace deps against workspace Cargo.toml to get real crate name | OPEN |
| 30 | 04 | F-04-09 | R55-R57 are all Info severity -- report but never enforce edition, publish, or release profile | `workspace_metadata.rs` | Elevate critical settings (edition, publish=false) to Warn/Error | OPEN |
| 31 | 04 | F-04-10 | TOML parse errors silently skip all dependency/architecture checks on that file | `workspace_metadata.rs`, `hex_arch_checks.rs`, `dependency_allowlist.rs` | Emit Error on parse failure | OPEN |
| 32 | 04 | F-04-11 | R-ARCH-01 only checks for `domain` and `adapters` layers -- `ports` and `app` not verified | `hex_arch_checks.rs:113` | Add ports and app to structure check | **FIXED** -- `application` layer added as part of GAP-TS-ARCH-06 fix (TS side). Rust hex arch check at F-04-11 is a separate finding for the Rust-side check. OPEN for Rust side. |
| 33 | 04 | F-04-12 | R-ARCH-04 cargo_path assumes config key matches directory path -- wrong for nested crate paths | `mod.rs:242` | Use actual crate path from workspace discovery | OPEN |
| 34 | 04 | F-04-14 | No check for `[patch]` or `[replace]` sections -- can override any dependency undetected | `hex_arch_checks.rs`, `dependency_allowlist.rs` | Scan and report [patch]/[replace] sections | OPEN |
| 35 | 05 | F03 | String literals in banned-import messages produce false passes -- "moment" appears in message text | `eslint_check.rs` | Parse config structure, not raw content | OPEN |
| 36 | 05 | F13 | Spread operator not verified for presets -- assigned but unused variable passes | `eslint_check.rs` | Verify spread into config array | OPEN |
| 37 | 05 | F15 | T50 does not verify all four route wrapper variants (missing withPublicBody/withPublicRoute) | `eslint_check.rs:342` | Check all four wrapper names | OPEN |
| 38 | 05 | F16 | T50 does not verify AST selectors are correct -- selectors 2-4 can be removed | `eslint_check.rs` | Validate all AST selector patterns | OPEN |
| 39 | 05 | F18 | T51 does not verify env.ts exemption exists and is narrow -- `**/*.ts` exemption passes | `eslint_check.rs` | Validate ignore pattern is narrow | OPEN |
| 40 | 05 | F21 | T-ESLP-04 regexp marker "flat/recommended" too generic -- may match other plugins | `eslint_plugin_checks.rs:236` | Use more specific marker string | OPEN |
| 41 | 05 | F23 | T-ESLP-11 test relaxation rules checked for presence, not "off" state -- main config rules match too | `eslint_plugin_checks.rs` | Verify rules appear in test override section with relaxed severity | OPEN |
| 42 | 05 | F25 | Rules provided by presets not explicitly configured could cause false negatives | `eslint_check.rs` | Document that explicit rule configuration is required | OPEN |
| 43 | 05 | F27 | Prefix matching causes false positives -- `no-empty` matches `no-empty-function` | ESLint check files | Use exact rule name matching with delimiters | OPEN |
| 44 | 05 | F29 | T46 checks `"max-dependencies"` which substring-matches `"import-x/max-dependencies"` -- works but fragile | `eslint_check.rs` | Use full qualified rule name | OPEN |
| 45 | 05 | F30 | T45 checks `"no-cycle"` as substring of `"import-x/no-cycle"` -- works but fragile | `eslint_check.rs` | Use full qualified rule name | OPEN |
| 46 | 05 | F35 | No check for `parserOptions.project: true` -- type-checked rules silently become no-ops without it | `eslint_check.rs` | Add parserOptions.project verification | OPEN |
| 47 | 05 | F39 | Test `test_core_plugins_all_pass` creates unrealistic content -- joins rule names, no actual config syntax | `eslint_plugin_checks_tests.rs` | Use realistic ESLint config syntax in tests | OPEN |
| 48 | 06 | F-05 | T35 coverage suppression missing `v8 ignore` pattern | `ts_comment_checks.rs` | Add `v8 ignore` to suppression patterns | OPEN |
| 49 | 06 | F-07 | T23 reason detection uses `"-- "` (trailing space) -- `--reason` (no space) rejected, `-- ` (empty reason) accepted | `ts_comment_checks.rs` | Accept `--` without trailing space, reject empty reasons | OPEN |
| 50 | 06 | F-09 | T59 banned packages only checks top-level node_modules -- pnpm non-hoisted transitive deps invisible | `source_scan.rs` | Use `pnpm list --json` for complete dependency tree | OPEN |
| 51 | 06 | F-10 | `is_ts_test_file` incomplete -- missing `.test.mjs`, `__mocks__/`, `*.stories.ts`, `*.e2e.ts`, `test/`, `tests/` | `source_scan.rs` | Expand test file detection patterns | OPEN |
| 52 | 06 | F-11 | `check_file_length` comment filter incomplete -- `/* comment */` single-line and `/**` JSDoc opening counted as effective lines | `source_scan.rs:237-240` | Improve comment line detection | OPEN |
| 53 | 07 | TSC-04 | BOM (Byte Order Mark) causes JSON parse failure -- all checks silently skipped | `tsconfig_check.rs:108` | Strip BOM before parsing | OPEN |
| 54 | 07 | TSC-05/TSC-06 | Check ID collision: T60 and T61 used for two different check pairs | `tsconfig_check.rs:141-142`, `jscpd_check.rs:241,269` | (Duplicate of CROSS-01 above) | OPEN |
| 55 | 07 | NPM-03 | Quoted values not handled -- `save-prefix=""` parsed as `"\"\""` not empty string | `npmrc_check.rs:63` | Strip surrounding quotes from values | OPEN |
| 56 | 07 | JSCPD-01 | `$schema` field not validated -- IDE validation/autocompletion won't work | `jscpd_check.rs:43-217` | Add $schema presence check | OPEN |
| 57 | 07 | JSCPD-05 | JSONC not supported in .jscpd.json -- comments cause silent bypass (combined with JSCPD-03) | `jscpd_check.rs:43` | Use JSONC-aware parser | OPEN |
| 58 | 07 | CROSS-03 | No BOM handling in tsconfig, npmrc, or jscpd parsers | All three check files | Add BOM stripping utility, apply to all parsers | OPEN |
| 59 | 09 | FINDING-H-05 | H-TOOL-03 lockfile check: `content.contains("lockfile")` overly broad | `hook_script_checks.rs` | Use more specific pattern | OPEN |
| 60 | 09 | FINDING-H-07 | H8 no version checking for tools | `tool_checks.rs` | Add minimum version validation | OPEN |
| 61 | 09 | FINDING-H-08 | H7 permissions not checked on modular scripts in `pre-commit.d/` | `hook_checks.rs` | Check permissions on all modular scripts | OPEN |
| 62 | 09 | FINDING-H-11 | Deployment checks skipped for non-standard project layouts (no apps/ dir) | `validate.rs:31` | Check for deployment configs regardless of directory structure | OPEN |
| 63 | 09 | FINDING-H-12 | D2 provider heuristic is filename-based, not content-based | `deploy_checks.rs` | Inspect railpack config content for provider detection | OPEN |
| 64 | 09 | FINDING-H-13 | D3 standalone check is bare string match -- `// standalone` in comment passes | `deploy_checks.rs` | Parse Next.js config, check `output: "standalone"` | OPEN |
| 65 | 09 | FINDING-H-14 | D4 outputFileTracingRoot value not validated -- any mention passes | `deploy_checks.rs` | Parse config and validate value points to monorepo root | OPEN |
| 66 | 09 | FINDING-H-16 | No deployment checks for Dockerfile-based services | `deploy_checks.rs` | Add Dockerfile existence and best-practice checks | OPEN |
| 67 | 09 | FINDING-H-18 | H-TOOL-01..05 only check monolithic script -- modular hook scripts ignored | `hook_checks.rs:164-175` | Also scan modular scripts in pre-commit.d/ | OPEN |
| 68 | 09 | FINDING-H-23 | No validation of test conditionality logic -- `# cargo test` (commented) passes | `hook_checks.rs` | Verify test commands are in executable context | OPEN |
| 69 | 10 | COV-MED-01 | Shadow detection nearest-parent overwrite -- deep config could point to wrong parent | `engine.rs:185-228` | Report only nearest parent shadow | OPEN |
| 70 | 10 | COV-MED-02 | tsconfig parse_details uses string matching, not JSON parsing | `tsconfig.rs:43-45` | Use proper JSON/JSONC parser | OPEN |
| 71 | 10 | COV-MED-03 | Crawler misses `package.json` "prettier" key as config source | `crawl.rs:199` | Add package.json prettier field detection | OPEN |
| 72 | 10 | COV-MED-04 | No symlink handling in crawler -- symlinked configs/sources invisible | `crawl.rs:76-81` | Document limitation or add `.follow_links(true)` | OPEN |
| 73 | 10 | COV-MED-05 | walk_up_resolve can walk past project root on non-canonical paths | `engine.rs:297-315` | Canonicalize paths before comparison | OPEN |
| 74 | 11 | 11-01 | `all_modules()` missing DENY_BANS_LIBRARY_IO and STYLELINT -- invisible to list-modules/show-module | `domain/modules/mod.rs:19-78` | Register missing modules | OPEN |

---

## LOW Findings (48)

| # | Report | ID | Description | Affected File(s) | Fix | Status |
|---|--------|----|-------------|-------------------|-----|--------|
| 1 | 01 | LOW-01 | R6/R7 extra bans reported as Info -- no detection of typo'd extra bans | `clippy_coverage.rs:206-218` | Consider fuzzy-match or path validation for extra bans | OPEN |
| 2 | 01 | LOW-02 | R28 missing expected "allow" entries only Info -- broken builds from noisy lints not flagged | `cargo_lints.rs:285-295` | Consider Warn for missing allow entries | OPEN |
| 3 | 01 | LOW-03 | R22 wrong value message has awkward double-quoting from TOML Display | `rustfmt_check.rs:95` | Format message without TOML display wrapper | OPEN |
| 4 | 01 | LOW-04 | Inconsistent `read_file` vs `read_file_err` between R2 and R3 | `config_files.rs:150,213` | Consistently use `read_file_err` | OPEN |
| 5 | 01 | LOW-05 | Duplicate ban entries in disallowed-methods/types silently deduplicated | `clippy_coverage.rs:168-176` | Flag duplicate entries | OPEN |
| 6 | 01 | LOW-06 | `check_rustfmt_str` has different code path from int/bool -- dead code potential | `rustfmt_check.rs` | Unify through `check_rustfmt_setting` | OPEN |
| 7 | 01 | LOW-07 | Nightly-only rustfmt settings not flagged on stable toolchain | `rustfmt_check.rs` | Cross-reference with toolchain channel | OPEN |
| 8 | 02 | FIND-02-08 | No `db-urls` or `git-fetch-with-cli` check | `deny_audit.rs` | Add advisory DB URL validation | OPEN |
| 9 | 02 | FIND-02-10 | `confidence-threshold` accepts dangerously low and integer values silently | `deny_licenses.rs:95-135` | Warn on threshold < 0.5, handle integer TOML values | OPEN |
| 10 | 02 | FIND-02-15 | No check for contradictory license deny+allow | `deny_licenses.rs` | Detect contradictions in allow vs deny lists | OPEN |
| 11 | 02 | FIND-02-19 | Missing inventory emissions for correct ban settings (`multiple-versions`, `highlight`) | `deny_audit.rs` | Emit Info/inventory on correct values for consistency | OPEN |
| 12 | 02 | FIND-02-20 | Feature deny array not validated for conflicts -- `deny = ["full", "rt-multi-thread"]` passes | `deny_bans.rs` | Warn if deny array contains non-"full" entries | OPEN |
| 13 | 03 | LOW-R32-block | Block comments `/* reason: */` not recognized as justification | `allow_checks.rs` | Add block comment detection | OPEN |
| 14 | 03 | LOW-R32-multi | Multi-line `#[allow(...)]` -- comment must be on span-start line | `allow_checks.rs` | Check all lines of multi-line attribute | OPEN |
| 15 | 03 | LOW-R37-stmt | `cfg_attr` on `let` bindings and match arms not detected | `allow_checks.rs` | Extend visitor to local/stmt/arm attributes | OPEN |
| 16 | 03 | LOW-R38 | Multi-line string literals inflating effective line count | `source_scan.rs` | Skip content inside raw string literals | OPEN |
| 17 | 03 | LOW-R34 | Type aliases (e.g. `type MyBool = bool`) cause false positives in garde skip check | `allow_checks.rs` | Document limitation | OPEN |
| 18 | 04 | F-04-06 | `is_service_internal` misses `src/<layer>/` structure | `hex_arch_checks.rs:387-392` | Also match `apps/<name>/src/<layer>` pattern | OPEN |
| 19 | 04 | F-04-07 | Allowlist implicitly skips dev/build deps -- uncontrolled test dependencies | `dependency_allowlist.rs:9-11` | Document or extend allowlist to dev deps | OPEN |
| 20 | 04 | F-04-13 | `normalize_path` doesn't handle absolute paths or leading `/` | `hex_arch_checks.rs:228-240` | Handle absolute path prefix | OPEN |
| 21 | 05 | F07 | Number extraction skips rule severity number -- `[2, {max: 300}]` extracts `2` not `300` | `eslint_rule_infra.rs` | Skip severity position in array config | OPEN |
| 22 | 06 | F-04 | T34/T35 `check_comment_pattern` uses substring match -- `// don't use noinspection` matches | `ts_comment_checks.rs` | Use prefix match on comment text | OPEN |
| 23 | 06 | F-06 | `eslint-enable` not tracked -- orphaned enables not flagged | `ts_comment_checks.rs` | Track enable/disable pairing | OPEN |
| 24 | 06 | F-08 | `.mjs` files scanned for T34/T35 but not T23-T31 -- inconsistent exclusion | `source_scan.rs:129` | Document or make consistent | OPEN |
| 25 | 06 | F-12 | Unicode/encoding bypass potential (zero-width chars, homoglyphs) -- theoretical only | `ts_code_analysis.rs` | No action needed (TS compiler rejects) | OPEN |
| 26 | 06 | F-13 | T30 eslint-disable-next-line suppression check is fragile -- uses raw line, not AST | `source_scan.rs:155-160` | Use AST comments for suppression detection | OPEN |
| 27 | 06 | F-15 | scoped_files path does not skip test fixtures -- staged fixture files get scanned | `source_scan.rs:18` | Apply fixtures exclusion in scoped_files path | OPEN |
| 28 | 07 | TSC-07 | `sourceMap` not validated | `tsconfig_check.rs:129-157` | Add sourceMap check (optional) | OPEN |
| 29 | 07 | TSC-09 | No validation of `lib`, `jsx`, `paths`, `baseUrl` values | `tsconfig_check.rs:353-408` | Add validation for key compiler options | OPEN |
| 30 | 07 | NPM-06 | `save-prefix` and `public-hoist-pattern` expected as empty string -- fragile if quoted in file | `npmrc_check.rs:87-88` | Handle quoted empty values | OPEN |
| 31 | 07 | JSCPD-04 | No glob syntax validation for ignore patterns -- typos silently exclude nothing | `jscpd_check.rs:116-136` | Validate glob syntax | OPEN |
| 32 | 07 | JSCPD-06 | Required ignore patterns use exact string match -- no normalization | `jscpd_check.rs:186-187` | Normalize glob patterns before comparison | OPEN |
| 33 | 08 | F-PKG-01 | T15 override values not validated -- `"zod": "*"` passes | `package_check.rs:62-63` | Validate semver range, reject wildcards | OPEN |
| 34 | 08 | F-PKG-02 | T17 does not check peerDependencies or optionalDependencies | `package_check.rs:156` | Add peer/optional to section list | OPEN |
| 35 | 08 | F-PKG-03 | T17 banned list no scoped/variant matching -- `lodash.merge`, `moment-timezone` bypass | `package_check.rs:133-153` | Add prefix bans for lodash., moment-, axios- | OPEN |
| 36 | 08 | F-PKG-04 | T55 preinstall check substring match -- `echo only-allow pnpm` bypasses | `package_check.rs:219` | Validate script starts with `npx only-allow pnpm` | OPEN |
| 37 | 08 | F-PKG-05 | T18 packageManager no format validation -- `npm@10.0.0` passes | `package_check.rs:182-210` | Validate value matches `pnpm@<semver>` | OPEN |
| 38 | 08 | F-PKG-06 | T57 engines no version range validation -- `>=0` or empty passes | `package_check.rs:347-375` | Validate node version constraint is reasonable | OPEN |
| 39 | 08 | F-PKG-08 | T-PLUG checks only verify presence, not version constraints | `package_deps.rs:11-45` | Add minimum version checks for critical packages | OPEN |
| 40 | 08 | F-PKG-09 | T-PLUG checks only check devDependencies -- packages in dependencies invisible | `package_deps.rs:64` | Also check dependencies section | OPEN |
| 41 | 08 | F-PKG-10 | T-TOOL-08/09/10 script checks verify existence only, not content | `tool_config_checks.rs:132-167` | Validate script contains expected tool name | OPEN |
| 42 | 08 | F-PKG-11 | T58 onlyBuiltDependencies missing -- no error when absent | `package_check.rs:411-427` | Emit Warn when missing | OPEN |
| 43 | 08 | F-PKG-12 | No workspace package validation -- sub-packages unchecked | `package_check.rs` | Iterate workspace members | OPEN |
| 44 | 08 | F-PKG-13 | Zero unit tests for any package.json check | `package_check.rs`, `package_deps.rs`, `tool_config_checks.rs` | Add unit tests with adversarial cases | OPEN |
| 45 | 08 | F-PKG-14 | package.json read and parsed independently 3 times -- wasted I/O | `package_check.rs`, `package_deps.rs`, `tool_config_checks.rs` | Parse once, pass Value to all check functions | OPEN |
| 46 | 08 | F-PKG-15 | Silent return on invalid JSON -- broken package.json invisible | All three package check files | Emit Error on parse failure | OPEN |
| 47 | 08 | F-PKG-16 | Missing banned packages (left-pad, colors, faker, event-stream, etc.) | `package_check.rs:133-153` | Review and expand banned list | OPEN |
| 48 | 08 | F-PKG-17 | T56 prepare script content not validated -- `echo no hooks` passes | `package_check.rs:249-280` | Validate script contains hook setup command | OPEN |

**Note:** Report 08 findings are classified as LOW because the report uses "GAP" severity (existence-not-value pattern), not CRITICAL/HIGH. Reports 09-14 LOW findings follow.

| # | Report | ID | Description | Affected File(s) | Fix | Status |
|---|--------|----|-------------|-------------------|-----|--------|
| -- | 08 | F-PKG-18 | T-PKG-02/03 lint/typecheck script content not validated | `package_check.rs:282-344` | Validate script contains eslint/tsc | OPEN |
| -- | 09 | FINDING-H-09 | H2 misleading error when git not installed | `hook_checks.rs` | Check git availability before running git commands | OPEN |
| -- | 09 | FINDING-H-15 | D5 Tailwind check only covers `apps/*/` -- misses root-level apps | `deploy_checks.rs` | Also check root package.json | OPEN |
| -- | 09 | FINDING-H-17 | No check for hook shadowing (`.git/hooks/pre-commit` vs `.githooks/`) | `hook_checks.rs` | Check for competing hook locations | OPEN |
| -- | 09 | FINDING-H-19 | Dispatcher pattern false positive via `. ` (period-space in comments) | `hook_checks.rs` | Use more specific dispatch pattern | OPEN |
| -- | 09 | FINDING-H-20 | No shebang validation in pre-commit hook | `hook_checks.rs` | Verify `#!/usr/bin/env bash` shebang | OPEN |
| -- | 09 | FINDING-H-22 | Duplicated railpack detection logic between validate.rs and deploy_checks.rs | `validate.rs`, `deploy_checks.rs` | Extract shared function | OPEN |
| -- | 10 | COV-LOW-01 | `check_thresholds` treats all integers as potential thresholds | `clippy.rs:127-131` | Only count threshold-like keys | OPEN |
| -- | 10 | COV-LOW-02 | Crawler misses `.cts` and `.cjs` file extensions | `crawl.rs:267` | Add .cts and .cjs extensions | OPEN |
| -- | 10 | COV-LOW-03 | Crawler misses CSS preprocessor files (.scss, .sass, .less) for Stylelint | `crawl.rs:271` | Add preprocessor extensions | OPEN |
| -- | 10 | COV-LOW-04 | No race condition protection during crawl | `crawl.rs:73-176` | Document limitation | OPEN |
| -- | 10 | COV-LOW-05 | npmrc walks_up semantic mismatch -- npm uses package.json location, not traditional walk-up | `npmrc.rs:49` | Document semantic difference | OPEN |
| -- | 11 | 11-02 | EXPECTED_TYPE_BANS missing 4 TYPE_GLOBAL_STATE entries (LazyLock, OnceLock, once_cell) | `clippy_coverage.rs:46-56` | Add missing entries | OPEN |
| -- | 11 | 11-03 | `generate_expected_ts` skips workspace_root hook replacement -- diff reports wrong for monorepos | `generate.rs:354-370` | Apply workspace_root replacement in diff path | OPEN |
| -- | 11 | 11-04 | `run_rs`/`run_ts` skip workspace_root hook replacement | `generate.rs:117-120,161-163` | Apply workspace_root replacement in rs/ts generate | OPEN |
| -- | 11 | 11-05 | Hook workspace_root replacement is fragile string manipulation | `generate.rs:279-282` | Use structured template system | OPEN |
| -- | 11 | 11-06 | Per-app deny.toml uses workspace profile, not effective profile -- library crates get wrong deny.toml | `generate_helpers.rs:293-298` | Use effective_profile for deny.toml generation | OPEN |
| -- | 11 | 11-07 | No override mechanism for non-TOML generated files (eslint, cspell, etc.) | `generate.rs:240-248` | Add override mechanism or document limitation | OPEN |
| -- | 11 | 11-08 | Staleness check sensitive to workspace structure changes | `check.rs:16-29` | Document or make check resilient to structure changes | OPEN |
| -- | 11 | 11-09 | diff.rs TOML section detection incomplete for deny.toml | `diff.rs:191-206` | Handle all deny.toml sections | OPEN |
| -- | 11 | 11-10 | Multiline override entries produce broken TOML | `generate_helpers.rs:56-75` | Support multiline TOML entries | OPEN |
| -- | 11 | 11-11 | Dedup uses substring matching, not TOML-aware comparison | `generate_helpers.rs:79-100` | Use TOML-aware deduplication | **FIXED** -- Regex bans added to EXPECTED_BANS in deny_audit.rs (7 crates: regex, fancy-regex, onig, pcre2, grep-cli, grep-regex, grep-matcher) and DENY_BANS_REGEX module created. This is a partial fix -- the dedup logic itself still uses substring matching, but the relevant bans are now properly registered. |
| -- | 11 | 11-12 | Pre-commit hook missing `set -e` -- future additions without error handling silently pass | `pre_commit.rs:11` | Add `set -e` to hook template | **FIXED** -- H-SAFE-01 check added for `set -e` validation. The template itself should also be updated. |
| -- | 11 | 11-13 | Pre-commit unquoted expansion breaks on filenames with spaces | `pre_commit.rs:142` | Quote command substitutions | OPEN |
| -- | 11 | 11-14 | CSS stylelint check operator precedence bug -- pnpm check not gating all config checks | `pre_commit.rs:174` | Add parentheses for correct operator precedence | OPEN |
| -- | 11 | 11-15 | Validator ignores profile distinction for type bans (service vs library) | `clippy_coverage.rs:46-56` | Profile-aware EXPECTED_TYPE_BANS | OPEN |
| -- | 11 | 11-17 | ESLINT_STARTER module doesn't match actual generated eslint config -- misleading | `canonical.rs:251-338` | Remove or update ESLINT_STARTER | OPEN |
| -- | 11 | 11-18 | release-plz.toml contains placeholder `your-crate-name` -- non-functional out of box | `release.rs:17-18` | Auto-populate from workspace or require user action | OPEN |
| -- | 11 | 11-19 | check/diff don't detect stale files from previous config | `check.rs`, `diff.rs` | Add reverse staleness detection | OPEN |
| -- | 12 | CLI-02 | `--garde` flag silently ignored for TypeScript -- runs ZERO TS checks | `main.rs:432-445` | Map --garde to TS categories or prevent its use with ts | OPEN |
| -- | 12 | CLI-04 | `--garde` in domains_from_args suppresses run_all but enables nothing | `main.rs:352-360` | Fix domains_from_args to handle --garde correctly | OPEN |
| -- | 12 | CLI-05 | Profile argument in rs init not validated -- `--profile banana` accepted | `cli.rs:84-89` | Add garde validation or clap value_parser | OPEN |
| -- | 12 | CLI-07 | Duplicate build_rs_categories/build_ts_categories between main.rs and commands/validate.rs | `main.rs`, `commands/validate.rs` | Extract shared functions | OPEN |
| -- | 12 | SCOPE-03 | --dirty missing diff-filter means deleted files included -- validates non-existent files | `commands/validate.rs:158-190` | Add `--diff-filter=ACM` to dirty mode | OPEN |
| -- | 12 | SCOPE-04 | --commits misses renamed file destinations | `commands/validate.rs:193-217` | Add R to diff filter | OPEN |
| -- | 12 | SCOPE-05 | --files paths not canonicalized or validated | `commands/validate.rs:118-120` | Canonicalize and validate paths | OPEN |
| -- | 12 | SCOPE-06 | Git command failures silently disable scoping -- `--staged` runs full validation if git fails | `commands/validate.rs:138-217` | Emit error when git unavailable with scope flags | OPEN |
| -- | 12 | DISC-02 | Discovery fallback cascade can lose data -- clears valid data before trying fallback | `discover.rs:60-77` | Preserve primary detection data during fallback | OPEN |
| -- | 12 | DISC-03 | Multiple `Path::exists()` calls bypass FileSystem trait in discover.rs | `discover.rs:62,72,93,136,184,283,310,355,365` | Use FileSystem trait consistently | OPEN |
| -- | 12 | FS-01 | `read_file` silently swallows all errors -- permission denied indistinguishable from missing | `fs.rs:12-14` | Return Result with error type | OPEN |
| -- | 12 | FS-03 | No file size limits on `read_file` -- OOM on huge files | `fs.rs:12-14` | Add file size limit check before read | OPEN |
| -- | 12 | RPT-03 | JSON output has no schema version field -- breaking changes undetectable by consumers | `json.rs:68-77` | Add `schema_version` field | OPEN |
| -- | 12 | RPT-05 | error_count includes inventory items if any have Error severity (nothing enforces severity constraint) | `domain/report.rs:159-165` | Enforce that inventory items cannot have Error severity | OPEN |
| -- | 12 | EXIT-01 | Exit code 1 used for both "violations found" and "tool failed" -- indistinguishable | `main.rs` | Use exit 1 for violations, exit 2 for tool errors | OPEN |
| -- | 13 | GAP-TS-ARCH-17 | `@domain/...` alias prefix matching too broad -- matches `@domain-utils` | `ts_arch_checks.rs:232-243` | Add delimiter check after layer name | OPEN |
| -- | 13 | GAP-TS-ARCH-19 | Files outside `modules/` directory not checked for boundary violations | `ts_arch_checks.rs:347` | Extend scope or document limitation | OPEN |
| -- | 13 | GAP-TS-ARCH-24 | Test `.skip()` reason check trivially bypassable -- `// no good reason` passes | `test_checks.rs:218` | Require `// reason:` prefix | OPEN |
| -- | 13 | GAP-TS-ARCH-28 | i18n only compares top-level keys -- nested key differences invisible | `i18n_check.rs:134` | Recursively compare nested keys | OPEN |
| -- | 13 | GAP-TS-ARCH-32 | All stylelint checks use `content.contains()` string matching | `stylelint_check.rs` | Parse config structure | OPEN |
| -- | 14 | FIND-14-05 | Property tests shallow -- none for TypeScript, hooks, or adversarial filesystem structures | Property test file | Add property tests for TS/hooks paths | OPEN |
| -- | 14 | FIND-14-07 | No negative self-validation test -- self-compliance is coincidental not proven | Test files | Add intentional-breakage regression test | OPEN |
| -- | 14 | FIND-14-08 | Missing edge case fixtures: huge files, binary files, symlink loops, null bytes, duplicate TOML keys | Fixture directories | Add adversarial fixture files | OPEN |
| -- | 14 | FIND-14-10 | Check ID numbering gaps: R39, R50-R52, R54, T33 referenced but missing from source | Source files | Reconcile IDs in documentation vs source | OPEN |

---

## Root Cause Patterns

### Pattern 1: `contains()` String Matching (72 findings)

**Description:** Using `content.contains(rule_name)` or `content.contains(keyword)` to check if a configuration setting exists. Cannot distinguish active config from comments, cannot detect severity ("error" vs "off"), matches substrings.

**Affected reports:** 01 (MED-08), 02 (FIND-02-16), 05 (F01, F02, F08, F09, F11, F12, F14, F17, F19, F20, F22, F26, F27, F28, F29, F30), 07 (NPM-01), 09 (FINDING-H-03, FINDING-H-04, FINDING-H-05, FINDING-H-13, FINDING-H-14), 10 (COV-MED-02), 13 (GAP-TS-ARCH-01, GAP-TS-ARCH-09, GAP-TS-ARCH-13, GAP-TS-ARCH-15, GAP-TS-ARCH-32)

**Root cause:** Design decision to use raw text matching instead of structural parsing for ESLint flat config, hook scripts, deploy configs, and some TOML checks.

**Status:** NPM-01 fixed (rfind + duplicate detection). Rest largely OPEN -- the ESLint findings require tree-sitter-javascript integration which is a major work item.

### Pattern 2: Existence-Not-Value (38 findings)

**Description:** Checking that a setting/field/file exists without validating its value or content. A field set to "off", empty, wildcard, or wrong value passes.

**Affected reports:** 05 (F11, F20, F26), 08 (F-PKG-01, F-PKG-05, F-PKG-06, F-PKG-08, F-PKG-10, F-PKG-11, F-PKG-17, F-PKG-18), 09 (FINDING-H-10, FINDING-H-13, FINDING-H-14), 13 (GAP-TS-ARCH-07, GAP-TS-ARCH-11, GAP-TS-ARCH-27)

**Root cause:** Checks verify presence (`is_some()`, `contains_key()`, `exists()`) without inspecting or validating the associated value.

**Status:** Largely OPEN.

### Pattern 3: Silent Failures / Swallowed Errors (22 findings)

**Description:** Parse errors, read failures, or invalid data cause silent returns with no diagnostic output. User sees a clean report for a broken file.

**Affected reports:** 01 (HIGH-01, HIGH-02, MED-06), 04 (F-04-10), 07 (JSCPD-03, TSC-01), 08 (F-PKG-15), 09 (FINDING-H-09), 12 (FS-01, SCOPE-06)

**Root cause:** Using `Err(_) => return` or `None => continue` patterns without emitting a CheckResult to inform the user.

**Status:** OPEN.

### Pattern 4: Dead Code / Unwired Checks (5 findings)

**Description:** Check functions exist but are never called from the orchestrator or main execution path.

**Affected reports:** 03 (CRIT-R42, CRIT-R53), 11 (11-01 missing module registrations), 12 (CLI-08 potential dead validation code)

**Root cause:** Functions added or moved without updating the caller chain.

**Status:** CRIT-R42 FIXED (removed -- redundant with clippy forbid). CRIT-R53 FIXED (wired into orchestrator). 11-01 and CLI-08 still OPEN.

### Pattern 5: Profile Parameter Ignored (4 findings)

**Description:** `_profile` parameter accepted but never used, causing library-profile-specific checks to be silently skipped.

**Affected reports:** 01 (CRIT-02), 02 (FIND-02-01), 11 (11-06, 11-15)

**Root cause:** Profile-aware validation planned but not implemented; service profile hardcoded as default.

**Status:** CRIT-02 REJECTED (intentional design -- both profiles use strictest ban set). FIND-02-01, 11-06, 11-15 still OPEN.

### Pattern 6: FileSystem Trait Bypass (3 findings)

**Description:** Using `std::path::Path::exists()` or `std::fs` directly instead of the injected `FileSystem` trait, breaking testability.

**Affected reports:** 01 (CRIT-03), 12 (DISC-03), 13 (GAP-TS-ARCH-08)

**Root cause:** Inconsistent discipline in using the centralized filesystem abstraction.

**Status:** OPEN (21 call sites, large refactor).

### Pattern 7: First-Match-Wins (4 findings)

**Description:** Using `find()` or first-match iteration when the correct semantics require last-match or nearest-match.

**Affected reports:** 05 (F06), 07 (NPM-01), 10 (COV-CRIT-02)

**Root cause:** Iterator methods defaulting to first match without considering config override semantics.

**Status:** NPM-01 FIXED (rfind + duplicate detection). COV-CRIT-02 FIXED (max_by_key for nearest ancestor). F06 still OPEN.

### Pattern 8: Missing Test Coverage (10 findings)

**Description:** Entire subsystems, check IDs, or module categories with zero test coverage.

**Affected reports:** 05 (F37, F38), 08 (F-PKG-13), 14 (FIND-14-01 through FIND-14-09)

**Root cause:** Tests written for happy paths; adversarial/error-path tests systematically missing.

**Status:** OPEN.

### Pattern 9: Hardcoded Values Not Linked to Canonical Source (6 findings)

**Description:** Expected values duplicated from canonical modules without compile-time or test-time consistency verification.

**Affected reports:** 01 (MED-01, MED-02), 05 (F04), 11 (11-02)

**Root cause:** Manual synchronization between canonical modules and validation expectations.

**Status:** F04 REJECTED (400 is correct baseline). CRIT-01 FIXED (missing_debug_implementations added; missing_docs removed by design). MED-01, MED-02, 11-02 still OPEN.

### Pattern 10: Scope/Boundary Gaps (8 findings)

**Description:** Checks that only validate one file/section when multiple files/sections affect the outcome (child tsconfigs, workspace members, dev-dependencies, modular hooks).

**Affected reports:** 04 (F-04-03), 07 (TSC-02, TSC-03), 08 (F-PKG-02, F-PKG-12), 09 (FINDING-H-18), 13 (GAP-TS-ARCH-41)

**Root cause:** Checks designed for single-file projects, not monorepo/multi-config scenarios.

**Status:** OPEN.

---

## Fix Prioritization

### Work Item 1: Wire Dead Code (2 fixes, CRITICAL) -- **DONE**
**Root cause:** Dead code
**Files:** `source_scan.rs`, `structure_checks.rs`
**Findings:** CRIT-R42, CRIT-R53
**Work:** ~~Call `check_unsafe()` and `check_unsafe_code_forbid()` from source_scan orchestrator.~~ R42 removed (redundant with clippy forbid). R53 wired into orchestrator.

### Work Item 2: Fix Profile-Aware Validation (4 fixes, CRITICAL+HIGH) -- PARTIALLY DONE
**Root cause:** Profile parameter ignored
**Files:** `clippy_coverage.rs`, `deny_bans.rs`, `generate_helpers.rs`
**Findings:** 01-CRIT-02 (**REJECTED**), 02-FIND-01, 11-06, 11-15
**Work:** CRIT-02 rejected as intentional. Remaining 3 findings still OPEN.

### Work Item 3: Fix First-Match-Wins Bugs (3 fixes, CRITICAL+HIGH) -- PARTIALLY DONE
**Root cause:** First-match-wins
**Files:** `npmrc_check.rs`, `engine.rs`, `eslint_rule_infra.rs`
**Findings:** NPM-01 (**FIXED**), COV-CRIT-02 (**FIXED**), F06 (OPEN)
**Work:** npmrc uses rfind + duplicate detection. Coverage engine uses max_by_key. ESLint F06 still needs last-occurrence matching.

### Work Item 4: Add Missing Lint Expectations (2 fixes, CRITICAL) -- **DONE**
**Root cause:** Hardcoded values
**Files:** `cargo_lints.rs`, `clippy_coverage.rs`
**Findings:** 01-CRIT-01 (**FIXED**), 11-02 (OPEN -- EXPECTED_TYPE_BANS still missing LazyLock/OnceLock/once_cell)
**Work:** CRIT-01 resolved (missing_docs removed, missing_debug_implementations added). 11-02 remains open.

### Work Item 5: Replace FileSystem Trait Bypasses (3 fixes, CRITICAL+LOW) -- OPEN
**Root cause:** FileSystem trait bypass
**Files:** `config_files.rs`, `clippy_coverage.rs`, `cargo_lints.rs`, `discover.rs`
**Findings:** 01-CRIT-03, 12-DISC-03, 13-GAP-TS-ARCH-08
**Work:** Replace all `path.exists()` calls with FileSystem trait methods.

### Work Item 6: Fix Silent Failure Patterns (10 fixes, HIGH+MEDIUM) -- OPEN
**Root cause:** Silent failures
**Files:** `config_files.rs`, `rustfmt_check.rs`, `toolchain_check.rs`, `cargo_lints.rs`, `jscpd_check.rs`, `workspace_metadata.rs`, `hex_arch_checks.rs`, `dependency_allowlist.rs`, `package_check.rs`
**Findings:** HIGH-01, HIGH-02, HIGH-03, HIGH-04, MED-06, JSCPD-03, F-04-10, F-PKG-15
**Work:** Replace `Err(_) => return`/`continue` with Error result emission.

### Work Item 7: ESLint Validation Overhaul (30+ fixes, CRITICAL+HIGH+MEDIUM) -- OPEN
**Root cause:** contains() string matching, existence-not-value
**Files:** `eslint_check.rs`, `eslint_plugin_checks.rs`, `eslint_rule_infra.rs`, `eslint_audit.rs`
**Findings:** F01, F02, F04 (**REJECTED**), F05, F06, F08, F09, F11, F12, F14, F17, F19, F20, F22, F26, F27, F28, F29, F30, F35, F37, F38, F39 + GAP-TS-ARCH-09, GAP-TS-ARCH-11, GAP-TS-ARCH-13, GAP-TS-ARCH-15
**Work:** Strip comments before matching; parse last effective config block per rule; verify severity; fix value extraction; add parserOptions check. Add comprehensive tests. Requires tree-sitter-javascript integration.
**Additional:** T-ESLP-15 check added (ban RegExp via no-restricted-globals and regex literals via no-restricted-syntax).

### Work Item 8: Fix Scope Flag Bugs (5 fixes, CRITICAL+MEDIUM) -- MOSTLY DONE
**Root cause:** Git integration gaps
**Files:** `commands/validate.rs`, `cli.rs`, `main.rs`
**Findings:** CLI-01 (**FIXED**), CLI-03 (**REJECTED**), SCOPE-01 (**FIXED**), SCOPE-02 (**FIXED**), SCOPE-06 (OPEN)
**Work:** Mutual exclusion added. ACMR diff filter applied. Untracked file collection added. CLI-03 rejected (auditor wrong). SCOPE-06 still needs error on git failures with scope flags.

### Work Item 9: Hook Script Validation Improvements (8 fixes, CRITICAL+HIGH+MEDIUM) -- PARTIALLY DONE
**Root cause:** contains() matching, existence-not-value
**Files:** `hook_checks.rs`, `hook_script_checks.rs`, `tool_checks.rs`, `pre_commit.rs`
**Findings:** FINDING-H-01 (**REJECTED**), FINDING-H-02 (**FIXED**), FINDING-H-03, FINDING-H-04, FINDING-H-06, FINDING-H-10, FINDING-H-18, FINDING-H-23
**Work:** set -e validation added. --no-verify rejected as not detectable. Remaining findings still OPEN.

### Work Item 10: JSON Config Parser Hardening (6 fixes, HIGH+MEDIUM) -- OPEN
**Root cause:** Parser limitations
**Files:** `tsconfig_check.rs`, `jscpd_check.rs`, `npmrc_check.rs`
**Findings:** TSC-01, TSC-02, TSC-03, TSC-04, NPM-02, NPM-03, CROSS-03
**Work:** Add JSONC support (or comment stripping); add BOM stripping; strip inline comments in npmrc; handle quoted values.

### Work Item 11: Coverage Engine Fixes (5 fixes, CRITICAL+HIGH) -- PARTIALLY DONE
**Root cause:** Path comparison bugs, missing file patterns
**Files:** `engine.rs`, `crawl.rs`, `prettier.rs`, `cspell.rs`, `jscpd.rs`
**Findings:** COV-CRIT-01 (**FIXED**), COV-CRIT-02 (**FIXED**), COV-HIGH-01, COV-HIGH-02, COV-HIGH-03, COV-HIGH-04, COV-HIGH-05, COV-HIGH-06
**Work:** Path comparison bugs fixed. Missing config file patterns and multi-language source_dirs still OPEN.

### Work Item 12: package.json Validation Improvements (17 fixes, LOW) -- OPEN
**Root cause:** Existence-not-value
**Files:** `package_check.rs`, `package_deps.rs`, `tool_config_checks.rs`
**Findings:** F-PKG-01 through F-PKG-18
**Work:** Validate field values (not just presence); add version constraints; expand banned list; add unit tests.

### Work Item 13: TS Architecture & Boundary Enforcement (8 fixes, CRITICAL+HIGH+MEDIUM) -- PARTIALLY DONE
**Root cause:** String matching instead of AST
**Files:** `ts_arch_checks.rs`, `ts_code_analysis.rs`
**Findings:** GAP-TS-ARCH-01 (OPEN), GAP-TS-ARCH-02, GAP-TS-ARCH-03, GAP-TS-ARCH-06 (**FIXED** -- application layer added), GAP-TS-ARCH-16, GAP-TS-ARCH-17, GAP-TS-ARCH-18, GAP-TS-ARCH-19
**Work:** Use tree-sitter for import extraction; fix path separator handling. Application layer check done. Rest requires tree-sitter-javascript.

### Work Item 14: deny.toml Validation Improvements (10 fixes, HIGH+MEDIUM) -- OPEN
**Root cause:** Missing checks, existence-not-value
**Files:** `deny_audit.rs`, `deny_bans.rs`, `deny_inventory.rs`, `deny_licenses.rs`
**Findings:** FIND-02-02, FIND-02-03, FIND-02-04, FIND-02-05, FIND-02-06, FIND-02-09, FIND-02-11, FIND-02-14, FIND-02-16, FIND-02-18
**Work:** Add [graph] validation; require reasons on ignores/skips; validate license list; check registry URLs exactly; support crate key format.
**Additional:** Regex crate bans added to EXPECTED_BANS (7 crates via DENY_BANS_REGEX module).

### Work Item 15: Test Coverage Expansion (10 fixes, CRITICAL+HIGH+MEDIUM) -- OPEN
**Root cause:** Missing test coverage
**Files:** 24+ test/source files
**Findings:** FIND-14-01, FIND-14-02, FIND-14-03, FIND-14-04, FIND-14-05, FIND-14-06, FIND-14-07, FIND-14-08, FIND-14-09, F-PKG-13, F37, F38
**Work:** Add targeted tests for 48 untested check IDs; add unit tests for 24 untested modules; add coverage engine tests; integrate golden tests in CI; add adversarial fixtures.

### Work Item 16: Rust Dependency & Architecture Fixes (6 fixes, HIGH+MEDIUM) -- OPEN
**Root cause:** Scope gaps, naming assumptions
**Files:** `hex_arch_checks.rs`, `dependency_allowlist.rs`, `dependency_scan.rs`
**Findings:** F-04-02, F-04-03, F-04-04, F-04-08, F-04-11, F-04-14
**Work:** Check dev/build-dependencies; read package field for renamed crates; fix workspace=true bypass; add ports/app layer checks; scan patch/replace sections.

### Work Item 17: Discovery & TypeScript Detection Fix (2 fixes, CRITICAL+MEDIUM) -- PARTIALLY DONE
**Root cause:** Overbroad detection
**Files:** `discover.rs`
**Findings:** DISC-01 (**FIXED**), DISC-02 (OPEN)
**Work:** TS detection now requires tsconfig.json or typescript dependency. DISC-02 (fallback data loss) still OPEN.

### Work Item 18: CLI & Reporting Fixes (6 fixes, LOW) -- OPEN
**Root cause:** Various
**Files:** `cli.rs`, `main.rs`, `json.rs`, `report.rs`
**Findings:** CLI-02, CLI-04, CLI-05, CLI-07, RPT-03, EXIT-01
**Work:** Fix flag mapping; add schema version; distinguish exit codes.

### Work Item 19: Pre-commit Template Fixes (4 fixes, LOW) -- PARTIALLY DONE
**Root cause:** Shell script bugs
**Files:** `pre_commit.rs`
**Findings:** 11-12 (**FIXED** -- H-SAFE-01 check validates set -e), FINDING-H-02 (**FIXED**), 11-13 (OPEN), 11-14 (OPEN)
**Work:** set -e validation added. Quoting and operator precedence fixes still OPEN.

### Work Item 20: Generate/Check/Diff Improvements (5 fixes, LOW) -- OPEN
**Root cause:** Various
**Files:** `generate.rs`, `generate_helpers.rs`, `check.rs`, `diff.rs`
**Findings:** 11-03, 11-04, 11-05, 11-09, 11-19
**Work:** Apply workspace_root replacement consistently; improve TOML section detection; detect stale files.
