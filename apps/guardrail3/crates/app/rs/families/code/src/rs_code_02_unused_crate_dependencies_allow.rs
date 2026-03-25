use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_crate_level_allows;

const ID: &str = "RS-CODE-02";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.ast) {
        if lint != "unused_crate_dependencies" {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "unused_crate_dependencies exemption".to_owned(),
            message: "unused_crate_dependencies is an approved universal exemption.".to_owned(),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_02_unused_crate_dependencies_allow_tests/mod.rs"]
mod tests;
