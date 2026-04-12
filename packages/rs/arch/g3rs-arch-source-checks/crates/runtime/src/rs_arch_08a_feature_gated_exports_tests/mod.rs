use g3rs_arch_source_checks_assertions::{ExpectedRuleResult, assert_rule_results, has_rule};
use g3rs_arch_types::{G3RsArchFacadeSurface, G3RsArchFeatureExport};
use guardrail3_check_types::G3Severity;

use crate::test_support::{input, source_crate};

#[test]
fn ungated_exports_fire_source_rule() {
    let results = crate::check(&input(
        vec![source_crate("crate_a")],
        vec![G3RsArchFacadeSurface {
            rel_path: "crate_a/src/lib.rs".to_owned(),
            is_lib_rs: true,
            is_mod_rs: false,
            body_items: Vec::new(),
            broad_reexports: Vec::new(),
            pub_exports: vec![G3RsArchFeatureExport {
                line: 3,
                name: "api".to_owned(),
                feature_gate: None,
                gated_on_all: false,
            }],
            pub_export_count: 1,
            ungated_export_count: 1,
            gated_on_all_count: 0,
        }],
        Vec::new(),
    ));

    assert_rule_results(
        &results,
        "RS-ARCH-SOURCE-08",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("facade exports not feature-gated"),
            file: Some("crate_a/src/lib.rs"),
            inventory: Some(false),
            message: None,
        }],
    );
}

#[test]
fn properly_gated_exports_inventory_without_feature_table_facts() {
    let results = crate::check(&input(
        vec![source_crate("crate_a")],
        vec![G3RsArchFacadeSurface {
            rel_path: "crate_a/src/lib.rs".to_owned(),
            is_lib_rs: true,
            is_mod_rs: false,
            body_items: Vec::new(),
            broad_reexports: Vec::new(),
            pub_exports: vec![G3RsArchFeatureExport {
                line: 3,
                name: "api".to_owned(),
                feature_gate: Some("api".to_owned()),
                gated_on_all: false,
            }],
            pub_export_count: 1,
            ungated_export_count: 0,
            gated_on_all_count: 0,
        }],
        Vec::new(),
    ));

    assert_rule_results(
        &results,
        "RS-ARCH-SOURCE-08",
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Info),
            title: Some("facade exports properly feature-gated"),
            file: Some("crate_a/src/lib.rs"),
            inventory: Some(true),
            message: None,
        }],
    );
    assert!(!has_rule(&results, "RS-ARCH-CONFIG-08"));
}
