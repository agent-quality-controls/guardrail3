# Parsing Method Inventory — Every Check Function Categorized

**Scope:** All files in `rs/validate/`, `ts/validate/`, `hooks/`

---

## Summary

| Category | Count |
|---|---|
| AST-based (syn for Rust) | 23 functions |
| AST-based (tree-sitter for TS) | 8 functions |
| Structured parser (toml crate) | 25 functions |
| Structured parser (serde_json) | 13 functions |
| Tool output / filesystem probe | 12 functions |
| **String matching (WRONG)** | **42 instances** |

---

## RUST CHECKS (`rs/validate/`)

### `allow_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_crate_level_allow` | R30-R31 | **syn AST** via `ast_helpers::parse_file` + `find_crate_level_allows` | CORRECT |
| `check_item_level_allow` / `check_item_level_allow_ast` | R32-R33 | **syn AST** via `find_item_allows`; comment check uses `l.contains("//")` on raw line | MIXED - AST for attribute detection, string for comment detection |
| `check_garde_skip` / `check_garde_skip_ast` | R34-R35 | **syn AST** via `find_garde_skips_with_types`; comment check uses `l.contains("//")` on raw line | MIXED |
| `check_exception_comments` | R36 | **STRING MATCHING** on config file content: `line.contains("// EXCEPTION:")` and `line.contains("# EXCEPTION:")` | WRONG |
| `check_cfg_attr_allow` / `check_cfg_attr_allow_ast` | R37 | **syn AST** via `find_cfg_attr_allows` | CORRECT |

#### String matching instances in `allow_checks.rs`:

1. **`check_item_level_allow_ast`** (R32-R33): `l.contains("//")` to detect whether a `#[allow]` has a reason comment. The attribute itself is found via syn, but reason-comment detection is string-based.
   - **What it checks:** Whether the raw source line containing `#[allow(...)]` also has a `//` comment.
   - **Correct approach:** This is actually acceptable -- the AST identifies the attribute, then checking the raw line for a trailing comment is reasonable since comments are not part of the syn AST. No false positive risk here.

2. **`check_garde_skip_ast`** (R34-R35): Same pattern as above -- `l.contains("//")` for reason comments after AST-detected attributes.
   - **Same verdict:** Acceptable hybrid approach.

3. **`check_exception_comments`** (R36): Iterates `content.lines()` checking `line.contains("// EXCEPTION:")` and `line.contains("# EXCEPTION:")` on **config files** (clippy.toml, deny.toml, Cargo.toml, rustfmt.toml).
   - **What it checks:** TOML/config file comment content.
   - **Correct approach:** TOML parsers strip comments, so there is NO structured way to extract comments from TOML. String matching is the ONLY option here. **This is correct given the constraint.**

### `ast_helpers.rs` / `ast_visitors.rs` / `extra_visitors.rs`

All functions are **syn AST-based**. These are the infrastructure, not check functions themselves. CORRECT.

### `cargo_lints.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check` | R26-R29 | **toml crate** (`content.parse::<toml::Value>()`) | CORRECT |
| `check_rust_lints` | R26 | **toml crate** (traverses `toml::Value` tree) | CORRECT |
| `check_clippy_lints` | R27-R28 | **toml crate** | CORRECT |
| `check_workspace_inheritance` | R29 | **toml crate** | CORRECT |

### `clippy_coverage.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check` | R4-R7 | **toml crate** (`content.parse::<toml::Value>()`) then traverses structured data | CORRECT |

### `code_quality_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_todo_macros` | R43 | **syn AST** via `find_forbidden_macros` | CORRECT |
| `check_unwrap_expect` | R44 | **syn AST** via `find_unwrap_expect` | CORRECT |
| `check_file_exists_at_root` | R49 etc. | **Filesystem probe** (`file_path.exists()`) | CORRECT (not source analysis) |
| `check_claude_md` | R49 | **Filesystem probe** | CORRECT |
| `check_direct_fs_usage` | R58 | **syn AST** via `find_std_fs_imports` + `find_inline_std_fs_calls` | CORRECT |

