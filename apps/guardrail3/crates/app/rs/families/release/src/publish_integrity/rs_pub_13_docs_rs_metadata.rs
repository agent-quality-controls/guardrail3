use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-13";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable || !krate.is_library {
        return;
    }
    results.push(if krate.docs_rs_present {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: docs.rs metadata present", krate.name),
            "`[package.metadata.docs.rs]` is present.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory()
    } else {
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            format!("{}: docs.rs metadata missing", krate.name),
            "Library crates should set `[package.metadata.docs.rs]` for reproducible docs.rs builds.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )
    });
}

#[cfg(test)]
pub(super) fn run_tree(
    tree: &guardrail3_app_rs_family_view::FamilyView,
    tc: &dyn guardrail3_outbound_traits::ToolChecker,
    thorough: bool,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::test_fixtures::run_tree(tree, tc, thorough)
}
#[cfg(test)]
pub(super) fn crate_facts(name: &str) -> crate::facts::PublishableCrateFacts {
    crate::test_fixtures::crate_facts(name)
}

#[cfg(test)]
pub(super) fn crate_input(
    krate: &crate::facts::PublishableCrateFacts,
) -> crate::inputs::PublishableCrateReleaseInput<'_> {
    crate::test_fixtures::crate_input(krate)
}
#[cfg(test)]
pub(super) use test_support::{StubToolChecker, dir_entry, project_tree, temp_root};

#[cfg(test)]
#[path = "rs_pub_13_docs_rs_metadata_tests/mod.rs"]
mod rs_pub_13_docs_rs_metadata_tests;
