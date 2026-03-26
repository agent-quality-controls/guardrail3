use guardrail3_app_rs_family_hexarch::{self as runtime, DependencyFamilyFacts};
use guardrail3_domain_project_tree::ProjectTree;

pub fn dependency_facts(tree: &ProjectTree) -> DependencyFamilyFacts {
    runtime::collect_dependency_facts_from_tree_for_tests(tree)
}

pub fn assert_member_count(facts: &DependencyFamilyFacts, expected: usize) {
    assert_eq!(facts.members.len(), expected);
}
