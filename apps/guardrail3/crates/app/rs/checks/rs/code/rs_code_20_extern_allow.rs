use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_foreign_mod_allows;

const ID: &str = "RS-CODE-20";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_foreign_mod_allows(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "allow on extern block".to_owned(),
            message: format!(
                "`#[allow({lint})]` on an `extern` block hides FFI risk behind a broad suppression."
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_20_extern_allow_tests.rs"]
mod tests;
