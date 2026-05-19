use guardrail3_check_types::G3Severity;
use guardrail3_rs_app_types::{ReportRenderer, SupportedFamily, ValidateReport};

#[derive(Debug, Default)]
pub struct PlainTextReportRenderer;

impl ReportRenderer for PlainTextReportRenderer {
    fn render(&self, report: &ValidateReport, include_inventory: bool) -> String {
        render_report(report, include_inventory)
    }
}

/// Renders the validation report into the plain text CLI format.
pub(crate) fn render_report(report: &ValidateReport, include_inventory: bool) -> String {
    let mut lines = Vec::new();
    if let Some(scope) = report.scope {
        lines.push(format!("scope: {scope}"));
    }
    if let Some(root) = &report.root {
        lines.push(format!("root: {}", root.display()));
    }
    if report.scope.is_some() || report.root.is_some() {
        lines.push(String::new());
    }

    for run in &report.runs {
        let mut family_lines = Vec::new();
        let mut has_visible_error = false;
        for result in &run.results {
            if !include_inventory && result.inventory() {
                continue;
            }
            has_visible_error |= result.severity() == G3Severity::Error;
            let subject = result.subject();
            family_lines.push(format!(
                "[{:?}] {} {} {}",
                result.severity(),
                result.id(),
                subject,
                result.title()
            ));
            family_lines.push(format!("  {}", result.message()));
            if let Some(reason) = result.waiver_reason() {
                family_lines.push(format!(
                    "  waiver: rule=\"{}\" subject=\"{}\" selector=\"{}\" reason=\"{}\"",
                    result.id(),
                    result.subject(),
                    result.selector(),
                    reason
                ));
            }
        }
        if family_lines.is_empty() {
            continue;
        }
        if has_visible_error {
            family_lines.push(format!(
                "  family: configure this family or set {} = false under [checks] in guardrail3-rs.toml.",
                family_check_key(run.family)
            ));
        }
        lines.push(format!("== {} ==", run.family.cli_name()));
        lines.extend(family_lines);
    }

    if lines.is_empty() || lines.last().is_some_and(String::is_empty) {
        lines.push("No findings.".to_owned());
    }

    format!("{}\n", lines.join("\n"))
}

/// Returns the `[checks]` key that disables one Rust family.
const fn family_check_key(family: SupportedFamily) -> &'static str {
    match family {
        SupportedFamily::Topology => "topology",
        SupportedFamily::Toolchain => "toolchain",
        SupportedFamily::Fmt => "fmt",
        SupportedFamily::Cargo => "cargo",
        SupportedFamily::Clippy => "clippy",
        SupportedFamily::Deny => "deny",
        SupportedFamily::Code => "code",
        SupportedFamily::Arch => "arch",
        SupportedFamily::Deps => "deps",
        SupportedFamily::Garde => "garde",
        SupportedFamily::Test => "test",
        SupportedFamily::Release => "release",
        SupportedFamily::Hooks => "hooks",
        SupportedFamily::Apparch => "apparch",
    }
}
