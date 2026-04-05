mod advisories;
mod bans;
mod coverage;
mod deny_support;
#[cfg(feature = "api")]
pub mod facts;
mod facts_support;
mod inputs;
mod licenses;
mod sources;

mod run;
pub use run::check;

#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView;
#[cfg(test)]
use guardrail3_app_rs_family_view::FamilyView as ProjectTree;
#[cfg(test)]
use guardrail3_domain_report::CheckResult;

#[cfg(test)]
use guardrail3_app_rs_family_deny_assertions as _;

#[cfg(test)]
use self::inputs::{ConfigDenyInput, SameRootConflictInput};

#[cfg(feature = "api")]
pub use self::deny_support::expected_ban_names;

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn config_facts(deny_toml: &str) -> facts::DenyConfigFacts {
    config_facts_for_test(deny_toml, None)
}

#[cfg(test)]
pub(crate) fn config_facts_with_profile(
    deny_toml: &str,
    profile_name: &str,
) -> facts::DenyConfigFacts {
    config_facts_for_test(deny_toml, Some(profile_name))
}

#[cfg(test)]
pub(crate) fn collected_facts(tree: &ProjectTree) -> facts::DenyFacts {
    collect_facts_for_test(tree)
}

#[cfg(test)]
pub(crate) fn same_root_conflict_input<'a>(
    facts: &'a facts::DenyFacts,
    policy_root_rel: &str,
) -> SameRootConflictInput<'a> {
    let conflict = facts
        .same_root_conflicts
        .iter()
        .find(|conflict| conflict.policy_root_rel == policy_root_rel)
        .expect("expected same-root deny conflict facts");
    SameRootConflictInput::new(conflict)
}

#[cfg(test)]
pub(crate) fn check_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    use guardrail3_adapters_outbound_fs::RealFileSystem;
    use guardrail3_app_core::project_walker::walk_project;
    use guardrail3_app_rs_family_mapper::FamilyMapper;
    use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

    let tree = walk_project(&RealFileSystem, root);
    let structure = guardrail3_app_rs_structure::collect(tree.clone(), &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        RustFamilySelection::new(std::collections::BTreeSet::from([RustValidateFamily::Deny]));
    let route = FamilyMapper::from_legality(&legality, None, &selected, None).map_rs_deny();
    let surface = guardrail3_app_rs_family_view::FamilyView::build(
        tree.root().clone(),
        tree.structure(),
        tree.content(),
        &["".to_owned()],
        &[],
        &[],
        None,
        &[],
    );
    check(&surface, &route)
}

#[cfg(test)]
pub(crate) fn config_facts_for_test(
    deny_toml: &str,
    profile_name: Option<&str>,
) -> facts::DenyConfigFacts {
    let (parsed, parsed_typed, parse_error) = match toml::from_str::<toml::Value>(deny_toml) {
        Ok(parsed) => (Some(parsed), deny_toml_parser::parse(deny_toml).ok(), None),
        Err(err) => (None, None, Some(err.to_string())),
    };
    facts::DenyConfigFacts {
        policy_root_rel: String::new(),
        rel_path: "deny.toml".to_owned(),
        file_kind: "deny.toml".to_owned(),
        parsed,
        parsed_typed,
        parse_error,
        profile_name: Some(profile_name.unwrap_or("service").to_owned()),
        policy_context_valid: true,
    }
}

#[cfg(test)]
pub(crate) fn run_config_rule_for_test(
    deny_toml: &str,
    profile_name: Option<&str>,
    rule: fn(&ConfigDenyInput<'_>, &mut Vec<CheckResult>),
) -> Vec<CheckResult> {
    let config = config_facts_for_test(deny_toml, profile_name);
    let input = ConfigDenyInput { config: &config };
    let mut results = Vec::new();
    rule(&input, &mut results);
    results
}

#[cfg(test)]
pub(crate) fn collect_facts_for_test(tree: &FamilyView) -> facts::DenyFacts {
    use guardrail3_app_rs_family_mapper::FamilyMapper;
    use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

    let pt = guardrail3_domain_project_tree::ProjectTree::new(tree.root_path().to_path_buf(), tree.structure().clone(), tree.content().clone());
    let structure = guardrail3_app_rs_structure::collect(pt, &[]);
    let legality = guardrail3_app_rs_legality::collect(structure);
    let selected =
        RustFamilySelection::new(std::collections::BTreeSet::from([RustValidateFamily::Deny]));
    let route = FamilyMapper::from_legality(&legality, None, &selected, None).map_rs_deny();
    facts::collect(tree, &route)
}
