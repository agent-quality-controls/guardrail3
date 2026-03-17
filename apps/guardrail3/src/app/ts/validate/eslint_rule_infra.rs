use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

/// A rule definition: (`check_id`, `rule_name`, `severity_if_missing`).
pub type RuleDef = (&'static str, &'static str, Severity);

/// Result of comparing an `ESLint` rule's actual value against the expected value.
#[derive(Debug, PartialEq, Eq)]
enum RuleValueResult {
    Pass,
    Fail,
}

#[allow(clippy::too_many_lines)] // reason: ESLint rule explanation maps many rules to descriptions sequentially
/// Return a short explanation of what an `ESLint` rule does and why it matters.
fn eslint_rule_explanation(rule_name: &str) -> &'static str {
    match rule_name {
        "max-lines" => {
            " This rule limits file length, preventing files from growing too large to reason about."
        }
        "max-lines-per-function" => {
            " This rule limits function length, keeping functions focused and testable."
        }
        "complexity" => {
            " This rule limits cyclomatic complexity, preventing deeply nested control flow that causes bugs."
        }
        "no-restricted-imports" => {
            " This rule bans specific imports (e.g., banned packages), enforcing approved alternatives."
        }
        "no-floating-promises" => {
            " Unhandled promises silently swallow errors. This rule requires all promises to be awaited or explicitly handled."
        }
        "no-explicit-any" => {
            " Using `any` disables type checking. This rule forces proper typing or `unknown` with runtime checks."
        }
        "no-console" => {
            " Console statements left in production code create noise in logs. Use a structured logger instead."
        }
        "eqeqeq" => {
            " `==` performs type coercion (`0 == ''` is true). This rule enforces `===` for predictable comparisons."
        }
        "no-restricted-globals" => {
            " Some globals (e.g., `event`, `name`) shadow local variables silently. This rule bans dangerous globals."
        }
        "no-cycle" => {
            " Circular imports cause initialization order bugs and make code impossible to tree-shake. This rule detects import cycles."
        }
        "max-dependencies" => {
            " Too many imports indicate a module is doing too much. This rule limits import count per file."
        }
        "explicit-function-return-type" => {
            " Without explicit return types, TypeScript infers types that may change unexpectedly. This rule ensures return types are documented."
        }
        "strict-boolean-expressions" => {
            " Prevents truthy/falsy coercion (`if (str)` passes for any non-empty string). Forces explicit boolean checks."
        }
        "no-misused-promises" => {
            " Catches promises used in boolean contexts or passed where void is expected, which silently drops errors."
        }
        "await-thenable" => {
            " Catches `await` on non-promise values, which indicates a logic error."
        }
        "consistent-type-imports" => {
            " Ensures `import type` is used for type-only imports, enabling better tree-shaking and faster builds."
        }
        "no-non-null-assertion" => {
            " The `!` postfix bypasses null checks without runtime validation. Use optional chaining or explicit checks."
        }
        "switch-exhaustiveness-check" => {
            " Ensures switch statements handle all union variants, catching missing cases at compile time."
        }
        "no-unused-vars" => {
            " Dead code clutters the codebase and confuses readers. This rule catches variables declared but never used."
        }
        "require-await" => {
            " Functions marked `async` that don't use `await` misleadingly wrap returns in promises. Remove `async` or add awaited calls."
        }
        "no-param-reassign" => {
            " Reassigning function parameters creates confusing side effects. Use a new variable instead."
        }
        "no-unsafe-assignment" => {
            " Catches `any` values being assigned to typed variables, which defeats type safety."
        }
        "no-unsafe-member-access" => {
            " Catches property access on `any` values, which bypasses type checking."
        }
        "no-unsafe-call" => {
            " Catches function calls on `any` values, which can fail at runtime with no type protection."
        }
        "no-unsafe-return" => {
            " Catches `any` values being returned from typed functions, spreading type unsafety to callers."
        }
        "no-unsafe-argument" => {
            " Catches `any` values passed as arguments to typed parameters, defeating the callee's type safety."
        }
        "explicit-module-boundary-types" => {
            " Exported functions without explicit types create fragile public APIs whose types change implicitly."
        }
        "promise-function-async" => {
            " Functions returning promises should be `async` for consistent error handling and stack traces."
        }
        "consistent-type-exports" => {
            " Ensures `export type` for type-only exports, enabling better tree-shaking."
        }
        "consistent-type-definitions" => {
            " Enforces consistent use of `type` vs `interface` for type definitions."
        }
        "no-unnecessary-condition" => {
            " Catches conditions that are always true or always false, indicating dead code or logic errors."
        }
        "prefer-nullish-coalescing" => {
            " `??` only triggers on null/undefined, unlike `||` which also triggers on `0`, `''`, `false`."
        }
        "prefer-optional-chain" => {
            " Optional chaining (`?.`) is cleaner and safer than manual null checks with `&&`."
        }
        "no-deprecated" => {
            " Catches usage of deprecated APIs that may be removed in future versions."
        }
        "restrict-template-expressions" => {
            " Prevents non-string values in template literals, which can produce `[object Object]` at runtime."
        }
        "no-throw-literal" => {
            " Throwing non-Error objects loses stack traces. Always throw `new Error(...)` or an Error subclass."
        }
        "no-empty" => {
            " Empty blocks usually indicate forgotten implementation. Add the logic or a comment explaining why it's empty."
        }
        _ => "",
    }
}

#[allow(clippy::string_slice)] // reason: parsing known ASCII ESLint rule names
pub fn check_eslint_rule(
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
            results.push(
                CheckResult {
                    id: id.to_owned(),
                    severity: Severity::Info,
                    title: format!("ESLint rule `{rule_name}` configured correctly"),
                    message: format!("`{rule_name}` set to {val} or stricter.{rule_explanation}"),
                    file: Some(eslint_path.display().to_string()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
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
        results.push(
            CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("ESLint rule `{rule_name}` configured"),
                message: format!("`{rule_name}` found in ESLint config.{rule_explanation}"),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

pub fn check_eslint_rule_presence(
    content: &str,
    eslint_path: &Path,
    id: &str,
    rule_name: &str,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    let explanation = eslint_rule_explanation(rule_name);
    if content.contains(rule_name) {
        results.push(
            CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("ESLint rule `{rule_name}` configured"),
                message: format!("`{rule_name}` found in ESLint config.{explanation}"),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
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
            let mut end = idx.saturating_add(1);
            while let Some(&(next_idx, next_ch)) = chars.peek() {
                if next_ch.is_ascii_digit() {
                    end = next_idx.saturating_add(1);
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
