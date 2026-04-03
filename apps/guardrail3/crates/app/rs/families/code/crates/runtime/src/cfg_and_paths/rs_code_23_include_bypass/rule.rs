use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_include_macros;

const ID: &str = "RS-CODE-23";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_include_macros(input.ast) {
        match info.macro_name.as_str() {
            "include" if info.build_script_pattern && info.path_traversal => {
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Warn,
                    "include path traversal".to_owned(),
                    "`include!()` build-script pattern appends a path containing `..`."
                        .to_owned(),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                    false,
                ));
            }
            "include" if info.build_script_pattern && !info.path_traversal => results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Warn,
                    "build-script include! inventory".to_owned(),
                    "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.".to_owned(),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                    false,
                ),
            ),
            "include" => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "include! bypass".to_owned(),
                "`include!()` pulls in Rust code outside the scanned file boundary.".to_owned(),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            )),
            "include_str" | "include_bytes" if info.path_traversal => results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "include path traversal".to_owned(),
                format!("`{}!()` uses a path containing `..`.", info.macro_name),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            )),
            _ => {}
        }
    }
}

