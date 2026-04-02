use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::find_impl_block_allows;

const ID: &str = "RS-CODE-17";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_impl_block_allows(input.ast) {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            if info.kind.attr_name() == "allow" {
                "blanket impl-level allow".to_owned()
            } else {
                "blanket impl-level expect".to_owned()
            },
            format!(
                "`#[{}({})]` covers an impl block with {} methods. Apply lint suppressions to individual methods instead.",
                info.kind.attr_name(),
                info.lint,
                info.method_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}





// reason: test-only sidecar module wiring
