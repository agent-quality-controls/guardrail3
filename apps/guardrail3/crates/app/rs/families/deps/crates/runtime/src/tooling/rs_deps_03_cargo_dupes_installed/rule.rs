use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-03";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "cargo-dupes" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "cargo-dupes installed".to_owned(),
                "`cargo-dupes` is available on PATH.".to_owned(),
                None,
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "cargo-dupes missing".to_owned(),
            "`cargo-dupes` was not found on PATH. Install with `cargo install cargo-dupes`."
                .to_owned(),
            None,
            None,
            false,
        ));
    }
}

