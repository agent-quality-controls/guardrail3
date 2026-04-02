use guardrail3_domain_report::{CheckResult, Severity};

use crate::inventory::push_success;
use crate::inputs::MemberConfigHexarchInput;

const ID: &str = "RS-HEXARCH-15";

pub fn check(input: &MemberConfigHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let boundary = input.member;
    if let Some(parse_error) = &boundary.parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "guardrail3.toml parse or validation error blocks hexarch boundary checks"
                .to_owned(),
            format!(
                "Failed to parse or validate `guardrail3.toml`, so guardrail3 cannot fully verify app boundary configuration: {parse_error}"
            ),
            Some("guardrail3.toml".to_owned()),
            None,
            false,
        ));
        return;
    }

    if !boundary.is_app_boundary {
        return;
    }
    if boundary.has_config_entry {
        push_success(
            results,
            ID,
            format!("app boundary `{}` has rust.apps config", boundary.rel_dir),
            format!(
                "App boundary `{}` is covered by an explicit `[rust.apps.*]` configuration entry.",
                boundary.rel_dir
            ),
            Some("guardrail3.toml".to_owned()),
        );
        return;
    }

    let app_name = boundary
        .rel_dir
        .rsplit('/')
        .next()
        .unwrap_or(&boundary.rel_dir);
    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Warn,
    format!("app boundary `{}` missing rust.apps config", boundary.rel_dir),
    format!(
            "Add `[rust.apps.{app_name}]` to `guardrail3.toml` so guardrail3 can enforce app-specific architecture policy."
        ),
    Some("guardrail3.toml".to_owned()),
    None,
    false,
    ));
}

