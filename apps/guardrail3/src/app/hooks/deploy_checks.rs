use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

pub fn check_deployment(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    // D1: Railpack config files
    let railpack_configs = find_railpack_configs(fs, path);
    if railpack_configs.is_empty() {
        results.push(CheckResult {
            id: "D1".to_owned(),
            severity: Severity::Warn,
            title: "No railpack config files found".to_owned(),
            message: "Expected railpack-*.json in project root".to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
    } else {
        results.push(CheckResult {
            id: "D1".to_owned(),
            severity: Severity::Info,
            title: format!("Found {} railpack config(s)", railpack_configs.len()),
            message: railpack_configs
                .iter()
                .filter_map(|p| p.file_name().and_then(|n| n.to_str()))
                .collect::<Vec<_>>()
                .join(", "),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });

        // D2: Check provider field in each config
        for config_path in &railpack_configs {
            check_railpack_provider(fs, config_path, results);
        }
    }

    // D3 & D4: Next.js configs in apps/*/
    check_nextjs_configs(fs, path, results);

    // D5: Tailwind in dependencies
    check_tailwind_deps(fs, path, results);
}

fn find_railpack_configs(fs: &dyn FileSystem, path: &Path) -> Vec<std::path::PathBuf> {
    let mut configs = Vec::new();
    for entry in fs.list_dir(path) {
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with("railpack-") && Path::new(name).extension().is_some_and(|e| e == "json") {
                configs.push(entry.path());
            }
        }
    }
    configs.sort();
    configs
}

#[allow(clippy::disallowed_methods)] // reason: guardrail3 JSON config inspection — not a trust boundary
fn check_railpack_provider(
    fs: &dyn FileSystem,
    config_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let content = match fs.read_file_err(config_path) {
        Ok(content) => content,
        Err(e) => {
            results.push(CheckResult {
                id: "D2".to_owned(),
                severity: Severity::Warn,
                title: "Railpack config unreadable".to_owned(),
                message: format!("{e}"),
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
            });
            return;
        }
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "D2".to_owned(),
                severity: Severity::Error,
                title: "Railpack config invalid JSON".to_owned(),
                message: format!("{e}"),
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
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
                id: "D2".to_owned(),
                severity: Severity::Info,
                title: format!("{filename}: provider = \"{p}\""),
                message: "Provider field present".to_owned(),
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        None => {
            let severity = if looks_like_node {
                Severity::Error
            } else {
                Severity::Warn
            };
            results.push(CheckResult {
                id: "D2".to_owned(),
                severity,
                title: format!("{filename}: no provider field"),
                message: if looks_like_node {
                    "Node.js service needs \"provider\": \"node\" to prevent Rust auto-detection"
                        .to_owned()
                } else {
                    "No provider field — Railpack will auto-detect".to_owned()
                },
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

fn check_nextjs_configs(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let apps_parent = path.join("apps");
    if !apps_parent.is_dir() {
        return;
    }

    for entry in fs.list_dir(&apps_parent) {
        let entry_dir = entry.path();
        if !entry_dir.is_dir() {
            continue;
        }

        // Look for next.config.mjs or next.config.js
        let config_path = if entry_dir.join("next.config.mjs").exists() {
            Some(entry_dir.join("next.config.mjs"))
        } else if entry_dir.join("next.config.js").exists() {
            Some(entry_dir.join("next.config.js"))
        } else if entry_dir.join("next.config.ts").exists() {
            Some(entry_dir.join("next.config.ts"))
        } else {
            None
        };

        let Some(config_path) = config_path else {
            continue; // Not a Next.js app
        };

        let Some(content) = fs.read_file(&config_path) else {
            continue;
        };

        let app_name = entry_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        // D3: standalone output
        if content.contains("standalone") {
            results.push(CheckResult {
                id: "D3".to_owned(),
                severity: Severity::Info,
                title: format!("{app_name}: standalone output configured"),
                message: "output: \"standalone\" found".to_owned(),
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
            });
        } else {
            results.push(CheckResult {
                id: "D3".to_owned(),
                severity: Severity::Error,
                title: format!("{app_name}: standalone output missing"),
                message: "Next.js needs output: \"standalone\" for Railway deployment".to_owned(),
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }

        // D4: outputFileTracingRoot
        if content.contains("outputFileTracingRoot") {
            results.push(CheckResult {
                id: "D4".to_owned(),
                severity: Severity::Info,
                title: format!("{app_name}: outputFileTracingRoot configured"),
                message: "outputFileTracingRoot found".to_owned(),
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
            });
        } else {
            results.push(CheckResult {
                id: "D4".to_owned(),
                severity: Severity::Warn,
                title: format!("{app_name}: outputFileTracingRoot missing"),
                message: "Monorepo needs outputFileTracingRoot pointing to repo root".to_owned(),
                file: Some(config_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}

#[allow(clippy::disallowed_methods)] // reason: guardrail3 JSON config inspection — not a trust boundary
fn check_tailwind_deps(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let apps_parent = path.join("apps");
    if !apps_parent.is_dir() {
        return;
    }

    for entry in fs.list_dir(&apps_parent) {
        let entry_dir = entry.path();
        let pkg_json_path = entry_dir.join("package.json");
        if !pkg_json_path.exists() {
            continue;
        }

        let Some(content) = fs.read_file(&pkg_json_path) else {
            continue;
        };

        let json: serde_json::Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => continue,
        };

        let app_name = entry_dir
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
                id: "D5".to_owned(),
                severity: Severity::Warn,
                title: format!("{app_name}: tailwindcss in devDependencies"),
                message: "Railway skips devDeps — move tailwindcss to dependencies".to_owned(),
                file: Some(pkg_json_path.display().to_string()),
                line: None,
                inventory: false,
            });
        } else if in_deps {
            results.push(CheckResult {
                id: "D5".to_owned(),
                severity: Severity::Info,
                title: format!("{app_name}: tailwindcss in dependencies"),
                message: "Correctly in dependencies (not devDependencies)".to_owned(),
                file: Some(pkg_json_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
        // If neither, it's not a Tailwind app — skip silently
    }
}
