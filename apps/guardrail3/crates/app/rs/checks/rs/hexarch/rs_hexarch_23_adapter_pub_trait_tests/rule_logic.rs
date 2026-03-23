use super::super::super::dependency_facts::Layer;
use super::super::super::inputs::SourceCrateHexarchInput;
use super::super::super::source_facts::SourceCrateFacts;
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn adapter_public_trait_errors() {
    let source = SourceCrateFacts {
        crate_name: "api-adapter-http".to_owned(),
        rel_dir: "apps/api/crates/adapters/http".to_owned(),
        layer: Some(Layer::Adapters),
        pub_trait_count: 1,
        impl_count: 0,
        source_error_rel_path: None,
        source_error_message: None,
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);

    assert_eq!(
        results.len(),
        1,
        "expected one adapter pub-trait error: {results:#?}"
    );
    assert_eq!(results[0].severity, Severity::Error);
    assert_eq!(
        results[0].file.as_deref(),
        Some("apps/api/crates/adapters/http")
    );
}
