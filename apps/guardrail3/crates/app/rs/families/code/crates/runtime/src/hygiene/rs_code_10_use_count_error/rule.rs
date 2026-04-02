use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::count_top_level_use_imports;

const ID: &str = "RS-CODE-10";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test_root {
        return;
    }

    let use_count = count_top_level_use_imports(input.ast);
    if use_count <= 20 {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "too many use imports".to_owned(),
        format!("{use_count} top-level use imports (max 20)."),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

