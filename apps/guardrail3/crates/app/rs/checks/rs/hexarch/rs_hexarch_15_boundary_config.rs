use crate::domain::report::{CheckResult, Severity};

use super::inputs::MemberConfigHexarchInput;

const ID: &str = "RS-HEXARCH-15";

pub fn check(input: &MemberConfigHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    let boundary = input.member;
    if !boundary.is_app_boundary || boundary.has_config_entry {
        return;
    }

    let app_name = boundary.rel_dir.rsplit('/').next().unwrap_or(&boundary.rel_dir);
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: format!("app boundary `{}` missing rust.apps config", boundary.rel_dir),
        message: format!(
            "Add `[rust.apps.{app_name}]` to `guardrail3.toml` so guardrail3 can enforce app-specific architecture policy."
        ),
        file: Some("guardrail3.toml".to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_15_boundary_config_tests.rs"]
mod tests;
