use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-01";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "cargo-deny" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "cargo-deny installed".to_owned(),
                "`cargo-deny` is available on PATH.".to_owned(),
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
            "cargo-deny missing".to_owned(),
            "`cargo-deny` is required for Rust dependency guardrails but was not found on PATH."
                .to_owned(),
            None,
            None,
            false,
        ));
    }
}

