use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-02";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "cargo-machete" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "cargo-machete installed".to_owned(),
                "`cargo-machete` is available on PATH.".to_owned(),
                None,
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "cargo-machete missing".to_owned(),
            "`cargo-machete` was not found on PATH. Install with `cargo install cargo-machete`."
                .to_owned(),
            None,
            None,
            false,
        ));
    }
}

