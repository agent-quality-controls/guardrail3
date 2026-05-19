use guardrail3_check_types::G3Severity;
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
                "  family: configure this family or set {} = false under [checks] in guardrail3-ts.toml.",
                family_check_key(run.family)
            ));
        }
        lines.push(format!("== {} ==", family_cli_name(run.family)));
        lines.extend(family_lines);
    }

    if lines.is_empty() {
        lines.push("No findings.".to_owned());
    }

    format!("{}\n", lines.join("\n"))
}

/// Returns the `[checks]` key that disables one TS family.
const fn family_check_key(family: SupportedFamily) -> &'static str {
    match family {
        SupportedFamily::Eslint => "eslint",
        SupportedFamily::AstroSetup => "astro_setup",
        SupportedFamily::AstroContent => "astro_content",
        SupportedFamily::AstroMdx => "astro_mdx",
        SupportedFamily::AstroI18n => "astro_i18n",
        SupportedFamily::AstroMedia => "astro_media",
        SupportedFamily::AstroSeo => "astro_seo",
        SupportedFamily::AstroState => "astro_state",
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
