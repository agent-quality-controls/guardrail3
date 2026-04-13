use guardrail3_rs_app_types::{ReportRenderer, ValidateReport};

#[derive(Debug, Default)]
pub struct PlainTextReportRenderer;

impl ReportRenderer for PlainTextReportRenderer {
    fn render(&self, report: &ValidateReport, include_inventory: bool) -> String {
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
}

#[cfg(test)]
mod tests {
    use guardrail3_check_types::{G3CheckResult, G3Severity};
    use guardrail3_rs_app_types::{FamilyRun, SupportedFamily, ValidateReport};

    use super::{PlainTextReportRenderer, ReportRenderer};

    #[test]
    fn renderer_hides_inventory_by_default() {
        let report = ValidateReport {
            runs: vec![FamilyRun {
                family: SupportedFamily::Fmt,
                results: vec![
                    G3CheckResult::new(
                        "RS-FMT-CONFIG-01".to_owned(),
                        G3Severity::Info,
                        "inventory".to_owned(),
                        "inventory".to_owned(),
                        Some("rustfmt.toml".to_owned()),
                        None,
                    )
                    .into_inventory(),
                    G3CheckResult::new(
                        "RS-FMT-CONFIG-02".to_owned(),
                        G3Severity::Warn,
                        "warn".to_owned(),
                        "warn".to_owned(),
                        Some("rustfmt.toml".to_owned()),
                        None,
                    ),
                ],
            }],
        };

        let output = PlainTextReportRenderer.render(&report, false);

        assert!(output.contains("== fmt =="));
        assert!(output.contains("RS-FMT-CONFIG-02"));
        assert!(!output.contains("RS-FMT-CONFIG-01"));
    }

    #[test]
    fn renderer_reports_no_findings_when_all_results_are_hidden() {
        let report = ValidateReport {
            runs: vec![FamilyRun {
                family: SupportedFamily::Fmt,
                results: vec![
                    G3CheckResult::new(
                        "RS-FMT-CONFIG-01".to_owned(),
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

        let output = PlainTextReportRenderer.render(&report, false);

        assert_eq!(output, "No findings.\n");
    }
}
