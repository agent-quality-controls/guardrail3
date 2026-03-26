use std::collections::BTreeSet;

use guardrail3_app_rs_family_hexarch::{self as runtime, DependencyFamilyFacts};
use guardrail3_app_rs_family_mapper::{FamilyMapper, RsHexarchRoute};
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::ProjectTree;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub fn family_route(tree: &ProjectTree) -> RsHexarchRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Hexarch]));
    FamilyMapper::new(tree, &scope, config.as_ref(), &selection, None).map_rs_hexarch()
}

pub fn dependency_facts(tree: &ProjectTree) -> DependencyFamilyFacts {
    runtime::collect_dependency_facts_for_tests(tree, &family_route(tree))
}

pub fn assert_member_count(facts: &DependencyFamilyFacts, expected: usize) {
    assert_eq!(facts.members.len(), expected);
}
