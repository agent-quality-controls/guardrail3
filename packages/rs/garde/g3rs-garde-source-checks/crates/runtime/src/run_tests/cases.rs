use g3rs_garde_source_checks_assertions::run as assertions;
use g3rs_garde_types::{
    G3RsGardeApplicability, G3RsGardeBoundaryKind, G3RsGardeDerivedBoundaryTypeSite,
    G3RsGardeRustPolicyInput, G3RsGardeSourceChecksInput,
};

#[test]
fn returns_no_results_when_family_is_inactive() {
    let input = G3RsGardeSourceChecksInput {
        applicability: G3RsGardeApplicability::Inactive,
        garde_dependency_present: false,
        rust_policy: G3RsGardeRustPolicyInput::Missing,
        input_failures: Vec::new(),
        struct_targets: Vec::new(),
        enum_targets: Vec::new(),
        manual_deserialize_impls: Vec::new(),
        boundary_fields: Vec::new(),
        query_as_macros: Vec::new(),
    };

    let results = crate::run::check(&input);

    assertions::assert_no_results(&results);
}

#[test]
fn dispatches_prebound_source_inputs() {
    let input = G3RsGardeSourceChecksInput {
        applicability: G3RsGardeApplicability::Active,
        garde_dependency_present: true,
        rust_policy: G3RsGardeRustPolicyInput::Missing,
        input_failures: Vec::new(),
        struct_targets: vec![G3RsGardeDerivedBoundaryTypeSite {
            rel_path: "src/lib.rs".to_owned(),
            line: 3,
            name: "Input".to_owned(),
            boundary_kind: G3RsGardeBoundaryKind::Struct,
            boundary_macros: vec!["Deserialize".to_owned()],
            has_validate: false,
        }],
        enum_targets: Vec::new(),
        manual_deserialize_impls: Vec::new(),
        boundary_fields: Vec::new(),
        query_as_macros: Vec::new(),
    };

    let results = crate::run::check(&input);

    assertions::assert_contains_result(&results, "g3rs-garde/struct-derive-validate");
}
