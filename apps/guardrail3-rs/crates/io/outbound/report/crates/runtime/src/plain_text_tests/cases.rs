use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_app_types::{FamilyRun, SupportedFamily, ValidateReport};
use guardrail3_rs_report_assertions::plain_text as assertions;

#[test]
fn renderer_hides_inventory_by_default() {
    let report = ValidateReport {
        runs: vec![FamilyRun {
            family: SupportedFamily::Fmt,
            results: vec![
                G3CheckResult::new(
                    "g3rs-fmt/rustfmt-required-settings".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("rustfmt.toml".to_owned()),
                    None,
                )
                .into_inventory(),
                G3CheckResult::new(
                    "g3rs-fmt/rustfmt-extra-settings-inventory".to_owned(),
                    G3Severity::Warn,
                    "warn".to_owned(),
                    "warn".to_owned(),
                    Some("rustfmt.toml".to_owned()),
                    None,
                ),
            ],
        }],
    };

    let output = super::super::render_report(&report, false);

    assertions::assert_inventory_hidden(&output);
}

#[test]
fn renderer_reports_no_findings_when_all_results_are_hidden() {
    let report = ValidateReport {
        runs: vec![FamilyRun {
            family: SupportedFamily::Fmt,
            results: vec![
                G3CheckResult::new(
                    "g3rs-fmt/rustfmt-required-settings".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("rustfmt.toml".to_owned()),
                    None,
                )
                .into_inventory(),
            ],
        }],
    };

    let output = super::super::render_report(&report, false);

    assertions::assert_no_findings(&output);
}

#[test]
fn renderer_includes_rule_message() {
    let report = ValidateReport {
        runs: vec![FamilyRun {
            family: SupportedFamily::Deny,
            results: vec![G3CheckResult::new(
                "g3rs-deny/wrappers".to_owned(),
                G3Severity::Warn,
                "managed ban wrappers changed".to_owned(),
                "`deny.toml` ban `regex` adds local wrappers `tree-sitter`.".to_owned(),
                Some("deny.toml".to_owned()),
                None,
            )],
        }],
    };

    let output = super::super::render_report(&report, false);

    assertions::assert_includes_rule_message(&output);
}
