use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_pub_use_glob_reexports;

const ID: &str = "RS-CODE-26";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") || !is_lib_rs(input.rel_path) {
        return;
    }

    for (line, target) in find_pub_use_glob_reexports(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "glob re-export in lib.rs".to_owned(),
            message: format!("`pub use {target}` creates an unstable API surface."),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
}

fn is_lib_rs(rel_path: &str) -> bool {
    rel_path
        .rsplit('/')
        .next()
        .is_some_and(|name| name == "lib.rs")
}

#[cfg(test)]
#[path = "rs_code_26_lib_glob_reexport_tests/mod.rs"]
mod tests;
