use super::super::super::dependency_facts::Layer;
use super::super::super::inputs::SourceCrateHexarchInput;
use super::super::super::source_facts::SourceCrateFacts;
use super::super::check;

#[test]
fn impl_heavy_ports_warns() {
    let source = SourceCrateFacts {
        crate_name: "api-ports-http".to_owned(),
        rel_dir: "apps/api/crates/ports/http".to_owned(),
        layer: Some(Layer::Ports),
        pub_trait_count: 1,
        impl_count: 3,
        source_error_rel_path: None,
        source_error_message: None,
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);

    assert_eq!(
        results.len(),
        1,
        "expected one trait-dominance warning: {results:#?}"
    );
}

#[test]
fn equal_impl_and_public_trait_counts_do_not_warn() {
    let source = SourceCrateFacts {
        crate_name: "api-ports-http".to_owned(),
        rel_dir: "apps/api/crates/ports/http".to_owned(),
        layer: Some(Layer::Ports),
        pub_trait_count: 2,
        impl_count: 2,
        source_error_rel_path: None,
        source_error_message: None,
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);

    assert!(
        results.is_empty(),
        "expected no warning when impls and public traits are balanced: {results:#?}"
    );
}

#[test]
fn dto_only_ports_crate_stays_clean() {
    let source = SourceCrateFacts {
        crate_name: "api-ports-http".to_owned(),
        rel_dir: "apps/api/crates/ports/http".to_owned(),
        layer: Some(Layer::Ports),
        pub_trait_count: 0,
        impl_count: 0,
        source_error_rel_path: None,
        source_error_message: None,
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);

    assert!(
        results.is_empty(),
        "expected DTO-only ports crates to stay clean: {results:#?}"
    );
}

#[test]
fn non_ports_crates_are_ignored() {
    let source = SourceCrateFacts {
        crate_name: "api-adapters-http".to_owned(),
        rel_dir: "apps/api/crates/adapters/http".to_owned(),
        layer: Some(Layer::Adapters),
        pub_trait_count: 0,
        impl_count: 99,
        source_error_rel_path: None,
        source_error_message: None,
    };
    let mut results = Vec::new();
    check(&SourceCrateHexarchInput::new(&source), &mut results);

    assert!(
        results.is_empty(),
        "expected non-ports crates to be ignored by RS-HEXARCH-22: {results:#?}"
    );
}
