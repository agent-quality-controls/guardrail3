use guardrail3_outbound_traits::ToolChecker;
use std::path::{Path, PathBuf};
use std::process::Command;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

mod checks {
    pub(super) use super::super::hook_script_checks::{
        check_audit_hook, check_conflict_marker_hook, check_cspell_hook, check_dispatcher_pattern,
        check_local_scripts, check_lockfile_hook, check_modular_scripts, check_monolithic_patterns,
        check_prettier_hook, check_set_e_safety, check_stylelint_hook, emit_script_stats,
        inventory_scripts,
    };
    pub(super) use super::super::tool_checks::{check_duplication_tools, check_required_tools};
}

pub fn check_hooks(
    fs: &dyn FileSystem,
    tc: &dyn ToolChecker,
    path: &Path,
    has_rust: bool,
    has_typescript: bool,
    pre_commit_hooks: &[PathBuf],
    results: &mut Vec<CheckResult>,
) {
    // Use crawler-discovered pre-commit hook, fall back to conventional path
    let fallback = path.join(".githooks").join("pre-commit");
    let pre_commit_path = pre_commit_hooks.first().unwrap_or(&fallback);
    let pre_commit_d = pre_commit_path
        .parent()
        .unwrap_or(path)
        .join("pre-commit.d");

    // H1: .githooks/pre-commit exists
    if !check_pre_commit_exists(pre_commit_path, path, tc, results) {
        return;
    }

    // H2: core.hooksPath configured
    check_hooks_path(path, results);

    let is_modular = pre_commit_d.is_dir();
    let pre_commit_content = fs.read_file(pre_commit_path).unwrap_or_default();

    let ctx = HookContext {
        pre_commit_path,
        pre_commit_d: &pre_commit_d,
        is_modular,
        pre_commit_content: &pre_commit_content,
    };

    check_hook_structure(fs, &ctx, path, has_rust, has_typescript, results);
    check_hook_stats_and_tools(fs, tc, &ctx, path, results);
}

struct HookContext<'a> {
    pre_commit_path: &'a Path,
    pre_commit_d: &'a Path,
    is_modular: bool,
    pre_commit_content: &'a str,
}

