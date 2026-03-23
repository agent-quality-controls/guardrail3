use super::super::dependency_facts::{Layer, PatchEntryFacts};
use super::super::inputs::PatchHexarchInput;
use super::check;

#[test]
fn patch_into_layered_tree_errors() {
    let patch = PatchEntryFacts {
        cargo_rel_path: "apps/api/Cargo.toml".to_owned(),
        key: "api-domain".to_owned(),
        resolved_rel_dir: "apps/api/crates/domain".to_owned(),
        target_layer: Some(Layer::Domain),
    };
    let mut results = Vec::new();
    check(&PatchHexarchInput::new(&patch), &mut results);

    assert_eq!(results.len(), 1, "expected one patch bypass error: {results:#?}");
}
