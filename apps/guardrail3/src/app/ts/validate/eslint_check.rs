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
            title: "ESLint config `eslint.config.mjs` not found".to_owned(),
            message: "ESLint enforces code quality rules (no-unused-vars, naming conventions, import order, \
                     type safety). Without it, no static analysis runs on TypeScript code. \
                     Run `guardrail3 ts generate` to create it, or create `eslint.config.mjs` manually \
                     with the flat config format.".to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    results.push(CheckResult {
        id: "T1".to_owned(),
        severity: Severity::Info,
        title: "ESLint config exists".to_owned(),
        message: "ESLint flat config `eslint.config.mjs` found at project root.".to_owned(),
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
            title: "Import boundary enforcement configured".to_owned(),
            message: "`eslint-plugin-boundaries` found in config. This enforces hexagonal architecture \
                     import rules — domain cannot import adapters, ports cannot import application, etc."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T6".to_owned(),
            severity: Severity::Warn,
            title: "No import boundary enforcement".to_owned(),
            message: "No `eslint-plugin-boundaries` found in ESLint config. Without boundary enforcement, \
                     domain code can accidentally import from adapters, creating tight coupling that makes \
                     the codebase harder to test and refactor. Install `eslint-plugin-boundaries` and configure \
                     zone definitions in `eslint.config.mjs`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T7: Lines containing "off" or "warn" — Info inventory.
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
                title: "ESLint rule relaxed to off/warn".to_owned(),
                message: format!(
                    "Rule set to `off` or `warn`: `{trimmed}`. Rules turned off disable protection entirely; \
                     rules set to `warn` allow the build to pass with violations. Review whether this relaxation \
                     is justified and add `// EXCEPTION: <reason>` if intentional."
                ),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            }.as_inventory());
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
                title: "File-specific ESLint override".to_owned(),
                message: format!(
                    "File-scoped rule override: `{trimmed}`. File overrides apply different rules to specific \
                     file patterns (e.g., relaxed rules for test files). Verify the scope is narrow and justified."
                ),
                file: Some(eslint_path.display().to_string()),
                line: Some(line_num.saturating_add(1)),
                inventory: false,
            }.as_inventory());
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
                title: "Test file ESLint relaxation".to_owned(),
                message: format!(
                    "Test-specific rule override: `{trimmed}`. Test files often need relaxed rules \
                     (e.g., no-explicit-any for mocks, max-lines for integration tests). \
                     Verify relaxations are scoped only to test files."
                ),
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
            message: "`withBody`/`withRoute` patterns found in ESLint config. Route wrappers ensure \
                     all API routes go through validation and error handling middleware."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T50".to_owned(),
            severity: Severity::Warn,
            title: "No route wrapper enforcement in ESLint".to_owned(),
            message: "No `withBody`/`withRoute` patterns found in ESLint config. Route wrappers ensure \
                     all API endpoints validate input and handle errors consistently. Add restricted import \
                     rules that require route handlers to use wrapper functions."
                .to_owned(),
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
            title: "`process.env` restriction configured in ESLint".to_owned(),
            message: "`process.env` ban found in ESLint config. This forces environment variable access \
                     through a centralized env module, making configuration auditable and validated."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: "T51".to_owned(),
            severity: Severity::Error,
            title: "No `process.env` restriction in ESLint".to_owned(),
            message: "No `process.env` restriction found in ESLint config. Without this, any file can read \
                     environment variables directly, scattering configuration across the codebase and making it \
                     impossible to audit what config a service needs. Add a `no-restricted-globals` or \
                     `no-restricted-properties` rule banning `process.env` in `eslint.config.mjs`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Return a short explanation of what an `ESLint` rule does and why it matters.