### `config_files.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check` | R1-R3, R21-R25 | **Filesystem probe** (exists checks) + delegates to sub-modules | CORRECT |
| `check_per_crate_clippy` | R2 | **toml crate** + `.contains()` on type paths | MIXED |
| `check_per_crate_clippy_content` | R2 | **toml crate** for parsing, then `tp.contains(gs_type)` to check if paths contain global-state type names | MIXED |
| `check_clippy_thresholds` | R3 | **toml crate** | CORRECT |

#### String matching instance:
4. **`check_per_crate_clippy_content`** (R2): After TOML-parsing the file, uses `tp.contains(gs_type)` where `tp` is a path string from the parsed TOML array and `gs_type` is "LazyLock"/"OnceLock"/"once_cell".
   - **What it checks:** Whether TOML-extracted type paths contain global-state type name substrings.
   - **Correct approach:** This is string matching on *parsed TOML data*, not raw source. The TOML structure is correctly parsed; the substring check on the extracted path value is a reasonable heuristic. **Acceptable.**

### `deny_audit.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check` | R8-R11 | **toml crate** | CORRECT |
| All sub-functions | R8-R11, R19-R20 | **toml crate** (traverse `toml::Value`) | CORRECT |

### `deny_bans.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R12-R13, R17-R18 | **toml crate** | CORRECT |

### `deny_inventory.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R19-R20 | **toml crate** | CORRECT |

### `deny_licenses.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R14-R16 | **toml crate** | CORRECT |

### `dependency_allowlist.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_dependency_allowlist` | R-DEPS-01 | **toml crate** | CORRECT |
| `check_library_has_allowlist` | R-DEPS-02 | Config struct inspection (no file parsing) | CORRECT |

### `dependency_scan.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check` | R45-R48 | **Tool checker** (PATH probing) | CORRECT |

### `garde_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_garde_dependency` / `content_has_garde_dependency` | R-GARDE-01 | **STRING MATCHING** on Cargo.toml content | **WRONG** |
| `check_ban_presence` | R-GARDE-02 to R-GARDE-04 | **toml crate** (via `extract_ban_paths`) | CORRECT |
| `check_derive_inventory` / `find_unvalidated_input_structs` | R-GARDE-05 | **syn AST** via `find_derive_attributes` | CORRECT |

#### String matching instances:
5. **`content_has_garde_dependency`** (R-GARDE-01): Manually iterates `content.lines()`, checks `trimmed.starts_with('[')` to detect section headers, `lower.contains("dependencies")`, `trimmed.starts_with("garde")`, then `rest.trim_start().starts_with('=')`.
   - **What it checks:** Whether Cargo.toml has `garde` in a `[*dependencies*]` section.
   - **Correct approach:** Parse with `toml` crate, traverse `[workspace.dependencies]`, `[dependencies]`, etc. to check for a `garde` key.
   - **Why it's wrong:** This line-by-line parser will miss `garde` if it's nested under `[workspace.dependencies.garde]` using TOML's table syntax, or if the section header is on a continued line, etc. The TOML crate is already used everywhere else in the codebase.

### `hex_arch_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_hex_arch_structure` | R-ARCH-01 | **Filesystem probe** + **toml crate** for Cargo.toml | CORRECT |
| `check_dependency_flow` | R-ARCH-02 | **toml crate** for Cargo.toml dependency parsing | CORRECT |
| `check_library_service_boundary` | R-ARCH-03 | **toml crate** | CORRECT |
| `check_unconfigured_members` | R-ARCH-04 | Config struct + filesystem probe | CORRECT |
| `contains_segment` | helper | `path.split('/').any(|s| s == segment)` | Acceptable (path segment matching, not source parsing) |
| `is_service_internal` | helper | Path segment checking | Acceptable |

### `release_bin_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_binary_release_workflow` | R-BIN-01 | **STRING MATCHING** on YAML workflow content | **WRONG** |
| `check_binary_linux_target` | R-BIN-02 | **STRING MATCHING** on YAML workflow content | **WRONG** |
| `check_binstall_metadata` | R-BIN-03 | **toml crate** (via `CrateInfo.table`) | CORRECT |

#### String matching instances:
6. **`check_binary_release_workflow`** (R-BIN-01): `content.contains("--release") && content.contains("action-gh-release")` on raw YAML workflow files.
   - **What it checks:** Whether any GitHub Actions workflow builds with `--release` and uses `action-gh-release`.
   - **Correct approach:** Parse YAML with `serde_yaml` or a YAML parser, then inspect the structured workflow steps.

