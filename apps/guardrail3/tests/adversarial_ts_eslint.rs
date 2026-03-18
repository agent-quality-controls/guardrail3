//! Adversarial integration tests for T-ESLP `ESLint` plugin configuration checks.
//!
//! Each test creates a minimal TypeScript project in a temp directory with
//! `eslint.config.mjs` content, runs `guardrail3 ts validate --format json`,
//! and asserts that the correct T-ESLP check IDs fire or pass.
use garde as _;

// Suppress unused crate dependency warnings for crates used only by the main binary
use clap as _;
use colored as _;
use glob as _;
use guardrail3 as _;
use ignore as _;
use proc_macro2 as _;
use proptest as _;
use quote as _;
use serde as _;
use std::path::Path;
use std::process::Command;
use syn as _;
use toml as _;
use tree_sitter as _;
use tree_sitter_typescript as _;
use walkdir as _;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Create a temp TypeScript project with the given `eslint.config.mjs` content
/// and an optional `guardrail3.toml`.
#[allow(clippy::disallowed_methods)] // reason: test helper — fs operations to set up temp TS project
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn setup_ts_with_eslint(eslint_content: &str, config: Option<&str>) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    std::fs::write(tmp.path().join("package.json"), r#"{"name":"test"}"#).expect("pkg");
    std::fs::write(tmp.path().join("eslint.config.mjs"), eslint_content).expect("eslint");
    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).expect("src");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("ts");
    if let Some(cfg) = config {
        std::fs::write(tmp.path().join("guardrail3.toml"), cfg).expect("config");
    }
    tmp
}

/// Run `guardrail3 ts validate --format json` on the given path.
#[allow(clippy::disallowed_methods)] // reason: test helper — Command::new for binary under test
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn run_ts_validate(path: &Path) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_guardrail3"))
        .args([
            "ts",
            "validate",
            "--format",
            "json",
            path.to_str().expect("path"),
        ])
        .output()
        .expect("failed to run guardrail3");
    String::from_utf8_lossy(&out.stdout).to_string()
}

/// Collect all (`check_id`, `severity`) pairs from JSON output.
#[allow(clippy::expect_used)] // reason: test helper — JSON parsing for assertion
#[allow(clippy::indexing_slicing)] // reason: test helper — JSON structure known from guardrail3 output
#[allow(clippy::type_complexity)] // reason: test helper — tuple vec is clear in context
fn collect_check_ids(json_output: &str) -> Vec<(String, String)> {
    #[allow(clippy::disallowed_methods)] // reason: test helper — JSON parsing of guardrail3 output
    let parsed: serde_json::Value =
        serde_json::from_str(json_output).expect("guardrail3 output should be valid JSON");

    let sections = parsed["sections"]
        .as_array()
        .expect("sections should be array");

    let mut ids = Vec::new();
    for section in sections {
        let results = section["results"].as_array().expect("results array");
        for result in results {
            let id = result["id"].as_str().unwrap_or("").to_owned();
            let severity = result["severity"].as_str().unwrap_or("").to_owned();
            ids.push((id, severity));
        }
    }
    ids
}

