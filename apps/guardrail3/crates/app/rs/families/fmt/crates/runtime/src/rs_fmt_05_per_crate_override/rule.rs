use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::RustfmtConfigKind;
use crate::inputs::RustfmtExtraConfigInput;

const ID: &str = "RS-FMT-05";

pub fn check(input: &RustfmtExtraConfigInput, results: &mut Vec<CheckResult>) {
    let kind = match input.config_kind {
        RustfmtConfigKind::RustfmtToml => "rustfmt.toml",
        RustfmtConfigKind::DotRustfmtToml => ".rustfmt.toml",
    };

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Illegal nested rustfmt config".to_owned(),
        format!("{kind} below repository root is forbidden; rustfmt policy is root-only"),
        Some(input.config_rel.clone()),
        None,
        false,
    ));
}