7. **`check_binary_linux_target`** (R-BIN-02): `lower.contains("linux") || lower.contains("x86_64") || lower.contains("amd64") || lower.contains("ubuntu")` on raw YAML.
   - **What it checks:** Whether workflows reference linux targets.
   - **Correct approach:** Parse YAML, inspect `runs-on` and matrix strategy fields.

### `release_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `discover_crates` | utility | **toml crate** for each Cargo.toml | CORRECT |
| `check` | orchestrator | delegates | CORRECT |

### `release_crate_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R-PUB-01 to R-PUB-08 | **toml crate** (via `CrateInfo.table`) | CORRECT |
| `check_readme_quality` | R-PUB-05 | **STRING MATCHING**: `content.lines().any(\|line\| line.starts_with('#'))` | **WRONG** |

#### String matching instance:
8. **`check_readme_quality`** (R-PUB-05): `content.lines().any(|line| line.starts_with('#'))` to check README has a heading.
   - **What it checks:** Whether the README has a markdown heading.
   - **Correct approach:** This is checking PROSE markdown content for a formatting convention. A markdown parser could be used, but `starts_with('#')` on a markdown file is actually the standard way to detect headings. **Acceptable -- markdown heading detection by convention.**

### `release_crate_deps.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R-PUB-06 to R-PUB-11 | **toml crate** | CORRECT |
| `is_valid_semver` / `parse_version_parts` | R-PUB-08 | String parsing of version numbers | Acceptable (not source code, parsing a version string) |

### `release_repo_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_license_file` | R-REL-01 | **Filesystem probe** | CORRECT |
| `check_release_plz_toml` | R-REL-02, R-REL-03 | **toml crate** | CORRECT |
| `check_cliff_toml` | R-REL-04 | **Filesystem probe** | CORRECT |
| `check_workflow_contains` | R-REL-05 to R-REL-07 | **STRING MATCHING** on YAML workflow content | **WRONG** |
| `check_semver_checks_installed` | R-REL-08 | **Tool checker** | CORRECT |
| `read_workflow_files` | utility | File reader | CORRECT |

#### String matching instances:
9. **`check_workflow_contains`** (R-REL-05): `content.contains("release-plz")` on raw YAML.
10. **`check_workflow_contains`** (R-REL-06): `content.contains("cargo publish --dry-run")` on raw YAML.
11. **`check_workflow_contains`** (R-REL-07): `content.contains("CARGO_REGISTRY_TOKEN")` on raw YAML.
   - **What they check:** Whether GitHub Actions workflow files contain specific strings.
   - **Correct approach:** Parse YAML, inspect steps/env sections. Note: these are checking for the *presence* of patterns in YAML files. A YAML parser would be more robust but the risk of false positives is low since these are very specific strings. **Borderline -- low risk but technically wrong.**

### `rustfmt_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R22-R23 | **toml crate** | CORRECT |

### `source_scan.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check` | orchestrator | Delegates to AST-based checks | CORRECT |
| `filter_non_comment_lines` | utility | **STRING MATCHING** for block comment tracking (`find("*/")`, `find("/*")`, `starts_with("//")`) | MIXED |
| `strip_inline_block_comments` | utility | **STRING MATCHING** for block comment stripping | MIXED |
| `strip_string_literals` | utility | Character-by-character string literal stripping | MIXED |

#### Note on `filter_non_comment_lines`:
This function is used by `check_file_length` (R38) to count "effective lines" excluding comments. It's a hand-rolled comment stripper, not a proper parser. However, it's only counting lines, not analyzing code structure. The syn-based alternative would be to count AST nodes, but line counting is inherently a text operation. **Borderline acceptable for the line-counting use case.**

### `structure_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_file_length` | R38 | Uses `filter_non_comment_lines` (string-based line counter) | MIXED (see above) |
| `check_use_count` | R40-R41 | **syn AST** via `count_use_statements` | CORRECT |
| `check_unsafe_code_forbid` | R53 | **toml crate** | CORRECT |

