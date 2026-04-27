# Code Fixes — Implementation bugs to fix during migration

These are NOT new rules. They are bugs in existing rule implementations that need fixing when we migrate to the new checker architecture.

## RS-RELEASE

| Location | Bug | Fix |
|----------|-----|-----|
| `workspace_metadata.rs` | R56/R57 IDs never renamed to RS-RELEASE-09/10 | Renumber during migration |
| `release_checks.rs` `discover_crates` | `.exists()` on raw paths bypasses FileSystem trait | Use `fs.metadata()` |
| `release_crate_deps.rs` `check_path_deps` | Only scans `[dependencies]` and `[build-dependencies]`, misses `[target.*.dependencies]` | Iterate target-conditional dep sections too |
| `release_repo_checks.rs` | Workflow checks use `.contains()` on raw YAML | Pragmatic but violates structured parsing. Accept for now, note for future YAML parser. |
| `release_crate_deps.rs` `check_publish_dry_run` | Checks stderr for "error" substring instead of exit code | Check process exit code first, stderr as fallback. |
| `release_repo_checks.rs` | Workflow files re-read from disk 3+2N times | Cache workflow file contents in a Vec, iterate the cache. |
| `release_crate_checks.rs` | `readme = false` (Cargo opt-out) falls through to default README.md check | Handle `readme = false` explicitly — skip README check for that crate. |

## RS-DEPS

| Location | Bug | Fix |
|----------|-----|-----|
| `dependency_scan.rs:48` | gitleaks install message says `cargo install gitleaks` but gitleaks is a Go binary | Fix to `brew install gitleaks` or platform-appropriate message |
| `dependency_scan.rs` | Still emits old R45-R48 IDs | Renumber to RS-DEPS-01..04 during migration |
| `dependency_allowlist.rs:87-89` | `workspace = true` deps skip allowlist entirely — any crate in [workspace.dependencies] bypasses the check | Resolve workspace deps before allowlist check (same pattern as RS-HEXARCH-10) |

## RS-TOOLCHAIN

| Location | Bug | Fix |
|----------|-----|-----|
| `toolchain_check.rs` | `nightly-YYYY-MM-DD` falls into `Some(other)` catch-all, gets Info ("pinned version") instead of Error. Pinned nightlies are still nightly. | Match `starts_with("nightly")` before the catch-all. |
| `toolchain_check.rs` | `channel = "beta"` same catch-all, gets Info instead of Warn. Beta is pre-release. | Add explicit `"beta"` arm with Warn severity. |
| `toolchain_check.rs` | All sub-checks share ID "R25" — JSON filtering ambiguous | Split into distinct sub-IDs during g3rs-toolchain/channel-and-components migration. |

## RS-FMT

| Location | Bug | Fix |
|----------|-----|-----|
| `rustfmt_check.rs` | R22 parse/read errors are Warn, should be Error | Escalate severity |
| `rustfmt_check.rs` | `check_rustfmt_str` emits no result on success (inconsistent with int/bool checks) | Emit Info inventory on success |
| `rustfmt_check.rs` | Expected rustfmt values hardcoded, not linked to canonical module | Link to canonical RUSTFMT module content |
| `rustfmt_check.rs` | Extra settings check only works for top-level keys (nested tables missed) | Walk full TOML tree |
| `rustfmt_check.rs` | Wrong-value message has awkward double-quoting | Fix format string |
| `rustfmt_check.rs` | `check_rustfmt_str` separate code path from int/bool (dead code risk) | Unify check functions |
| `config_files.rs` | `find_root_config` may pick `.rustfmt.toml` when `rustfmt.toml` also exists | Prefer `rustfmt.toml` over `.rustfmt.toml` explicitly |

## RS-CLIPPY

| Location | Bug | Fix |
|----------|-----|-----|
| `clippy_coverage.rs` | `_profile` parameter unused — library profile uses same expected bans as service | Thread profile through, extend expected bans for library |

## RS-CARGO

| Location | Bug | Fix |
|----------|-----|-----|
| `cargo_lints.rs` line 313 | `crate_cargo.exists()` bypasses FileSystem trait | Use `fs.metadata()`. Also emit Warn when declared member has no Cargo.toml. |
| `workspace_metadata.rs` wiring | g3rs-cargo/priority-order called from config_files.rs, not cargo_lints.rs | Move call into cargo_lints module |
| Canonical lint drift | EXPECTED_* arrays can drift from canonical.rs CARGO_LINTS module | Add `#[test]` for consistency |

## RS-GARDE

| Location | Bug | Fix |
|----------|-----|-----|
| `garde_checks.rs` lines 148-171 | `content_has_garde_dependency()` uses line-by-line parsing instead of TOML parser | Use `toml::Value` parsing |
| `ast_visitors.rs` SKIP_OK_TYPES | `char` missing from primitive skip list | Add `"char"` |
| `ast_visitors.rs` DeriveVisitor | Enum handling defaults `has_non_primitive_fields = true` for all enums | Add `enum_has_non_primitive_fields()` for C-like enum false positive fix |

## RS-TEST

| Location | Bug | Fix |
|----------|-----|-----|
| `test_checks.rs` lines 131-139 | `has_mutants_profile()` uses line-by-line parsing instead of TOML parser | Use `toml::Value` parsing |
| `test_quality_checks.rs` lines 257-281 | Mutation hook check uses `.contains("mutant")` on raw file content. Matches comments. | Check executable lines only for hooks |
| `test_checks.rs` line 233 | `path_str.contains("/src/")` for path filtering | Use `contains_segment()` |
| ast_helpers `IgnoreVisitor` | RS-TEST-07 accepts ANY `//` comment as reason, not `// reason:` prefix | Require `// reason:` prefix — same fix needed for R32-R33 (RS-SOURCE-03) |
| `test_checks.rs` RS-TEST-09 | Zero targeted tests for inline test detection | Add adversarial test fixtures |

## RS-SOURCE

| Location | Bug | Fix |
|----------|-----|-----|
| `code_quality_checks.rs` line 49 | `panic!` macro detected by AST but silently dropped by `_ => {}` catch-all | Add `"panic"` match arm (becomes RS-SOURCE-16) |
| `ast_helpers.rs` `item_attrs` | Returns `&[]` for `ForeignMod` — `#[allow]` on extern blocks invisible | Add `ForeignMod(f) => &f.attrs` |
| `ast_helpers.rs` `is_cfg_attr_always_true` | Only detects `all()` with empty args | Expand heuristics for `any(unix, windows)`, `not(nonexistent)` |

## RS-DENY

| Location | Bug | Fix |
|----------|-----|-----|
| `deny_audit.rs` RS-DENY-09 | Registry URL check doesn't accept sparse protocol URL | Accept `sparse+https://index.crates.io/` |
| `deny_inventory.rs` g3rs-deny/wildcards-inventory | Malformed skip entries silently report "unknown" | Warn on entries missing both `crate` and `name` fields |
| `deny_inventory.rs` g3rs-deny/wildcards-inventory | `reason` field present but not a string treated as empty | Warn on wrong type |
| `deny_inventory.rs` g3rs-deny/license-allow-baseline | Missing `.as_inventory()` on advisory ignore Info entries | Add for `--inventory` consistency |
