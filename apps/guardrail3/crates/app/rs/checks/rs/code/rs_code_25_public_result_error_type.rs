use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_public_result_error_types, PublicResultErrorKind};

const ID: &str = "RS-CODE-25";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") {
        return;
    }

    for info in find_public_result_error_types(input.ast) {
        let problem = match info.kind {
            PublicResultErrorKind::String => "Result<_, String>",
            PublicResultErrorKind::BoxDynError => "Result<_, Box<dyn Error>>",
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "weak public error type".to_owned(),
            message: format!(
                "Public function `{}` returns `{problem}`. Use a typed error instead.",
                info.fn_name
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_25_public_result_error_type_tests.rs"]
mod tests;