### `test_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_cargo_mutants_installed` | R-TEST-01 | **Tool checker** | CORRECT |
| `check_mutants_toml` | R-TEST-02 | **Filesystem probe** | CORRECT |
| `check_mutants_profile` / `has_mutants_profile` | R-TEST-03 | **STRING MATCHING**: `trimmed == "[profile.mutants]"` on Cargo.toml | **WRONG** |
| `check_tests_exist` / `content_has_test` | R-TEST-04 | **syn AST** via `has_test_attribute` | CORRECT |
| `check_no_tests_in_src` | R-TEST-09 | **syn AST** via `has_test_attribute` + `file_has_inline_cfg_test_module` | CORRECT |

#### String matching instance:
12. **`has_mutants_profile`** (R-TEST-03): `trimmed == "[profile.mutants]"` iterating lines of Cargo.toml.
   - **What it checks:** Whether Cargo.toml has a `[profile.mutants]` section.
   - **Correct approach:** Parse with `toml` crate, check `table.get("profile").and_then(|p| p.get("mutants"))`.
   - **Why it's wrong:** TOML allows `[profile.mutants]` to be expressed as `[profile]\n[profile.mutants]` or via dotted keys. The line-matching approach misses valid TOML representations.

### `test_quality_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_test_coverage_inventory` | R-TEST-05 | **syn AST** via `count_pub_fn_decls` + `count_test_attrs` | CORRECT |
| `check_integration_tests` | R-TEST-06 | **Filesystem probe** | CORRECT |
| `check_ignore_without_reason` | R-TEST-07 | **syn AST** via `find_ignore_without_reason` | CORRECT |
| `check_mutation_hook` | R-TEST-08 | **STRING MATCHING** on file content | **WRONG** |

#### String matching instances:
13. **`check_mutation_hook`** (R-TEST-08): `content.contains("mutant")`, `content.contains("cargo-mutants")`, `content.contains("cargo mutants")`, `content.contains("stryker")` on `.claude/` directory files and `.git/hooks/pre-commit`.
   - **What it checks:** Whether mutation testing is configured in hooks.
   - **Correct approach:** These are shell scripts and JSON config files. For shell scripts, string matching is the only practical option. For JSON configs, `serde_json` could be used. **Mostly acceptable for shell script content.**

### `toolchain_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R25 | **toml crate** | CORRECT |

### `workspace_metadata.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | R55-R57 | **toml crate** | CORRECT |

---

## TYPESCRIPT CHECKS (`ts/validate/`)

### `ast_helpers.rs` / `ts_code_analysis.rs`

All functions use **tree-sitter** AST parsing. Infrastructure, not check functions. CORRECT.

### `ts_comment_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_eslint_disable` | T23-T26 | **tree-sitter AST** (extracts comment nodes, then checks comment text) | CORRECT |
| `check_ts_ignore` | T27-T29 | **tree-sitter AST** (extracts comment nodes, then checks comment text) | CORRECT |

Note: These check `.contains("eslint-disable")` etc., but ONLY on text extracted from AST comment nodes. This is the correct approach -- you MUST string-match inside comments since comments have no further AST structure.

### `source_scan.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_process_env` | T30 | **tree-sitter AST** via `find_process_env` | CORRECT |
| `check_any_types` | T31 | **tree-sitter AST** via `find_any_types` | CORRECT |
| `check_file_length` | T32 | **STRING MATCHING**: line-by-line filtering `!trimmed.starts_with("//")` etc. | **WRONG** |
| `check_comment_pattern` | T34-T35 | **tree-sitter AST** (extracts comments, then string-matches inside them) | CORRECT |
| `check_banned_in_node_modules` | T59 | **Filesystem probe** | CORRECT |

#### String matching instance:
14. **`check_file_length`** (T32): Counts effective lines by filtering `trimmed.starts_with("//")` and `trimmed.starts_with('*')`.
   - **What it checks:** File length in effective (non-comment, non-blank) lines.
   - **Correct approach:** Use tree-sitter to extract non-comment nodes and count their line spans. The current approach will miscount: multi-line template literals starting with `//` would be excluded, block comments not starting with `*` could be included.
   - **Note:** The Rust equivalent (`filter_non_comment_lines`) has the same issue but is more sophisticated with block comment tracking.

