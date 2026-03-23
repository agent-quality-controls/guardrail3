use super::super::dependency_facts::Layer;
use super::super::inputs::SourceCrateHexarchInput;
use super::super::source_facts::SourceCrateFacts;
use super::check;

#[test]
fn adapter_public_trait_errors() {
    let source = SourceCrateFacts {
        crate_name: "api-adapter-http".to_owned(),
        rel_dir: "apps/api/crates/adapters/http".to_owned(),
        layer: Some(Layer::Adapters),
        pub_trait_count: 1,
        impl_count: 0,
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);

    assert_eq!(results.len(), 1, "expected one adapter pub-trait error: {results:#?}");
}
