use std::path::Path;

use crate::report::types::{CheckResult, Severity};

/// A tsconfig boolean check: (`setting_name`, `check_id`).
type TsConfigBool = (&'static str, &'static str);

#[allow(clippy::too_many_lines, clippy::disallowed_methods)] // reason: comprehensive tsconfig validation; guardrail3 JSON config inspection
#[allow(clippy::or_fun_call)] // reason: map_or with function call is intentional for display
pub fn check_tsconfig(path: &Path, results: &mut Vec<CheckResult>) {
    let tsconfig_path = path.join("tsconfig.base.json");
    let tsconfig_path = if tsconfig_path.exists() {
        tsconfig_path
    } else {
        let alt = path.join("tsconfig.json");
        if alt.exists() {
            alt
        } else {
            results.push(CheckResult {
                id: "T9".to_owned(),
                severity: Severity::Error,
                title: "tsconfig missing".to_owned(),
                message: "No tsconfig.base.json or tsconfig.json found".to_owned(),
                file: Some(path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    results.push(CheckResult {
        id: "T9".to_owned(),
        severity: Severity::Info,
        title: "tsconfig exists".to_owned(),
        message: format!("Found: {}", tsconfig_path.display()),
        file: Some(tsconfig_path.display().to_string()),
        line: None,
    });

    let Some(content) = crate::fs::read_file(&tsconfig_path) else {
        return;
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "T9".to_owned(),
                severity: Severity::Error,
                title: "tsconfig parse error".to_owned(),
                message: format!("Invalid JSON: {e}"),
                file: Some(tsconfig_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let compiler_options = json.get("compilerOptions");
    let required_bools: &[TsConfigBool] = &[
        ("strict", "T9"),
        ("noImplicitReturns", "T9"),
        ("noUnusedLocals", "T9"),
        ("noUnusedParameters", "T9"),
        ("noUncheckedIndexedAccess", "T52"),
        ("exactOptionalPropertyTypes", "T53"),
        ("forceConsistentCasingInFileNames", "T9"),
    ];

    // Settings that must be true
    let additional_required_bools: &[TsConfigBool] = &[
        ("noPropertyAccessFromIndexSignature", "T60"),
        ("noImplicitOverride", "T61"),
        ("noFallthroughCasesInSwitch", "T62"),
    ];

    // Settings that must be false
    let required_false_bools: &[TsConfigBool] = &[
        ("allowUnreachableCode", "T63"),
        ("allowUnusedLabels", "T64"),
    ];

    let warn_bools: &[TsConfigBool] = &[("isolatedModules", "T54")];

    for (key, id) in required_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);

        match val {
            Some(true) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Info,
                    title: format!("{key}: true"),
                    message: format!("{key} is enabled"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            Some(false) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("{key}: false"),
                    message: format!("{key} should be true"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("{key} missing"),
                    message: format!("{key} not set in compilerOptions"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    for (key, id) in warn_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);

        match val {
            Some(true) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Info,
                    title: format!("{key}: true"),
                    message: format!("{key} is enabled"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            _ => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Warn,
                    title: format!("{key} not enabled"),
                    message: format!("{key} should be true"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // Additional required true bools
    for (key, id) in additional_required_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);

        match val {
            Some(true) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Info,
                    title: format!("{key}: true"),
                    message: format!("{key} is enabled"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            Some(false) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("{key}: false"),
                    message: format!("{key} should be true"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("{key} missing"),
                    message: format!("{key} not set in compilerOptions"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // Required false bools (must be explicitly set to false)
    for (key, id) in required_false_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(serde_json::Value::as_bool);

        match val {
            Some(false) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Info,
                    title: format!("{key}: false"),
                    message: format!("{key} is correctly set to false"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            Some(true) => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("{key}: true"),
                    message: format!("{key} should be false"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: (*id).to_owned(),
                    severity: Severity::Error,
                    title: format!("{key} missing"),
                    message: format!("{key} not set in compilerOptions (should be false)"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // T10: Extra compiler options — Info inventory
    let known_keys: &[&str] = &[
        "strict",
        "noImplicitReturns",
        "noUnusedLocals",
        "noUnusedParameters",
        "noUncheckedIndexedAccess",
        "exactOptionalPropertyTypes",
        "forceConsistentCasingInFileNames",
        "isolatedModules",
        "noPropertyAccessFromIndexSignature",
        "noImplicitOverride",
        "noFallthroughCasesInSwitch",
        "allowUnreachableCode",
        "allowUnusedLabels",
        // Common standard options (not flagged as "extra")
        "target",
        "module",
        "moduleResolution",
        "lib",
        "jsx",
        "outDir",
        "rootDir",
        "baseUrl",
        "paths",
        "declaration",
        "declarationMap",
        "sourceMap",
        "esModuleInterop",
        "allowImportingTsExtensions",
        "noEmit",
        "resolveJsonModule",
        "skipLibCheck",
        "incremental",
        "tsBuildInfoFile",
        "allowJs",
        "plugins",
        "customConditions",
        "verbatimModuleSyntax",
    ];

    if let Some(co) = compiler_options.and_then(|co| co.as_object()) {
        for key in co.keys() {
            if !known_keys.contains(&key.as_str()) {
                results.push(CheckResult {
                    id: "T10".to_owned(),
                    severity: Severity::Info,
                    title: format!("Extra tsconfig option: {key}"),
                    message: format!(
                        "{key} = {}",
                        co.get(key)
                            .map_or("?".to_owned(), std::string::ToString::to_string)
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
        }
    }
}
