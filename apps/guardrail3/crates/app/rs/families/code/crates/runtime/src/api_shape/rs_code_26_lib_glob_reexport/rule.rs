use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_pub_use_glob_reexports;

const ID: &str = "RS-CODE-26";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") || !is_lib_rs(input.rel_path) {
        return;
    }

    for (line, target) in find_pub_use_glob_reexports(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "glob re-export in lib.rs".to_owned(),
            format!("`pub use {target}` creates an unstable API surface."),
            Some(input.rel_path.to_owned()),
            Some(line),
            false,
        ));
    }
}

fn is_lib_rs(rel_path: &str) -> bool {
    rel_path
        .rsplit('/')
        .next()
        .is_some_and(|name| name == "lib.rs")
}

