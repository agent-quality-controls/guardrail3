use guardrail3_rs_app_types::{ReportRenderer, ValidateReport};

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

    for run in &report.runs {
        let mut family_lines = Vec::new();
        for result in &run.results {
            if !include_inventory && result.inventory() {
                continue;
            }
            let file = result.file().unwrap_or("-");
            family_lines.push(format!(
                "[{:?}] {} {} {}",
                result.severity(),
                result.id(),
                file,
                result.title()
            ));
            family_lines.push(format!("  {}", result.message()));
        }
        if family_lines.is_empty() {
            continue;
        }
        lines.push(format!("== {} ==", run.family.cli_name()));
        lines.extend(family_lines);
    }

    if lines.is_empty() {
        lines.push("No findings.".to_owned());
    }

    format!("{}\n", lines.join("\n"))
}

#[cfg(test)]
#[path = "plain_text_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod plain_text_tests;
