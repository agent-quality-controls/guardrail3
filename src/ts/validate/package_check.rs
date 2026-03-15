use std::path::Path;

use crate::report::types::{CheckResult, Severity};

#[allow(clippy::too_many_lines, clippy::disallowed_methods)] // reason: comprehensive package.json validation; guardrail3 JSON config inspection
#[allow(clippy::or_fun_call)] // reason: map_or with function call is intentional for display
pub fn check_package_json(path: &Path, results: &mut Vec<CheckResult>) {
    let pkg_path = path.join("package.json");
    if !pkg_path.exists() {
        return;
    }

    let Some(content) = crate::fs::read_file(&pkg_path) else {
        return;
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return,
    };

    // T15: pnpm.overrides
    let overrides = json.get("pnpm").and_then(|p| p.get("overrides"));
    match overrides {
        Some(ov) if ov.is_object() => {
            let Some(ov_obj) = ov.as_object() else {
                return;
            };
            let has_zod = ov_obj.contains_key("zod");
            let has_eslint_js = ov_obj.contains_key("@eslint/js");

            if !has_zod {
                results.push(CheckResult {
                    id: "T15".to_owned(),
                    severity: Severity::Error,
                    title: "pnpm.overrides missing zod".to_owned(),
                    message: "No zod override in pnpm.overrides".to_owned(),
                    file: Some(pkg_path.display().to_string()),
                    line: None,
                });
            }
            if !has_eslint_js {
                results.push(CheckResult {
                    id: "T15".to_owned(),
                    severity: Severity::Error,
                    title: "pnpm.overrides missing @eslint/js".to_owned(),
                    message: "No @eslint/js override in pnpm.overrides".to_owned(),
                    file: Some(pkg_path.display().to_string()),
                    line: None,
                });
            }

            // T16: Extra overrides
            let known_overrides = ["zod", "@eslint/js"];
            for key in ov_obj.keys() {
                if !known_overrides.contains(&key.as_str()) {
                    results.push(CheckResult {
                        id: "T16".to_owned(),
                        severity: Severity::Info,
                        title: format!("Extra pnpm override: {key}"),
                        message: format!(
                            "{key} = {}",
                            ov_obj
                                .get(key)
                                .map_or("?".to_owned(), std::string::ToString::to_string)
                        ),
                        file: Some(pkg_path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
        _ => {
            results.push(CheckResult {
                id: "T15".to_owned(),
                severity: Severity::Error,
                title: "pnpm.overrides missing".to_owned(),
                message: "No pnpm.overrides section in package.json".to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
            });
        }
    }

    // T17: Banned dependencies
    let banned_deps: &[&str] = &[
        "axios",
        "lodash",
        "moment",
        "uuid",
        "nanoid",
        "pg",
        "express",
        "classnames",
        "winston",
        "pino",
        "request",
        "got",
        "superagent",
        "node-fetch",
        "isomorphic-fetch",
        "underscore",
        "request-promise",
        "postgres",
        "cross-fetch",
    ];
    let banned_prefixes: &[&str] = &["embla-carousel"];

    for section_name in &["dependencies", "devDependencies"] {
        if let Some(deps) = json.get(section_name).and_then(|d| d.as_object()) {
            for dep_name in deps.keys() {
                let is_banned = banned_deps.contains(&dep_name.as_str())
                    || banned_prefixes.iter().any(|p| dep_name.starts_with(p));

                if is_banned {
                    results.push(CheckResult {
                        id: "T17".to_owned(),
                        severity: Severity::Error,
                        title: format!("Banned dependency: {dep_name}"),
                        message: format!("{dep_name} found in {section_name}"),
                        file: Some(pkg_path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
    }

    // T18: packageManager field
    if json.get("packageManager").is_some() {
        results.push(CheckResult {
            id: "T18".to_owned(),
            severity: Severity::Info,
            title: "packageManager field present".to_owned(),
            message: format!(
                "packageManager = {}",
                json.get("packageManager")
                    .map_or("?".to_owned(), std::string::ToString::to_string)
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T18".to_owned(),
            severity: Severity::Warn,
            title: "packageManager field missing".to_owned(),
            message: "No packageManager field in package.json".to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    }

    // T55: preinstall script contains only-allow pnpm
    let preinstall = json
        .get("scripts")
        .and_then(|s| s.get("preinstall"))
        .and_then(|v| v.as_str());

    match preinstall {
        Some(script) if script.contains("only-allow pnpm") => {
            results.push(CheckResult {
                id: "T55".to_owned(),
                severity: Severity::Info,
                title: "preinstall enforces pnpm".to_owned(),
                message: "preinstall script contains only-allow pnpm".to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
            });
        }
        _ => {
            results.push(CheckResult {
                id: "T55".to_owned(),
                severity: Severity::Warn,
                title: "preinstall missing pnpm enforcement".to_owned(),
                message: "No preinstall script with only-allow pnpm".to_owned(),
                file: Some(pkg_path.display().to_string()),
                line: None,
            });
        }
    }

    // T56: prepare script exists
    let prepare = json.get("scripts").and_then(|s| s.get("prepare"));

    if prepare.is_some() {
        results.push(CheckResult {
            id: "T56".to_owned(),
            severity: Severity::Info,
            title: "prepare script exists".to_owned(),
            message: "prepare script found".to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T56".to_owned(),
            severity: Severity::Warn,
            title: "prepare script missing".to_owned(),
            message: "No prepare script in package.json".to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    }

    // T57: engines field
    if json.get("engines").is_some() {
        results.push(CheckResult {
            id: "T57".to_owned(),
            severity: Severity::Info,
            title: "engines field present".to_owned(),
            message: format!(
                "engines = {}",
                json.get("engines")
                    .map_or("?".to_owned(), std::string::ToString::to_string)
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T57".to_owned(),
            severity: Severity::Warn,
            title: "engines field missing".to_owned(),
            message: "No engines field in package.json".to_owned(),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    }

    // T58: onlyBuiltDependencies
    if let Some(obd) = json
        .get("pnpm")
        .and_then(|p| p.get("onlyBuiltDependencies"))
    {
        results.push(CheckResult {
            id: "T58".to_owned(),
            severity: Severity::Info,
            title: "onlyBuiltDependencies configured".to_owned(),
            message: format!("onlyBuiltDependencies = {obd}"),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    }
}
