# guardrail3 — Agent Instructions

> **Agent-managed codebase.** Do not say "you can edit X" — just do it. Never estimate workload — just execute.

## Philosophy

guardrail3 exists because **agents break things**. Code must be resilient to agents making mistakes. The solution: machine-enforced guardrails that agents literally cannot bypass without the violation being detected and reported.

**What guardrail3 IS and IS NOT:**

guardrail3 is NOT a linter. It does NOT re-implement clippy, rustc, ESLint, or any existing tool. Those tools already catch code-level violations (unused variables, unwrap calls, unsafe blocks, etc.).

guardrail3 is a **configuration and architecture enforcer**. It ensures that:
1. The right linter rules ARE configured (clippy lints, ESLint rules, deny.toml bans)
2. The project structure follows hex arch (domain/ports/app/adapters)
3. Every escape hatch is documented (every `#[allow]` has a reason)
4. Dependency boundaries are respected (libraries can't use I/O crates)
5. Input validation exists at every trust boundary (garde/Validate on all input structs)

**The division of labor:**
- **Clippy/rustc** catch code violations (unwrap, unsafe, todo, etc.) at compile time
- **R26** verifies that those clippy lints are correctly configured in `[workspace.lints]`
- **R4-R7** verify that clippy.toml has the right method/type bans
- **guardrail3 does NOT scan source for what clippy already catches** — that would be redundant

The only source scan checks guardrail3 performs are for things NO existing tool catches:
- **R30-R36**: Suppression hygiene (`#[allow]` without reason, garde skip, EXCEPTION comments)
- **R38, R40-R41**: Structural health (file length, import count)
- **R58**: The clippy aliased-import hole (`use std::fs; fs::read()` bypasses clippy bans)
- **R-TEST-09**: Test code organization (inline tests vs separate files)
- **R-ARCH-01/02/03/04**: Hex arch structure and dependency flow
- **R-DEPS-01/02**: Dependency allowlists

**Core principles:**

1. **Least privilege.** Ban everything by default. Allow only what's explicitly needed, with a justification comment. If a method, type, or crate isn't on the allow-list, it's banned.

2. **Self-validating.** guardrail3 enforces the same rules on itself that it enforces on other projects. If it can't pass its own validation, it has no business validating others.

3. **Total visibility.** Every suppression (`#[allow]`, `#[garde(skip)]`, `eslint-disable`, EXCEPTION comments) is reported as an audit trail. Use `--verbose` to see each individually, `--inventory` to see passing confirmations.

4. **Modular by language.** Rust guardrails and TypeScript guardrails are completely independent. Each has its own `init`, `generate`, `validate` commands. For monorepos, run both.

5. **Every escape hatch is documented.** Every `#[allow(...)]` must have a `// reason:` comment on the same line. Every `#[garde(skip)]` on a non-primitive field is an error — use a real validator. Every config relaxation must have `// EXCEPTION: reason`.

6. **Enforce configuration, not violations.** guardrail3 checks that clippy is configured to catch unwrap. Clippy catches the actual unwrap at compile time. guardrail3 checks that deny.toml bans dangerous crates. cargo-deny enforces the ban. guardrail3 is the meta-tool that ensures all other tools are properly configured.

7. **AST-based scanning only.** All source scan checks use syn (Rust) or tree-sitter (TypeScript) for AST parsing. Zero grep, zero line matching. This eliminates false positives from strings, comments, and macros.

8. **Tests prove guardrails work.** Every new check has adversarial test fixtures that try to break it. Every bug fix has a regression test.

## What This Tool Does

Single Rust binary (`cargo install guardrail3`) that:
- **Validates** any project against a canonical guardrail rule set (config-free, auto-detects stacks)
- **Generates** config files (clippy.toml, deny.toml, rustfmt.toml, pre-commit hooks) from composable modules
- **Reports** in text, JSON, or markdown format
- **Self-validates** — eats its own dogfood

## Commands

| Command | Config needed? | What it does |
|---|---|---|
| `guardrail3 validate [path]` | No | Auto-detect stacks, run all checks |
| `guardrail3 rs validate [path]` | No | Rust checks only |
| `guardrail3 ts validate [path]` | No | TypeScript checks only |
| `guardrail3 hooks validate [path]` | No | Hook + deployment checks |
| `guardrail3 rs init --profile <name>` | No (creates it) | Scaffold Rust guardrail3.toml + local/ |
| `guardrail3 ts init` | No (creates it) | Scaffold TypeScript section in guardrail3.toml |
| `guardrail3 generate` | Yes | Produce config files from modules + profile |
| `guardrail3 check` | Yes | CI: verify generated files are current |
| `guardrail3 diff` | Yes | Dry run of generate with diffs |
| `guardrail3 list-modules` | No | List all embedded modules |
| `guardrail3 show-module <name>` | No | Print module content |
| `guardrail3 rs generate` | Yes | Rust configs only |
| `guardrail3 ts generate` | Yes | TS configs only |
| `guardrail3 hooks install` | Yes | Install pre-commit hook |

### Scope flags (for validate)

```
--staged           only staged files
--dirty            staged + unstaged
--commits N        files changed in last N commits
--files a.rs b.rs  explicit file list
--format text|json|md
```

## Profiles

| Profile | What it does |
|---|---|
| `service` | Full guardrails for Axum/tokio HTTP services. All clippy bans, all deny bans, tokio feature gating. |
| `library` | Same as service + ban ALL I/O crates (axum, tokio, reqwest, sqlx, etc.) + global-state bans on every crate (no composition root exception). |

For monorepos, run `guardrail3 rs init` and `guardrail3 ts init` separately — there is no dedicated monorepo profile.

The `minimal` profile was removed. If you need fewer bans, use `local/` overrides to add exceptions — don't weaken the baseline.

## Architecture

```
src/
  main.rs                        — CLI entry point (clap)
  cli.rs                         — command definitions
  fs.rs                          — CENTRALIZED filesystem module (the ONE place std::fs is allowed)
  discover.rs                    — auto-detect Rust/TS stacks, workspace members
  config/                        — guardrail3.toml parser + types
  modules/                       — embedded guardrail content
    clippy.rs                    — 6 method modules + 4 type modules + generator
    deny.rs                      — 16 deny.toml section/ban modules + generator
    canonical.rs                 — rustfmt, toolchain, cargo-lints, npmrc, tsconfig, jscpd, eslint
    pre_commit.rs                — pre-commit hook builder (profile-aware duplication tool)
  commands/
    validate.rs                  — top-level validate orchestrator + git scope resolution
    generate.rs                  — config file generation from modules + profile
    init.rs                      — scaffold guardrail3.toml + local/
    check.rs                     — staleness verification
    diff.rs                      — dry run with diffs
    modules_cmd.rs               — list-modules, show-module
  rs/validate/                   — Rust validation (57+ checks)
    mod.rs                       — orchestrator
    config_files.rs              — R1-R3, R21-R25 (clippy, rustfmt, toolchain existence + thresholds)
    clippy_coverage.rs           — R4-R7 (ban completeness)
    deny_audit.rs                — R8-R11 (deny.toml structure + advisories)
    deny_bans.rs                 — R12-R13, R17-R18 (ban list + feature bans)
    deny_licenses.rs             — R14-R16 (licenses + sources)
    deny_inventory.rs            — R19-R20 (skip + advisory ignore inventory)
    cargo_lints.rs               — R26-R29 (workspace lint completeness + inheritance)
    source_scan.rs               — orchestrator + comment filter utilities
    allow_checks.rs              — R30-R37 (crate/item allow, garde skip, cfg_attr, exception comments)
    structure_checks.rs          — R38-R42, R53 (file length, use count, unsafe, unsafe_code=forbid)
    code_quality_checks.rs       — R43-R44, R49, R58 (todo/unwrap, CLAUDE.md, direct std::fs usage)
    dependency_direction.rs      — R51-R52 (hex arch dependency graph)
    dependency_scan.rs           — R45-R50 (tool installation, banned crates in lockfile)
    rustfmt_check.rs             — R21-R23
    toolchain_check.rs           — R24-R25
    workspace_metadata.rs        — R55-R57
  ts/validate/                   — TypeScript validation (61+ checks)
    mod.rs                       — orchestrator
    eslint_check.rs              — T1-T8, T36-T51, T60+ (ESLint config, boundaries, rules)
    tsconfig_check.rs            — T9-T10, T52-T54 (tsconfig strict settings)
    npmrc_check.rs               — T11-T14 (pnpm strict settings)
    package_check.rs             — T15-T18, T55-T58 (package.json overrides, banned deps, scripts)
    jscpd_check.rs               — T19-T22, T60-T61 (jscpd config, content imports, velite)
    source_scan.rs               — T23-T35, T59 (eslint-disable, ts-ignore, process.env, any, file length)
  hooks/                         — Hook + deployment validation
    mod.rs, validate.rs          — orchestrator
    hook_checks.rs               — H1-H11 (pre-commit existence, patterns, permissions)
    tool_checks.rs               — H8, H12 (tool installation, duplication tool detection)
    deploy_checks.rs             — D1-D5 (railpack, next.js standalone, tailwind)
  report/
    types.rs                     — CheckResult, Section, Report, Severity
    text.rs                      — colored terminal output
    json.rs                      — JSON output
    markdown.rs                  — markdown tables
```

## Validation Checks (162 total)

### Rust (R1-R58)

**Config completeness:** clippy.toml existence + thresholds (R1-R3), method/type ban coverage (R4-R7), deny.toml structure + bans + licenses + sources (R8-R20), rustfmt + toolchain existence + settings (R21-R25), Cargo.toml workspace lints completeness + inheritance (R26-R29).

**Source scan:** Crate-level `#![allow]` (R30-R31), item-level `#[allow]` without reason (R32-R33), `#[garde(skip)]` without reason (R34-R35), EXCEPTION comments (R36), cfg_attr allow (R37), file length >500 (R38-R39), use count >20 (R40-R41), unsafe (R42), todo/unimplemented (R43), unwrap/expect (R44).

**Tools + deps:** cargo-deny/machete/dupes/gitleaks installed (R45-R48), CLAUDE.md exists (R49), banned crates in Cargo.lock (R50).

**Architecture:** Dependency direction violations (R51), dependency graph inventory (R52), unsafe_code=forbid (R53), workspace metadata (R55-R57), direct std::fs usage (R58).

### TypeScript (T1-T83)

**Config completeness:** ESLint config + rules (T1-T8, T36-T51, T60-T83), tsconfig strict settings (T9-T10, T52-T54), npmrc settings (T11-T14), package.json overrides + banned deps + scripts (T15-T18, T55-T58), jscpd config (T19-T22), content/velite (T60-T61).

**Source scan:** eslint-disable without reason (T23-T26), @ts-ignore (T27), @ts-expect-error (T28-T29), process.env direct access (T30), any type usage (T31), file length >300 (T32-T33), IDE/coverage suppressions (T34-T35), banned packages in node_modules (T59).

### Hooks + Deployment (H1-H12, D1-D5)

**Hooks:** Pre-commit existence + hooksPath (H1-H2), script structure (H3-H7), tool installation (H8), script inventory (H9-H11), duplication tool detection (H12 — warns when wrong tool for project type).

**Deployment:** Railpack configs (D1-D2), Next.js standalone + tracing root (D3-D4), Tailwind in deps not devDeps (D5).

## Guardrails (self-enforced)

This project uses profile `service` with its own guardrail3 config. Run `guardrail3 validate .` to check compliance.

| Guardrail | How enforced |
|---|---|
| All clippy method/type bans | clippy.toml (generated by `guardrail3 generate`) |
| 26 crate bans | deny.toml (generated) |
| 40 clippy lint rules | Cargo.toml `[workspace.lints]` |
| Formatting | rustfmt.toml (generated) + cargo fmt in pre-commit |
| Stable toolchain | rust-toolchain.toml (generated) |
| Centralized filesystem | src/fs.rs + R58 source scan + clippy disallowed_methods |
| No duplicates | cargo-dupes --exclude-tests in pre-commit |
| No secrets | gitleaks in pre-commit |
| No unused deps | cargo-machete in pre-commit |
| All tests pass | cargo test in pre-commit |
| Max 500 lines/file | Structural health in pre-commit |
| `#[allow]` requires reason | Tamper detection in pre-commit |
| Self-validation | `guardrail3 validate .` reports 0 errors |

## Embedded Modules

Modules are the composable units of guardrail content. Each is a const string embedded in the binary.

**Clippy methods (6):** env-vars, env-mutation, process-control, blocking-sleep, filesystem, http-client

**Clippy types (4):** collections (HashMap→BTreeMap), sync (Mutex→parking_lot), filesystem (File), global-state (LazyLock/OnceLock)

**Deny sections (16):** graph, bans-base, 10 ban categories (json, tls, http, logging, async, error, web, datetime, orm, serialization), feature-bans (tokio), licenses, advisories, sources

**Canonical (7):** rustfmt.toml, rust-toolchain.toml, cargo-lints, npmrc, tsconfig-base, jscpd, eslint-starter

**Pre-commit:** Profile-aware builder — cargo-dupes for Rust, jscpd for TS, both for mixed

## Project Config (guardrail3.toml)

Only needed for `generate`/`check`/`diff`. Not needed for `validate`.

```toml
version = "0.1"

[profile]
name = "service"           # service | library

[rust]
workspace_root = "."       # or "apps/backend" for monorepos (run rs init + ts init separately)

[rust.crates.api]
layer = "composition-root" # allows LazyLock (no global-state ban)

[rust.crates.domain-types]
layer = "pure"             # gets global-state ban

[local]
clippy_methods = "local/clippy-methods.toml"
clippy_types = "local/clippy-types.toml"
deny_bans = "local/deny-bans.toml"
deny_skip = "local/deny-skip.toml"
deny_feature_bans = "local/deny-feature-bans.toml"
```

## Centralized Filesystem Module (src/fs.rs)

ALL `std::fs` calls go through `src/fs.rs`. Every other file is banned from direct filesystem access.

**Enforced by:**
1. clippy.toml `disallowed_methods` — bans `std::fs::read_to_string`, `std::fs::read_dir`, `std::fs::write`, etc.
2. R58 source scan — catches `use std::fs` imports that clippy misses due to aliasing

**Ban reason string:** "BANNED: Create a centralized fs module and route all filesystem operations through it — no scattered std::fs calls"

**Functions provided:**
- `read_file(path) -> Option<String>`
- `read_file_err(path) -> Result<String, io::Error>`
- `list_dir(path) -> Vec<DirEntry>`
- `metadata(path) -> Option<Metadata>`
- `write_file(path, content) -> Result<(), io::Error>`
- `create_dir_all(path) -> Result<(), io::Error>`
- `set_permissions(path, perms) -> Result<(), io::Error>`

## Pre-commit Hook

The generated hook is profile-aware. For a service profile:

1. Secret scanning (gitleaks) — hard fail if not installed
2. File size check (1MB limit)
3. Guardrail tamper detection (allow without reason, config relaxation)
4. cargo fmt --check
5. cargo clippy -D warnings
6. cargo-deny check — hard fail if not installed
7. Structural health (500 lines, 20 uses, no crate-wide allow)
8. cargo-machete — hard fail if not installed
9. cargo test --workspace
10. cargo-dupes --exclude-tests (Rust) / jscpd (TypeScript)

The duplication tool is language-specific:
- Rust-only → cargo-dupes (AST-aware, no Node.js dependency)
- TS-only → jscpd
- Mixed → both

## Planned: Garde Boundary Validation

The Garde pattern (from `docs/GARDE_GUARDRAILS.md` in the template) enforces validation at adapter boundaries using clippy bans + trait bounds. NOT YET INTEGRATED into guardrail3's modules or validation. Planned additions:

**Clippy method bans:** `serde_json::from_str/slice/value/reader`, `reqwest::Response::json`, `toml::from_str`, `serde_yaml::from_str` — force `Validated<T>::new()` wrapper

**Clippy type bans:** `axum::extract::Json`, `axum::Json`, `axum::extract::Query`, `axum::extract::Form` — force `ValidatedJson<T>`, `ValidatedQuery<T>`, etc.

**Pre-commit addition:** `#[garde(skip)]` without reason comment check (already implemented in guardrail3's source scan as R34/R35)

## Known Limitations

1. **clippy disallowed_methods has a hole.** `use std::fs; fs::read_to_string()` is NOT caught by clippy even though `std::fs::read_to_string` is in the disallowed list. R58 exists as belt-and-suspenders.

2. **ESLint rules checked by pattern matching.** guardrail3 greps `eslint.config.mjs` for rule names. It checks ~35 key rules individually but cannot detect if a rule's configuration (options, severity) was changed — only presence/absence.

3. **walkdir bypasses fs.rs.** The `walkdir` crate does directory traversal outside the centralized fs module. clippy.toml only bans `std::fs`, not third-party crate I/O. This is a design gap.

4. **Pre-commit hook uses guardrail3 validate.** The hook runs `guardrail3 rs validate --staged` and `guardrail3 ts validate --staged` for AST-based tamper detection instead of grep. This eliminates false positives from string literals containing patterns like `#[allow(`.

## Adding a New Check

1. Add the check function to the appropriate validate module (rs/ts/hooks)
2. Return `Vec<CheckResult>` with correct ID, severity, title, message
3. Wire it into the module's `check()` orchestrator
4. Keep files under 500 lines — split into sub-modules if needed
5. Add a regression test in the same file's `#[cfg(test)] mod tests`
6. Mutation-test the regression test (reintroduce bug, verify test fails)
7. Use per-function `#[allow(...)] // reason:` in tests, NOT module-level `#![allow]`

## Adding a New Module

1. Add const `Module` in `src/modules/clippy.rs` or `src/modules/deny.rs`
2. Register in the profile functions (`service_profile_methods()`, `library_profile_types()`, etc.)
3. Register in `src/modules/mod.rs` `all_modules()` function
4. If it adds new expected bans, update `EXPECTED_METHOD_BANS` / `EXPECTED_TYPE_BANS` / `EXPECTED_BANS` in the validation code
5. Run `guardrail3 generate` to regenerate own config files
6. Run `guardrail3 validate .` to verify self-compliance

## Don'ts

1. Don't use `.unwrap()` — use `?`, `if let`, or `match`
2. Don't use `unsafe` — forbidden at workspace level
3. Don't use `HashMap`/`HashSet` — use `BTreeMap`/`BTreeSet`
4. Don't use `std::fs` directly — use `crate::fs::*` (src/fs.rs)
5. Don't create files over 500 effective lines — split into modules
6. Don't add `#[allow]` without a `// reason:` comment on the SAME LINE
7. Don't add `#![allow]` (module/crate-wide) — only per-function `#[allow]`
8. Don't relax guardrail configs without `// EXCEPTION: reason`
9. Don't use jscpd for Rust — use cargo-dupes (AST-aware)
10. Don't fix a bug before the guardrail catches it — add the check first, verify it flags the issue, then fix
11. Don't write tests that pass regardless of the bug — mutation-test them
12. Don't use `#[cfg(test)] #![allow(...)]` — pre-commit hook can't distinguish from crate-wide allows

## Session Cold-Start Reading List

If starting a new session with no context, read these files in order:
1. This file (CLAUDE.md)
2. `guardrail3.toml` — current profile and config
3. `src/modules/clippy.rs` — what bans exist and how they compose
4. `src/modules/deny.rs` — what crate bans exist
5. `src/rs/validate/mod.rs` — how Rust validation is orchestrated
6. `src/commands/validate.rs` — how the top-level validate works
7. `.worklogs/` — recent worklogs for decision context

<!-- gitnexus:start -->
# GitNexus — Code Intelligence

This project is indexed by GitNexus as **guardrail3** (475 symbols, 1155 relationships, 34 execution flows). Use the GitNexus MCP tools to understand code, assess impact, and navigate safely.

> If any GitNexus tool warns the index is stale, run `npx gitnexus analyze` in terminal first.

## Always Do

- **MUST run impact analysis before editing any symbol.** Before modifying a function, class, or method, run `gitnexus_impact({target: "symbolName", direction: "upstream"})` and report the blast radius (direct callers, affected processes, risk level) to the user.
- **MUST run `gitnexus_detect_changes()` before committing** to verify your changes only affect expected symbols and execution flows.
- **MUST warn the user** if impact analysis returns HIGH or CRITICAL risk before proceeding with edits.
- When exploring unfamiliar code, use `gitnexus_query({query: "concept"})` to find execution flows instead of grepping. It returns process-grouped results ranked by relevance.
- When you need full context on a specific symbol — callers, callees, which execution flows it participates in — use `gitnexus_context({name: "symbolName"})`.

## When Debugging

1. `gitnexus_query({query: "<error or symptom>"})` — find execution flows related to the issue
2. `gitnexus_context({name: "<suspect function>"})` — see all callers, callees, and process participation
3. `READ gitnexus://repo/guardrail3/process/{processName}` — trace the full execution flow step by step
4. For regressions: `gitnexus_detect_changes({scope: "compare", base_ref: "main"})` — see what your branch changed

## When Refactoring

- **Renaming**: MUST use `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` first. Review the preview — graph edits are safe, text_search edits need manual review. Then run with `dry_run: false`.
- **Extracting/Splitting**: MUST run `gitnexus_context({name: "target"})` to see all incoming/outgoing refs, then `gitnexus_impact({target: "target", direction: "upstream"})` to find all external callers before moving code.
- After any refactor: run `gitnexus_detect_changes({scope: "all"})` to verify only expected files changed.

## Never Do

- NEVER edit a function, class, or method without first running `gitnexus_impact` on it.
- NEVER ignore HIGH or CRITICAL risk warnings from impact analysis.
- NEVER rename symbols with find-and-replace — use `gitnexus_rename` which understands the call graph.
- NEVER commit changes without running `gitnexus_detect_changes()` to check affected scope.

## Tools Quick Reference

| Tool | When to use | Command |
|------|-------------|---------|
| `query` | Find code by concept | `gitnexus_query({query: "auth validation"})` |
| `context` | 360-degree view of one symbol | `gitnexus_context({name: "validateUser"})` |
| `impact` | Blast radius before editing | `gitnexus_impact({target: "X", direction: "upstream"})` |
| `detect_changes` | Pre-commit scope check | `gitnexus_detect_changes({scope: "staged"})` |
| `rename` | Safe multi-file rename | `gitnexus_rename({symbol_name: "old", new_name: "new", dry_run: true})` |
| `cypher` | Custom graph queries | `gitnexus_cypher({query: "MATCH ..."})` |

## Impact Risk Levels

| Depth | Meaning | Action |
|-------|---------|--------|
| d=1 | WILL BREAK — direct callers/importers | MUST update these |
| d=2 | LIKELY AFFECTED — indirect deps | Should test |
| d=3 | MAY NEED TESTING — transitive | Test if critical path |

## Resources

| Resource | Use for |
|----------|---------|
| `gitnexus://repo/guardrail3/context` | Codebase overview, check index freshness |
| `gitnexus://repo/guardrail3/clusters` | All functional areas |
| `gitnexus://repo/guardrail3/processes` | All execution flows |
| `gitnexus://repo/guardrail3/process/{name}` | Step-by-step execution trace |

## Self-Check Before Finishing

Before completing any code modification task, verify:
1. `gitnexus_impact` was run for all modified symbols
2. No HIGH/CRITICAL risk warnings were ignored
3. `gitnexus_detect_changes()` confirms changes match expected scope
4. All d=1 (WILL BREAK) dependents were updated

## CLI

- Re-index: `npx gitnexus analyze`
- Check freshness: `npx gitnexus status`
- Generate docs: `npx gitnexus wiki`

<!-- gitnexus:end -->
