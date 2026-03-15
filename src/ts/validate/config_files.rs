use std::path::Path;

use crate::report::types::{CheckResult, Severity};

pub fn check(path: &Path) -> Vec<CheckResult> {
    let mut results = Vec::new();

    check_eslint_config(path, &mut results);
    check_tsconfig(path, &mut results);
    check_npmrc(path, &mut results);
    check_package_json(path, &mut results);
    check_jscpd(path, &mut results);

    results
}

// ── ESLint config (T1-T8, T40-T49, T50-T51) ──

fn check_eslint_config(path: &Path, results: &mut Vec<CheckResult>) {
    let eslint_path = path.join("eslint.config.mjs");
    if !eslint_path.exists() {
        results.push(CheckResult {
            id: "T1".to_string(),
            severity: Severity::Error,
            title: "eslint.config.mjs missing".to_string(),
            message: "No eslint.config.mjs found at project root".to_string(),
            file: Some(path.display().to_string()),
            line: None,
        });
        return;
    }

    results.push(CheckResult {
        id: "T1".to_string(),
        severity: Severity::Info,
        title: "eslint.config.mjs exists".to_string(),
        message: "Found at project root".to_string(),
        file: Some(eslint_path.display().to_string()),
        line: None,
    });

    let content = match std::fs::read_to_string(&eslint_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    // T2: max-lines with value 300
    check_eslint_rule(
        &content, &eslint_path, "T2", "max-lines",
        Some("300"), Severity::Error, results,
    );

    // T3: max-lines-per-function with value 100
    check_eslint_rule(
        &content, &eslint_path, "T3", "max-lines-per-function",
        Some("100"), Severity::Warn, results,
    );

    // T4: complexity with value 25
    check_eslint_rule(
        &content, &eslint_path, "T4", "complexity",
        Some("25"), Severity::Warn, results,
    );

    // T5: no-restricted-imports
    check_eslint_rule(
        &content, &eslint_path, "T5", "no-restricted-imports",
        None, Severity::Error, results,
    );

    // T6: boundaries or eslint-plugin-boundaries
    if content.contains("boundaries") || content.contains("eslint-plugin-boundaries") {
        results.push(CheckResult {
            id: "T6".to_string(),
            severity: Severity::Info,
            title: "Boundary enforcement configured".to_string(),
            message: "eslint-plugin-boundaries found in config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T6".to_string(),
            severity: Severity::Warn,
            title: "No boundary enforcement".to_string(),
            message: "No boundaries or eslint-plugin-boundaries in config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }

    // T7: Lines containing "off" or "warn" — Info inventory
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if (trimmed.contains("\"off\"") || trimmed.contains("'off'")
            || trimmed.contains("\"warn\"") || trimmed.contains("'warn'"))
            && !trimmed.starts_with("//")
            && !trimmed.starts_with("*")
        {
            results.push(CheckResult {
                id: "T7".to_string(),
                severity: Severity::Info,
                title: "Relaxed ESLint rule".to_string(),
                message: trimmed.to_string(),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
            });
        }
    }

    // T8: File-specific overrides
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains("files:") || trimmed.contains("files =") {
            results.push(CheckResult {
                id: "T8".to_string(),
                severity: Severity::Info,
                title: "File-specific override".to_string(),
                message: trimmed.to_string(),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
            });
        }
    }

    // T40-T49: ESLint rule presence checks
    check_eslint_rule_presence(
        &content, &eslint_path, "T40", "no-floating-promises",
        Severity::Error, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T41", "no-explicit-any",
        Severity::Error, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T42", "no-console",
        Severity::Warn, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T43", "eqeqeq",
        Severity::Warn, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T44", "no-restricted-globals",
        Severity::Error, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T45", "no-cycle",
        Severity::Error, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T46", "max-dependencies",
        Severity::Warn, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T47", "explicit-function-return-type",
        Severity::Warn, results,
    );
    check_eslint_rule_presence(
        &content, &eslint_path, "T48", "strict-boolean-expressions",
        Severity::Warn, results,
    );

    // T49: Test file relaxations
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if (trimmed.contains("test") || trimmed.contains("spec"))
            && (trimmed.contains("files") || trimmed.contains("overrides"))
        {
            results.push(CheckResult {
                id: "T49".to_string(),
                severity: Severity::Info,
                title: "Test file relaxation".to_string(),
                message: trimmed.to_string(),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
            });
        }
    }

    // T50: Route wrapper enforcement
    if content.contains("withBody") || content.contains("withRoute") {
        results.push(CheckResult {
            id: "T50".to_string(),
            severity: Severity::Info,
            title: "Route wrapper enforcement configured".to_string(),
            message: "withBody/withRoute patterns found".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T50".to_string(),
            severity: Severity::Warn,
            title: "No route wrapper enforcement".to_string(),
            message: "No withBody/withRoute patterns in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }

    // T51: process.env ban
    if content.contains("process.env") {
        results.push(CheckResult {
            id: "T51".to_string(),
            severity: Severity::Info,
            title: "process.env restriction configured".to_string(),
            message: "process.env ban found in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T51".to_string(),
            severity: Severity::Error,
            title: "No process.env ban".to_string(),
            message: "No process.env restriction in ESLint config".to_string(),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }
}

fn check_eslint_rule(
    content: &str,
    eslint_path: &Path,
    id: &str,
    rule_name: &str,
    expected_value: Option<&str>,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    if !content.contains(rule_name) {
        results.push(CheckResult {
            id: id.to_string(),
            severity: missing_severity,
            title: format!("{rule_name} not configured"),
            message: format!("No {rule_name} rule found in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
        return;
    }

    if let Some(val) = expected_value {
        // Check if the expected value appears near the rule name
        let has_value = content.lines().any(|line| {
            line.contains(rule_name) && line.contains(val)
        }) || {
            // Check within a few lines of the rule mention
            let lines: Vec<&str> = content.lines().collect();
            let mut found = false;
            for (i, line) in lines.iter().enumerate() {
                if line.contains(rule_name) {
                    // Check surrounding lines (up to 5 lines after)
                    let end = (i.saturating_add(6)).min(lines.len());
                    for check_line in &lines[i..end] {
                        if check_line.contains(val) {
                            found = true;
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
            found
        };

        if has_value {
            results.push(CheckResult {
                id: id.to_string(),
                severity: Severity::Info,
                title: format!("{rule_name} configured"),
                message: format!("{rule_name} with value {val}"),
                file: Some(eslint_path.display().to_string()),
                line: None,
            });
        } else {
            results.push(CheckResult {
                id: id.to_string(),
                severity: missing_severity,
                title: format!("{rule_name} value mismatch"),
                message: format!("{rule_name} found but expected value {val} not detected"),
                file: Some(eslint_path.display().to_string()),
                line: None,
            });
        }
    } else {
        results.push(CheckResult {
            id: id.to_string(),
            severity: Severity::Info,
            title: format!("{rule_name} configured"),
            message: format!("{rule_name} rule found in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }
}

fn check_eslint_rule_presence(
    content: &str,
    eslint_path: &Path,
    id: &str,
    rule_name: &str,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    if content.contains(rule_name) {
        results.push(CheckResult {
            id: id.to_string(),
            severity: Severity::Info,
            title: format!("{rule_name} configured"),
            message: format!("{rule_name} found in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: id.to_string(),
            severity: missing_severity,
            title: format!("{rule_name} missing"),
            message: format!("No {rule_name} rule in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
        });
    }
}

// ── tsconfig (T9-T10, T52-T54) ──

fn check_tsconfig(path: &Path, results: &mut Vec<CheckResult>) {
    let tsconfig_path = path.join("tsconfig.base.json");
    let tsconfig_path = if tsconfig_path.exists() {
        tsconfig_path
    } else {
        let alt = path.join("tsconfig.json");
        if alt.exists() {
            alt
        } else {
            results.push(CheckResult {
                id: "T9".to_string(),
                severity: Severity::Error,
                title: "tsconfig missing".to_string(),
                message: "No tsconfig.base.json or tsconfig.json found".to_string(),
                file: Some(path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    results.push(CheckResult {
        id: "T9".to_string(),
        severity: Severity::Info,
        title: "tsconfig exists".to_string(),
        message: format!("Found: {}", tsconfig_path.display()),
        file: Some(tsconfig_path.display().to_string()),
        line: None,
    });

    let content = match std::fs::read_to_string(&tsconfig_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            results.push(CheckResult {
                id: "T9".to_string(),
                severity: Severity::Error,
                title: "tsconfig parse error".to_string(),
                message: format!("Invalid JSON: {e}"),
                file: Some(tsconfig_path.display().to_string()),
                line: None,
            });
            return;
        }
    };

    let compiler_options = json.get("compilerOptions");

    let required_bools: &[(&str, &str)] = &[
        ("strict", "T9"),
        ("noImplicitReturns", "T9"),
        ("noUnusedLocals", "T9"),
        ("noUnusedParameters", "T9"),
        ("noUncheckedIndexedAccess", "T52"),
        ("exactOptionalPropertyTypes", "T53"),
        ("forceConsistentCasingInFileNames", "T9"),
    ];

    let warn_bools: &[(&str, &str)] = &[
        ("isolatedModules", "T54"),
    ];

    for (key, id) in required_bools {
        let val = compiler_options
            .and_then(|co| co.get(key))
            .and_then(|v| v.as_bool());

        match val {
            Some(true) => {
                results.push(CheckResult {
                    id: id.to_string(),
                    severity: Severity::Info,
                    title: format!("{key}: true"),
                    message: format!("{key} is enabled"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            Some(false) => {
                results.push(CheckResult {
                    id: id.to_string(),
                    severity: Severity::Error,
                    title: format!("{key}: false"),
                    message: format!("{key} should be true"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: id.to_string(),
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
            .and_then(|v| v.as_bool());

        match val {
            Some(true) => {
                results.push(CheckResult {
                    id: id.to_string(),
                    severity: Severity::Info,
                    title: format!("{key}: true"),
                    message: format!("{key} is enabled"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
            _ => {
                results.push(CheckResult {
                    id: id.to_string(),
                    severity: Severity::Warn,
                    title: format!("{key} not enabled"),
                    message: format!("{key} should be true"),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // T10: Extra compiler options — Info inventory
    let known_keys: &[&str] = &[
        "strict", "noImplicitReturns", "noUnusedLocals", "noUnusedParameters",
        "noUncheckedIndexedAccess", "exactOptionalPropertyTypes",
        "forceConsistentCasingInFileNames", "isolatedModules",
        // Common standard options (not flagged as "extra")
        "target", "module", "moduleResolution", "lib", "jsx",
        "outDir", "rootDir", "baseUrl", "paths", "declaration",
        "declarationMap", "sourceMap", "esModuleInterop",
        "allowImportingTsExtensions", "noEmit", "resolveJsonModule",
        "skipLibCheck", "incremental", "tsBuildInfoFile",
        "allowJs", "plugins", "customConditions", "verbatimModuleSyntax",
    ];

    if let Some(co) = compiler_options.and_then(|co| co.as_object()) {
        for key in co.keys() {
            if !known_keys.contains(&key.as_str()) {
                results.push(CheckResult {
                    id: "T10".to_string(),
                    severity: Severity::Info,
                    title: format!("Extra tsconfig option: {key}"),
                    message: format!(
                        "{key} = {}",
                        co.get(key).map_or("?".to_string(), |v| v.to_string())
                    ),
                    file: Some(tsconfig_path.display().to_string()),
                    line: None,
                });
            }
        }
    }
}

// ── .npmrc (T11-T14) ──

fn check_npmrc(path: &Path, results: &mut Vec<CheckResult>) {
    let npmrc_path = path.join(".npmrc");
    if !npmrc_path.exists() {
        results.push(CheckResult {
            id: "T11".to_string(),
            severity: Severity::Error,
            title: ".npmrc missing".to_string(),
            message: "No .npmrc found at project root".to_string(),
            file: Some(path.display().to_string()),
            line: None,
        });
        return;
    }

    results.push(CheckResult {
        id: "T11".to_string(),
        severity: Severity::Info,
        title: ".npmrc exists".to_string(),
        message: "Found at project root".to_string(),
        file: Some(npmrc_path.display().to_string()),
        line: None,
    });

    let content = match std::fs::read_to_string(&npmrc_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    // Parse key=value pairs
    let mut settings: Vec<(String, String)> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }
        if let Some(eq_pos) = trimmed.find('=') {
            let key = trimmed[..eq_pos].trim().to_string();
            let value = trimmed[eq_pos.saturating_add(1)..].trim().to_string();
            settings.push((key, value));
        }
    }

    let expected: &[(&str, &str)] = &[
        ("strict-peer-dependencies", "true"),
        ("disallow-workspace-cycles", "true"),
        ("save-workspace-protocol", "rolling"),
        ("engine-strict", "true"),
        ("package-manager-strict-version", "true"),
        ("strict-dep-builds", "true"),
        ("verify-deps-before-run", "error"),
        ("minimum-release-age", "1440"),
        ("block-exotic-subdeps", "true"),
        ("trust-policy", "warn"),
        ("save-prefix", ""),
        ("shamefully-hoist", "false"),
    ];

    let expected_keys: Vec<&str> = expected.iter().map(|(k, _)| *k).collect();

    // T12: Check each expected setting
    for (key, expected_val) in expected {
        let found = settings.iter().find(|(k, _)| k == key);
        match found {
            Some((_, val)) if val == expected_val => {
                // Correct — no output needed
            }
            Some((_, val)) => {
                // T13: Weaker value
                results.push(CheckResult {
                    id: "T13".to_string(),
                    severity: Severity::Error,
                    title: format!(".npmrc {key} wrong value"),
                    message: format!("Expected \"{expected_val}\", got \"{val}\""),
                    file: Some(npmrc_path.display().to_string()),
                    line: None,
                });
            }
            None => {
                results.push(CheckResult {
                    id: "T12".to_string(),
                    severity: Severity::Error,
                    title: format!(".npmrc {key} missing"),
                    message: format!("Expected {key}={expected_val}"),
                    file: Some(npmrc_path.display().to_string()),
                    line: None,
                });
            }
        }
    }

    // T14: Extra settings not in expected list
    for (key, val) in &settings {
        if !expected_keys.contains(&key.as_str()) {
            results.push(CheckResult {
                id: "T14".to_string(),
                severity: Severity::Info,
                title: format!(".npmrc extra setting: {key}"),
                message: format!("{key}={val}"),
                file: Some(npmrc_path.display().to_string()),
                line: None,
            });
        }
    }
}

// ── package.json (T15-T18, T55-T58) ──

fn check_package_json(path: &Path, results: &mut Vec<CheckResult>) {
    let pkg_path = path.join("package.json");
    if !pkg_path.exists() {
        return;
    }

    let content = match std::fs::read_to_string(&pkg_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return,
    };

    // T15: pnpm.overrides
    let overrides = json.get("pnpm").and_then(|p| p.get("overrides"));
    match overrides {
        Some(ov) if ov.is_object() => {
            let ov_obj = ov.as_object().unwrap(); // reason: just checked is_object
            let has_zod = ov_obj.contains_key("zod");
            let has_eslint_js = ov_obj.contains_key("@eslint/js");

            if !has_zod {
                results.push(CheckResult {
                    id: "T15".to_string(),
                    severity: Severity::Error,
                    title: "pnpm.overrides missing zod".to_string(),
                    message: "No zod override in pnpm.overrides".to_string(),
                    file: Some(pkg_path.display().to_string()),
                    line: None,
                });
            }
            if !has_eslint_js {
                results.push(CheckResult {
                    id: "T15".to_string(),
                    severity: Severity::Error,
                    title: "pnpm.overrides missing @eslint/js".to_string(),
                    message: "No @eslint/js override in pnpm.overrides".to_string(),
                    file: Some(pkg_path.display().to_string()),
                    line: None,
                });
            }

            // T16: Extra overrides
            let known_overrides = ["zod", "@eslint/js"];
            for key in ov_obj.keys() {
                if !known_overrides.contains(&key.as_str()) {
                    results.push(CheckResult {
                        id: "T16".to_string(),
                        severity: Severity::Info,
                        title: format!("Extra pnpm override: {key}"),
                        message: format!(
                            "{key} = {}",
                            ov_obj.get(key).map_or("?".to_string(), |v| v.to_string())
                        ),
                        file: Some(pkg_path.display().to_string()),
                        line: None,
                    });
                }
            }
        }
        _ => {
            results.push(CheckResult {
                id: "T15".to_string(),
                severity: Severity::Error,
                title: "pnpm.overrides missing".to_string(),
                message: "No pnpm.overrides section in package.json".to_string(),
                file: Some(pkg_path.display().to_string()),
                line: None,
            });
        }
    }

    // T17: Banned dependencies
    let banned_deps: &[&str] = &[
        "axios", "lodash", "moment", "uuid", "nanoid", "pg", "express",
        "classnames", "winston", "pino", "request", "got", "superagent",
        "node-fetch", "isomorphic-fetch", "underscore",
    ];
    let banned_prefixes: &[&str] = &["embla-carousel"];

    for section_name in &["dependencies", "devDependencies"] {
        if let Some(deps) = json.get(section_name).and_then(|d| d.as_object()) {
            for dep_name in deps.keys() {
                let is_banned = banned_deps.contains(&dep_name.as_str())
                    || banned_prefixes.iter().any(|p| dep_name.starts_with(p));

                if is_banned {
                    results.push(CheckResult {
                        id: "T17".to_string(),
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
            id: "T18".to_string(),
            severity: Severity::Info,
            title: "packageManager field present".to_string(),
            message: format!(
                "packageManager = {}",
                json.get("packageManager").map_or("?".to_string(), |v| v.to_string())
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T18".to_string(),
            severity: Severity::Warn,
            title: "packageManager field missing".to_string(),
            message: "No packageManager field in package.json".to_string(),
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
                id: "T55".to_string(),
                severity: Severity::Info,
                title: "preinstall enforces pnpm".to_string(),
                message: "preinstall script contains only-allow pnpm".to_string(),
                file: Some(pkg_path.display().to_string()),
                line: None,
            });
        }
        _ => {
            results.push(CheckResult {
                id: "T55".to_string(),
                severity: Severity::Warn,
                title: "preinstall missing pnpm enforcement".to_string(),
                message: "No preinstall script with only-allow pnpm".to_string(),
                file: Some(pkg_path.display().to_string()),
                line: None,
            });
        }
    }

    // T56: prepare script exists
    let prepare = json
        .get("scripts")
        .and_then(|s| s.get("prepare"));

    if prepare.is_some() {
        results.push(CheckResult {
            id: "T56".to_string(),
            severity: Severity::Info,
            title: "prepare script exists".to_string(),
            message: "prepare script found".to_string(),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T56".to_string(),
            severity: Severity::Warn,
            title: "prepare script missing".to_string(),
            message: "No prepare script in package.json".to_string(),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    }

    // T57: engines field
    if json.get("engines").is_some() {
        results.push(CheckResult {
            id: "T57".to_string(),
            severity: Severity::Info,
            title: "engines field present".to_string(),
            message: format!(
                "engines = {}",
                json.get("engines").map_or("?".to_string(), |v| v.to_string())
            ),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    } else {
        results.push(CheckResult {
            id: "T57".to_string(),
            severity: Severity::Warn,
            title: "engines field missing".to_string(),
            message: "No engines field in package.json".to_string(),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    }

    // T58: onlyBuiltDependencies
    if let Some(obd) = json.get("pnpm").and_then(|p| p.get("onlyBuiltDependencies")) {
        results.push(CheckResult {
            id: "T58".to_string(),
            severity: Severity::Info,
            title: "onlyBuiltDependencies configured".to_string(),
            message: format!("onlyBuiltDependencies = {obd}"),
            file: Some(pkg_path.display().to_string()),
            line: None,
        });
    }
}

// ── .jscpd.json (T19-T22) ──

fn check_jscpd(path: &Path, results: &mut Vec<CheckResult>) {
    let jscpd_path = path.join(".jscpd.json");
    if !jscpd_path.exists() {
        results.push(CheckResult {
            id: "T19".to_string(),
            severity: Severity::Warn,
            title: ".jscpd.json missing".to_string(),
            message: "No .jscpd.json found at project root".to_string(),
            file: Some(path.display().to_string()),
            line: None,
        });
        return;
    }

    results.push(CheckResult {
        id: "T19".to_string(),
        severity: Severity::Info,
        title: ".jscpd.json exists".to_string(),
        message: "Found at project root".to_string(),
        file: Some(jscpd_path.display().to_string()),
        line: None,
    });

    let content = match std::fs::read_to_string(&jscpd_path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let json: serde_json::Value = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return,
    };

    // T20: threshold = 0
    match json.get("threshold") {
        Some(serde_json::Value::Number(n)) => {
            let val = n.as_f64().unwrap_or(1.0);
            if val != 0.0 {
                results.push(CheckResult {
                    id: "T20".to_string(),
                    severity: Severity::Error,
                    title: "jscpd threshold not 0".to_string(),
                    message: format!("threshold = {n}, expected 0"),
                    file: Some(jscpd_path.display().to_string()),
                    line: None,
                });
            } else {
                results.push(CheckResult {
                    id: "T20".to_string(),
                    severity: Severity::Info,
                    title: "jscpd threshold correct".to_string(),
                    message: "threshold = 0".to_string(),
                    file: Some(jscpd_path.display().to_string()),
                    line: None,
                });
            }
        }
        _ => {
            results.push(CheckResult {
                id: "T20".to_string(),
                severity: Severity::Error,
                title: "jscpd threshold missing".to_string(),
                message: "No threshold field in .jscpd.json".to_string(),
                file: Some(jscpd_path.display().to_string()),
                line: None,
            });
        }
    }

    // T21: minTokens differs from 50
    if let Some(serde_json::Value::Number(n)) = json.get("minTokens") {
        let val = n.as_u64().unwrap_or(0);
        if val != 50 {
            results.push(CheckResult {
                id: "T21".to_string(),
                severity: Severity::Info,
                title: "jscpd minTokens non-default".to_string(),
                message: format!("minTokens = {val} (default is 50)"),
                file: Some(jscpd_path.display().to_string()),
                line: None,
            });
        }
    }

    // T22: Extra ignore patterns
    if let Some(serde_json::Value::Array(ignore)) = json.get("ignore") {
        for pattern in ignore {
            if let Some(p) = pattern.as_str() {
                results.push(CheckResult {
                    id: "T22".to_string(),
                    severity: Severity::Info,
                    title: "jscpd ignore pattern".to_string(),
                    message: p.to_string(),
                    file: Some(jscpd_path.display().to_string()),
                    line: None,
                });
            }
        }
    }
}
