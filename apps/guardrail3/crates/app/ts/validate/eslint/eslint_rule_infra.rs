use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};

use super::eslint_parser::EslintConfig;

/// A rule definition: (`check_id`, `rule_name`, `severity_if_missing`).
pub type RuleDef = (&'static str, &'static str, Severity);

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

/// Check an `ESLint` rule using the parsed config struct.
///
/// Looks up the rule in `config.rules()`, verifies severity and optional numeric value.
/// Rules that only exist as test overrides are treated as missing.
pub fn check_eslint_rule(
    config: &EslintConfig,
    eslint_path: &Path,
    id: &str,
    rule_name: &str,
    expected_value: Option<&str>,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    let rule_explanation = eslint_rule_explanation(rule_name);

    // Look up the rule — try both bare name and common prefixed variants
    let rule_entry = find_rule(config, rule_name);

    // Filter out test-override-only rules
    let rule_entry = rule_entry.filter(|r| !r.is_test_override());

    let Some(rule) = rule_entry else {
        results.push(CheckResult::from_parts(
            id.to_owned(),
            missing_severity,
            format!("ESLint rule `{rule_name}` not configured"),
            format!(
                "ESLint rule `{rule_name}` not found in config.{rule_explanation} \
                 Add it to `eslint.config.mjs` in the rules section."
            ),
            Some(eslint_path.display().to_string()),
            None,
            false,
        ));
        return;
    };

    if let Some(val) = expected_value {
        let pass = check_rule_value(rule, val);

        if pass {
            results.push(
                CheckResult::from_parts(
                    id.to_owned(),
                    Severity::Info,
                    format!("ESLint rule `{rule_name}` configured correctly"),
                    format!("`{rule_name}` set to {val} or stricter.{rule_explanation}"),
                    Some(eslint_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
                id.to_owned(),
                missing_severity,
                format!("ESLint rule `{rule_name}` value mismatch"),
                format!(
                    "`{rule_name}` found but expected value {val} (or stricter) not detected.{rule_explanation} \
                     Update the rule value in `eslint.config.mjs`."
                ),
                Some(eslint_path.display().to_string()),
                None,
                false,
            ));
        }
    } else {
        results.push(
            CheckResult::from_parts(
                id.to_owned(),
                Severity::Info,
                format!("ESLint rule `{rule_name}` configured"),
                format!("`{rule_name}` found in ESLint config.{rule_explanation}"),
                Some(eslint_path.display().to_string()),
                None,
                false,
            )
            .as_inventory(),
        );
    }
}

/// Check that an `ESLint` rule is present and set to "error" severity.
///
/// Rules that only exist as test overrides are treated as missing.
pub fn check_eslint_rule_presence(
    config: &EslintConfig,
    eslint_path: &Path,
    id: &str,
    rule_name: &str,
    missing_severity: Severity,
    results: &mut Vec<CheckResult>,
) {
    let explanation = eslint_rule_explanation(rule_name);

    // Look up the rule — try both bare name and common prefixed variants
    let rule_entry = find_rule(config, rule_name);

    // Filter out test-override-only rules
    let rule_entry = rule_entry.filter(|r| !r.is_test_override());

    if let Some(rule) = rule_entry {
        if rule.severity() == "error" {
            results.push(
                CheckResult::from_parts(
                    id.to_owned(),
                    Severity::Info,
                    format!("ESLint rule `{rule_name}` configured"),
                    format!("`{rule_name}` found in ESLint config.{explanation}"),
                    Some(eslint_path.display().to_string()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        } else {
            results.push(CheckResult::from_parts(
                id.to_owned(),
                missing_severity,
                format!("ESLint rule `{rule_name}` not set to error"),
                format!(
                    "`{rule_name}` found but severity is `{}`, expected `error`.{explanation} \
                     Update the severity in `eslint.config.mjs`.",
                    rule.severity()
                ),
                Some(eslint_path.display().to_string()),
                None,
                false,
            ));
        }
    } else {
        results.push(CheckResult::from_parts(
            id.to_owned(),
            missing_severity,
            format!("ESLint rule `{rule_name}` missing"),
            format!(
                "`{rule_name}` not found in ESLint config.{explanation} \
                 Add it to `eslint.config.mjs` in the rules section."
            ),
            Some(eslint_path.display().to_string()),
            None,
            false,
        ));
    }
}

/// Find a rule in the parsed config, trying the exact name first,
/// then common prefixed variants (`@typescript-eslint/`, `import/`).
fn find_rule<'a>(
    config: &'a EslintConfig,
    rule_name: &str,
) -> Option<&'a super::eslint_parser::RuleConfig> {
    // Exact match first
    if let Some(r) = config.rules().get(rule_name) {
        return Some(r);
    }
    // Try with @typescript-eslint/ prefix
    let ts_prefixed = format!("@typescript-eslint/{rule_name}");
    if let Some(r) = config.rules().get(&ts_prefixed) {
        return Some(r);
    }
    // Try with import-x/ prefix (eslint-plugin-import-x)
    let import_x_prefixed = format!("import-x/{rule_name}");
    if let Some(r) = config.rules().get(&import_x_prefixed) {
        return Some(r);
    }
    // Try with import/ prefix (legacy eslint-plugin-import)
    let import_prefixed = format!("import/{rule_name}");
    if let Some(r) = config.rules().get(&import_prefixed) {
        return Some(r);
    }
    None
}

/// Check if a rule's configured value matches or is stricter than the expected value.
///
/// For numeric values (e.g., max-lines: 200 vs expected 300), actual <= expected means
/// stricter, which passes. For non-numeric values, compares against the rule's severity.
fn check_rule_value(rule: &super::eslint_parser::RuleConfig, expected_value: &str) -> bool {
    if let Ok(expected_n) = expected_value.parse::<u32>() {
        // Numeric comparison: rule must have a numeric_value that is <= expected (stricter)
        if let Some(actual_n) = rule.numeric_value() {
            return actual_n <= expected_n;
        }
        // No numeric value found — fail
        return false;
    }

    // Non-numeric: check if the severity matches the expected value
    rule.severity() == expected_value
}
