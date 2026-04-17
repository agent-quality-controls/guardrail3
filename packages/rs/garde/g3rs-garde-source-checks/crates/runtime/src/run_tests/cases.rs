use g3rs_garde_source_checks_assertions::run as assertions;
use g3rs_garde_types::{
    G3RsGardeApplicability, G3RsGardeRustPolicyInput, G3RsGardeSourceChecksInput,
};

#[test]
fn returns_no_results_when_family_is_inactive() {
    let input = G3RsGardeSourceChecksInput {
        applicability: G3RsGardeApplicability::Inactive,
        garde_dependency_present: false,
        source_files: Vec::new(),
        rust_policy: G3RsGardeRustPolicyInput::Missing,
    };

    let results = crate::run::check(&input);

    assertions::assert_no_results(&results);
}
