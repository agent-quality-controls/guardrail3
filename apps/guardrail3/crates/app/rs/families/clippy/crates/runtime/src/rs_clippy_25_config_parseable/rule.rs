use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ConfigClippyInput;

const ID: &str = "RS-CLIPPY-25";

pub fn check(input: &ConfigClippyInput<'_>, results: &mut Vec<CheckResult>) {
    match (
        &input.config.parsed_typed,
        input.config.parse_error.as_deref(),
        input.config.typed_parse_error.as_deref(),
    ) {
        (Some(_), None, None) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "clippy.toml parseable".to_owned(),
                format!("`{}` parsed successfully.", input.config.rel_path),
                Some(input.config.rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
        (_, Some(parse_error), _) | (None, None, Some(parse_error)) => results.push(
            CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "clippy.toml parse error".to_owned(),
            format!("Failed to parse `{}`: {parse_error}", input.config.rel_path),
            Some(input.config.rel_path.clone()),
            None,
            false,
        )),
        (None, None, None) => {}
        (Some(_), None, Some(_)) => {}
    }
}
