use std::path::Path;
use std::process::Command;

use crate::report::types::{CheckResult, Report, Section, Severity};

pub fn run(path: &Path, has_rust: bool, has_typescript: bool) -> Report {
    let mut report = Report::new(
        path.display().to_string(),
        vec!["Hooks".to_string()],
    );

    let mut hook_results = Vec::new();
    check_hooks(path, has_rust, has_typescript, &mut hook_results);
    report.add_section(Section {
        name: "Hook checks".to_string(),
        results: hook_results,
    });

    // D1-D5 only run if the project has deployment configs
    let has_railpack = has_railpack_files(path);
    let has_apps_dir = path.join("apps").is_dir();
    if has_railpack || has_apps_dir {
        let mut deploy_results = Vec::new();
        check_deployment(path, &mut deploy_results);
        report.add_section(Section {
            name: "Deployment checks".to_string(),
            results: deploy_results,
        });
    }

    report
}

fn has_railpack_files(path: &Path) -> bool {
    let entries = match std::fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return false,
    };
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with("railpack-") && name.ends_with(".json") {
                return true;
            }
        }
    }
    false
}

fn check_hooks(
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
            id: "H1".to_string(),
            severity: Severity::Info,
            title: ".githooks/pre-commit exists".to_string(),
            message: "Found".to_string(),
            file: Some(pre_commit_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "H1".to_string(),
            severity: Severity::Error,
            title: ".githooks/pre-commit missing".to_string(),
            message: "No pre-commit hook found".to_string(),
            file: Some(path.join(".githooks").display().to_string()),
            line: None,
        });
        // Can't do further hook checks without the file
        check_hooks_path(path, results);
        check_required_tools(results);
        return;
    }

    // H2: core.hooksPath configured
    check_hooks_path(path, results);

    let is_modular = pre_commit_d.is_dir();

    // H3: pre-commit.d/ directory
    if is_modular {
        results.push(CheckResult {
            id: "H3".to_string(),
            severity: Severity::Info,
            title: "pre-commit.d/ exists".to_string(),
            message: "Using modular hook scripts".to_string(),
            file: Some(pre_commit_d.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "H3".to_string(),
            severity: Severity::Info,
            title: "No pre-commit.d/ directory".to_string(),
            message: "Using monolithic pre-commit script".to_string(),
            file: Some(path.join(".githooks").display().to_string()),
            line: None,
        });
    }

    let pre_commit_content = std::fs::read_to_string(&pre_commit_path).unwrap_or_default();

    // H4: Dispatcher script
    if is_modular {
        let has_dispatcher = pre_commit_content.contains("pre-commit.d")
            && (pre_commit_content.contains("source ")
                || pre_commit_content.contains(". ")
                || pre_commit_content.contains("for ")
                || pre_commit_content.contains("run-parts"));
        if has_dispatcher {
            results.push(CheckResult {
                id: "H4".to_string(),
                severity: Severity::Info,
                title: "Dispatcher pattern found".to_string(),
                message: "pre-commit sources scripts from pre-commit.d/".to_string(),
                file: Some(pre_commit_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "H4".to_string(),
                severity: Severity::Error,
                title: "Dispatcher pattern missing".to_string(),
                message: "pre-commit.d/ exists but pre-commit doesn't dispatch to it".to_string(),
                file: Some(pre_commit_path.display().to_string()),
                line: None,
            });
        }
    } else {
        results.push(CheckResult {
            id: "H4".to_string(),
            severity: Severity::Info,
            title: "Monolithic script (no dispatcher needed)".to_string(),
            message: "No pre-commit.d/, so no dispatcher check".to_string(),
            file: Some(pre_commit_path.display().to_string()),
            line: None,
        });
    }

    // H5: Expected scripts/patterns present
    if is_modular {
        check_modular_scripts(&pre_commit_d, has_rust, has_typescript, results);
    } else {
        check_monolithic_patterns(
            &pre_commit_content,
            &pre_commit_path,
            has_rust,
            has_typescript,
            results,
        );
    }

    // H6: Script checksums (monolithic)
    let line_count = pre_commit_content.lines().count();
    let metadata = std::fs::metadata(&pre_commit_path);
    let modified = metadata
        .as_ref()
        .ok()
        .and_then(|m| m.modified().ok())
        .map(|t| {
            // Format as rough timestamp
            t.duration_since(std::time::SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });
    let size = metadata.as_ref().ok().map(|m| m.len()).unwrap_or(0);

    results.push(CheckResult {
        id: "H6".to_string(),
        severity: Severity::Info,
        title: "Pre-commit script stats".to_string(),
        message: format!(
            "{line_count} lines, {size} bytes{}",
            modified.map_or(String::new(), |t| format!(", mtime unix {t}"))
        ),
        file: Some(pre_commit_path.display().to_string()),
        line: None,
    });

    // H7: Script permissions
    check_permissions(&pre_commit_path, results);

    // H8: Required tools installed
    check_required_tools(results);

    // H9: Extra scripts in pre-commit.d/
    if is_modular {
        inventory_scripts(&pre_commit_d, "H9", "Extra scripts in pre-commit.d/", results);
    }

    // H10: Script modifications (already covered by H6 size/hash, but
    // report the file size as a separate line for clarity)
    results.push(CheckResult {
        id: "H10".to_string(),
        severity: Severity::Info,
        title: "Pre-commit file size".to_string(),
        message: format!("{size} bytes"),
        file: Some(pre_commit_path.display().to_string()),
        line: None,
    });

    // H11: Local pre-commit scripts
    let local_d = path.join("local").join("pre-commit.d");
    if local_d.is_dir() {
        inventory_scripts(&local_d, "H11", "Local pre-commit scripts", results);
    } else {
        results.push(CheckResult {
            id: "H11".to_string(),
            severity: Severity::Info,
            title: "No local/pre-commit.d/ directory".to_string(),
            message: "No local hook overrides found".to_string(),
            file: None,
            line: None,
        });
    }
}

fn check_hooks_path(path: &Path, results: &mut Vec<CheckResult>) {
    let output = Command::new("git")
        .args(["config", "core.hooksPath"])
        .current_dir(path)
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let val = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if val == ".githooks" {
                results.push(CheckResult {
                    id: "H2".to_string(),
                    severity: Severity::Info,
                    title: "core.hooksPath configured".to_string(),
                    message: "core.hooksPath = .githooks".to_string(),
                    file: None,
                    line: None,
                });
            } else {
                results.push(CheckResult {
                    id: "H2".to_string(),
                    severity: Severity::Error,
                    title: "core.hooksPath wrong value".to_string(),
                    message: format!("Expected .githooks, got \"{val}\""),
                    file: None,
                    line: None,
                });
            }
        }
        _ => {
            results.push(CheckResult {
                id: "H2".to_string(),
                severity: Severity::Error,
                title: "core.hooksPath not configured".to_string(),
                message: "Run: git config core.hooksPath .githooks".to_string(),
                file: None,
                line: None,
            });
        }
    }
}

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
                id: "H5".to_string(),
                severity: Severity::Info,
                title: format!("{} found in pre-commit", check.label),
                message: "Pattern present in monolithic script".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "H5".to_string(),
                severity: check.severity_if_missing,
                title: format!("{} not found in pre-commit", check.label),
                message: "Pattern missing from monolithic script".to_string(),
                file: Some(file_path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_modular_scripts(
    pre_commit_d: &Path,
    has_rust: bool,
    has_typescript: bool,
    results: &mut Vec<CheckResult>,
) {
    // Read all script contents to search for patterns
    let mut all_content = String::new();
    if let Ok(entries) = std::fs::read_dir(pre_commit_d) {
        for entry in entries.flatten() {
            if let Ok(content) = std::fs::read_to_string(entry.path()) {
                all_content.push_str(&content);
                all_content.push('\n');
            }
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

fn check_permissions(file_path: &Path, results: &mut Vec<CheckResult>) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        match std::fs::metadata(file_path) {
            Ok(meta) => {
                let mode = meta.permissions().mode();
                let is_executable = mode & 0o111 != 0;
                if is_executable {
                    results.push(CheckResult {
                        id: "H7".to_string(),
                        severity: Severity::Info,
                        title: "Pre-commit is executable".to_string(),
                        message: format!("mode: {mode:o}"),
                        file: Some(file_path.display().to_string()),
                        line: None,
                    });
                } else {
                    results.push(CheckResult {
                        id: "H7".to_string(),
                        severity: Severity::Error,
                        title: "Pre-commit is NOT executable".to_string(),
                        message: format!("mode: {mode:o} — run: chmod +x {}", file_path.display()),
                        file: Some(file_path.display().to_string()),
                        line: None,
                    });
                }
            }
            Err(e) => {
                results.push(CheckResult {
                    id: "H7".to_string(),
                    severity: Severity::Error,
                    title: "Cannot read pre-commit permissions".to_string(),
                    message: format!("{e}"),
                    file: Some(file_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    #[cfg(not(unix))]
    {
        results.push(CheckResult {
            id: "H7".to_string(),
            severity: Severity::Info,
            title: "Permission check skipped".to_string(),
            message: "Not on Unix — cannot check executable bit".to_string(),
            file: Some(file_path.display().to_string()),
            line: None,
        });
    }
}

fn check_required_tools(results: &mut Vec<CheckResult>) {
    let tools = [
        ("gitleaks", Severity::Error),
        ("cargo-deny", Severity::Error),
        ("cargo-machete", Severity::Error),
    ];

    for (tool, severity) in &tools {
        let found = Command::new("which")
            .arg(tool)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if found {
            results.push(CheckResult {
                id: "H8".to_string(),
                severity: Severity::Info,
                title: format!("{tool} installed"),
                message: "Found on PATH".to_string(),
                file: None,
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "H8".to_string(),
                severity: *severity,
                title: format!("{tool} not installed"),
                message: format!("{tool} not found on PATH"),
                file: None,
                line: None,
            });
        }
    }
}

fn inventory_scripts(
    dir: &Path,
    id: &str,
    title_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    match std::fs::read_dir(dir) {
        Ok(entries) => {
            let mut names: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
            names.sort();

            if names.is_empty() {
                results.push(CheckResult {
                    id: id.to_string(),
                    severity: Severity::Info,
                    title: format!("{title_prefix}: empty"),
                    message: "No scripts found".to_string(),
                    file: Some(dir.display().to_string()),
                    line: None,
                });
            } else {
                results.push(CheckResult {
                    id: id.to_string(),
                    severity: Severity::Info,
                    title: format!("{title_prefix}: {} scripts", names.len()),
                    message: names.join(", "),
                    file: Some(dir.display().to_string()),
                    line: None,
                });
            }
        }
        Err(e) => {
            results.push(CheckResult {
                id: id.to_string(),
                severity: Severity::Warn,
                title: format!("{title_prefix}: unreadable"),
                message: format!("{e}"),
                file: Some(dir.display().to_string()),
                line: None,
            });
        }
    }
}

// --- Deployment checks (D1-D5) ---

fn check_deployment(path: &Path, results: &mut Vec<CheckResult>) {
    // D1: Railpack config files
    let railpack_configs = find_railpack_configs(path);
    if railpack_configs.is_empty() {
        results.push(CheckResult {
            id: "D1".to_string(),
            severity: Severity::Warn,
            title: "No railpack config files found".to_string(),
            message: "Expected railpack-*.json in project root".to_string(),
            file: Some(path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "D1".to_string(),
            severity: Severity::Info,
            title: format!("Found {} railpack config(s)", railpack_configs.len()),
            message: railpack_configs
                .iter()
                .filter_map(|p| p.file_name().and_then(|n| n.to_str()))
                .collect::<Vec<_>>()
                .join(", "),
            file: Some(path.display().to_string()),
            line: None,
        });

        // D2: Check provider field in each config
        for config_path in &railpack_configs {
            check_railpack_provider(config_path, results);
        }
    }

    // D3 & D4: Next.js configs in apps/*/
    check_nextjs_configs(path, results);

    // D5: Tailwind in dependencies
    check_tailwind_deps(path, results);
}

fn find_railpack_configs(path: &Path) -> Vec<std::path::PathBuf> {
    let mut configs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("railpack-") && name.ends_with(".json") {
                    configs.push(entry.path());
                }
            }
        }
    }
    configs.sort();
    configs
}

fn check_railpack_provider(
    config_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let content = match std::fs::read_to_string(config_path) {
        Ok(c) => c,
        Err(e) => {
            results.push(CheckResult {
                id: "D2".to_string(),
                severity: Severity::Warn,
                title: "Railpack config unreadable".to_string(),
                message: format!("{e}"),
                file: Some(config_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "D2".to_string(),
                severity: Severity::Error,
                title: "Railpack config invalid JSON".to_string(),
                message: format!("{e}"),
                file: Some(config_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let provider = json.get("provider").and_then(|v| v.as_str());
    let filename = config_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // Heuristic: if filename contains "web" or "landing", it's a Node service
    let looks_like_node = filename.contains("web") || filename.contains("landing");

    match provider {
        Some(p) => {
            results.push(CheckResult {
                id: "D2".to_string(),
                severity: Severity::Info,
                title: format!("{filename}: provider = \"{p}\""),
                message: "Provider field present".to_string(),
                file: Some(config_path.display().to_string()),
                line: None,
            });
        }
        None => {
            let severity = if looks_like_node {
                Severity::Error
            } else {
                Severity::Warn
            };
            results.push(CheckResult {
                id: "D2".to_string(),
                severity,
                title: format!("{filename}: no provider field"),
                message: if looks_like_node {
                    "Node.js service needs \"provider\": \"node\" to prevent Rust auto-detection"
                        .to_string()
                } else {
                    "No provider field — Railpack will auto-detect".to_string()
                },
                file: Some(config_path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_nextjs_configs(path: &Path, results: &mut Vec<CheckResult>) {
    let apps_dir = path.join("apps");
    if !apps_dir.is_dir() {
        return;
    }

    let entries = match std::fs::read_dir(&apps_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let app_dir = entry.path();
        if !app_dir.is_dir() {
            continue;
        }

        // Look for next.config.mjs or next.config.js
        let config_path = if app_dir.join("next.config.mjs").exists() {
            Some(app_dir.join("next.config.mjs"))
        } else if app_dir.join("next.config.js").exists() {
            Some(app_dir.join("next.config.js"))
        } else if app_dir.join("next.config.ts").exists() {
            Some(app_dir.join("next.config.ts"))
        } else {
            None
        };

        let config_path = match config_path {
            Some(p) => p,
            None => continue, // Not a Next.js app
        };

        let content = match std::fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let app_name = app_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // D3: standalone output
        if content.contains("standalone") {
            results.push(CheckResult {
                id: "D3".to_string(),
                severity: Severity::Info,
                title: format!("{app_name}: standalone output configured"),
                message: "output: \"standalone\" found".to_string(),
                file: Some(config_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "D3".to_string(),
                severity: Severity::Error,
                title: format!("{app_name}: standalone output missing"),
                message: "Next.js needs output: \"standalone\" for Railway deployment".to_string(),
                file: Some(config_path.display().to_string()),
                line: None,
            });
        }

        // D4: outputFileTracingRoot
        if content.contains("outputFileTracingRoot") {
            results.push(CheckResult {
                id: "D4".to_string(),
                severity: Severity::Info,
                title: format!("{app_name}: outputFileTracingRoot configured"),
                message: "outputFileTracingRoot found".to_string(),
                file: Some(config_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: "D4".to_string(),
                severity: Severity::Warn,
                title: format!("{app_name}: outputFileTracingRoot missing"),
                message: "Monorepo needs outputFileTracingRoot pointing to repo root".to_string(),
                file: Some(config_path.display().to_string()),
                line: None,
            });
        }
    }
}

fn check_tailwind_deps(path: &Path, results: &mut Vec<CheckResult>) {
    let apps_dir = path.join("apps");
    if !apps_dir.is_dir() {
        return;
    }

    let entries = match std::fs::read_dir(&apps_dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let app_dir = entry.path();
        let pkg_json_path = app_dir.join("package.json");
        if !pkg_json_path.exists() {
            continue;
        }

        let content = match std::fs::read_to_string(&pkg_json_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let json: serde_json::Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let app_name = app_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        let in_deps = json
            .get("dependencies")
            .and_then(|d| d.get("tailwindcss"))
            .is_some();
        let in_dev_deps = json
            .get("devDependencies")
            .and_then(|d| d.get("tailwindcss"))
            .is_some();

        if in_dev_deps {
            results.push(CheckResult {
                id: "D5".to_string(),
                severity: Severity::Warn,
                title: format!("{app_name}: tailwindcss in devDependencies"),
                message: "Railway skips devDeps — move tailwindcss to dependencies".to_string(),
                file: Some(pkg_json_path.display().to_string()),
                line: None,
            });
        } else if in_deps {
            results.push(CheckResult {
                id: "D5".to_string(),
                severity: Severity::Info,
                title: format!("{app_name}: tailwindcss in dependencies"),
                message: "Correctly in dependencies (not devDependencies)".to_string(),
                file: Some(pkg_json_path.display().to_string()),
                line: None,
            });
        }
        // If neither, it's not a Tailwind app — skip silently
    }
}
