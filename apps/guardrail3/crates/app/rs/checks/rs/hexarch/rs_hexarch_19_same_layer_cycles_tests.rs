use super::super::dependency_facts::{CycleFacts, Layer};
use super::super::inputs::CycleHexarchInput;
use super::check;

#[test]
fn same_layer_cycle_errors() {
    let cycle = CycleFacts {
        layer: Layer::Domain,
        members: vec!["apps/api/crates/domain/a".to_owned(), "apps/api/crates/domain/b".to_owned()],
    };
    let mut results = Vec::new();
    check(&CycleHexarchInput::new(&cycle), &mut results);

    assert_eq!(results.len(), 1, "expected one cycle error: {results:#?}");
    assert!(results[0].message.contains("domain"));
}
