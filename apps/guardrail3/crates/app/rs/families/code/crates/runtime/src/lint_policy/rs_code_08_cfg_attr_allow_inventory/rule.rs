use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{CfgPredicateTruth, find_cfg_attr_lint_policies};

const ID: &str = "RS-CODE-08";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.ast) {
        if info.truth != CfgPredicateTruth::Unknown {
            continue;
        }
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                if info.kind.attr_name() == "allow" {
                    "conditional cfg_attr allow".to_owned()
                } else {
                    "conditional cfg_attr expect".to_owned()
                },
                format!(
                    "Conditional cfg_attr {} for `{}`.",
                    info.kind.attr_name(),
                    info.lint
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            )
            .as_inventory(),
        );
    }
}




// reason: test-only sidecar module wiring