fn eslint_rule_explanation(rule_name: &str) -> &'static str {
    match rule_name {
        "max-lines" => " This rule limits file length, preventing files from growing too large to reason about.",
        "max-lines-per-function" => " This rule limits function length, keeping functions focused and testable.",
        "complexity" => " This rule limits cyclomatic complexity, preventing deeply nested control flow that causes bugs.",
        "no-restricted-imports" => " This rule bans specific imports (e.g., banned packages), enforcing approved alternatives.",
        "no-floating-promises" => " Unhandled promises silently swallow errors. This rule requires all promises to be awaited or explicitly handled.",
        "no-explicit-any" => " Using `any` disables type checking. This rule forces proper typing or `unknown` with runtime checks.",
        "no-console" => " Console statements left in production code create noise in logs. Use a structured logger instead.",
        "eqeqeq" => " `==` performs type coercion (`0 == ''` is true). This rule enforces `===` for predictable comparisons.",
        "no-restricted-globals" => " Some globals (e.g., `event`, `name`) shadow local variables silently. This rule bans dangerous globals.",
        "no-cycle" => " Circular imports cause initialization order bugs and make code impossible to tree-shake. This rule detects import cycles.",
        "max-dependencies" => " Too many imports indicate a module is doing too much. This rule limits import count per file.",
        "explicit-function-return-type" => " Without explicit return types, TypeScript infers types that may change unexpectedly. This rule ensures return types are documented.",
        "strict-boolean-expressions" => " Prevents truthy/falsy coercion (`if (str)` passes for any non-empty string). Forces explicit boolean checks.",
        "no-misused-promises" => " Catches promises used in boolean contexts or passed where void is expected, which silently drops errors.",
        "await-thenable" => " Catches `await` on non-promise values, which indicates a logic error.",
        "consistent-type-imports" => " Ensures `import type` is used for type-only imports, enabling better tree-shaking and faster builds.",
        "no-non-null-assertion" => " The `!` postfix bypasses null checks without runtime validation. Use optional chaining or explicit checks.",
        "switch-exhaustiveness-check" => " Ensures switch statements handle all union variants, catching missing cases at compile time.",
        "no-unused-vars" => " Dead code clutters the codebase and confuses readers. This rule catches variables declared but never used.",
        "require-await" => " Functions marked `async` that don't use `await` misleadingly wrap returns in promises. Remove `async` or add awaited calls.",
        "no-param-reassign" => " Reassigning function parameters creates confusing side effects. Use a new variable instead.",
        "no-unsafe-assignment" => " Catches `any` values being assigned to typed variables, which defeats type safety.",
        "no-unsafe-member-access" => " Catches property access on `any` values, which bypasses type checking.",
        "no-unsafe-call" => " Catches function calls on `any` values, which can fail at runtime with no type protection.",
        "no-unsafe-return" => " Catches `any` values being returned from typed functions, spreading type unsafety to callers.",
        "no-unsafe-argument" => " Catches `any` values passed as arguments to typed parameters, defeating the callee's type safety.",
        "explicit-module-boundary-types" => " Exported functions without explicit types create fragile public APIs whose types change implicitly.",
        "promise-function-async" => " Functions returning promises should be `async` for consistent error handling and stack traces.",
        "consistent-type-exports" => " Ensures `export type` for type-only exports, enabling better tree-shaking.",
        "consistent-type-definitions" => " Enforces consistent use of `type` vs `interface` for type definitions.",
        "no-unnecessary-condition" => " Catches conditions that are always true or always false, indicating dead code or logic errors.",
        "prefer-nullish-coalescing" => " `??` only triggers on null/undefined, unlike `||` which also triggers on `0`, `''`, `false`.",
        "prefer-optional-chain" => " Optional chaining (`?.`) is cleaner and safer than manual null checks with `&&`.",
        "no-deprecated" => " Catches usage of deprecated APIs that may be removed in future versions.",
        "restrict-template-expressions" => " Prevents non-string values in template literals, which can produce `[object Object]` at runtime.",
        "no-throw-literal" => " Throwing non-Error objects loses stack traces. Always throw `new Error(...)` or an Error subclass.",
        "no-empty" => " Empty blocks usually indicate forgotten implementation. Add the logic or a comment explaining why it's empty.",
        _ => "",
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
    let rule_explanation = eslint_rule_explanation(rule_name);

    if !content.contains(rule_name) {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: missing_severity,
            title: format!("ESLint rule `{rule_name}` not configured"),
            message: format!(
                "ESLint rule `{rule_name}` not found in config.{rule_explanation} \
                 Add it to `eslint.config.mjs` in the rules section."
            ),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    if let Some(val) = expected_value {
        let value_result = check_rule_value(content, rule_name, val);

        if value_result == RuleValueResult::Pass {
            results.push(CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("ESLint rule `{rule_name}` configured correctly"),
                message: format!("`{rule_name}` set to {val} or stricter.{rule_explanation}"),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }.as_inventory());
        } else {
            results.push(CheckResult {
                id: id.to_owned(),
                severity: missing_severity,
                title: format!("ESLint rule `{rule_name}` value mismatch"),
                message: format!(
                    "`{rule_name}` found but expected value {val} (or stricter) not detected.{rule_explanation} \
                     Update the rule value in `eslint.config.mjs`."
                ),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    } else {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Info,
            title: format!("ESLint rule `{rule_name}` configured"),
            message: format!("`{rule_name}` found in ESLint config.{rule_explanation}"),
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

/// Result of comparing an ESLint rule's actual value against the expected value.
#[derive(Debug, PartialEq, Eq)]
enum RuleValueResult {
    Pass,
    Fail,
}

/// Check if a rule's configured value matches or is stricter than the expected value.
///
/// For numeric values (e.g., max-lines: 200 vs expected 300), actual <= expected means
/// stricter, which passes. For non-numeric values, falls back to exact string matching.
fn check_rule_value(content: &str, rule_name: &str, expected_value: &str) -> RuleValueResult {
    let lines: Vec<&str> = content.lines().collect();
    let expected_num: Option<u64> = expected_value.parse().ok();

    // Collect all number tokens found near the rule name (same line + up to 5 lines after)
    for (i, line) in lines.iter().enumerate() {
        if !line.contains(rule_name) {
            continue;
        }

        // Check same line and up to 5 lines after
        let end = (i.saturating_add(6)).min(lines.len());
        for check_line in lines.get(i..end).unwrap_or_default() {
            if let Some(expected_n) = expected_num {
                // Numeric comparison: stricter (<=) passes
                if let Some(actual_n) = extract_number_from_line(check_line) {
                    if actual_n <= expected_n {
                        return RuleValueResult::Pass;
                    }
                    return RuleValueResult::Fail;
                }
            } else {
                // Non-numeric: exact string match (with word boundary awareness)
                if check_line.contains(expected_value) {
                    return RuleValueResult::Pass;
                }
            }
        }
    }

    RuleValueResult::Fail
}

/// Extract the first bare integer from a line (skipping things that look like rule names).
///
/// Looks for patterns like `: 200`, `max: 200`, `"max": 200`, etc.
fn extract_number_from_line(line: &str) -> Option<u64> {
    // Match digits preceded by non-alphanumeric (to avoid matching inside rule names)
    let trimmed = line.trim();
    // Skip comment lines
    if trimmed.starts_with("//") || trimmed.starts_with('*') {
        return None;
    }
    // Find numbers in the line — look for digit sequences
    let mut chars = trimmed.char_indices().peekable();
    while let Some((idx, ch)) = chars.next() {
        if ch.is_ascii_digit() {
            // Make sure it's not part of a word (like a rule name "T2")
            if idx > 0 {
                let prev = trimmed.as_bytes().get(idx.saturating_sub(1)).copied();
                if prev.is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_') {
                    continue;
                }
            }
            let start = idx;
            let mut end = idx + 1;
            while let Some(&(next_idx, next_ch)) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    end = next_idx + 1;
                    let _ = chars.next();
                } else {
                    break;
                }
            }
            if let Ok(n) = trimmed.get(start..end).unwrap_or("").parse::<u64>() {
                return Some(n);
            }
        }
    }
    None
}

fn check_eslint_rule_presence(
    content: &str,
    eslint_path: &Path,
    id: &str,
    rule_name: &str,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    let explanation = eslint_rule_explanation(rule_name);
    if content.contains(rule_name) {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Info,
            title: format!("ESLint rule `{rule_name}` configured"),
            message: format!("`{rule_name}` found in ESLint config.{explanation}"),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        }.as_inventory());
    } else {
        results.push(CheckResult {
            id: id.to_owned(),
            severity: missing_severity,
            title: format!("ESLint rule `{rule_name}` missing"),
            message: format!(
                "`{rule_name}` not found in ESLint config.{explanation} \
                 Add it to `eslint.config.mjs` in the rules section."
            ),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
