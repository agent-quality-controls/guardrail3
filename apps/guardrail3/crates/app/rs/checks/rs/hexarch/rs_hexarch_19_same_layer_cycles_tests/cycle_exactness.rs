use super::super::super::dependency_facts::{CycleFacts, Layer};
use super::super::super::inputs::CycleHexarchInput;
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn same_layer_cycle_reports_exact_layer_and_path_chain() {
    let cycle = CycleFacts {
        layer: Layer::Domain,
        members: vec![
            "apps/api/crates/domain/a".to_owned(),
            "apps/api/crates/domain/b".to_owned(),
            "apps/api/crates/domain/c".to_owned(),
        ],
    };
    let mut results = Vec::new();
    check(&CycleHexarchInput::new(&cycle), &mut results);

    assert_eq!(results.len(), 1, "expected one cycle error: {results:#?}");
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(results[0].file, None);
    assert!(
        results[0]
            .title
            .contains("same-layer domain dependency cycle")
    );
    assert!(results[0].message.contains(
        "apps/api/crates/domain/a -> apps/api/crates/domain/b -> apps/api/crates/domain/c"
    ));
}
