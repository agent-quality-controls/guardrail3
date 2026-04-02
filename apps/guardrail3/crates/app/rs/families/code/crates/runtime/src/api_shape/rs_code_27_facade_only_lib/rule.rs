use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_facade_body_items;

const ID: &str = "RS-CODE-27";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") || !is_lib_rs(input.rel_path) {
        return;
    }

    for item in find_facade_body_items(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "lib.rs should stay facade-only".to_owned(),
            format!(
                "lib.rs contains {} `{}`. Keep lib.rs limited to facade declarations and type/const definitions.",
                item.kind, item.name
            ),
            Some(input.rel_path.to_owned()),
            Some(item.line),
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

