use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RootWorkspaceHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-11";

pub fn check(input: &RootWorkspaceHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(parse_error) = input.cargo_parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "Root Cargo.toml parse error".to_owned(),
            format!("Invalid TOML in root Cargo.toml: {parse_error}"),
            Some("Cargo.toml".to_owned()),
            None,
            false,
        ));
        return;
    }

    let before = results.len();
    for member in &input.workspace_members {
        if !input
            .rust_app_roots
            .iter()
            .any(|app_root| member.covers_dir(app_root))
        {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("root workspace includes app member `{}`", member.raw),
            format!(
                "Root workspace must not include Rust app roots like `{}`. Apps own their own workspace boundary.",
                member.raw
            ),
            Some("Cargo.toml".to_owned()),
            None,
            false,
        ));
    }

    if results.len() == before {
        push_success(
            results,
            ID,
            "root workspace excludes app roots".to_owned(),
            "Root `Cargo.toml` does not claim any routed Rust app root as a workspace member."
                .to_owned(),
            Some("Cargo.toml".to_owned()),
        );
    }
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[path = "rs_hexarch_11_root_workspace_doesnt_include_apps_tests/mod.rs"]
mod rs_hexarch_11_root_workspace_doesnt_include_apps_tests;
