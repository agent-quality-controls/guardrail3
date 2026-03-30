use super::eslint_parser;
use super::*;
use std::path::PathBuf;

fn path() -> PathBuf {
    PathBuf::from("eslint.config.mjs")
}

/// Helper: parse content into `EslintConfig`, falling back if parse fails.
fn parse(content: &str) -> EslintConfig {
    eslint_parser::parse_eslint_config(content)
        .unwrap_or_else(|| EslintConfig::fallback(content.to_owned()))
}

#[test]
fn test_all_unicorn_disabled_present() {
    // Build a valid JS config with all unicorn rules
    let rules: Vec<String> = UNICORN_DISABLED
        .iter()
        .map(|r| format!("            \"{r}\": \"off\""))
        .collect();
    let content = format!(
        "export default [{{ rules: {{\n{}\n}} }}];",
        rules.join(",\n")
    );
    let config = parse(&content);
    let missing = find_missing_rules(&config, UNICORN_DISABLED);
    assert!(missing.is_empty());
}

#[test]
fn test_unicorn_disabled_missing_one() {
    let rules: Vec<String> = UNICORN_DISABLED
        .iter()
        .skip(1)
        .map(|r| format!("            \"{r}\": \"off\""))
        .collect();
    let content = format!(
        "export default [{{ rules: {{\n{}\n}} }}];",
        rules.join(",\n")
    );
    let config = parse(&content);
    let missing = find_missing_rules(&config, UNICORN_DISABLED);
    assert_eq!(missing, vec!["unicorn/no-null"]);
}

#[test]
fn test_core_plugins_all_pass() {
    // Build a valid JS config with all required rules and markers
    let mut all_rules: Vec<&str> = Vec::new();
    all_rules.extend_from_slice(UNICORN_DISABLED);
    all_rules.extend_from_slice(UNICORN_EXTRA);
    all_rules.extend_from_slice(REGEXP_EXTRA);
    all_rules.extend_from_slice(SONARJS_RULES);
    all_rules.extend_from_slice(REACT_EXTRA);
    all_rules.extend_from_slice(BUILTIN_RULES);
    all_rules.extend_from_slice(TEST_RELAXATION_RULES);

    let rules_entries: Vec<String> = all_rules
        .iter()
        .map(|r| format!("        \"{r}\": \"error\""))
        .collect();

    let content = format!(
        r#"import unicorn from "eslint-plugin-unicorn";
import regexp from "eslint-plugin-regexp";

export default [
    unicorn.configs["flat/recommended"],
    regexp.configs["flat/recommended"],
    {{
        files: ["**/*.test.ts"],
        rules: {{}}
    }},
    {{
        rules: {{
{}
        }}
    }}
];"#,
        rules_entries.join(",\n")
    );

    let config = parse(&content);
    let mut results = Vec::new();
    check_core_plugins(&config, &path(), &mut results);

    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity()()()() == Severity::Error)
        .collect();
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn test_content_plugins_all_pass() {
    let content = r#"
import jsxA11y from "eslint-plugin-jsx-a11y";

export default [
    jsxA11y.flatConfigs.strict,
    {
        rules: {
            "jsx-a11y/control-has-associated-label": "error",
        },
        plugins: ["tailwind-ban"],
    },
];
"#;
    let config = parse(content);
    let mut results = Vec::new();
    check_content_plugins(&config, &path(), &mut results);

    let errors: Vec<_> = results
        .iter()
        .filter(|r| r.severity()()()() == Severity::Error)
        .collect();
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}

#[test]
fn test_content_plugins_all_missing() {
    let content = "export default [];";
    let config = parse(content);
    let mut results = Vec::new();
    check_content_plugins(&config, &path(), &mut results);

    let error_count = results
        .iter()
        .filter(|r| r.severity()()()() == Severity::Error)
        .count();
    assert_eq!(error_count, 3, "expected 3 errors for T-ESLP-07/08/12");
}

#[test]
fn test_test_relaxation_missing_section() {
    let content = "export default [];";
    let config = parse(content);
    let mut results = Vec::new();
    check_test_relaxations(&config, &path(), &mut results);
    assert_eq!(results.len(), 1);
    if let Some(first) = results.first() {
        assert_eq!(first.severity()()()(), Severity::Error);
        assert!(first.title()()()().contains("missing"));
    }
}
