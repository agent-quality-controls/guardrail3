use std::path::Path;

use walkdir::WalkDir;

use crate::report::types::{CheckResult, Severity};

pub fn check(path: &Path, scoped_files: Option<&[String]>) -> Vec<CheckResult> {
    let mut results = Vec::new();

    let ts_files: Vec<String> = match scoped_files {
        Some(files) => files
            .iter()
            .filter(|f| is_ts_file(f))
            .cloned()
            .collect(),
        None => collect_ts_files(path),
    };

    for file_path in &ts_files {
        let fp = Path::new(file_path);
        let content = match std::fs::read_to_string(fp) {
            Ok(c) => c,
            Err(_) => continue,
        };

        check_eslint_disable(fp, &content, &mut results);
        check_ts_ignore(fp, &content, &mut results);
        check_process_env(fp, &content, &mut results);
        check_any_types(fp, &content, &mut results);
        check_file_length(fp, &content, &mut results);
        check_noinspection(fp, &content, &mut results);
        check_coverage_ignore(fp, &content, &mut results);
    }

    // T59: Banned packages in node_modules
    check_banned_in_node_modules(path, &mut results);

    results
}

fn is_ts_file(path: &str) -> bool {
    path.ends_with(".ts")
        || path.ends_with(".tsx")
        || path.ends_with(".mjs")
}

fn collect_ts_files(root: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != "node_modules"
                && name != ".next"
                && name != "dist"
                && name != "target"
                && name != "coverage"
                && name != ".git"
        })
    {
        if let Ok(entry) = entry {
            if entry.file_type().is_file() {
                let path_str = entry.path().display().to_string();
                if is_ts_file(&path_str) {
                    files.push(path_str);
                }
            }
        }
    }
    files
}

