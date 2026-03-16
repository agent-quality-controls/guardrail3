use crate::ports::outbound::ToolChecker;
use std::path::Path;
use std::process::Command;

use super::tool_checks::{check_duplication_tools, check_required_tools};
use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

#[allow(clippy::too_many_lines)] // reason: comprehensive hook validation
pub fn check_hooks(
    fs: &dyn FileSystem,
    tc: &dyn ToolChecker,
    path: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    let pre_commit_path = path.join(".githooks").join("pre-commit");
    let pre_commit_d = path.join(".githooks").join("pre-commit.d");

    // H1: .githooks/pre-commit exists
    if pre_commit_path.exists() {
        results.push(CheckResult {
            id: "H1".to_owned(),
            severity: Severity::Info,
            title: ".githooks/pre-commit exists".to_owned(),
            message: "Found".to_owned(),
            file: Some(pre_commit_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "H1".to_owned(),
            severity: Severity::Error,
            title: ".githooks/pre-commit missing".to_owned(),
            message: "No pre-commit hook found".to_owned(),
            file: Some(path.join(".githooks").display().to_string()),
            line: None,
        });
        // Can't do further hook checks without the file
        check_hooks_path(path, results);
        check_required_tools(tc, results);
        return;
    }

    // H2: core.hooksPath configured
    check_hooks_path(path, results);

    let is_modular = pre_commit_d.is_dir();

    // H3: pre-commit.d/ directory
    if is_modular {
        results.push(CheckResult {
            id: "H3".to_owned(),
            severity: Severity::Info,
            title: "pre-commit.d/ exists".to_owned(),
            message: "Using modular hook scripts".to_owned(),
            file: Some(pre_commit_d.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "H3".to_owned(),
            severity: Severity::Info,
            title: "No pre-commit.d/ directory".to_owned(),
            message: "Using monolithic pre-commit script".to_owned(),
            file: Some(path.join(".githooks").display().to_string()),
            line: None,
        });
    }

    let pre_commit_content = fs.read_file(&pre_commit_path).unwrap_or_default();

    // H4: Dispatcher script
    if is_modular {
        let has_dispatcher = pre_commit_content.contains("pre-commit.d")
            && (pre_commit_content.contains("source ")
                || pre_commit_content.contains(". ")
                || pre_commit_content.contains("for ")
                || pre_commit_content.contains("run-parts"));
        if has_dispatcher {
            results.push(CheckResult {
                id: "H4".to_owned(),
                severity: Severity::Info,
                title: "Dispatcher pattern found".to_owned(),
                message: "pre-commit sources scripts from pre-commit.d/".to_owned(),
                file: Some(pre_commit_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "H4".to_owned(),
                severity: Severity::Error,
                title: "Dispatcher pattern missing".to_owned(),
                message: "pre-commit.d/ exists but pre-commit doesn't dispatch to it".to_owned(),
                file: Some(pre_commit_path.display().to_string()),
                line: None,
            });
        }
    } else {
        results.push(CheckResult {
            id: "H4".to_owned(),
            severity: Severity::Info,
            title: "Monolithic script (no dispatcher needed)".to_owned(),
            message: "No pre-commit.d/, so no dispatcher check".to_owned(),
            file: Some(pre_commit_path.display().to_string()),
            line: None,
        });
    }

    // H5: Expected scripts/patterns present
    if is_modular {
        check_modular_scripts(fs, &pre_commit_d, has_rust, has_typescript, results);
    } else {
        check_monolithic_patterns(
            &pre_commit_content,
            &pre_commit_path,
            has_rust,
            has_typescript,
            results,
        );
    }

    // H12: Duplication tool checks
    check_duplication_tools(
        &pre_commit_content,
        &pre_commit_path,
        has_rust,
        has_typescript,
        results,
    );

    // H6: Script checksums (monolithic)
    let line_count = pre_commit_content.lines().count();
    let metadata = fs.metadata(&pre_commit_path);
    let modified = metadata.as_ref().and_then(|m| m.modified().ok()).map(|t| {
        // Format as rough timestamp
        t.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    });
    let size = match metadata.as_ref() {
        Some(m) => m.len(),
        None => 0,
    };

    results.push(CheckResult {
        id: "H6".to_owned(),
        severity: Severity::Info,
        title: "Pre-commit script stats".to_owned(),
        message: format!(
            "{line_count} lines, {size} bytes{}",
            modified.map_or(String::new(), |t| format!(", mtime unix {t}"))
        ),
        file: Some(pre_commit_path.display().to_string()),
        line: None,
    });

    // H7: Script permissions
    check_permissions(fs, &pre_commit_path, results);

    // H8: Required tools installed
    check_required_tools(tc, results);

    // H9: Extra scripts in pre-commit.d/
    if is_modular {
        inventory_scripts(
            fs,
            &pre_commit_d,
            "H9",
            "Extra scripts in pre-commit.d/",
            results,
        );
    }

    // H10: Script modifications (already covered by H6 size/hash, but
    // report the file size as a separate line for clarity)
    results.push(CheckResult {
        id: "H10".to_owned(),
        severity: Severity::Info,
        title: "Pre-commit file size".to_owned(),
        message: format!("{size} bytes"),
        file: Some(pre_commit_path.display().to_string()),
        line: None,
    });

    // H11: Local pre-commit scripts
    let local_d = path.join("local").join("pre-commit.d");
    if local_d.is_dir() {
        inventory_scripts(fs, &local_d, "H11", "Local pre-commit scripts", results);
    } else {
        results.push(CheckResult {
            id: "H11".to_owned(),
            severity: Severity::Info,
            title: "No local/pre-commit.d/ directory".to_owned(),
            message: "No local hook overrides found".to_owned(),
            file: None,
            line: None,
        });
    }
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
                results.push(CheckResult {
                    id: "H2".to_owned(),
                    severity: Severity::Info,
                    title: "core.hooksPath configured".to_owned(),
                    message: "core.hooksPath = .githooks".to_owned(),
                    file: None,
                    line: None,
                });
            } else {
                results.push(CheckResult {
                    id: "H2".to_owned(),
                    severity: Severity::Error,
                    title: "core.hooksPath wrong value".to_owned(),
                    message: format!("Expected .githooks, got \"{val}\""),
                    file: None,
                    line: None,
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
            });
        }
    }
}

#[allow(clippy::too_many_lines)] // reason: hook pattern checking across multiple tools
fn check_monolithic_patterns(
    content: &str,
    file_path: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    struct PatternCheck {
        pattern: &'static [&'static str],
        label: &'static str,
        severity_if_missing: Severity,
        requires_rust: bool,
        requires_ts: bool,
    }

    let checks = [
        PatternCheck {
            pattern: &["gitleaks"],
            label: "gitleaks",
            severity_if_missing: Severity::Error,
            requires_rust: false,
            requires_ts: false,
        },
        PatternCheck {
            pattern: &["cargo fmt", "rustfmt"],
            label: "cargo fmt / rustfmt",
            severity_if_missing: Severity::Error,
            requires_rust: true,
            requires_ts: false,
        },
        PatternCheck {
            pattern: &["cargo clippy", "clippy"],
            label: "cargo clippy",
            severity_if_missing: Severity::Error,
            requires_rust: true,
            requires_ts: false,
        },
        PatternCheck {
            pattern: &["cargo deny", "cargo-deny"],
            label: "cargo deny",
            severity_if_missing: Severity::Error,
            requires_rust: true,
            requires_ts: false,
        },
        PatternCheck {
            pattern: &["cargo test"],
            label: "cargo test",
            severity_if_missing: Severity::Warn,
            requires_rust: true,
            requires_ts: false,
        },
        PatternCheck {
            pattern: &["cargo machete", "cargo-machete"],
            label: "cargo machete",
            severity_if_missing: Severity::Warn,
            requires_rust: true,
            requires_ts: false,
        },
        PatternCheck {
            pattern: &["tsc", "--noEmit"],
            label: "tsc / --noEmit",
            severity_if_missing: Severity::Warn,
            requires_rust: false,
            requires_ts: true,
        },
        PatternCheck {
            pattern: &["eslint"],
            label: "eslint",
            severity_if_missing: Severity::Warn,
            requires_rust: false,
            requires_ts: true,
        },
        PatternCheck {
            pattern: &["jscpd"],
            label: "jscpd",
            severity_if_missing: Severity::Warn,
            requires_rust: false,
            requires_ts: false,
        },
        PatternCheck {
            pattern: &["cargo dupes", "cargo-dupes"],
            label: "cargo dupes",
            severity_if_missing: Severity::Info,
            requires_rust: true,
            requires_ts: false,
        },
    ];

    for check in &checks {
        if check.requires_rust && !has_rust {
            continue;
        }
        if check.requires_ts && !has_typescript {
            continue;
        }

        let found = check.pattern.iter().any(|p| content.contains(p));
        if found {
            results.push(CheckResult {
                id: "H5".to_owned(),
                severity: Severity::Info,
                title: format!("{} found in pre-commit", check.label),
                message: "Pattern present in monolithic script".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "H5".to_owned(),
                severity: check.severity_if_missing,
                title: format!("{} not found in pre-commit", check.label),
                message: "Pattern missing from monolithic script".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_modular_scripts(
    fs: &dyn FileSystem,
    pre_commit_d: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    // Read all script contents to search for patterns
    let mut all_content = String::new();
    for entry in fs.list_dir(pre_commit_d) {
        if let Some(content) = fs.read_file(&entry.path()) {
            all_content.push_str(&content);
            all_content.push('\n');
        }
    }

    // Reuse the same pattern checks against the combined script content
    check_monolithic_patterns(
        &all_content,
        pre_commit_d,
        has_rust,
        has_typescript,
        results,
    );
}

fn check_permissions(fs: &dyn FileSystem, file_path: &Path, results: &mut Vec<CheckResult>) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        match fs.metadata(file_path) {
            Some(meta) => {
                let mode = meta.permissions().mode();
                let is_executable = mode & 0o111 != 0;
                if is_executable {
                    results.push(CheckResult {
                        id: "H7".to_owned(),
                        severity: Severity::Info,
                        title: "Pre-commit is executable".to_owned(),
                        message: format!("mode: {mode:o}"),
                        file: Some(file_path.display().to_string()),
                        line: None,
                    });
                } else {
                    results.push(CheckResult {
                        id: "H7".to_owned(),
                        severity: Severity::Error,
                        title: "Pre-commit is NOT executable".to_owned(),
                        message: format!("mode: {mode:o} — run: chmod +x {}", file_path.display()),
                        file: Some(file_path.display().to_string()),
                        line: None,
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
                });
            }
        }
    }

    #[cfg(not(unix))]
    {
        results.push(CheckResult {
            id: "H7".to_owned(),
            severity: Severity::Info,
            title: "Permission check skipped".to_owned(),
            message: "Not on Unix — cannot check executable bit".to_owned(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
    }
}

fn inventory_scripts(
    fs: &dyn FileSystem,
    dir: &Path,
    id: &str,
    title_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    if !dir.exists() {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Warn,
            title: format!("{title_prefix}: unreadable"),
            message: "Directory does not exist".to_owned(),
            file: Some(dir.display().to_string()),
            line: None,
        });
        return;
    }

    let entries = fs.list_dir(dir);
    let mut names: Vec<String> = Vec::new();
    for entry in entries {
        if let Some(name) = entry.file_name().to_str() {
            names.push(name.to_owned());
        }
    }
    names.sort();

    if names.is_empty() {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Info,
            title: format!("{title_prefix}: empty"),
            message: "No scripts found".to_owned(),
            file: Some(dir.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Info,
            title: format!("{title_prefix}: {} scripts", names.len()),
            message: names.join(", "),
            file: Some(dir.display().to_string()),
            line: None,
        });
    }
}
