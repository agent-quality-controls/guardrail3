use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

fn deploy_result(
    id: &str,
    severity: Severity,
    title: String,
    message: String,
    file: Option<String>,
) -> CheckResult {
    CheckResult::new(id.to_owned(), severity, title, message).with_optional_file(file)
}

pub fn check_deployment(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let railpack_configs = find_railpack_configs(fs, path);
    if railpack_configs.is_empty() {
        results.push(deploy_result(
            "D1",
            Severity::Warn,
            "No railpack config files found".to_owned(),
            "Expected railpack-*.json in project root".to_owned(),
            Some(path.display().to_string()),
        ));
    } else {
        results.push(deploy_result(
            "D1",
            Severity::Info,
            format!("Found {} railpack config(s)", railpack_configs.len()),
            railpack_configs
                .iter()
                .filter_map(|p| p.file_name().and_then(|n| n.to_str()))
                .collect::<Vec<_>>()
                .join(", "),
            Some(path.display().to_string()),
        ));

        for config_path in &railpack_configs {
            check_railpack_provider(fs, config_path, results);
        }
    }

    check_nextjs_configs(fs, path, results);
    check_tailwind_deps(fs, path, results);
}

fn find_railpack_configs(fs: &dyn FileSystem, path: &Path) -> Vec<std::path::PathBuf> {
    let mut configs = Vec::new();
    for entry in fs.list_dir(path) {
        if let Some(name) = entry.file_name().to_str() {
            if name.starts_with("railpack-")
                && Path::new(name).extension().is_some_and(|e| e == "json")
            {
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
            results.push(deploy_result(
                "D2",
                Severity::Warn,
                "Railpack config unreadable".to_owned(),
                format!("{e}"),
                Some(config_path.display().to_string()),
            ));
            return;
        }
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            results.push(deploy_result(
                "D2",
                Severity::Error,
                "Railpack config invalid JSON".to_owned(),
                format!("{e}"),
                Some(config_path.display().to_string()),
            ));
            return;
        }
    };

    let provider = json.get("provider").and_then(|v| v.as_str());
    let filename = config_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    let looks_like_node = filename.contains("web") || filename.contains("landing");

    match provider {
        Some(p) => {
            results.push(deploy_result(
                "D2",
                Severity::Info,
                format!("{filename}: provider = \"{p}\""),
                "Provider field present".to_owned(),
                Some(config_path.display().to_string()),
            ));
        }
        None => {
            let severity = if looks_like_node {
                Severity::Error
            } else {
                Severity::Warn
            };
            let message = if looks_like_node {
                "Node.js service needs \"provider\": \"node\" to prevent Rust auto-detection"
                    .to_owned()
            } else {
                "No provider field — Railpack will auto-detect".to_owned()
            };
            results.push(deploy_result(
                "D2",
                severity,
                format!("{filename}: no provider field"),
                message,
                Some(config_path.display().to_string()),
            ));
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
            continue;
        };

        let Some(content) = fs.read_file(&config_path) else {
            continue;
        };

        let app_name = entry_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        if content.contains("standalone") {
            results.push(deploy_result(
                "D3",
                Severity::Info,
                format!("{app_name}: standalone output configured"),
                "output: \"standalone\" found".to_owned(),
                Some(config_path.display().to_string()),
            ));
        } else {
            results.push(deploy_result(
                "D3",
                Severity::Error,
                format!("{app_name}: standalone output missing"),
                "Next.js needs output: \"standalone\" for Railway deployment".to_owned(),
                Some(config_path.display().to_string()),
            ));
        }

        if content.contains("outputFileTracingRoot") {
            results.push(deploy_result(
                "D4",
                Severity::Info,
                format!("{app_name}: outputFileTracingRoot configured"),
                "outputFileTracingRoot found".to_owned(),
                Some(config_path.display().to_string()),
            ));
        } else {
            results.push(deploy_result(
                "D4",
                Severity::Warn,
                format!("{app_name}: outputFileTracingRoot missing"),
                "Monorepo needs outputFileTracingRoot pointing to repo root".to_owned(),
                Some(config_path.display().to_string()),
            ));
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
            results.push(deploy_result(
                "D5",
                Severity::Warn,
                format!("{app_name}: tailwindcss in devDependencies"),
                "Railway skips devDeps — move tailwindcss to dependencies".to_owned(),
                Some(pkg_json_path.display().to_string()),
            ));
        } else if in_deps {
            results.push(deploy_result(
                "D5",
                Severity::Info,
                format!("{app_name}: tailwindcss in dependencies"),
                "Correctly in dependencies (not devDependencies)".to_owned(),
                Some(pkg_json_path.display().to_string()),
            ));
        }
    }
}
