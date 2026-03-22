use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_path_attrs, same_line_reason};

const ID: &str = "RS-CODE-24";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_path_attrs(input.ast) {
        if info.path.contains("..") {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "#[path] escapes parent directory".to_owned(),
                message: format!("`#[path = \"{}\"]` escapes the standard module boundary.", info.path),
                file: Some(input.rel_path.to_owned()),
                line: Some(info.line),
                inventory: false,
            });
            continue;
        }

        let Some(reason) = same_line_reason(input.content, info.line) else {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "#[path] without reason".to_owned(),
                message: format!(
                    "`#[path = \"{}\"]` changes module resolution and requires `// reason:` on the same line.",
                    info.path
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(info.line),
                inventory: false,
            });
            continue;
        };

        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "#[path] usage".to_owned(),
            message: format!("#[path = \"{}\"] reason: {reason}", info.path),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_24_path_attr_tests.rs"]
mod tests;
