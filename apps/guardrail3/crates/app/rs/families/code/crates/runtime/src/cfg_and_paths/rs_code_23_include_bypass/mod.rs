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
                    Severity::Info,
                    "build-script include! inventory".to_owned(),
                    "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.".to_owned(),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                    false,
                )
                .as_inventory(),
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

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = crate::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod tests;
