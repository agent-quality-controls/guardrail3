use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_include_macros;

const ID: &str = "RS-CODE-23";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_include_macros(input.ast) {
        match info.macro_name.as_str() {
            "include" if info.build_script_pattern => results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "build-script include! inventory".to_owned(),
                    message: "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.".to_owned(),
                    file: Some(input.rel_path.to_owned()),
                    line: Some(info.line),
                    inventory: false,
                }
                .as_inventory(),
            ),
            "include" => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "include! bypass".to_owned(),
                message: "`include!()` pulls in Rust code outside the scanned file boundary.".to_owned(),
                file: Some(input.rel_path.to_owned()),
                line: Some(info.line),
                inventory: false,
            }),
            "include_str" | "include_bytes" if info.path_traversal => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "include path traversal".to_owned(),
                message: format!("`{}!()` uses a path containing `..`.", info.macro_name),
                file: Some(input.rel_path.to_owned()),
                line: Some(info.line),
                inventory: false,
            }),
            _ => {}
        }
    }
}

#[cfg(test)]
#[path = "rs_code_23_include_bypass_tests.rs"]
mod tests;
