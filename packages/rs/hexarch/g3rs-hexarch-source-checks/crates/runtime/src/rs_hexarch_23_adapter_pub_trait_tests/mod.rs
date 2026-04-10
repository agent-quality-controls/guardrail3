use g3rs_hexarch_types::{G3RsHexarchLayer, G3RsHexarchSourceChecksInput, G3RsHexarchSourceCrateFacts};
use guardrail3_check_types::G3Severity;

fn input(
    layer: Option<G3RsHexarchLayer>,
    pub_trait_count: usize,
    source_error_message: Option<&str>,
) -> G3RsHexarchSourceChecksInput {
    G3RsHexarchSourceChecksInput {
        crate_facts: G3RsHexarchSourceCrateFacts {
            crate_name: "adapter-sql".to_owned(),
            rel_dir: "apps/demo/crates/adapters/sql".to_owned(),
            layer,
            pub_trait_count,
            public_free_fn_count: 0,
            public_inherent_method_count: 0,
            source_error_rel_path: source_error_message.map(|_| "apps/demo/crates/adapters/sql/src/lib.rs".to_owned()),
            source_error_message: source_error_message.map(str::to_owned),
        },
    }
}

#[test]
fn errors_for_public_adapter_traits() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Adapters), 1, None));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-23");
    assert_eq!(results[0].severity(), G3Severity::Error);
    assert!(results[0].title().contains("defines public traits"));
}

#[test]
fn ignores_non_adapter_crates() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Ports), 1, None));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-22");
    assert!(results[0].inventory());
}

#[test]
fn inventories_clean_adapter_surface() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Adapters), 0, None));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity(), G3Severity::Info);
    assert!(results[0].inventory());
}

#[test]
fn errors_on_source_analysis_failure() {
    let results = crate::run::check(&input(
        Some(G3RsHexarchLayer::Adapters),
        0,
        Some("Failed to determine Rust source entrypoint for hexarch checks"),
    ));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].severity(), G3Severity::Error);
    assert!(results[0].title().contains("source analysis failed"));
}
