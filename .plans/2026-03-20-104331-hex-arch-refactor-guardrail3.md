# Refactor guardrail3 into hex arch crates

**Date:** 2026-03-20 10:43
**Task:** Split guardrail3 from a single flat crate into proper hex arch crates under `crates/`

## Goal
Transform the current flat `crates/` (renamed from `src/`) into proper hex arch layout where each module group is its own crate with Cargo.toml, clear dependencies, and enforced boundaries.

## Current state
Single crate with `lib.rs` + `main.rs` at `crates/` root. Module directories (`domain/`, `ports/`, `app/`, `adapters/`, `commands/`, `report/`) are Rust modules, not separate crates. Everything uses `use crate::` imports.

## Target layout

```
apps/guardrail3/
├── Cargo.toml                              (workspace manifest, lists all crates as members)
├── crates/
│   ├── domain/
│   │   ├── config/                         (crate: guardrail3-domain-config)
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── mod.rs → types.rs       (guardrail3.toml config types)
│   │   ├── modules/                        (crate: guardrail3-domain-modules)
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── canonical.rs
│   │   │       ├── clippy.rs
│   │   │       ├── cspell.rs
│   │   │       ├── deny.rs
│   │   │       ├── eslint.rs
│   │   │       ├── guide.rs
│   │   │       ├── pre_commit.rs
│   │   │       ├── release.rs
│   │   │       └── stylelint.rs
│   │   └── report/                         (crate: guardrail3-domain-report)
│   │       ├── Cargo.toml
│   │       └── src/
│   │           └── lib.rs                  (CheckResult, Section, Severity, Report, categories)
│   │
│   ├── ports/
│   │   ├── inbound/
│   │   │   └── .gitkeep
│   │   └── outbound/
│   │       └── traits/                     (crate: guardrail3-ports-outbound-traits)
│   │           ├── Cargo.toml
│   │           └── src/
│   │               └── lib.rs              (FileSystem, ToolChecker traits)
│   │
│   ├── app/
│   │   ├── core/                           (crate: guardrail3-app-core)
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── crawl.rs
│   │   │       ├── discover.rs
│   │   │       ├── gitignore.rs
│   │   │       └── project_map.rs
│   │   ├── rs-validate/                    (crate: guardrail3-app-rs-validate)
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── allow_checks.rs
│   │   │       ├── ast_helpers.rs
│   │   │       ├── ast_visitors.rs
│   │   │       ├── cargo_lints.rs
│   │   │       ├── clippy_coverage.rs
│   │   │       ├── code_quality_checks.rs
│   │   │       ├── config_files.rs
│   │   │       ├── deny_audit.rs
│   │   │       ├── deny_bans.rs
│   │   │       ├── deny_inventory.rs
│   │   │       ├── deny_licenses.rs
│   │   │       ├── dependency_allowlist.rs
│   │   │       ├── dependency_scan.rs
│   │   │       ├── extra_visitors.rs
│   │   │       ├── garde_checks.rs
│   │   │       ├── hex_arch_checks.rs
│   │   │       ├── hex_arch_structure.rs
│   │   │       ├── release_bin_checks.rs
│   │   │       ├── release_checks.rs
│   │   │       ├── release_crate_checks.rs
│   │   │       ├── release_crate_deps.rs
│   │   │       ├── release_repo_checks.rs
│   │   │       ├── rustfmt_check.rs
│   │   │       ├── source_scan.rs
│   │   │       ├── structure_checks.rs
│   │   │       ├── test_checks.rs
│   │   │       ├── test_quality_checks.rs
│   │   │       ├── toolchain_check.rs
│   │   │       └── workspace_metadata.rs
│   │   ├── ts-validate/                    (crate: guardrail3-app-ts-validate)
│   │   │   ├── Cargo.toml
│   │   │   └── src/
│   │   │       ├── lib.rs
│   │   │       ├── ast_helpers.rs
│   │   │       ├── config_files.rs
│   │   │       ├── eslint_audit.rs
│   │   │       ├── eslint_check.rs
│   │   │       ├── eslint_parser.rs
│   │   │       ├── eslint_plugin_checks.rs
│   │   │       ├── eslint_rule_infra.rs
│   │   │       ├── i18n_check.rs
│   │   │       ├── jscpd_check.rs
│   │   │       ├── npmrc_check.rs
│   │   │       ├── package_check.rs
│   │   │       ├── package_deps.rs
│   │   │       ├── source_scan.rs
│   │   │       ├── stylelint_check.rs
│   │   │       ├── test_checks.rs
│   │   │       ├── tool_config_checks.rs
│   │   │       ├── ts_arch_checks.rs
│   │   │       ├── ts_code_analysis.rs
│   │   │       ├── ts_comment_checks.rs
│   │   │       └── tsconfig_check.rs
│   │   └── hooks-validate/                 (crate: guardrail3-app-hooks-validate)
│   │       ├── Cargo.toml
│   │       └── src/
│   │           ├── lib.rs
│   │           ├── deploy_checks.rs
│   │           ├── hook_checks.rs
│   │           ├── hook_script_checks.rs
│   │           ├── tool_checks.rs
│   │           └── validate.rs
│   │
│   ├── adapters/
│   │   ├── inbound/
│   │   │   └── cli/                        (crate: guardrail3-adapters-inbound-cli)
│   │   │       ├── Cargo.toml
│   │   │       └── src/
│   │   │           ├── lib.rs
│   │   │           ├── cli.rs              (clap definitions)
│   │   │           ├── help_gen.rs
│   │   │           ├── commands/
│   │   │           │   ├── mod.rs
│   │   │           │   ├── check.rs
│   │   │           │   ├── coverage/       (submodule)
│   │   │           │   ├── diff.rs
│   │   │           │   ├── generate.rs
│   │   │           │   ├── generate_helpers.rs
│   │   │           │   ├── init.rs
│   │   │           │   ├── map.rs
│   │   │           │   ├── modules_cmd.rs
│   │   │           │   └── validate.rs
│   │   │           └── main.rs             (binary entry point)
│   │   └── outbound/
│   │       ├── fs/                         (crate: guardrail3-adapters-outbound-fs)
│   │       │   ├── Cargo.toml
│   │       │   └── src/
│   │       │       └── lib.rs              (RealFileSystem impl)
│   │       ├── tool-runner/                (crate: guardrail3-adapters-outbound-tool-runner)
│   │       │   ├── Cargo.toml
│   │       │   └── src/
│   │       │       └── lib.rs              (RealToolChecker impl)
│   │       └── report/                     (crate: guardrail3-adapters-outbound-report)
│   │           ├── Cargo.toml
│   │           └── src/
│   │               ├── lib.rs
│   │               ├── json.rs
│   │               ├── markdown.rs
│   │               ├── text.rs
│   │               └── types.rs
│   │
│   └── main.rs                             (thin binary — just calls cli crate, composition root)
```