/// Assert that a specific check ID fired as an error.
#[allow(clippy::type_complexity)] // reason: test helper — tuple vec is clear in context
fn assert_has_check(ids: &[(String, String)], check_id: &str, json_output: &str) {
    let found = ids.iter().any(|(id, sev)| id == check_id && sev == "error");
    assert!(
        found,
        "Expected check '{check_id}' to fire as error.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Assert that a specific check ID did NOT fire as an error (it either passes/info or is absent).
#[allow(clippy::type_complexity)] // reason: test helper — tuple vec is clear in context
fn assert_no_check(ids: &[(String, String)], check_id: &str, json_output: &str) {
    let found_error = ids.iter().any(|(id, sev)| id == check_id && sev == "error");
    assert!(
        !found_error,
        "Did NOT expect check '{check_id}' to fire as error, but it did.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// T-ESLP-01: unicorn config import present — `unicorn` + `flat/recommended` → passes.
#[test]
fn t_eslp_01_unicorn_present() {
    let eslint = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'unicorn/no-null': 'off',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-01 should NOT fire as error — unicorn + flat/recommended are present
    assert_no_check(&ids, "T-ESLP-01", &output);
}

/// T-ESLP-01: unicorn config import missing — no unicorn at all → fires.
#[test]
fn t_eslp_01_unicorn_missing() {
    let eslint = r"
import regexp from 'eslint-plugin-regexp';

export default [
  regexp.configs['flat/recommended'],
  {
    rules: {
      'no-param-reassign': 'error',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-01 should fire — no unicorn import or flat/recommended
    assert_has_check(&ids, "T-ESLP-01", &output);
}

/// T-ESLP-06: all 24 sonarjs rules present → passes.
#[test]
fn t_eslp_06_sonarjs_all_present() {
    // Include unicorn + flat/recommended so T-ESLP-01 doesn't distract
    let eslint = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'sonarjs/cognitive-complexity': ['error', 15],
      'sonarjs/no-identical-functions': 'error',
      'sonarjs/no-all-duplicated-branches': 'error',
      'sonarjs/no-duplicated-branches': 'error',
      'sonarjs/no-collapsible-if': 'error',
      'sonarjs/no-identical-conditions': 'error',
      'sonarjs/no-identical-expressions': 'error',
      'sonarjs/no-inverted-boolean-check': 'error',
      'sonarjs/no-redundant-boolean': 'error',
      'sonarjs/prefer-single-boolean-return': 'error',
      'sonarjs/no-gratuitous-expressions': 'error',
      'sonarjs/no-invariant-returns': 'error',
      'sonarjs/no-collection-size-mischeck': 'error',
      'sonarjs/no-empty-collection': 'error',
      'sonarjs/no-element-overwrite': 'error',
      'sonarjs/no-unused-collection': 'error',
      'sonarjs/no-use-of-empty-return-value': 'error',
      'sonarjs/no-nested-switch': 'error',
      'sonarjs/no-nested-template-literals': 'error',
      'sonarjs/no-redundant-jump': 'error',
      'sonarjs/expression-complexity': 'error',
      'sonarjs/no-async-constructor': 'error',
      'sonarjs/no-hook-setter-in-body': 'error',
      'sonarjs/no-useless-react-setstate': 'error',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-06 should NOT fire as error — all 24 sonarjs rules present
    assert_no_check(&ids, "T-ESLP-06", &output);
}

/// T-ESLP-06: only 10 sonarjs rules present → fires.
#[test]
fn t_eslp_06_sonarjs_missing_some() {
    let eslint = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'sonarjs/cognitive-complexity': ['error', 15],
      'sonarjs/no-identical-functions': 'error',
      'sonarjs/no-all-duplicated-branches': 'error',
      'sonarjs/no-duplicated-branches': 'error',
      'sonarjs/no-collapsible-if': 'error',
      'sonarjs/no-identical-conditions': 'error',
      'sonarjs/no-identical-expressions': 'error',
      'sonarjs/no-inverted-boolean-check': 'error',
      'sonarjs/no-redundant-boolean': 'error',
      'sonarjs/prefer-single-boolean-return': 'error',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-06 should fire — missing 14 sonarjs rules
    assert_has_check(&ids, "T-ESLP-06", &output);
}

/// T-ESLP-07: non-content project does NOT fire; content project without jsx-a11y fires.
#[test]
fn t_eslp_07_jsx_a11y_content_only() {
    let eslint_no_a11y = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'no-param-reassign': 'error',
    }
  }
];
";

    // Part 1: Non-content project — T-ESLP-07 should NOT fire (not applicable)
    let tmp = setup_ts_with_eslint(eslint_no_a11y, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // No config → not a content app → T-ESLP-07 should not fire at all
    assert_no_check(&ids, "T-ESLP-07", &output);

    // Part 2: Content project without jsx-a11y → T-ESLP-07 should fire
    let content_config = r#"
version = "0.1"

[profile]
name = "service"

[typescript.checks]
content = true
"#;
    let tmp2 = setup_ts_with_eslint(eslint_no_a11y, Some(content_config));
    let output2 = run_ts_validate(tmp2.path());
    let ids2 = collect_check_ids(&output2);

    // Content project without jsx-a11y/strict → T-ESLP-07 should fire
    assert_has_check(&ids2, "T-ESLP-07", &output2);
}

/// T-ESLP-10: all 17 built-in rules present → passes.
#[test]
fn t_eslp_10_builtin_rules_present() {
    let eslint = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'no-param-reassign': 'error',
      '@typescript-eslint/no-shadow': 'error',
      'complexity': ['error', 10],
      'max-depth': ['error', 4],
      'max-params': ['error', 4],
      'max-nested-callbacks': ['error', 3],
      'no-return-assign': 'error',
      '@typescript-eslint/only-throw-error': 'error',
      'prefer-template': 'error',
      'object-shorthand': 'error',
      'no-sequences': 'error',
      'no-void': 'error',
      '@typescript-eslint/switch-exhaustiveness-check': 'error',
      '@typescript-eslint/no-confusing-void-expression': 'error',
      '@typescript-eslint/naming-convention': 'error',
      '@typescript-eslint/method-signature-style': 'error',
      'react/jsx-no-leaked-render': 'error',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-10 should NOT fire as error — all 17 built-in rules present
    assert_no_check(&ids, "T-ESLP-10", &output);
}

/// T-ESLP-10: built-in rules missing most entries → fires.
#[test]
fn t_eslp_10_builtin_rules_missing() {
    let eslint = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'no-param-reassign': 'error',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-10 should fire — missing 16 of 17 built-in rules
    assert_has_check(&ids, "T-ESLP-10", &output);
}

// ---------------------------------------------------------------------------
// Warn-severity helpers
// ---------------------------------------------------------------------------

/// Assert that a specific check ID fired as warn.
#[allow(clippy::type_complexity)] // reason: test helper — tuple vec is clear in context
fn assert_has_warn(ids: &[(String, String)], check_id: &str, json_output: &str) {
    let found = ids.iter().any(|(id, sev)| id == check_id && sev == "warn");
    assert!(
        found,
        "Expected check '{check_id}' to fire as warn.\nCheck IDs found: {ids:?}\nFull output:\n{json_output}"
    );
}

/// Create a content-type TS project with eslint config (for content-only checks).
#[allow(clippy::disallowed_methods)] // reason: test helper — fs operations for content project with eslint
#[allow(clippy::expect_used)] // reason: test helper — panics indicate broken test infrastructure
fn setup_content_ts_with_eslint(eslint_content: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("tempdir");
    std::fs::write(tmp.path().join("package.json"), r#"{"name":"test"}"#).expect("pkg");
    std::fs::write(tmp.path().join("eslint.config.mjs"), eslint_content).expect("eslint");

    let src = tmp.path().join("src");
    std::fs::create_dir_all(&src).expect("src");
    std::fs::write(src.join("index.ts"), "export const x = 1;").expect("ts");

    // Content-type config
    std::fs::write(
        tmp.path().join("guardrail3.toml"),
        r#"
version = "0.1"

[profile]
name = "service"

[typescript.checks]
content = true
"#,
    )
    .expect("config");

    tmp
}

// ---------------------------------------------------------------------------
// Gap-fix adversarial tests
// ---------------------------------------------------------------------------

/// T-ESLP-12: tailwind-ban rule present but no `denyList` keyword → fires as warn.
#[test]
fn t_eslp_12_tailwind_ban_no_denylist() {
    // eslint config has tailwind-ban rule but no denyList configuration
    let eslint = r"
import tailwindBan from 'eslint-plugin-tailwind-ban';

export default [
  {
    plugins: { 'tailwind-ban': tailwindBan },
    rules: {
      'tailwind-ban/ban-classes': 'error',
    }
  }
];
";
    let tmp = setup_content_ts_with_eslint(eslint);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-12 should fire as warn — tailwind-ban present but no denyList
    assert_has_warn(&ids, "T-ESLP-12", &output);
}

/// T-ESLP-10: naming-convention rule present but no `selector` keyword → fires as warn.
#[test]
fn t_eslp_10_naming_convention_no_selector() {
    // eslint config has naming-convention rule but no selector configuration
    let eslint = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'no-param-reassign': 'error',
      '@typescript-eslint/no-shadow': 'error',
      'complexity': ['error', 10],
      'max-depth': ['error', 4],
      'max-params': ['error', 4],
      'max-nested-callbacks': ['error', 3],
      'no-return-assign': 'error',
      '@typescript-eslint/only-throw-error': 'error',
      'prefer-template': 'error',
      'object-shorthand': 'error',
      'no-sequences': 'error',
      'no-void': 'error',
      '@typescript-eslint/switch-exhaustiveness-check': 'error',
      '@typescript-eslint/no-confusing-void-expression': 'error',
      '@typescript-eslint/naming-convention': 'error',
      '@typescript-eslint/method-signature-style': 'error',
      'react/jsx-no-leaked-render': 'error',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-10 should fire as warn — naming-convention present but no selector
    assert_has_warn(&ids, "T-ESLP-10", &output);
}

/// T-ESLP-10: jsx-no-leaked-render rule present but no `validStrategies` → fires as warn.
#[test]
fn t_eslp_10_leaked_render_no_strategies() {
    // eslint config has jsx-no-leaked-render but no validStrategies configuration
    let eslint = r"
import unicorn from 'eslint-plugin-unicorn';

export default [
  unicorn.configs['flat/recommended'],
  {
    rules: {
      'no-param-reassign': 'error',
      '@typescript-eslint/no-shadow': 'error',
      'complexity': ['error', 10],
      'max-depth': ['error', 4],
      'max-params': ['error', 4],
      'max-nested-callbacks': ['error', 3],
      'no-return-assign': 'error',
      '@typescript-eslint/only-throw-error': 'error',
      'prefer-template': 'error',
      'object-shorthand': 'error',
      'no-sequences': 'error',
      'no-void': 'error',
      '@typescript-eslint/switch-exhaustiveness-check': 'error',
      '@typescript-eslint/no-confusing-void-expression': 'error',
      '@typescript-eslint/naming-convention': ['error', { selector: 'default', format: ['camelCase'] }],
      '@typescript-eslint/method-signature-style': 'error',
      'react/jsx-no-leaked-render': 'error',
    }
  }
];
";
    let tmp = setup_ts_with_eslint(eslint, None);
    let output = run_ts_validate(tmp.path());
    let ids = collect_check_ids(&output);

    // T-ESLP-10 should fire as warn — jsx-no-leaked-render present but no validStrategies
    assert_has_warn(&ids, "T-ESLP-10", &output);
}
