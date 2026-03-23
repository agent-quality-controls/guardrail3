use super::super::dependency_facts::Layer;
use super::super::inputs::SourceCrateHexarchInput;
use super::super::source_facts::SourceCrateFacts;
use super::check;

#[test]
fn impl_heavy_ports_warns() {
    let source = SourceCrateFacts {
        crate_name: "api-ports-http".to_owned(),
        rel_dir: "apps/api/crates/ports/http".to_owned(),
        layer: Some(Layer::Ports),
        pub_trait_count: 1,
        impl_count: 3,
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);

    assert_eq!(results.len(), 1, "expected one trait-dominance warning: {results:#?}");
}
