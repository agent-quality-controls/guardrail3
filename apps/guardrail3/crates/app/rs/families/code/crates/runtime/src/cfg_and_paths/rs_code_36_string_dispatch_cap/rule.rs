use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_string_dispatch_sites;

const ID: &str = "RS-CODE-36";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_string_dispatch_sites(input.ast, input.is_test_root) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "string dispatch is too large".to_owned(),
            format!(
                "{} site has {} string-literal branches (cap 10). Replace string dispatch with typed models.",
                info.site_kind, info.string_literal_branch_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}