### `eslint_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_eslint_config` | T1 | **Filesystem probe** for existence, then delegates | CORRECT |
| `check_eslint_value_rules` | T2-T5 | **STRING MATCHING** via `eslint_rule_infra::check_eslint_rule` | **WRONG** |
| `check_boundary_enforcement` | T6 | **STRING MATCHING**: `content.contains("boundaries")` on JS config file | **WRONG** |
| `check_eslint_presets` | T-ESLP-13, T-ESLP-14 | **STRING MATCHING**: `content.contains("strictTypeChecked")` etc. | **WRONG** |
| `check_relaxed_rules` | T7 | **STRING MATCHING**: line-by-line `trimmed.contains("\"off\"")` etc. | **WRONG** |
| `check_file_overrides` | T8 | **STRING MATCHING**: `trimmed.contains("files:")` | **WRONG** |
| `check_rule_presence_t40_t48` | T40-T48 | **STRING MATCHING** via `check_eslint_rule_presence` | **WRONG** |
| `check_all_eslint_rules` | T60-T83 | **STRING MATCHING** via `check_eslint_rule_presence` | **WRONG** |
| `check_test_relaxations` | T49 | **STRING MATCHING**: line-by-line | **WRONG** |
| `check_route_wrappers` | T50 | **STRING MATCHING**: `content.contains("withBody")` | **WRONG** |
| `check_process_env_ban` | T51 | **STRING MATCHING**: `content.contains("process.env")` | **WRONG** |

#### String matching instances (ESLint config -- the biggest cluster):
15-25. **All ESLint config checks** (T2-T8, T40-T83, T-ESLP-13/14, T49-T51): The entire `eslint.config.mjs` file is checked using `.contains()` on the raw JavaScript source.
   - **What they check:** Whether ESLint rules are configured in `eslint.config.mjs`.
   - **Correct approach:** Parse the JavaScript config with tree-sitter-javascript, extract the exported config object, and traverse its structure to find rule definitions.
   - **Why it's wrong:** `.contains("no-console")` matches if the string appears in a comment, a variable name, a string literal being used for something else, or anywhere. The file is JavaScript source code, not a flat config file.
   - **KNOWN LIMITATION:** CLAUDE.md explicitly documents this: "ESLint rules checked by pattern matching. guardrail3 greps eslint.config.mjs for rule names. It checks ~35 key rules individually but cannot detect if a rule's configuration (options, severity) was changed -- only presence/absence."

### `eslint_audit.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_zone_definitions` | T36 | **STRING MATCHING**: `content.contains("element-types")` etc. on ESLint JS config | **WRONG** |
| `check_import_direction` | T37 | **STRING MATCHING**: `content.contains("boundaries/element-types")` | **WRONG** |
| `check_entry_point` | T38 | **STRING MATCHING**: `content.contains("boundaries/entry-point")` | **WRONG** |
| `check_external_deps` | T39 | **STRING MATCHING**: `content.contains("boundaries/external")` | **WRONG** |

26-29. Same cluster as above -- ESLint config string matching.

### `eslint_plugin_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_core_plugins` | T-ESLP-01 to T-ESLP-11 | **STRING MATCHING**: `find_missing_rules` uses `.contains()` on JS source | **WRONG** |
| `check_content_plugins` | T-ESLP-07, T-ESLP-08, T-ESLP-12 | **STRING MATCHING**: `.contains()` on JS source | **WRONG** |
| `check_test_relaxations` | T-ESLP-11 | **STRING MATCHING**: line-by-line on JS source | **WRONG** |

30-32. Same ESLint cluster.

### `eslint_rule_infra.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_eslint_rule` | shared | **STRING MATCHING**: `content.contains(rule_name)` | **WRONG** |
| `check_eslint_rule_presence` | shared | **STRING MATCHING**: `content.contains(rule_name)` | **WRONG** |
| `check_rule_value` | shared | **STRING MATCHING**: line-by-line number extraction near rule name | **WRONG** |

33-35. Infrastructure for the ESLint string matching.

### `tsconfig_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_tsconfig` | T9-T10, T52-T68 | **serde_json** parser | CORRECT |

### `npmrc_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_npmrc` | T11-T14, T-NPMRC-01 | **STRING MATCHING**: hand-rolled `key=value` line parser | **WRONG** |

#### String matching instance:
36. **`parse_npmrc_settings`** (T11-T14): Iterates `content.lines()`, splits on `=`, manually extracts key-value pairs.
   - **What it checks:** `.npmrc` settings (key=value INI-like format).
   - **Correct approach:** There is no standard `.npmrc` parser crate. The `.npmrc` format IS a simple `key=value` format with `#`/`;` comments. **This is actually the correct approach** -- `.npmrc` has no formal structured parser, and the format is trivially simple. **Acceptable.**

