use g3rs_arch_source_checks_assertions::run as assertions;
use g3rs_arch_types::types::{
    G3RsArchFacadeItem, G3RsArchFacadeSurface, G3RsArchLibFacadeChecksInput,
    G3RsArchSourceChecksInput, G3RsArchSourceCrate,
};

#[test]
fn run_dispatches_prebound_lib_facade_inputs() {
    let input = G3RsArchSourceChecksInput {
        lib_facade_checks: vec![G3RsArchLibFacadeChecksInput {
            krate: G3RsArchSourceCrate {
                rel_dir: "packages/demo".to_owned(),
                lib_rs_rel: Some("packages/demo/src/lib.rs".to_owned()),
            },
            lib_surface: Some(G3RsArchFacadeSurface {
                rel_path: "packages/demo/src/lib.rs".to_owned(),
                is_lib_rs: true,
                is_mod_rs: false,
                body_items: vec![G3RsArchFacadeItem {
                    line: 3,
                    kind: "function",
                    name: "leaked_impl".to_owned(),
                    is_broad_reexport: false,
                    feature_gate: None,
                    gated_on_all: false,
                }],
                broad_reexports: Vec::new(),
                pub_exports: Vec::new(),
                pub_export_count: 0,
                ungated_export_count: 0,
                gated_on_all_count: 0,
            }),
        }],
        mod_facade_surfaces: Vec::new(),
        path_attr_sites: Vec::new(),
    };

    let results = crate::run::check(&input);

    assertions::assert_has_finding_id(&results, "g3rs-arch/lib-facade-only");
}
