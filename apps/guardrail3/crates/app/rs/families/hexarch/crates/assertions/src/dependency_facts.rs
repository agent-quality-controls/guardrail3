use guardrail3_app_rs_family_hexarch::DependencyFamilyFacts;

pub fn assert_member_count(facts: &DependencyFamilyFacts, expected: usize) {
    assert_eq!(facts.members.len(), expected, "{facts:#?}");
}

pub fn assert_no_cycles(facts: &DependencyFamilyFacts) {
    assert!(facts.cycles.is_empty(), "{facts:#?}");
}
