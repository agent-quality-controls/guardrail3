use g3rs_arch_source_checks_assertions::rs_arch_08a_feature_gated_exports as assertions;
use g3rs_arch_types::types::{G3RsArchFacadeSurface, G3RsArchFeatureExport};

use super::helpers::{run_rule, source_crate};

#[test]
fn ungated_exports_fire_source_rule() {
    let results = run_rule(
        &source_crate("crate_a"),
        &G3RsArchFacadeSurface {
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
        },
    );

    assertions::assert_ungated_exports(&results, "crate_a/src/lib.rs");
}

#[test]
fn properly_gated_exports_inventory_without_feature_table_facts() {
    let results = run_rule(
        &source_crate("crate_a"),
        &G3RsArchFacadeSurface {
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
        },
    );

    assertions::assert_gated_inventory(&results, "crate_a/src/lib.rs");
}
