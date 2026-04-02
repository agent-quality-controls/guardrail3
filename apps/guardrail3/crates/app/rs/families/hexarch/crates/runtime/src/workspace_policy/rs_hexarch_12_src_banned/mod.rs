use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::AppHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-12";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.src_dir_exists {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` has no app-level src/ directory",
                input.app_name
            ),
            format!(
                "Service `{}` keeps app code under `crates/` instead of `{}/src`.",
                input.app_name, input.app_rel_dir
            ),
            Some(input.app_rel_dir.to_owned()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("Service `{}` has src/ directory", input.app_name),
    format!(
            "Service `{}` has an `src/` directory. Code must be in `crates/` following hexarch layout. Move code into `crates/{{adapters,app,domain,ports}}` subcrates, with optional `crates/macros/` only when needed.",
            input.app_name
        ),
    Some(format!("{}/src", input.app_rel_dir)),
    None,
    false,
    ));
}

#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod tests;
