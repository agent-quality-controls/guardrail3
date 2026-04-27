use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_ts_app_types::{FamilyRun, SupportedFamily, ValidateReport};
use guardrail3_ts_report_assertions::plain_text as assertions;

#[test]
fn renderer_hides_inventory_by_default() {
    let report = ValidateReport {
        runs: vec![FamilyRun {
            family: SupportedFamily::Eslint,
            results: vec![
                G3CheckResult::new(
                    "TS-ESLINT-CONFIG-01".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
                    None,
                )
                .into_inventory(),
                G3CheckResult::new(
                    "TS-ESLINT-CONFIG-02".to_owned(),
                    G3Severity::Warn,
                    "warn".to_owned(),
                    "warn".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
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
            family: SupportedFamily::Eslint,
            results: vec![
                G3CheckResult::new(
                    "TS-ESLINT-CONFIG-01".to_owned(),
                    G3Severity::Info,
                    "inventory".to_owned(),
                    "inventory".to_owned(),
                    Some("eslint.config.mjs".to_owned()),
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
            family: SupportedFamily::Eslint,
            results: vec![G3CheckResult::new(
                "TS-ESLINT-CONFIG-09".to_owned(),
                G3Severity::Warn,
                "typed lint configuration drifted".to_owned(),
                "`eslint.config.mjs` must keep `projectService: true` for typed linting."
                    .to_owned(),
                Some("eslint.config.mjs".to_owned()),
                None,
            )],
        }],
    };

    let output = super::super::render_report(&report, false);

    assertions::assert_includes_rule_message(&output);
}

#[test]
fn renderer_uses_astro_family_header() {
    let report = ValidateReport {
        runs: vec![FamilyRun {
            family: SupportedFamily::AstroSetup,
            results: vec![G3CheckResult::new(
                "TS-ASTRO-SETUP-FILETREE-01".to_owned(),
                G3Severity::Error,
                "astro config missing".to_owned(),
                "Add `astro.config.*` to the app root.".to_owned(),
                Some("astro.config.mjs".to_owned()),
                None,
            )],
        }],
    };

    let output = super::super::render_report(&report, false);

    assert!(
        output.contains("== astro-setup =="),
        "expected astro family heading, got: {output}"
    );
}
