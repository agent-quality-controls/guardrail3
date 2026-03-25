use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_foreign_mod_allows;

const ID: &str = "RS-CODE-20";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_foreign_mod_allows(input.ast) {
        let lint = info.lint;
        let message = if info.via_cfg_attr {
            format!(
                "`#[cfg_attr(..., allow({lint}))]` on an `extern` block hides FFI risk behind a broad suppression."
            )
        } else {
            format!(
                "`#[allow({lint})]` on an `extern` block hides FFI risk behind a broad suppression."
            )
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "allow on extern block".to_owned(),
            message,
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_20_extern_allow_tests/mod.rs"]
mod tests;
