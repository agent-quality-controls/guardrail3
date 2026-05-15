use std::path::PathBuf;

use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_app_types::{FamilyRun, ReportRenderer as _, SupportedFamily, ValidateReport};
use guardrail3_rs_report::PlainTextReportRenderer;

type FixtureResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Renders one report-output fixture as plain text.
///
/// # Errors
///
/// Returns an error when no fixture path is passed, an unknown fixture is requested, or the fixture
/// path cannot be mapped to a fixture id.
pub(crate) fn render(fixture_paths: &[String]) -> FixtureResult<String> {
    if fixture_paths.is_empty() {
        return Err("usage: guardrail3-rs-report-fixture-output <fixture.toml>...".into());
    }
    if fixture_paths.len() != 1 {
        return Err("usage: guardrail3-rs-report-fixture-output <fixture.toml>".into());
    }
    let fixture_path = fixture_paths
        .first()
        .ok_or("usage: guardrail3-rs-report-fixture-output <fixture.toml>")?;
    rendered_for_fixture(fixture_path)
}

fn rendered_for_fixture(path: &str) -> FixtureResult<String> {
    let fixture_id = fixture_id(path)?;
    let report = match fixture_id.as_str() {
        "P10-hidden-inventory-with-visible-warning" => report_with_hidden_inventory_and_warning(),
        "P20-all-results-hidden" => report_with_only_hidden_inventory(),
        "P30-rule-message" => report_with_rule_message(),
        "P40-scope-root" => ValidateReport::scoped("workspace", PathBuf::from("/tmp/example")),
        _ => return Err(format!("unknown report fixture: {fixture_id}").into()),
    };
    Ok(PlainTextReportRenderer.render(&report, false))
}

fn fixture_id(path: &str) -> FixtureResult<String> {
    let path = PathBuf::from(path);
    let parent = path
        .parent()
        .and_then(std::path::Path::file_name)
        .and_then(std::ffi::OsStr::to_str)
        .ok_or_else(|| format!("fixture path has no fixture directory: {}", path.display()))?;
    Ok(parent.to_owned())
}

fn report_with_hidden_inventory_and_warning() -> ValidateReport {
    ValidateReport {
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
        ..ValidateReport::default()
    }
}

fn report_with_only_hidden_inventory() -> ValidateReport {
    ValidateReport {
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
        ..ValidateReport::default()
    }
}

fn report_with_rule_message() -> ValidateReport {
    ValidateReport {
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
        ..ValidateReport::default()
    }
}