/// H1: check pre-commit exists. Returns false if missing (caller should return early).
fn check_pre_commit_exists(
    pre_commit_path: &Path,
    path: &Path,
    tc: &dyn ToolChecker,
    results: &mut Vec<CheckResult>,
) -> bool {
    if pre_commit_path.exists() {
        results.push(
            CheckResult {
                id: "H1".to_owned(),
                severity: Severity::Info,
                title: ".githooks/pre-commit exists".to_owned(),
                message: "Found".to_owned(),
                file: Some(pre_commit_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        true
    } else {
        results.push(CheckResult {
            id: "H1".to_owned(),
            severity: Severity::Error,
            title: ".githooks/pre-commit missing".to_owned(),
            message: "No pre-commit hook found".to_owned(),
            file: Some(path.join(".githooks").display().to_string()),
            line: None,
            inventory: false,
        });
        check_hooks_path(path, results);
        checks::check_required_tools(tc, results);
        false
    }
}

/// H3, H4, H5, H12: hook structure checks (modular vs monolithic, dispatcher, patterns)
fn check_hook_structure(
    fs: &dyn FileSystem,
    ctx: &HookContext<'_>,
    path: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    // H3: pre-commit.d/ directory
    if ctx.is_modular {
        results.push(
            CheckResult {
                id: "H3".to_owned(),
                severity: Severity::Info,
                title: "pre-commit.d/ exists".to_owned(),
                message: "Using modular hook scripts".to_owned(),
                file: Some(ctx.pre_commit_d.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(
            CheckResult {
                id: "H3".to_owned(),
                severity: Severity::Info,
                title: "No pre-commit.d/ directory".to_owned(),
                message: "Using monolithic pre-commit script".to_owned(),
                file: Some(path.join(".githooks").display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }

    // H4: Dispatcher script
    checks::check_dispatcher_pattern(
        ctx.pre_commit_path,
        ctx.is_modular,
        ctx.pre_commit_content,
        results,
    );

    // H5: Expected scripts/patterns present
    if ctx.is_modular {
        checks::check_modular_scripts(fs, ctx.pre_commit_d, has_rust, has_typescript, results);
    } else {
        checks::check_monolithic_patterns(
            ctx.pre_commit_content,
            ctx.pre_commit_path,
            has_rust,
            has_typescript,
            results,
        );
    }

    // H12: Duplication tool checks
    checks::check_duplication_tools(
        ctx.pre_commit_content,
        ctx.pre_commit_path,
        has_rust,
        has_typescript,
        results,
    );

    // H-CSS-01: Stylelint in pre-commit (only relevant for web/TS projects)
    if has_typescript {
        checks::check_stylelint_hook(ctx.pre_commit_content, results);
    }

    // H-SAFE-01: Shell error handling (all projects)
    checks::check_set_e_safety(ctx.pre_commit_content, results);

    // H-TOOL-02: Conflict marker check (all projects)
    checks::check_conflict_marker_hook(ctx.pre_commit_content, results);

    // H-TOOL-03: Lockfile integrity (all projects)
    checks::check_lockfile_hook(ctx.pre_commit_content, results);

    // H-TOOL-01, H-TOOL-04, H-TOOL-05: TS-specific tool checks
    if has_typescript {
        checks::check_cspell_hook(ctx.pre_commit_content, results);
        checks::check_prettier_hook(ctx.pre_commit_content, results);
        checks::check_audit_hook(ctx.pre_commit_content, results);
    }
}

/// H6-H11: script stats, permissions, tools, inventory
fn check_hook_stats_and_tools(
    fs: &dyn FileSystem,
    tc: &dyn ToolChecker,
    ctx: &HookContext<'_>,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let (_line_count, size) =
        checks::emit_script_stats(fs, ctx.pre_commit_path, ctx.pre_commit_content, results);

    // H7: Script permissions
    check_permissions(fs, ctx.pre_commit_path, results);

    // H8: Required tools installed
    checks::check_required_tools(tc, results);

    // H9: Extra scripts in pre-commit.d/
    if ctx.is_modular {
        checks::inventory_scripts(
            fs,
            ctx.pre_commit_d,
            "H9",
            "Extra scripts in pre-commit.d/",
            results,
        );
    }

    // H10: Script modifications
    results.push(
        CheckResult {
            id: "H10".to_owned(),
            severity: Severity::Info,
            title: "Pre-commit file size".to_owned(),
            message: format!("{size} bytes"),
            file: Some(ctx.pre_commit_path.display().to_string()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );

    // H11: Local pre-commit scripts
    checks::check_local_scripts(fs, path, results);
}

fn check_hooks_path(path: &Path, results: &mut Vec<CheckResult>) {
    #[allow(clippy::disallowed_methods)] // reason: CLI tool needs to run git commands
    let output = Command::new("git")
        .args(["config", "core.hooksPath"])
        .current_dir(path)
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let val = String::from_utf8_lossy(&o.stdout).trim().to_owned();
            if val == ".githooks" {
                results.push(
                    CheckResult {
                        id: "H2".to_owned(),
                        severity: Severity::Info,
                        title: "core.hooksPath configured".to_owned(),
                        message: "core.hooksPath = .githooks".to_owned(),
                        file: None,
                        line: None,
                        inventory: false,
                    }
                    .as_inventory(),
                );
            } else {
                results.push(CheckResult {
                    id: "H2".to_owned(),
                    severity: Severity::Error,
                    title: "core.hooksPath wrong value".to_owned(),
                    message: format!("Expected .githooks, got \"{val}\""),
                    file: None,
                    line: None,
                    inventory: false,
                });
            }
        }
        _ => {
            results.push(CheckResult {
                id: "H2".to_owned(),
                severity: Severity::Error,
                title: "core.hooksPath not configured".to_owned(),
                message: "Run: git config core.hooksPath .githooks".to_owned(),
                file: None,
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_permissions(fs: &dyn FileSystem, file_path: &Path, results: &mut Vec<CheckResult>) {
    #[cfg(unix)]
    {
        match fs.metadata(file_path) {
            Some(meta) => {
                let mode = meta.permissions().mode();
                let is_executable = mode & 0o111 != 0;
                if is_executable {
                    results.push(
                        CheckResult {
                            id: "H7".to_owned(),
                            severity: Severity::Info,
                            title: "Pre-commit is executable".to_owned(),
                            message: format!("mode: {mode:o}"),
                            file: Some(file_path.display().to_string()),
                            line: None,
                            inventory: false,
                        }
                        .as_inventory(),
                    );
                } else {
                    results.push(CheckResult {
                        id: "H7".to_owned(),
                        severity: Severity::Error,
                        title: "Pre-commit is NOT executable".to_owned(),
                        message: format!("mode: {mode:o} — run: chmod +x {}", file_path.display()),
                        file: Some(file_path.display().to_string()),
                        line: None,
                        inventory: false,
                    });
                }
            }
            None => {
                results.push(CheckResult {
                    id: "H7".to_owned(),
                    severity: Severity::Error,
                    title: "Cannot read pre-commit permissions".to_owned(),
                    message: "Failed to read file metadata".to_owned(),
                    file: Some(file_path.display().to_string()),
                    line: None,
                    inventory: false,
                });
            }
        }
    }

    #[cfg(not(unix))]
    {
        results.push(
            CheckResult {
                id: "H7".to_owned(),
                severity: Severity::Info,
                title: "Permission check skipped".to_owned(),
                message: "Not on Unix — cannot check executable bit".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}
