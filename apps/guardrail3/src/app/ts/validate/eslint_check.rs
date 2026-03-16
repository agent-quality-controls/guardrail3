use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

/// A rule definition: (`check_id`, `rule_name`, `severity_if_missing`).
type RuleDef = (&'static str, &'static str, Severity);

pub fn check_eslint_config(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    let eslint_path = path.join("eslint.config.mjs");
    if !eslint_path.exists() {
        results.push(CheckResult {
            id: "T1".to_owned(),
            severity: Severity::Error,
            title: "eslint.config.mjs missing".to_owned(),
            message: "No eslint.config.mjs found at project root".to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    results.push(CheckResult {
        id: "T1".to_owned(),
        severity: Severity::Info,
        title: "eslint.config.mjs exists".to_owned(),
        message: "Found at project root".to_owned(),
        file: Some(eslint_path.display().to_string()),
        line: None,
        inventory: false,
    }.as_inventory());

    let Some(content) = fs.read_file(&eslint_path) else {
        return;
    };

    check_eslint_value_rules(&content, &eslint_path, results);
    check_boundary_enforcement(&content, &eslint_path, results);
    check_relaxed_rules(&content, &eslint_path, results);
    check_file_overrides(&content, &eslint_path, results);
    check_rule_presence_t40_t48(&content, &eslint_path, results);
    check_all_eslint_rules(&content, &eslint_path, results);
    check_test_relaxations(&content, &eslint_path, results);
    check_route_wrappers(&content, &eslint_path, results);
    check_process_env_ban(&content, &eslint_path, results);
}

/// T2-T5: `ESLint` rules with expected values.
fn check_eslint_value_rules(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    check_eslint_rule(content, eslint_path, "T2", "max-lines", Some("300"), Severity::Error, results);
    check_eslint_rule(content, eslint_path, "T3", "max-lines-per-function", Some("100"), Severity::Warn, results);
    check_eslint_rule(content, eslint_path, "T4", "complexity", Some("25"), Severity::Warn, results);
    check_eslint_rule(content, eslint_path, "T5", "no-restricted-imports", None, Severity::Error, results);
}

/// T6: Boundary enforcement (boundaries or eslint-plugin-boundaries).
fn check_boundary_enforcement(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    if content.contains("boundaries") || content.contains("eslint-plugin-boundaries") {
        results.push(CheckResult {
            id: "T6".to_owned(),
            severity: Severity::Info,
            title: "Boundary enforcement configured".to_owned(),
            message: "eslint-plugin-boundaries found in config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T6".to_owned(),
            severity: Severity::Warn,
            title: "No boundary enforcement".to_owned(),
            message: "No boundaries or eslint-plugin-boundaries in config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T7: Lines containing "off" or "warn" — Info inventory.
/// T8: File-specific overrides.
fn check_relaxed_rules(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if (trimmed.contains("\"off\"")
            || trimmed.contains("'off'")
            || trimmed.contains("\"warn\"")
            || trimmed.contains("'warn'"))
            && !trimmed.starts_with("//")
            && !trimmed.starts_with('*')
        {
            results.push(CheckResult {
                id: "T7".to_owned(),
                severity: Severity::Info,
                title: "Relaxed ESLint rule".to_owned(),
                message: trimmed.to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            });
        }
    }
}

/// T8: File-specific overrides.
fn check_file_overrides(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.contains("files:") || trimmed.contains("files =") {
            results.push(CheckResult {
                id: "T8".to_owned(),
                severity: Severity::Info,
                title: "File-specific override".to_owned(),
                message: trimmed.to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            });
        }
    }
}

/// T40-T48: `ESLint` rule presence checks.
fn check_rule_presence_t40_t48(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    check_eslint_rule_presence(content, eslint_path, "T40", "no-floating-promises", Severity::Error, results);
    check_eslint_rule_presence(content, eslint_path, "T41", "no-explicit-any", Severity::Error, results);
    check_eslint_rule_presence(content, eslint_path, "T42", "no-console", Severity::Warn, results);
    check_eslint_rule_presence(content, eslint_path, "T43", "eqeqeq", Severity::Warn, results);
    check_eslint_rule_presence(content, eslint_path, "T44", "no-restricted-globals", Severity::Error, results);
    check_eslint_rule_presence(content, eslint_path, "T45", "no-cycle", Severity::Error, results);
    check_eslint_rule_presence(content, eslint_path, "T46", "max-dependencies", Severity::Warn, results);
    check_eslint_rule_presence(content, eslint_path, "T47", "explicit-function-return-type", Severity::Warn, results);
    check_eslint_rule_presence(content, eslint_path, "T48", "strict-boolean-expressions", Severity::Warn, results);
}

/// T49: Test file relaxations.
fn check_test_relaxations(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if (trimmed.contains("test") || trimmed.contains("spec"))
            && (trimmed.contains("files") || trimmed.contains("overrides"))
        {
            results.push(CheckResult {
                id: "T49".to_owned(),
                severity: Severity::Info,
                title: "Test file relaxation".to_owned(),
                message: trimmed.to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            }.as_inventory());
        }
    }
}

/// T50: Route wrapper enforcement.
fn check_route_wrappers(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    if content.contains("withBody") || content.contains("withRoute") {
        results.push(CheckResult {
            id: "T50".to_owned(),
            severity: Severity::Info,
            title: "Route wrapper enforcement configured".to_owned(),
            message: "withBody/withRoute patterns found".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T50".to_owned(),
            severity: Severity::Warn,
            title: "No route wrapper enforcement".to_owned(),
            message: "No withBody/withRoute patterns in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T51: process.env ban.
fn check_process_env_ban(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    if content.contains("process.env") {
        results.push(CheckResult {
            id: "T51".to_owned(),
            severity: Severity::Info,
            title: "process.env restriction configured".to_owned(),
            message: "process.env ban found in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T51".to_owned(),
            severity: Severity::Error,
            title: "No process.env ban".to_owned(),
            message: "No process.env restriction in ESLint config".to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

#[allow(clippy::string_slice)] // reason: parsing known ASCII ESLint rule names
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
            id: id.to_owned(),
            severity: missing_severity,
            title: format!("{rule_name} not configured"),
            message: format!("No {rule_name} rule found in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    if let Some(val) = expected_value {
        // Check if the expected value appears near the rule name
        let has_value = content
            .lines()
            .any(|line| line.contains(rule_name) && line.contains(val))
            || {
                // Check within a few lines of the rule mention
                let lines: Vec<&str> = content.lines().collect();
                let mut found = false;
                for (i, line) in lines.iter().enumerate() {
                    if line.contains(rule_name) {
                        // Check surrounding lines (up to 5 lines after)
                        let end = (i.saturating_add(6)).min(lines.len());
                        for check_line in lines.get(i..end).unwrap_or_default() {
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
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("{rule_name} configured"),
                message: format!("{rule_name} with value {val}"),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        } else {
            results.push(CheckResult {
                id: id.to_owned(),
                severity: missing_severity,
                title: format!("{rule_name} value mismatch"),
                message: format!("{rule_name} found but expected value {val} not detected"),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    } else {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Info,
            title: format!("{rule_name} configured"),
            message: format!("{rule_name} rule found in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    }
}

/// Check all expected `ESLint` rules from the template.
/// Each rule is checked for presence (`content.contains(rule_name)`).
fn check_all_eslint_rules(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    // (check_id, rule_name, severity_if_missing)
    let rules: &[RuleDef] = &[
        ("T60", "no-misused-promises", Severity::Warn),
        ("T61", "await-thenable", Severity::Warn),
        ("T62", "consistent-type-imports", Severity::Warn),
        ("T63", "no-non-null-assertion", Severity::Warn),
        ("T64", "switch-exhaustiveness-check", Severity::Warn),
        ("T65", "no-unused-vars", Severity::Warn),
        ("T66", "require-await", Severity::Warn),
        ("T67", "no-param-reassign", Severity::Warn),
        ("T68", "no-unsafe-assignment", Severity::Warn),
        ("T69", "no-unsafe-member-access", Severity::Warn),
        ("T70", "no-unsafe-call", Severity::Warn),
        ("T71", "no-unsafe-return", Severity::Warn),
        ("T72", "no-unsafe-argument", Severity::Warn),
        ("T73", "explicit-module-boundary-types", Severity::Warn),
        ("T74", "promise-function-async", Severity::Warn),
        ("T75", "consistent-type-exports", Severity::Warn),
        ("T76", "consistent-type-definitions", Severity::Warn),
        ("T77", "no-unnecessary-condition", Severity::Warn),
        ("T78", "prefer-nullish-coalescing", Severity::Warn),
        ("T79", "prefer-optional-chain", Severity::Warn),
        ("T80", "no-deprecated", Severity::Warn),
        ("T81", "restrict-template-expressions", Severity::Warn),
        ("T82", "no-throw-literal", Severity::Warn),
        ("T83", "no-empty", Severity::Warn),
    ];

    for (id, rule_name, severity) in rules {
        check_eslint_rule_presence(content, eslint_path, id, rule_name, *severity, results);
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
            id: id.to_owned(),
            severity: Severity::Info,
            title: format!("{rule_name} configured"),
            message: format!("{rule_name} found in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: missing_severity,
            title: format!("{rule_name} missing"),
            message: format!("No {rule_name} rule in ESLint config"),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