### `package_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_package_json` | T15-T18, T55-T58, T-PKG-01 to T-PKG-04 | **serde_json** parser | CORRECT |

Note: `preinstall` script check uses `script.contains("only-allow pnpm")` but on a JSON-extracted string value, not raw source. **Acceptable.**

### `package_deps.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| All functions | T-PLUG-*, T-TOOL-01 to T-TOOL-06 | **serde_json** parser | CORRECT |

### `jscpd_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_jscpd` | T19-T22, T-JSCPD-01 to T-JSCPD-04 | **serde_json** parser | CORRECT |
| `check_content_import_restriction` | T60 | **STRING MATCHING**: `content.contains("content/")` on ESLint JS config | **WRONG** |
| `check_velite_config` | T61 | **Filesystem probe** | CORRECT |

37. **`check_content_import_restriction`** (T60): Part of ESLint config string matching cluster.

### `stylelint_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_stylelint` | T-STYL-01 to T-STYL-06 | **STRING MATCHING**: `.contains()` on stylelint config content (JS/JSON/YAML) | **WRONG** |

#### String matching instances:
38. **`check_stylelint`** (T-STYL-01 to T-STYL-06): Uses `content.contains("@double-great/stylelint-a11y")`, `content.contains(rule)` for every a11y rule, `content.contains("stylelint-config-standard")`, etc.
   - **What it checks:** Stylelint config file content (could be JSON, JS, or YAML).
   - **Correct approach:** Detect the format from the file extension. For JSON: `serde_json`. For JS/MJS: tree-sitter-javascript. For YAML: `serde_yaml`. Currently treats all formats as raw text.

### `i18n_check.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_i18n` | T-TOOL-12 | **serde_json** for package.json and locale JSON files | CORRECT |

### `tool_config_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_tool_configs` | T-TOOL-07 to T-TOOL-11 | **serde_json** for package.json | CORRECT |
| `check_cspell_config` | T-TOOL-07 | **Filesystem probe** | CORRECT |

### `test_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_stryker_config` | T-TEST-01 | **Filesystem probe** | CORRECT |
| `check_test_files_exist` | T-TEST-02 | **Filesystem probe** | CORRECT |
| `check_test_runner_config` | T-TEST-03 | **Filesystem probe** | CORRECT |
| `check_skip_without_reason_content` | T-TEST-04 | **tree-sitter AST** via `find_test_method_calls`; reason check is `.contains("// reason")` on extracted line text | CORRECT (hybrid like Rust allow checks) |
| `check_only_in_source_content` | T-TEST-05 | **tree-sitter AST** via `find_test_method_calls` | CORRECT |

### `ts_arch_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_hex_arch_structure` / `check_single_app_structure` | T-ARCH-01 | **Filesystem probe** | CORRECT |
| `check_import_boundaries` / `check_file_imports` | T-ARCH-02 | **STRING MATCHING**: line-by-line import extraction via `extract_import_path` | **WRONG** |

#### String matching instance:
39. **`check_file_imports` / `extract_import_path`** (T-ARCH-02): Iterates `content.lines()`, skips lines starting with `//`/`*`/`/*`, extracts import paths via `line.find("from '")` and similar patterns.
   - **What it checks:** TypeScript import statements for architectural boundary violations.
   - **Correct approach:** Use tree-sitter to parse the file, extract `import_statement` nodes, and get their `source` child (the import path string).
   - **Why it's wrong:** The comment-skipping heuristic misses multi-line comments, and the `from '...'` extraction can match inside string literals or template expressions. Dynamic imports (`await import(...)`) are also missed.

---

## HOOKS CHECKS (`hooks/`)

### `hook_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_hooks` | orchestrator | Delegates | CORRECT |
| `check_pre_commit_exists` | H1 | **Filesystem probe** | CORRECT |
| `check_hooks_path` | H2 | **Tool output** (`git config`) | CORRECT |
| `check_permissions` | H7 | **Filesystem metadata** | CORRECT |