// T23-T26: eslint-disable checks
fn check_eslint_disable(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_number = line_num.saturating_add(1);

        // Block-level eslint-disable (T23/T24)
        if trimmed.contains("eslint-disable")
            && !trimmed.contains("eslint-disable-next-line")
            && !trimmed.contains("eslint-disable-line")
        {
            if trimmed.contains("-- ") {
                // T24: with reason
                results.push(CheckResult {
                    id: "T24".to_string(),
                    severity: Severity::Info,
                    title: "eslint-disable with reason".to_string(),
                    message: trimmed.to_string(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                // T23: without reason
                results.push(CheckResult {
                    id: "T23".to_string(),
                    severity: Severity::Error,
                    title: "eslint-disable without reason".to_string(),
                    message: format!("eslint-disable missing `-- ` reason: {trimmed}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }

        // eslint-disable-next-line (T25/T26)
        if trimmed.contains("eslint-disable-next-line") {
            if trimmed.contains("-- ") {
                // T26: with reason
                results.push(CheckResult {
                    id: "T26".to_string(),
                    severity: Severity::Info,
                    title: "eslint-disable-next-line with reason".to_string(),
                    message: trimmed.to_string(),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            } else {
                // T25: without reason
                results.push(CheckResult {
                    id: "T25".to_string(),
                    severity: Severity::Error,
                    title: "eslint-disable-next-line without reason".to_string(),
                    message: format!("Missing `-- ` reason: {trimmed}"),
                    file: Some(path.display().to_string()),
                    line: Some(line_number),
                });
            }
        }
    }
}

// T27-T29: @ts-ignore / @ts-expect-error
fn check_ts_ignore(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        let line_number = line_num.saturating_add(1);

        // T27: @ts-ignore
        if trimmed.contains("@ts-ignore") {
            results.push(CheckResult {
                id: "T27".to_string(),
                severity: Severity::Error,
                title: "@ts-ignore usage".to_string(),
                message: format!("Use @ts-expect-error instead: {trimmed}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }

        // T28/T29: @ts-expect-error
        if trimmed.contains("@ts-expect-error") {
            // Check if there's text after @ts-expect-error
            if let Some(pos) = trimmed.find("@ts-expect-error") {
                let after = trimmed[pos.saturating_add(16)..].trim();
                if after.is_empty() || after == "*/" {
                    // T28: without explanation
                    results.push(CheckResult {
                        id: "T28".to_string(),
                        severity: Severity::Warn,
                        title: "@ts-expect-error without explanation".to_string(),
                        message: trimmed.to_string(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                } else {
                    // T29: with explanation
                    results.push(CheckResult {
                        id: "T29".to_string(),
                        severity: Severity::Info,
                        title: "@ts-expect-error with explanation".to_string(),
                        message: trimmed.to_string(),
                        file: Some(path.display().to_string()),
                        line: Some(line_number),
                    });
                }
            }
        }
    }
}

// T30: process.env direct access
fn check_process_env(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // Skip env.ts and env.mjs files
    if file_name == "env.ts" || file_name == "env.mjs" {
        return;
    }

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        // Skip comment lines
        if trimmed.starts_with("//") || trimmed.starts_with("*") || trimmed.starts_with("/*") {
            continue;
        }

        if trimmed.contains("process.env.") || trimmed.contains("process.env[") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "T30".to_string(),
                severity: Severity::Error,
                title: "Direct process.env access".to_string(),
                message: format!("Use env() import instead: {trimmed}"),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// T31: `as any` / `: any` type assertions
fn check_any_types(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        // Skip comment lines
        if trimmed.starts_with("//") || trimmed.starts_with("*") || trimmed.starts_with("/*") {
            continue;
        }

        if trimmed.contains("as any") || trimmed.contains(": any") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "T31".to_string(),
                severity: Severity::Info,
                title: "any type usage".to_string(),
                message: trimmed.to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// T32-T33: File line count
fn check_file_length(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    let effective_lines = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty()
                && !trimmed.starts_with("//")
                && !trimmed.starts_with("*")
        })
        .count();

    if effective_lines > 300 {
        results.push(CheckResult {
            id: "T32".to_string(),
            severity: Severity::Warn,
            title: "File too long".to_string(),
            message: format!("{effective_lines} effective lines (max 300)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    } else if effective_lines > 250 {
        results.push(CheckResult {
            id: "T33".to_string(),
            severity: Severity::Info,
            title: "File approaching limit".to_string(),
            message: format!("{effective_lines} effective lines (warn at 300)"),
            file: Some(path.display().to_string()),
            line: None,
        });
    }
}

// T34: // noinspection
fn check_noinspection(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        if line.contains("// noinspection") || line.contains("/* noinspection") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "T34".to_string(),
                severity: Severity::Info,
                title: "noinspection comment".to_string(),
                message: line.trim().to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// T35: istanbul ignore / c8 ignore
fn check_coverage_ignore(path: &Path, content: &str, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        if line.contains("istanbul ignore") || line.contains("c8 ignore") {
            let line_number = line_num.saturating_add(1);
            results.push(CheckResult {
                id: "T35".to_string(),
                severity: Severity::Info,
                title: "Coverage ignore comment".to_string(),
                message: line.trim().to_string(),
                file: Some(path.display().to_string()),
                line: Some(line_number),
            });
        }
    }
}

// T59: Banned packages in node_modules
fn check_banned_in_node_modules(path: &Path, results: &mut Vec<CheckResult>) {
    let nm_path = path.join("node_modules");
    if !nm_path.exists() {
        return;
    }

    let banned: &[&str] = &[
        "axios", "lodash", "moment", "uuid", "nanoid", "pg", "express",
        "classnames", "winston", "pino", "request", "got", "superagent",
        "node-fetch", "isomorphic-fetch", "underscore",
    ];
    let banned_prefixes: &[&str] = &["embla-carousel"];

    for dep in banned {
        let dep_path = nm_path.join(dep);
        if dep_path.exists() {
            results.push(CheckResult {
                id: "T59".to_string(),
                severity: Severity::Error,
                title: format!("Banned package in node_modules: {dep}"),
                message: format!("{dep} found in node_modules (transitive dependency?)"),
                file: Some(dep_path.display().to_string()),
                line: None,
            });
        }
    }

    // Check embla-carousel prefix
    if let Ok(entries) = std::fs::read_dir(&nm_path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if banned_prefixes.iter().any(|p| name.starts_with(p)) {
                results.push(CheckResult {
                    id: "T59".to_string(),
                    severity: Severity::Error,
                    title: format!("Banned package in node_modules: {name}"),
                    message: format!("{name} found in node_modules"),
                    file: Some(entry.path().display().to_string()),
                    line: None,
                });
            }
        }
    }
}
