use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{PublicResultErrorKind, find_public_result_error_types};

const ID: &str = "RS-CODE-33";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_public_result_error_types(input.ast) {
        let problem = match info.kind {
            PublicResultErrorKind::String => "Result<_, String>",
            PublicResultErrorKind::StrRef => "Result<_, &str>",
            PublicResultErrorKind::AnyhowError => "Result<_, anyhow::Error>",
            PublicResultErrorKind::BoxDynError => "Result<_, Box<dyn Error>>",
        };
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "weak public error form".to_owned(),
            format!(
                "Public function `{}` returns `{problem}`. Use a typed public error instead.",
                info.fn_name
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
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
        profile_name: Some("library"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_33_public_weak_error_forms_tests/mod.rs"]
mod rs_code_33_public_weak_error_forms_tests;
