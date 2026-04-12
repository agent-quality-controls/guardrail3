use g3rs_hexarch_types::{G3RsHexarchLayer, G3RsHexarchSourceChecksInput, G3RsHexarchSourceCrateFacts};
use guardrail3_check_types::G3Severity;

fn input(
    layer: Option<G3RsHexarchLayer>,
    public_free_fn_count: usize,
    public_inherent_method_count: usize,
    source_error_message: Option<&str>,
) -> G3RsHexarchSourceChecksInput {
    G3RsHexarchSourceChecksInput {
        crate_facts: G3RsHexarchSourceCrateFacts {
            crate_name: "ports-http".to_owned(),
            rel_dir: "apps/demo/crates/ports/http".to_owned(),
            layer,
            pub_trait_count: 0,
            public_free_fn_count,
            public_inherent_method_count,
            source_error_rel_path: source_error_message.map(|_| "apps/demo/crates/ports/http/src/lib.rs".to_owned()),
            source_error_message: source_error_message.map(str::to_owned),
        },
    }
}

#[test]
fn warns_for_public_free_functions() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Ports), 1, 0, None));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-SOURCE-22");
    assert_eq!(results[0].severity(), G3Severity::Warn);
    assert_eq!(results[0].file(), Some("apps/demo/crates/ports/http"));
    assert!(results[0].title().contains("public free functions"));
    assert!(results[0].message().contains("1 public free function"));
}

#[test]
fn warns_for_public_inherent_methods() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Ports), 0, 1, None));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-SOURCE-22");
    assert_eq!(results[0].severity(), G3Severity::Warn);
    assert!(results[0].title().contains("public inherent methods"));
    assert!(results[0].message().contains("1 public inherent method"));
}

#[test]
fn emits_two_findings_when_both_ports_violations_exist() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Ports), 1, 1, None));

    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.id() == "RS-HEXARCH-SOURCE-22"));
    assert!(results.iter().all(|result| result.severity() == G3Severity::Warn));
    assert!(results.iter().all(|result| result.file() == Some("apps/demo/crates/ports/http")));
    assert!(results
        .iter()
        .any(|result| result.title().contains("public free functions")));
    assert!(results
        .iter()
        .any(|result| result.title().contains("public inherent methods")));
}

#[test]
fn ignores_non_ports_layers() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Domain), 1, 1, None));

    assert!(results.is_empty());
}

#[test]
fn inventories_clean_ports_surface() {
    let results = crate::run::check(&input(Some(G3RsHexarchLayer::Ports), 0, 0, None));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-SOURCE-22");
    assert_eq!(results[0].severity(), G3Severity::Info);
    assert!(results[0].inventory());
}

#[test]
fn warns_on_source_analysis_failure() {
    let results = crate::run::check(&input(
        Some(G3RsHexarchLayer::Ports),
        0,
        0,
        Some("Failed to parse Rust source file for hexarch checks: expected item"),
    ));

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id(), "RS-HEXARCH-SOURCE-22");
    assert_eq!(results[0].severity(), G3Severity::Warn);
    assert_eq!(results[0].file(), Some("apps/demo/crates/ports/http/src/lib.rs"));
    assert!(results[0].title().contains("source analysis failed"));
    assert!(results[0].message().contains("Failed to parse Rust source file"));
}
