use guardrail3_ts_app_types::{ReportRenderer, SupportedFamily, ValidateReport};

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
        lines.push(format!("== {} ==", family_cli_name(run.family)));
        lines.extend(family_lines);
    }

    if lines.is_empty() {
        lines.push("No findings.".to_owned());
    }

    format!("{}\n", lines.join("\n"))
}

/// Returns the stable CLI name for one supported family.
const fn family_cli_name(family: SupportedFamily) -> &'static str {
    match family {
        SupportedFamily::Eslint => "eslint",
        SupportedFamily::AstroSetup => "astro-setup",
        SupportedFamily::AstroContent => "astro-content",
        SupportedFamily::AstroMdx => "astro-mdx",
        SupportedFamily::AstroI18n => "astro-i18n",
        SupportedFamily::AstroMedia => "astro-media",
        SupportedFamily::AstroSeo => "astro-seo",
        SupportedFamily::AstroState => "astro-state",
        SupportedFamily::Arch => "arch",
        SupportedFamily::Apparch => "apparch",
        SupportedFamily::Tsconfig => "tsconfig",
        SupportedFamily::Package => "package",
        SupportedFamily::Npmrc => "npmrc",
        SupportedFamily::Jscpd => "jscpd",
        SupportedFamily::Style => "style",
        SupportedFamily::Fmt => "fmt",
        SupportedFamily::Spelling => "spelling",
        SupportedFamily::Typecov => "typecov",
        SupportedFamily::Hooks => "hooks",
        SupportedFamily::Topology => "topology",
    }
}
