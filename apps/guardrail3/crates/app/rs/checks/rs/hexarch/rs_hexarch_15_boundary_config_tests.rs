use super::super::dependency_facts::BoundaryConfigFacts;
use super::super::inputs::MemberConfigHexarchInput;
use super::check;

#[test]
fn missing_app_boundary_config_warns() {
    let input = BoundaryConfigFacts {
        rel_dir: "apps/api".to_owned(),
        has_config_entry: false,
        is_app_boundary: true,
    };
    let mut results = Vec::new();
    check(&MemberConfigHexarchInput::new(&input), &mut results);

    assert_eq!(results.len(), 1, "expected one config warning: {results:#?}");
    assert!(results[0].title.contains("missing rust.apps config"));
}
