use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::PublishableCrateReleaseInput;

const ID: &str = "RS-PUB-06";

pub fn check(input: &PublishableCrateReleaseInput<'_>, results: &mut Vec<CheckResult>) {
    let krate = input.krate;
    if !krate.publishable {
        return;
    }
    match krate.keywords_count {
        Some(0) | None => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!("{}: keywords missing", krate.name),
            "Publishable crates should set 1-5 `[package].keywords`.".to_owned(),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )),
        Some(count) if count > 5 => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            format!("{}: too many keywords", krate.name),
            format!("`[package].keywords` has {count} entries; crates.io allows at most 5."),
            Some(krate.cargo_rel_path.clone()),
            None,
            false,
        )),
        Some(count) => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                format!("{}: keywords present", krate.name),
                format!("`[package].keywords` has {count} entries."),
                Some(krate.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        ),
    }
}

#[cfg(test)]
pub(super) fn run_tree(
    tree: &guardrail3_app_rs_family_mapper::RsProjectSurface,
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
#[path = "rs_pub_06_keywords_present_tests/mod.rs"]
mod rs_pub_06_keywords_present_tests;
