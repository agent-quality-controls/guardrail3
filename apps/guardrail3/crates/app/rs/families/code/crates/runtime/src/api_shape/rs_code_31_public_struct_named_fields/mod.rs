use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_public_struct_field_bags;

const ID: &str = "RS-CODE-31";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_public_struct_field_bags(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "public struct exposes named public fields".to_owned(),
            format!(
                "Public struct `{}` exposes {} named `pub` fields. Prefer private fields and explicit accessors or constructors.",
                info.struct_name, info.public_field_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str) -> Vec<CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = crate::inputs::RustCodeFileInput {
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

mod rs_code_31_public_struct_named_fields_tests;
