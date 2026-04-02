use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::AppHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-01";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.top_level_crates_entry_count > 0 {
        push_success(
            results,
            ID,
            format!("Service `{}` has crates/ directory", input.app_name),
            format!(
                "Service `{}` owns app-local crates under `{}/crates`.",
                input.app_name, input.app_rel_dir
            ),
            Some(input.app_rel_dir.to_owned()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        format!("Service `{}` missing crates/ directory", input.app_name),
        format!(
            "Service `{}` has no `crates/` directory. Create it with the hexarch template: `crates/{{adapters/{{inbound,outbound}}, app, domain, ports/{{inbound,outbound}}}}` and add optional `crates/macros/` only if needed.",
            input.app_name
        ),
        Some(input.app_rel_dir.to_owned()),
        None,
        false,
    ));
}

