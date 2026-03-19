use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// Config file names for cspell.
const CSPELL_CONFIG_FILES: &[&str] = &[
    "cspell.json",
    ".cspell.json",
    "cspell.config.js",
    "cspell.config.cjs",
    "cspell.config.yaml",
    "cspell.config.yml",
];

/// Check tool configurations and scripts.
#[allow(clippy::disallowed_methods)] // reason: serde_json::from_str for package.json inspection
#[allow(clippy::too_many_lines)] // reason: checks multiple tool configs sequentially
pub fn check_tool_configs(
    fs: &dyn FileSystem,
    path: &Path,
    content_enabled: bool,
    results: &mut Vec<CheckResult>,
) {
    // T-TOOL-07: cspell config exists
    check_cspell_config(fs, path, results);

    // Script checks from package.json
    let pkg_path = path.join("package.json");
    if let Some(content) = fs.read_file(&pkg_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let scripts = json.get("scripts").and_then(|s| s.as_object());

            // T-TOOL-08: type-coverage script
            check_script(
                scripts,
                "T-TOOL-08",
                "type-coverage",
                "type-coverage --at-least 95",
                Severity::Error,
                &pkg_path,
                results,
            );

            // T-TOOL-09: license-check script
            check_script(
                scripts,
                "T-TOOL-09",
                "license-check",
                "license-checker --onlyAllow '...'",
                Severity::Error,
                &pkg_path,
                results,
            );

            // T-TOOL-10: audit script
            check_script(
                scripts,
                "T-TOOL-10",
                "audit",
                "pnpm audit --prod",
                Severity::Error,
                &pkg_path,
                results,
            );

            // T-TOOL-11: size-limit config (content profile)
            if content_enabled {
                let has_size_limit = json.get("size-limit").is_some();
                if has_size_limit {
                    results.push(
                        CheckResult {
                            id: "T-TOOL-11".to_owned(),
                            severity: Severity::Info,
                            title: "size-limit config found".to_owned(),
                            message: "size-limit configuration found in package.json.".to_owned(),
                            file: Some(pkg_path.display().to_string()),
                            line: None,
                            inventory: false,
                        }
                        .as_inventory(),
                    );
                } else {
                    results.push(CheckResult {
                        id: "T-TOOL-11".to_owned(),
                        severity: Severity::Warn,
                        title: "size-limit config missing".to_owned(),
                        message: "No \"size-limit\" config in package.json. Add a size-limit array with path and limit entries for bundle size budgets.".to_owned(),
                        file: Some(pkg_path.display().to_string()),
                        line: None,
                        inventory: false,
                    });
                }
            }
        }
    }
}

fn check_cspell_config(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    for filename in CSPELL_CONFIG_FILES {
        let p = path.join(filename);
        if fs.read_file(&p).is_some() {
            results.push(
                CheckResult {
                    id: "T-TOOL-07".to_owned(),
                    severity: Severity::Info,
                    title: "cspell config found".to_owned(),
                    message: format!("Spell check config {filename} found."),
                    file: Some(p.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
            return;
        }
    }
    results.push(CheckResult {
        id: "T-TOOL-07".to_owned(),
        severity: Severity::Error,
        title: "cspell config missing".to_owned(),
        message: "No cspell config file found. Create cspell.json with language, ignorePaths, and project-specific words for spell checking.".to_owned(),
        file: Some(path.display().to_string()),
        line: None,
        inventory: false,
    });
}

/// Type alias for JSON object map.
type JsonMap = serde_json::Map<String, serde_json::Value>;

fn check_script(
    scripts: Option<&JsonMap>,
    check_id: &str,
    script_name: &str,
    example: &str,
    missing_severity: Severity,
    pkg_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let found = scripts.is_some_and(|s| s.contains_key(script_name));
    if found {
        results.push(
            CheckResult {
                id: check_id.to_owned(),
                severity: Severity::Info,
                title: format!("\"{script_name}\" script configured"),
                message: format!("\"{script_name}\" script found in package.json."),
                file: Some(pkg_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: missing_severity,
            title: format!("\"{script_name}\" script missing"),
            message: format!(
                "No \"{script_name}\" script in package.json. Add: \"{script_name}\": \"{example}\""
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
