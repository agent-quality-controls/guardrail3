use guardrail3_domain_report::CheckResult;

use crate::inputs::RustfmtRootInput;

const ID: &str = "RS-FMT-01";

pub fn check(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    match input.config_rel.as_deref() {
        Some(_rel) => {}
        None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            guardrail3_domain_report::Severity::Error,
            "rustfmt config missing".to_owned(),
            "Expected rustfmt.toml or .rustfmt.toml at workspace root".to_owned(),
            Some("".to_owned()),
            None,
            false,
        )),
    }
}

