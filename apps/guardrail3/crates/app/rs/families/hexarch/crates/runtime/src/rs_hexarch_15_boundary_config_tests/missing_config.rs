use super::super::super::dependency_facts::BoundaryConfigFacts;
use super::super::super::inputs::MemberConfigHexarchInput;
use super::super::check;

#[test]
fn non_app_boundaries_do_not_warn() {
    let input = BoundaryConfigFacts {
        rel_dir: "packages/shared".to_owned(),
        has_config_entry: false,
        is_app_boundary: false,
        parse_error: None,
    };
    let mut results = Vec::new();
    check(&MemberConfigHexarchInput::new(&input), &mut results);

    assert!(
        results.is_empty(),
        "non-app boundaries should stay out of scope for RS-HEXARCH-15: {results:#?}"
    );
}
