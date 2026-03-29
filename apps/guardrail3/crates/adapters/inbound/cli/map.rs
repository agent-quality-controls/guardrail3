//! `guardrail3 map` — crawl project and show discovered structure.

use std::path::Path;

use guardrail3_app_commands::command_ids::RS_INIT;
use guardrail3_app_core::crawl::crawl;
use guardrail3_app_core::project_map::{self, ProjectMap, RustScopeKind, TsScope, TsScopeKind};

/// Run the map command — crawl, build structure, display.
#[allow(clippy::print_stdout)] // reason: CLI command — user-facing output
pub fn run(path: &str) {
    let project_path = Path::new(path);
    let crawl_result = crawl(project_path);
    let map = project_map::build(project_path, &crawl_result);

    println!(
        "Project: {}\n",
        project_path
            .canonicalize()
            .unwrap_or_else(|_| project_path.to_path_buf())
            .display()
    );

    print_rust(&map);
    print_ts(&map);
    print_shared(&map);

    if !map.shadows.is_empty() {
        print_shadows(&map);
    }
}

#[allow(clippy::print_stdout)] // reason: CLI output
fn print_rust(map: &ProjectMap) {
    if map.rust_scopes.is_empty() {
        return;
    }

    println!("── Rust ────────────────────────────────────────────────");
    println!();

    for scope in &map.rust_scopes {
        let kind_label = match &scope.kind {
            RustScopeKind::Workspace => format!("workspace, {} crates", scope.members.len()),
            RustScopeKind::StandaloneCrate => "standalone crate".to_owned(),
        };
        let root_display = if scope.root.display().to_string().is_empty() {
            ".".to_owned()
        } else {
            scope.root.display().to_string()
        };
        println!("  {root_display}/ ({kind_label})");

        // Members
        if matches!(scope.kind, RustScopeKind::Workspace) {
            for member in &scope.members {
                println!("    crate: {:<30} {}", member.name, member.dir.display());
            }
        }

        // Configs present
        let c = &scope.configs;
        if let Some(p) = &c.clippy_toml {
            let info = parse_clippy_info(p);
            println!("    clippy.toml                    {info}");
        } else {
            println!("    clippy.toml                    MISSING");
        }
        if let Some(p) = &c.deny_toml {
            let info = parse_deny_info(p);
            println!("    deny.toml                      {info}");
        } else {
            println!("    deny.toml                      MISSING");
        }
        if c.rustfmt_toml.is_some() {
            println!("    rustfmt.toml");
        } else {
            println!("    rustfmt.toml                   MISSING");
        }
        if !c.rust_toolchains.is_empty() {
            println!("    rust-toolchain.toml");
        }
        if c.jscpd_config.is_some() {
            println!("    .jscpd.json");
        }
        println!();
    }

    // Root-level rust-toolchain (if at project root, not inside a scope)
    if let Some(rt) = map.root_configs.rust_toolchains.first() {
        let rt_str = rt.display().to_string();
        let already_shown = map
            .rust_scopes
            .iter()
            .any(|s| !s.configs.rust_toolchains.is_empty());
        if !already_shown {
            println!("  {rt_str}");
            println!();
        }
    }
}

#[allow(clippy::print_stdout)] // reason: CLI output
fn print_ts(map: &ProjectMap) {
    if map.ts_scopes.is_empty() {
        return;
    }

    println!("── TypeScript ──────────────────────────────────────────");
    println!();

    // Group by kind
    let apps: Vec<_> = map
        .ts_scopes
        .iter()
        .filter(|s| s.kind == TsScopeKind::App)
        .collect();
    let packages: Vec<_> = map
        .ts_scopes
        .iter()
        .filter(|s| s.kind == TsScopeKind::Package)
        .collect();
    let tools: Vec<_> = map
        .ts_scopes
        .iter()
        .filter(|s| s.kind == TsScopeKind::Tool)
        .collect();

    if !apps.is_empty() {
        println!("  Apps:");
        print_ts_group(&apps);
        println!();
    }

    if !packages.is_empty() {
        println!("  Packages:");
        print_ts_group(&packages);
        println!();
    }

    if !tools.is_empty() {
        println!("  Tools:");
        print_ts_group(&tools);
        println!();
    }

    // Root-level TS guardrails
    if has_root_ts_guardrails(map) {
        print_root_ts_guardrails(map);
    }
}

#[allow(clippy::print_stdout)] // reason: CLI output
fn print_ts_scope(scope: &TsScope) {
    let mut signals = Vec::new();
    if scope.configs.next_config.is_some() {
        signals.push("Next.js");
    }
    if scope.configs.velite_config.is_some() {
        signals.push("Velite");
    }
    let signal_str = if signals.is_empty() {
        String::new()
    } else {
        format!(" ({})", signals.join(", "))
    };

    println!(
        "    {:<36} {}{signal_str}",
        scope.path.display(),
        scope.name
    );

    let c = &scope.configs;
    if c.tsconfig.is_some() {
        println!("      tsconfig.json");
    }
    if c.eslint_config.is_some() {
        println!("      eslint.config (per-app)");
    }
    if c.stylelint_config.is_some() {
        println!("      stylelint config (per-app)");
    }
}

