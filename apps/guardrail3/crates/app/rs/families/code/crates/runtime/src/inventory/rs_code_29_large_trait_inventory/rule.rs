use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_large_traits;

const ID: &str = "RS-CODE-29";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_large_traits(input.ast) {
        let severity = if info.method_count > 12 {
            Severity::Error
        } else {
            Severity::Warn
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity,
            title: "large trait surface".to_owned(),
            message: format!(
                "Trait `{}` has {} methods (warn above 8, error above 12).",
                info.trait_name, info.method_count
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

