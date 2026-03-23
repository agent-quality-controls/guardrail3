use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_deny_forbid_attrs, same_line_reason};

const ID: &str = "RS-CODE-22";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_deny_forbid_attrs(input.ast) {
        if info.level == "forbid" && info.lint == "unsafe_code" && info.crate_level_inner {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "forbid(unsafe_code)".to_owned(),
                    message: "`forbid(unsafe_code)` strengthens the local safety boundary."
                        .to_owned(),
                    file: Some(input.rel_path.to_owned()),
                    line: Some(info.line),
                    inventory: false,
                }
                .as_inventory(),
            );
            continue;
        }
        if same_line_reason(input.content, info.line).is_some() {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "#[deny]/#[forbid] without reason".to_owned(),
            message: format!(
                "`#[{}({})]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                info.level, info.lint
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_22_deny_forbid_without_reason_tests/mod.rs"]
mod tests;
