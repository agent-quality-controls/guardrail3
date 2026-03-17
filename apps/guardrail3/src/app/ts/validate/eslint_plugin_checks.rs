use std::path::Path;

use crate::domain::report::{CheckResult, Severity};

// ---------------------------------------------------------------------------
// Rule lists (const arrays)
// ---------------------------------------------------------------------------

const UNICORN_DISABLED: &[&str] = &[
    "unicorn/no-null",
    "unicorn/prevent-abbreviations",
    "unicorn/filename-case",
    "unicorn/no-process-exit",
    "unicorn/no-array-reduce",
    "unicorn/no-array-callback-reference",
    "unicorn/no-useless-undefined",
    "unicorn/prefer-module",
];

const UNICORN_EXTRA: &[&str] = &[
    "unicorn/no-keyword-prefix",
    "unicorn/no-unused-properties",
    "unicorn/require-post-message-target-origin",
    "unicorn/no-anonymous-default-export",
];

const REGEXP_EXTRA: &[&str] = &[
    "regexp/require-unicode-regexp",
    "regexp/require-unicode-sets-regexp",
    "regexp/prefer-named-capture-group",
    "regexp/prefer-named-backreference",
    "regexp/prefer-result-array-groups",
    "regexp/no-misleading-capturing-group",
];

const SONARJS_RULES: &[&str] = &[
    "sonarjs/cognitive-complexity",
    "sonarjs/no-identical-functions",
    "sonarjs/no-all-duplicated-branches",
    "sonarjs/no-duplicated-branches",
    "sonarjs/no-collapsible-if",
    "sonarjs/no-identical-conditions",
    "sonarjs/no-identical-expressions",
    "sonarjs/no-inverted-boolean-check",
    "sonarjs/no-redundant-boolean",
    "sonarjs/prefer-single-boolean-return",
    "sonarjs/no-gratuitous-expressions",
    "sonarjs/no-invariant-returns",
    "sonarjs/no-collection-size-mischeck",
    "sonarjs/no-empty-collection",
    "sonarjs/no-element-overwrite",
    "sonarjs/no-unused-collection",
    "sonarjs/no-use-of-empty-return-value",
    "sonarjs/no-nested-switch",
    "sonarjs/no-nested-template-literals",
    "sonarjs/no-redundant-jump",
    "sonarjs/expression-complexity",
    "sonarjs/no-async-constructor",
    "sonarjs/no-hook-setter-in-body",
    "sonarjs/no-useless-react-setstate",
];

const REACT_EXTRA: &[&str] = &[
    "react/no-unstable-nested-components",
    "react/no-danger",
    "react/iframe-missing-sandbox",
    "react/no-array-index-key",
    "react/button-has-type",
    "react/jsx-no-script-url",
    "react/jsx-no-constructed-context-values",
    "react/no-invalid-html-attribute",
    "react/hook-use-state",
    "react/checked-requires-onchange-or-readonly",
];

const BUILTIN_RULES: &[&str] = &[
    "no-param-reassign",
    "@typescript-eslint/no-shadow",
    "complexity",
    "max-depth",
    "max-params",
    "max-nested-callbacks",
    "no-return-assign",
    "@typescript-eslint/only-throw-error",
    "prefer-template",
    "object-shorthand",
    "no-sequences",
    "no-void",
    "@typescript-eslint/switch-exhaustiveness-check",
    "@typescript-eslint/no-confusing-void-expression",
    "@typescript-eslint/naming-convention",
    "@typescript-eslint/method-signature-style",
    "react/jsx-no-leaked-render",
];

const TEST_RELAXATION_RULES: &[&str] = &[
    "max-params",
    "@typescript-eslint/naming-convention",
    "sonarjs/cognitive-complexity",
    "sonarjs/no-identical-functions",
    "sonarjs/no-duplicate-string",
];

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Find which rules from `rules` are NOT present in `content`.
fn find_missing_rules<'a>(content: &str, rules: &[&'a str]) -> Vec<&'a str> {
    rules
        .iter()
        .filter(|rule| !content.contains(**rule))
        .copied()
        .collect()
}

