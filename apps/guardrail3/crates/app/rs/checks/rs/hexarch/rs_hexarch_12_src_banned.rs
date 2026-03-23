use crate::domain::report::{CheckResult, Severity};

use super::inputs::AppHexarchInput;

const ID: &str = "RS-HEXARCH-12";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.src_dir_exists {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("Service `{}` has src/ directory", input.app_name),
        message: format!(
            "Service `{}` has an `src/` directory. Code must be in `crates/` following hex arch layout. Move code into `crates/{{adapters,app,domain,ports}}` subcrates, with optional `crates/macros/` only when needed.",
            input.app_name
        ),
        file: Some(format!("{}/src", input.app_rel_dir)),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_hexarch_12_src_banned_tests.rs"]
mod tests;
