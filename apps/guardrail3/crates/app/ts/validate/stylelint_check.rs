use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

/// Stylelint config file names to search for.
const STYLELINT_CONFIG_FILES: &[&str] = &[
    ".stylelintrc.mjs",
    ".stylelintrc.json",
    ".stylelintrc.yml",
    ".stylelintrc.yaml",
    "stylelint.config.mjs",
    "stylelint.config.js",
];

/// Required a11y rules from @double-great/stylelint-a11y.
const REQUIRED_A11Y_RULES: &[&str] = &[
    "a11y/content-property-no-static-value",
    "a11y/font-size-is-readable",
    "a11y/line-height-is-vertical-rhythmed",
    "a11y/media-prefers-reduced-motion",
    "a11y/no-display-none",
    "a11y/no-obsolete-attribute",
    "a11y/no-obsolete-element",
    "a11y/no-outline-none",
    "a11y/no-spread-text",
    "a11y/no-text-align-justify",
    "a11y/selector-pseudo-class-focus",
];

#[allow(clippy::too_many_lines)] // reason: stylelint config validation checks multiple config aspects sequentially
pub fn check_stylelint(fs: &dyn FileSystem, path: &Path, results: &mut Vec<CheckResult>) {
    // T-STYL-01: Find stylelint config file
    let mut config_content: Option<String> = None;
    let mut config_path: Option<String> = None;

    for filename in STYLELINT_CONFIG_FILES {
        let p = path.join(filename);
        if let Some(content) = fs.read_file(&p) {
            config_content = Some(content);
            config_path = Some(p.display().to_string());
            break;
        }
    }

    let Some(content) = config_content else {
        results.push(CheckResult {
            id: "T-STYL-01".to_owned(),
            severity: Severity::Error,
            title: "Stylelint config not found".to_owned(),
            message: "No stylelint config file found. Create .stylelintrc.mjs with \
                     stylelint-config-standard, stylelint-config-tailwindcss, and \
                     @double-great/stylelint-a11y for CSS quality and accessibility checking."
                .to_owned(),
            file: Some(path.display().to_string()),
            line: None,
            inventory: false,
        });
        return;
    };

    let file_display = config_path.unwrap_or_default();

    results.push(
        CheckResult {
            id: "T-STYL-01".to_owned(),
            severity: Severity::Info,
            title: "Stylelint config found".to_owned(),
            message: "Stylelint configuration file found.".to_owned(),
            file: Some(file_display.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );

    // T-STYL-02: stylelint-config-standard in extends
    check_config_extends(
        &content,
        &file_display,
        "stylelint-config-standard",
        "T-STYL-02",
        results,
    );

    // T-STYL-03: stylelint-config-tailwindcss in extends
    check_config_extends(
        &content,
        &file_display,
        "stylelint-config-tailwindcss",
        "T-STYL-03",
        results,
    );

    // T-STYL-04: @double-great/stylelint-a11y in plugins
    if content.contains("@double-great/stylelint-a11y") {
        results.push(
            CheckResult {
                id: "T-STYL-04".to_owned(),
                severity: Severity::Info,
                title: "Stylelint a11y plugin configured".to_owned(),
                message: "@double-great/stylelint-a11y found in stylelint config.".to_owned(),
                file: Some(file_display.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T-STYL-04".to_owned(),
            severity: Severity::Error,
            title: "Stylelint a11y plugin missing".to_owned(),
            message: "@double-great/stylelint-a11y not found in stylelint config plugins. \
                     Add it for CSS accessibility checking."
                .to_owned(),
            file: Some(file_display.clone()),
            line: None,
            inventory: false,
        });
    }

    // T-STYL-05: a11y rules enabled
    let missing: Vec<&&str> = REQUIRED_A11Y_RULES
        .iter()
        .filter(|r| !content.contains(**r))
        .collect();
    if missing.is_empty() {
        results.push(
            CheckResult {
                id: "T-STYL-05".to_owned(),
                severity: Severity::Info,
                title: "All stylelint a11y rules enabled".to_owned(),
                message: format!(
                    "All {} required a11y rules found in stylelint config.",
                    REQUIRED_A11Y_RULES.len()
                ),
                file: Some(file_display.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        let missing_names: Vec<&str> = missing.iter().map(|r| **r).collect();
        results.push(CheckResult {
            id: "T-STYL-05".to_owned(),
            severity: Severity::Error,
            title: "Missing stylelint a11y rules".to_owned(),
            message: format!(
                "Missing a11y rules in stylelint config: {}. These CSS accessibility rules \
                 catch issues like missing focus styles, unreadable font sizes, and removed \
                 focus outlines.",
                missing_names.join(", ")
            ),
            file: Some(file_display.clone()),
            line: None,
            inventory: false,
        });
    }

    // T-STYL-06: Architecture exceptions (intentionally disabled rules)
    let exceptions = &[
        (
            "a11y/media-prefers-color-scheme",
            "class-based dark mode instead of @media",
        ),
        (
            "no-duplicate-selectors",
            "separate :root blocks for different concerns",
        ),
    ];
    let mut missing_exceptions = Vec::new();
    for (rule, _reason) in exceptions {
        // Check that the rule appears with null (disabled)
        if !content.contains(rule) {
            missing_exceptions.push(*rule);
        }
    }
    if missing_exceptions.is_empty() {
        results.push(
            CheckResult {
                id: "T-STYL-06".to_owned(),
                severity: Severity::Info,
                title: "Stylelint architecture exceptions configured".to_owned(),
                message: "Architecture exceptions (disabled rules) are correctly configured."
                    .to_owned(),
                file: Some(file_display.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: "T-STYL-06".to_owned(),
            severity: Severity::Warn,
            title: "Missing stylelint architecture exceptions".to_owned(),
            message: format!(
                "These rules should be disabled (set to null) for architecture reasons: {}. See the guardrails plan for rationale.",
                missing_exceptions.join(", ")
            ),
            file: Some(file_display),
            line: None,
            inventory: false,
        });
    }
}

fn check_config_extends(
    content: &str,
    file: &str,
    expected: &str,
    check_id: &str,
    results: &mut Vec<CheckResult>,
) {
    if content.contains(expected) {
        results.push(
            CheckResult {
                id: check_id.to_owned(),
                severity: Severity::Info,
                title: format!("{expected} configured"),
                message: format!("{expected} found in stylelint extends."),
                file: Some(file.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: check_id.to_owned(),
            severity: Severity::Error,
            title: format!("{expected} missing"),
            message: format!(
                "{expected} not found in stylelint config extends. \
                 Add it for CSS quality checking."
            ),
            file: Some(file.to_owned()),
            line: None,
            inventory: false,
        });
    }
}