/// Emit a single `CheckResult` for a rule-group check: Info inventory when all
/// rules present, Error listing missing ones otherwise.
fn rule_group_result(
    id: &str,
    group_name: &str,
    missing: &[&str],
    eslint_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    if missing.is_empty() {
        results.push(
            CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("{group_name} rules configured"),
                message: format!("All expected {group_name} rules found in ESLint config."),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        let list = missing.join("`, `");
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Error,
            title: format!("{group_name} rules incomplete"),
            message: format!(
                "Missing {group_name} rules in ESLint config: `{list}`. \
                 Add them to `eslint.config.mjs`."
            ),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// Check that `content` contains an import/config pattern.
fn check_config_import(
    content: &str,
    id: &str,
    plugin_name: &str,
    markers: &[&str],
    eslint_path: &Path,
    results: &mut Vec<CheckResult>,
) {
    let all_present = markers.iter().all(|m| content.contains(m));
    if all_present {
        results.push(
            CheckResult {
                id: id.to_owned(),
                severity: Severity::Info,
                title: format!("{plugin_name} config import present"),
                message: format!("{plugin_name} flat config import found in ESLint config."),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        let missing: Vec<&&str> = markers.iter().filter(|m| !content.contains(**m)).collect();
        let list = missing
            .iter()
            .map(|m| format!("`{m}`"))
            .collect::<Vec<_>>()
            .join(", ");
        results.push(CheckResult {
            id: id.to_owned(),
            severity: Severity::Error,
            title: format!("{plugin_name} config import missing"),
            message: format!(
                "{plugin_name} config import not found. Expected patterns: {list}. \
                 Add the {plugin_name} flat config to `eslint.config.mjs`."
            ),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

// ---------------------------------------------------------------------------
// Core plugin checks (always-on)
// ---------------------------------------------------------------------------

/// Check `ESLint` plugin configurations that apply to every `TypeScript` project.
#[allow(clippy::too_many_lines)] // reason: validates many plugin groups sequentially, splitting would fragment the orchestration
pub fn check_core_plugins(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    // T-ESLP-01: unicorn config import
    check_unicorn_import(content, eslint_path, results);

    // T-ESLP-02: unicorn disabled rules
    let missing = find_missing_rules(content, UNICORN_DISABLED);
    rule_group_result(
        "T-ESLP-02",
        "unicorn disabled",
        &missing,
        eslint_path,
        results,
    );

    // T-ESLP-03: unicorn extra rules
    let missing_unicorn_extra = find_missing_rules(content, UNICORN_EXTRA);
    rule_group_result(
        "T-ESLP-03",
        "unicorn extra",
        &missing_unicorn_extra,
        eslint_path,
        results,
    );

    // T-ESLP-04: regexp config import
    check_config_import(
        content,
        "T-ESLP-04",
        "regexp",
        &["regexp", "flat/recommended"],
        eslint_path,
        results,
    );

    // T-ESLP-05: regexp extra rules
    let missing_regexp = find_missing_rules(content, REGEXP_EXTRA);
    rule_group_result(
        "T-ESLP-05",
        "regexp extra",
        &missing_regexp,
        eslint_path,
        results,
    );

    // T-ESLP-06: sonarjs cherry-picked rules
    let missing_sonarjs = find_missing_rules(content, SONARJS_RULES);
    rule_group_result(
        "T-ESLP-06",
        "sonarjs",
        &missing_sonarjs,
        eslint_path,
        results,
    );

    // T-ESLP-09: React extra rules
    let missing_react = find_missing_rules(content, REACT_EXTRA);
    rule_group_result(
        "T-ESLP-09",
        "React extra",
        &missing_react,
        eslint_path,
        results,
    );

    // T-ESLP-10: built-in ESLint/TS rules
    let missing_builtin = find_missing_rules(content, BUILTIN_RULES);
    rule_group_result(
        "T-ESLP-10",
        "built-in ESLint/TS",
        &missing_builtin,
        eslint_path,
        results,
    );

    // Verify naming-convention has selector config (not just rule name)
    if content.contains("@typescript-eslint/naming-convention") && !content.contains("selector") {
        results.push(CheckResult {
            id: "T-ESLP-10".to_owned(),
            severity: Severity::Warn,
            title: "naming-convention missing selector config".to_owned(),
            message: "@typescript-eslint/naming-convention is present but no 'selector' \
                     configuration found. The rule needs selector-specific format rules \
                     (default, variable, typeLike, etc.) to be effective."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // Verify jsx-no-leaked-render has validStrategies config
    if content.contains("jsx-no-leaked-render") && !content.contains("validStrategies") {
        results.push(CheckResult {
            id: "T-ESLP-10".to_owned(),
            severity: Severity::Warn,
            title: "jsx-no-leaked-render missing validStrategies".to_owned(),
            message: "react/jsx-no-leaked-render is present but validStrategies option not \
                     found. Without it, the rule may flag valid ternary/coerce patterns. \
                     Add `{ validStrategies: ['ternary', 'coerce'] }`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    // T-ESLP-11: test file relaxations
    check_test_relaxations(content, eslint_path, results);
}

/// T-ESLP-01: Check unicorn config import — `unicorn` + (`flat/recommended` or `configs.recommended`).
fn check_unicorn_import(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    let has_unicorn = content.contains("unicorn");
    let has_config =
        content.contains("flat/recommended") || content.contains("configs.recommended");

    if has_unicorn && has_config {
        results.push(
            CheckResult {
                id: "T-ESLP-01".to_owned(),
                severity: Severity::Info,
                title: "unicorn config import present".to_owned(),
                message: "unicorn flat config import found in ESLint config.".to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        let mut missing_parts = Vec::new();
        if !has_unicorn {
            missing_parts.push("`unicorn`");
        }
        if !has_config {
            missing_parts.push("`flat/recommended` or `configs.recommended`");
        }
        results.push(CheckResult {
            id: "T-ESLP-01".to_owned(),
            severity: Severity::Error,
            title: "unicorn config import missing".to_owned(),
            message: format!(
                "unicorn config import not found. Missing: {}. \
                 Add the eslint-plugin-unicorn flat config to `eslint.config.mjs`.",
                missing_parts.join(", ")
            ),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T-ESLP-11: Check that test file overrides disable the expected relaxation rules.
fn check_test_relaxations(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    // Find a test override section: look for lines with test/spec file patterns
    let has_test_override = content.lines().any(|line| {
        let t = line.trim();
        (t.contains(".test.") || t.contains(".spec.") || t.contains("__tests__"))
            && (t.contains("files") || t.contains("overrides"))
    });

    if !has_test_override {
        results.push(CheckResult {
            id: "T-ESLP-11".to_owned(),
            severity: Severity::Error,
            title: "Test file relaxation section missing".to_owned(),
            message: "No test file override section found in ESLint config. \
                     Add a file override for `**/*.test.*` / `**/*.spec.*` / `**/__tests__/**` \
                     that disables rules inappropriate for tests."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    }

    let missing = find_missing_rules(content, TEST_RELAXATION_RULES);
    rule_group_result(
        "T-ESLP-11",
        "test file relaxation",
        &missing,
        eslint_path,
        results,
    );
}

// ---------------------------------------------------------------------------
// Content-profile plugin checks
// ---------------------------------------------------------------------------

/// Check `ESLint` plugin configurations that apply to `content-profile` projects.
pub fn check_content_plugins(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    // T-ESLP-07: jsx-a11y strict config
    check_jsx_a11y_strict(content, eslint_path, results);

    // T-ESLP-08: jsx-a11y/control-has-associated-label
    check_a11y_control_label(content, eslint_path, results);

    // T-ESLP-12: tailwind-ban plugin and rule
    check_tailwind_ban(content, eslint_path, results);
}

/// T-ESLP-07: Check jsx-a11y strict config (look for `jsxA11y` or `jsx-a11y` and `strict`).
fn check_jsx_a11y_strict(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    let has_a11y = content.contains("jsxA11y") || content.contains("jsx-a11y");
    let has_strict = content.contains("strict");

    if has_a11y && has_strict {
        results.push(
            CheckResult {
                id: "T-ESLP-07".to_owned(),
                severity: Severity::Info,
                title: "jsx-a11y strict config present".to_owned(),
                message: "jsx-a11y strict configuration found in ESLint config.".to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        let mut missing_parts = Vec::new();
        if !has_a11y {
            missing_parts.push("`jsxA11y`/`jsx-a11y` import");
        }
        if !has_strict {
            missing_parts.push("`strict` config");
        }
        results.push(CheckResult {
            id: "T-ESLP-07".to_owned(),
            severity: Severity::Error,
            title: "jsx-a11y strict config missing".to_owned(),
            message: format!(
                "jsx-a11y strict configuration not found. Missing: {}. \
                 Add `eslint-plugin-jsx-a11y` with `flatConfigs.strict` to `eslint.config.mjs`.",
                missing_parts.join(", ")
            ),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T-ESLP-08: Check `jsx-a11y/control-has-associated-label` rule.
fn check_a11y_control_label(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    let rule = "jsx-a11y/control-has-associated-label";
    if content.contains(rule) {
        results.push(
            CheckResult {
                id: "T-ESLP-08".to_owned(),
                severity: Severity::Info,
                title: format!("ESLint rule `{rule}` configured"),
                message: format!("`{rule}` found in ESLint config."),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T-ESLP-08".to_owned(),
            severity: Severity::Error,
            title: format!("ESLint rule `{rule}` missing"),
            message: format!(
                "`{rule}` not found in ESLint config. This rule ensures interactive \
                 controls have an accessible label. Add it to `eslint.config.mjs`."
            ),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

/// T-ESLP-12: Check tailwind-ban plugin and rule.
fn check_tailwind_ban(content: &str, eslint_path: &Path, results: &mut Vec<CheckResult>) {
    let has_plugin = content.contains("tailwind-ban");
    if has_plugin {
        results.push(
            CheckResult {
                id: "T-ESLP-12".to_owned(),
                severity: Severity::Info,
                title: "tailwind-ban plugin configured".to_owned(),
                message: "tailwind-ban plugin found in ESLint config.".to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        // Also verify denyList is configured
        if !content.contains("denyList") && !content.contains("deny-list") {
            results.push(CheckResult {
                id: "T-ESLP-12".to_owned(),
                severity: Severity::Warn,
                title: "tailwind-ban missing denyList".to_owned(),
                message: "eslint-plugin-tailwind-ban is configured but no denyList found. \
                         Without a denyList, the plugin has nothing to enforce. Add a denyList \
                         with banned Tailwind tokens that have semantic design token replacements."
                    .to_owned(),
                file: Some(eslint_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    } else {
        results.push(CheckResult {
            id: "T-ESLP-12".to_owned(),
            severity: Severity::Error,
            title: "tailwind-ban plugin missing".to_owned(),
            message: "tailwind-ban plugin not found in ESLint config. This plugin enforces \
                     design-token usage over arbitrary Tailwind classes. \
                     Add `eslint-plugin-tailwind-ban` to `eslint.config.mjs`."
                .to_owned(),
            file: Some(eslint_path.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "eslint_plugin_checks_tests.rs"]
mod tests;
