use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};
use test_support::root_workspace_tree;

#[test]
fn malformed_clippy_config_emits_a_single_parseability_result_through_family_orchestration() {
    let tree = root_workspace_tree("not = [valid");
    let scope = guardrail3_app_rs_placement::collect(&tree);
    let selected = RustFamilySelection::new(std::collections::BTreeSet::from([
        RustValidateFamily::Clippy,
    ]));
    let route = FamilyMapper::new(&tree, &scope, None, &selected, None).map_rs_clippy();

    let results = crate::check(&tree, &route);
    let parseability = results
        .iter()
        .filter(|result| result.id == "RS-CLIPPY-25")
        .collect::<Vec<_>>();

    assert_eq!(
        parseability.len(),
        1,
        "expected exactly one RS-CLIPPY-25 result: {results:#?}"
    );
    assert_eq!(
        parseability[0].severity,
        guardrail3_domain_report::Severity::Error
    );
    assert_eq!(parseability[0].file.as_deref(), Some("clippy.toml"));

    let duplicate_parse_ids = [
        "RS-CLIPPY-02",
        "RS-CLIPPY-03",
        "RS-CLIPPY-04",
        "RS-CLIPPY-05",
        "RS-CLIPPY-06",
        "RS-CLIPPY-07",
        "RS-CLIPPY-08",
        "RS-CLIPPY-09",
        "RS-CLIPPY-10",
        "RS-CLIPPY-11",
        "RS-CLIPPY-13",
        "RS-CLIPPY-14",
        "RS-CLIPPY-15",
        "RS-CLIPPY-16",
        "RS-CLIPPY-17",
        "RS-CLIPPY-18",
        "RS-CLIPPY-19",
        "RS-CLIPPY-20",
        "RS-CLIPPY-21",
        "RS-CLIPPY-22",
    ];
    assert!(
        results.iter().all(|result| {
            !(duplicate_parse_ids.contains(&result.id.as_str()) && !result.inventory)
        }),
        "malformed clippy.toml must not fan out into dependent-rule errors: {results:#?}"
    );
}