### `hook_script_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_dispatcher_pattern` | H4 | **STRING MATCHING**: `.contains("pre-commit.d")` on shell script | **WRONG** |
| `check_monolithic_patterns` | H5 | **STRING MATCHING**: `.contains()` for each pattern on shell script | **WRONG** |
| `check_modular_scripts` | H5 | **STRING MATCHING** (same as monolithic, concatenated) | **WRONG** |
| `check_stylelint_hook` | H-CSS-01 | **STRING MATCHING**: `.contains("stylelint")` on shell script | **WRONG** |
| `check_cspell_hook` | H-TOOL-01 | **STRING MATCHING**: `.contains("cspell")` on shell script | **WRONG** |
| `check_conflict_marker_hook` | H-TOOL-02 | **STRING MATCHING**: `.contains("<<<")` etc. on shell script | **WRONG** |
| `check_lockfile_hook` | H-TOOL-03 | **STRING MATCHING**: `.contains("frozen-lockfile")` on shell script | **WRONG** |
| `check_prettier_hook` | H-TOOL-04 | **STRING MATCHING**: `.contains("prettier") && .contains("--check")` on shell script | **WRONG** |
| `check_audit_hook` | H-TOOL-05 | **STRING MATCHING**: `.contains("pnpm audit")` on shell script | **WRONG** |
| `check_set_e_safety` | H-SAFE-01 | **STRING MATCHING**: `.contains("set -e")` on shell script | **WRONG** |
| `emit_script_stats` | H6 | **Filesystem metadata** + line count | CORRECT |
| `inventory_scripts` | H9, H11 | **Filesystem listing** | CORRECT |

#### String matching instances:
40. **All hook script pattern checks** (H4, H5, H-CSS-01, H-TOOL-01 to H-TOOL-05, H-SAFE-01): All use `.contains()` on raw shell script content.
   - **What they check:** Whether shell scripts contain specific tool invocations.
   - **Correct approach:** Parse shell with tree-sitter-bash, extract command nodes, check command names.
   - **Pragmatic note:** Shell scripts are notoriously hard to parse formally. `.contains("cargo test")` on a shell script is actually a reasonable heuristic -- false positives are unlikely since these patterns are highly specific command strings. A shell parser would be more robust but the cost/benefit is questionable.

### `tool_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_duplication_tools` | H12 | **STRING MATCHING**: `.contains("cargo dupes")` etc. on shell script | **WRONG** |
| `check_required_tools` | H8 | **Tool checker** | CORRECT |

41. Part of the shell script pattern matching cluster.

### `deploy_checks.rs`

| Function | Check IDs | Method | Verdict |
|---|---|---|---|
| `check_deployment` | orchestrator | Delegates | CORRECT |
| `check_railpack_provider` | D2 | **serde_json** | CORRECT |
| `check_nextjs_configs` | D3-D4 | **STRING MATCHING**: `content.contains("standalone")` and `content.contains("outputFileTracingRoot")` on JS config | **WRONG** |
| `check_tailwind_deps` | D5 | **serde_json** for package.json | CORRECT |

#### String matching instances:
42. **`check_nextjs_configs`** (D3-D4): `content.contains("standalone")` and `content.contains("outputFileTracingRoot")` on `next.config.mjs`/`.js`/`.ts` files.
   - **What it checks:** Whether Next.js config has standalone output and file tracing root.
   - **Correct approach:** Parse the JS/TS config with tree-sitter, extract the config object properties.
   - **Why it's wrong:** `"standalone"` could appear in a comment or unrelated context.

---

## DEFINITIVE LIST OF STRING-MATCHING VIOLATIONS

Grouped by severity/category:

### Category A: Source code / config files with available parsers (TRUE VIOLATIONS)

