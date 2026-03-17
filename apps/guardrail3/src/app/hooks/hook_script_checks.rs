use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// H4: Check dispatcher pattern in pre-commit script
pub(super) fn check_dispatcher_pattern(
    pre_commit_path: &Path,
    is_modular: bool,
    pre_commit_content: &str,
    results: &mut Vec<CheckResult>,
) {
    if is_modular {
        let has_dispatcher = pre_commit_content.contains("pre-commit.d")
            && (pre_commit_content.contains("source ")
                || pre_commit_content.contains(". ")
                || pre_commit_content.contains("for ")
                || pre_commit_content.contains("run-parts"));
        if has_dispatcher {
            results.push(
                CheckResult {
                    id: "H4".to_owned(),
                    severity: Severity::Info,
                    title: "Dispatcher pattern found".to_owned(),
                    message: "pre-commit sources scripts from pre-commit.d/".to_owned(),
                    file: Some(pre_commit_path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        } else {
            results.push(CheckResult {
                id: "H4".to_owned(),
                severity: Severity::Error,
                title: "Dispatcher pattern missing".to_owned(),
                message: "pre-commit.d/ exists but pre-commit doesn't dispatch to it".to_owned(),
                file: Some(pre_commit_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    } else {
        results.push(
            CheckResult {
                id: "H4".to_owned(),
                severity: Severity::Info,
                title: "Monolithic script (no dispatcher needed)".to_owned(),
                message: "No pre-commit.d/, so no dispatcher check".to_owned(),
                file: Some(pre_commit_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

/// H6: emit script stats. Returns (`line_count`, `size`).
pub(super) fn emit_script_stats(
    fs: &dyn FileSystem,
    pre_commit_path: &Path,
    pre_commit_content: &str,
    results: &mut Vec<CheckResult>,
) -> (usize, u64) {
    let line_count = pre_commit_content.lines().count();
    let metadata = fs.metadata(pre_commit_path);
    let modified = metadata.as_ref().and_then(|m| m.modified().ok()).map(|t| {
        t.duration_since(std::time::SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0)
    });
    let size = match metadata.as_ref() {
        Some(m) => m.len(),
        None => 0,
    };

    results.push(
        CheckResult {
            id: "H6".to_owned(),
            severity: Severity::Info,
            title: "Pre-commit script stats".to_owned(),
            message: format!(
                "{line_count} lines, {size} bytes{}",
                modified.map_or(String::new(), |t| format!(", mtime unix {t}"))
            ),
            file: Some(pre_commit_path.display().to_string()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );

    (line_count, size)
}

pub(super) fn check_local_scripts(
    fs: &dyn FileSystem,
    path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let local_d = path.join("local").join("pre-commit.d");
    if local_d.is_dir() {
        inventory_scripts(fs, &local_d, "H11", "Local pre-commit scripts", results);
    } else {
        results.push(
            CheckResult {
                id: "H11".to_owned(),
                severity: Severity::Info,
                title: "No local/pre-commit.d/ directory".to_owned(),
                message: "No local hook overrides found".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

struct PatternCheck {
    pattern: &'static [&'static str],
    label: &'static str,
    severity_if_missing: Severity,
    requires_rust: bool,
    requires_ts: bool,
}

const HOOK_PATTERN_CHECKS: &[PatternCheck] = &[
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

pub(super) fn check_monolithic_patterns(
    content: &str,
    file_path: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    for check in HOOK_PATTERN_CHECKS {
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
                inventory: false,
            });
        } else {
            results.push(CheckResult {
                id: "H5".to_owned(),
                severity: check.severity_if_missing,
                title: format!("{} not found in pre-commit", check.label),
                message: "Pattern missing from monolithic script".to_owned(),
                file: Some(file_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

pub(super) fn check_modular_scripts(
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

pub(super) fn inventory_scripts(
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
            inventory: false,
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
        results.push(
            CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("{title_prefix}: empty"),
                message: "No scripts found".to_owned(),
                file: Some(dir.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(
            CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("{title_prefix}: {} scripts", names.len()),
                message: names.join(", "),
                file: Some(dir.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}
