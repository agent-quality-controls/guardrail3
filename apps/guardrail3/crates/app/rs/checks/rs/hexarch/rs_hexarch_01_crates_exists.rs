use crate::domain::report::{CheckResult, Severity};

use super::inputs::AppHexarchInput;

const ID: &str = "RS-HEXARCH-01";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.top_level_crates_entry_count > 0 {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("Service `{}` missing crates/ directory", input.app_name),
        message: format!(
            "Service `{}` has no `crates/` directory. Create it with the hex arch template: `crates/{{adapters/{{inbound,outbound}}, app, domain, ports/{{inbound,outbound}}}}`.",
            input.app_name
        ),
        file: Some(input.app_rel_dir.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_01_crates_exists_tests.rs"]
mod tests;