fn print_ts_group(scopes: &[&TsScope]) {
    for scope in scopes {
        print_ts_scope(scope);
    }
}

fn has_root_ts_guardrails(map: &ProjectMap) -> bool {
    let rc = &map.root_configs;
    rc.eslint_config.is_some()
        || rc.stylelint_config.is_some()
        || rc.tsconfig_base.is_some()
        || rc.npmrc.is_some()
        || rc.jscpd_config.is_some()
        || rc.cspell_config.is_some()
        || rc.prettier_config.is_some()
}

#[allow(clippy::print_stdout)] // reason: CLI output
fn print_root_ts_guardrails(map: &ProjectMap) {
    let rc = &map.root_configs;
    println!("  Root guardrails:");
    if let Some(p) = &rc.eslint_config {
        println!(
            "    eslint.config.mjs              {} lines",
            count_lines(p)
        );
    }
    if let Some(p) = &rc.stylelint_config {
        println!(
            "    .stylelintrc                   {} lines",
            count_lines(p)
        );
    }
    if rc.tsconfig_base.is_some() {
        println!("    tsconfig.base.json");
    }
    if rc.npmrc.is_some() {
        println!("    .npmrc");
    }
    if rc.jscpd_config.is_some() {
        println!("    .jscpd.json");
    }
    if rc.cspell_config.is_some() {
        println!("    cspell config");
    } else {
        println!("    cspell config                  MISSING");
    }
    if rc.prettier_config.is_some() {
        println!("    prettier config");
    }
    println!();
}

#[allow(clippy::print_stdout)] // reason: CLI output
fn print_shared(map: &ProjectMap) {
    let rc = &map.root_configs;

    println!("── Shared ──────────────────────────────────────────────");
    println!();

    // Hooks
    if let Some(p) = rc.pre_commit_hooks.first() {
        println!(
            "  .githooks/pre-commit             {} lines",
            count_lines(p)
        );
    } else {
        println!("  pre-commit hook                  MISSING");
    }

    // Release
    if rc.release_plz_tomls.is_empty() {
        println!("  release-plz.toml                 MISSING");
    } else {
        println!("  release-plz.toml");
    }
    if rc.cliff_tomls.is_empty() {
        println!("  cliff.toml                       MISSING");
    } else {
        println!("  cliff.toml");
    }

    // CI
    if !rc.github_workflows.is_empty() {
        println!(
            "  .github/workflows/               {} files",
            rc.github_workflows.len()
        );
    }

    // Repo-level
    if rc.license_files.is_empty() {
        println!("  LICENSE                          MISSING");
    } else {
        println!("  LICENSE");
    }
    if !rc.claude_mds.is_empty() {
        println!("  CLAUDE.md");
    }

    // guardrail3
    println!();
    if rc.guardrail3_tomls.is_empty() {
        println!("  guardrail3.toml                  MISSING (run {RS_INIT})");
    } else {
        println!("  guardrail3.toml");
    }

    println!();
}

#[allow(clippy::print_stdout)] // reason: CLI output
fn print_shadows(map: &ProjectMap) {
    println!("── Warnings ────────────────────────────────────────────");
    println!();
    for shadow in &map.shadows {
        println!(
            "  SHADOW: {} at {} overrides {}/{} for crate {}",
            shadow.file_type,
            shadow.shadow_file.display(),
            shadow.scope_root.display(),
            shadow.file_type,
            shadow.affected_member.display(),
        );
    }
    println!();
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn count_lines(path: &Path) -> usize {
    guardrail3_shared_fs::read_file(path).map_or(0, |c| c.lines().count())
}

fn parse_clippy_info(path: &Path) -> String {
    let Some(content) = guardrail3_shared_fs::read_file(path) else {
        return String::new();
    };
    let Ok(table) = content.parse::<toml::Value>() else {
        return "parse error".to_owned();
    };
    let methods = table
        .get("disallowed-methods")
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    let types = table
        .get("disallowed-types")
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    format!("{methods} methods, {types} types")
}

fn parse_deny_info(path: &Path) -> String {
    let Some(content) = guardrail3_shared_fs::read_file(path) else {
        return String::new();
    };
    let Ok(table) = content.parse::<toml::Value>() else {
        return "parse error".to_owned();
    };
    let bans = table
        .get("bans")
        .and_then(|b| b.get("deny"))
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    let advisories = table
        .get("advisories")
        .and_then(|a| a.get("ignore"))
        .and_then(|v| v.as_array())
        .map_or(0, Vec::len);
    format!("{bans} bans, {advisories} advisory ignores")
}