| # | File | Function | Check ID(s) | Target | Fix |
|---|---|---|---|---|---|
| 1 | `garde_checks.rs` | `content_has_garde_dependency` | R-GARDE-01 | Cargo.toml (Rust source code) | Use `toml` crate |
| 2 | `test_checks.rs` | `has_mutants_profile` | R-TEST-03 | Cargo.toml | Use `toml` crate |
| 3 | `eslint_check.rs` | ALL rule checks | T2-T8, T40-T83 | `eslint.config.mjs` (JavaScript) | tree-sitter-javascript |
| 4 | `eslint_audit.rs` | ALL checks | T36-T39 | `eslint.config.mjs` (JavaScript) | tree-sitter-javascript |
| 5 | `eslint_plugin_checks.rs` | ALL checks | T-ESLP-01 to T-ESLP-12 | `eslint.config.mjs` (JavaScript) | tree-sitter-javascript |
| 6 | `eslint_rule_infra.rs` | ALL functions | shared infra | `eslint.config.mjs` (JavaScript) | tree-sitter-javascript |
| 7 | `stylelint_check.rs` | ALL checks | T-STYL-01 to T-STYL-06 | stylelint config (JS/JSON/YAML) | serde_json/serde_yaml/tree-sitter |
| 8 | `ts_arch_checks.rs` | `check_file_imports` | T-ARCH-02 | TypeScript source files | tree-sitter-typescript |
| 9 | `source_scan.rs` (TS) | `check_file_length` | T32 | TypeScript source files | tree-sitter for comment exclusion |
| 10 | `deploy_checks.rs` | `check_nextjs_configs` | D3-D4 | `next.config.mjs`/`.js`/`.ts` | tree-sitter-javascript/typescript |
| 11 | `release_bin_checks.rs` | workflow checks | R-BIN-01, R-BIN-02 | GitHub Actions YAML | serde_yaml |
| 12 | `release_repo_checks.rs` | `check_workflow_contains` | R-REL-05 to R-REL-07 | GitHub Actions YAML | serde_yaml |

### Category B: Shell scripts (no practical parser, string matching is acceptable)

| # | File | Function | Check ID(s) | Target | Verdict |
|---|---|---|---|---|---|
| 13 | `hook_script_checks.rs` | ALL pattern checks | H4, H5, H-CSS-01, H-TOOL-01 to H-TOOL-05, H-SAFE-01 | Shell scripts | **Acceptable** |
| 14 | `tool_checks.rs` | `check_duplication_tools` | H12 | Shell scripts | **Acceptable** |
| 15 | `test_quality_checks.rs` | `check_mutation_hook` | R-TEST-08 | Shell scripts + .claude/ configs | **Acceptable** |

### Category C: Format has no structured parser (string matching is the only option)

| # | File | Function | Check ID(s) | Target | Verdict |
|---|---|---|---|---|---|
| 16 | `allow_checks.rs` | `check_exception_comments` | R36 | TOML comments | **Correct** (TOML parsers strip comments) |
| 17 | `npmrc_check.rs` | `parse_npmrc_settings` | T11-T14 | `.npmrc` (INI-like) | **Correct** (no parser available, format is trivial) |

### Category D: Hybrid AST + string (string part is on comment text or parsed values)

| # | File | Function | Check ID(s) | Target | Verdict |
|---|---|---|---|---|---|
| 18 | `allow_checks.rs` | `check_item_level_allow_ast` | R32-R33 | Comment presence on AST-detected line | **Correct** |
| 19 | `allow_checks.rs` | `check_garde_skip_ast` | R34-R35 | Comment presence on AST-detected line | **Correct** |
| 20 | `release_crate_checks.rs` | `check_readme_quality` | R-PUB-05 | Markdown heading convention | **Correct** |

---

## PRIORITY FIX LIST (Category A only -- true violations)

1. **HIGHEST: ESLint config checks (T2-T83, T-ESLP-*, T36-T39)** -- ~35 check functions, all using `.contains()` on JavaScript source. This is the largest cluster and the most fragile. Fix: tree-sitter-javascript parser.

2. **HIGH: TS import boundary checks (T-ARCH-02)** -- String-based import extraction on TypeScript source. Fix: tree-sitter-typescript.

3. **MEDIUM: TS file length (T32)** -- Simple line filter misses block comments. Fix: tree-sitter comment exclusion.

4. **MEDIUM: Stylelint config (T-STYL-01 to T-STYL-06)** -- `.contains()` on multi-format config. Fix: detect format, use appropriate parser.

5. **MEDIUM: Next.js config (D3-D4)** -- `.contains()` on JS config. Fix: tree-sitter-javascript.

6. **LOW: GitHub Actions YAML (R-BIN-01, R-BIN-02, R-REL-05 to R-REL-07)** -- `.contains()` on YAML. Fix: serde_yaml.

7. **LOW: Cargo.toml string parsing (R-GARDE-01, R-TEST-03)** -- Two small functions using line-by-line instead of `toml` crate. Fix: use already-available `toml` crate.
