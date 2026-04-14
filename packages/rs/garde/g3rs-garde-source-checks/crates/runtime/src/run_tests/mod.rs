use g3rs_garde_source_checks_types::{G3RsGardeApplicability, G3RsGardeSourceChecksInput};

#[test]
fn returns_no_results_when_family_is_inactive() {
    let input = G3RsGardeSourceChecksInput {
        applicability: G3RsGardeApplicability::Inactive,
        garde_dependency_present: false,
        source_files: Vec::new(),
        guardrail_toml: None,
    };

    let results = crate::run::check(&input);

    assert!(results.is_empty(), "{results:#?}");
}
