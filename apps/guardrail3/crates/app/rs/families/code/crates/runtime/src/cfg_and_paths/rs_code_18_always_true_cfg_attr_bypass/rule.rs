use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{CfgPredicateTruth, find_cfg_attr_lint_policies};

const ID: &str = "RS-CODE-18";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.ast) {
        if info.truth != CfgPredicateTruth::KnownTrue {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "always-true cfg_attr bypass".to_owned(),
            format!(
                "`#[cfg_attr(..., {}({}))]` is effectively unconditional. Use a direct `#[{}]` with an explicit reason instead.",
                info.kind.attr_name(),
                info.lint,
                info.kind.attr_name()
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}





// reason: test-only sidecar module wiring
