use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_impl_block_allows;

const ID: &str = "RS-CODE-17";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_impl_block_allows(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "blanket impl-level allow".to_owned(),
            message: format!(
                "`#[allow({})]` covers an impl block with {} methods. Apply lint suppressions to individual methods instead.",
                info.lint, info.method_count
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_code_17_impl_allow_blast_radius_tests/mod.rs"]
mod tests;
