use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::ToolDepsInput;

const ID: &str = "RS-DEPS-04";

pub fn check(input: &ToolDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.tool.tool_name != "gitleaks" {
        return;
    }

    if input.tool.installed {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "gitleaks installed".to_owned(),
                "`gitleaks` is available on PATH.".to_owned(),
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
            "gitleaks missing".to_owned(),
            "`gitleaks` is required for Rust dependency guardrails but was not found on PATH."
                .to_owned(),
            None,
            None,
            false,
        ));
    }
}

