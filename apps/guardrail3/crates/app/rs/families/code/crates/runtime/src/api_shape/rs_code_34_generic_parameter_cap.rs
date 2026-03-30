use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_generic_parameter_caps;

const ID: &str = "RS-CODE-34";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_generic_parameter_caps(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "too many generic parameters".to_owned(),
            format!(
                "{} `{}` has {} type/const generic parameters (cap 6; lifetimes do not count).",
                info.item_kind, info.item_name, info.type_const_param_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root: false,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_34_generic_parameter_cap_tests/mod.rs"]
mod rs_code_34_generic_parameter_cap_tests;