## Dependency graph (allowed directions only)

```
                    domain/config
                    domain/modules
                    domain/report
                         ↑
                ports/outbound/traits
                    (depends on domain/report for CheckResult, Severity)
                         ↑
            ┌────────────┼─────────────┐
            ↑            ↑             ↑
      app/core     app/rs-validate  app/ts-validate  app/hooks-validate
      (crawl,      (depends on       (depends on      (depends on
       discover)    domain, ports,    domain, ports,   domain, ports,
                    app/core)         app/core)        app/core)
                         ↑
    ┌────────────────────┼───────────────────────┐
    ↑                    ↑                       ↑
adapters/outbound/fs   adapters/outbound/report  adapters/inbound/cli
(impl FileSystem)      (text/json/md output)     (clap, commands, main.rs)
                                                  depends on everything
```

## Approach — step by step

### Phase 1: Create crate skeletons (directories + Cargo.toml + empty lib.rs)
Create all 12 crate directories with Cargo.toml and `src/lib.rs`. Set up workspace Cargo.toml at app root listing all members. Everything compiles (empty crates).

### Phase 2: Move domain crates (bottom of dependency tree)
Move `domain/config/`, `domain/modules/`, `domain/report.rs` into their crates. These have no internal dependencies — they only depend on external crates (serde, toml, etc.). Update imports in the remaining flat code to use the new crate names.

### Phase 3: Move ports crate
Move `ports/outbound.rs` into `ports/outbound/traits/`. It depends only on domain types (std::path, std::fs types in trait signatures). Update imports.

### Phase 4: Move app crates
Move `app/` subdirectories into their crates. These depend on domain + ports. The trickiest part — lots of cross-references between `app/core` (crawl, discover) and `app/rs-validate`, `app/ts-validate`.

### Phase 5: Move adapter crates
Move `adapters/outbound/` (fs, tool_runner), `report/`, `commands/`, `cli.rs`, `help_gen.rs` into their crates. These depend on everything above.

### Phase 6: Thin main.rs
Replace the current fat `main.rs` (450 lines of dispatch) with a thin composition root that imports from `adapters/inbound/cli` and calls it.

### Phase 7: Clean up
Remove `lib.rs` at crates root (no longer needed). Remove `mod.rs` files at structural dirs. Delete `.gitkeep` files where real crates now exist. Verify all guardrail checks pass.

## Key decisions

### Binary crate location
- **Chose:** `main.rs` stays at `crates/main.rs` — it's the composition root, not part of any hex layer
- **Why:** The binary wires everything together. It's not an adapter or app — it's the outermost shell.
- **Alternative:** Put it in `adapters/inbound/cli/` — rejected because the CLI crate should be a library, main.rs just calls it

### Crate naming
- **Chose:** `guardrail3-{layer}-{sublayer}-{name}` (e.g., `guardrail3-app-rs-validate`)
- **Why:** Globally unique, reflects position in hex arch

### Test files
- **Chose:** Tests stay inside each crate's `src/` as `#[cfg(test)] mod tests`
- **Why:** Matches current pattern, keeps tests close to code
- **Integration tests** in `apps/guardrail3/tests/` stay where they are — they test across crates

## Risks
- Import rewriting is the bulk of the work — every `use crate::` becomes `use guardrail3_domain_report::` etc.
- Circular dependencies will surface — current code may have hidden cycles that work within a single crate but fail across crates
- `domain/report.rs` is imported by almost everything — changes to its public API ripple everywhere

## Files affected
Every `.rs` file in the project changes (import paths). The structural changes are just moves. The hard part is the import rewriting + resolving any circular deps.
