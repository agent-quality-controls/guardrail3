use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_foreign_mod_allows;

const ID: &str = "RS-CODE-20";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_foreign_mod_allows(input.ast) {
        let lint = info.lint;
        let message = if info.via_cfg_attr {
            format!(
                "`#[cfg_attr(..., {}({lint}))]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        } else {
            format!(
                "`#[{}({lint})]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: if info.kind.attr_name() == "allow" {
                "allow on extern block".to_owned()
            } else {
                "expect on extern block".to_owned()
            },
            message,
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

