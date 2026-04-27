use g3ts_astro_content_file_tree_checks_assertions::run as assertions;
use g3ts_astro_types::{G3TsAstroAppRootInput, G3TsAstroFileTreeChecksInput};

#[test]
fn golden_content_file_tree_package_reports_owned_id() {
    let app_root = G3TsAstroAppRootInput {
        app_root_rel_path: ".".to_owned(),
        astro_config_rel_path: Some("astro.config.mjs".to_owned()),
        content_config_rel_path: Some("src/content.config.ts".to_owned()),
        live_config_rel_path: None,
        velite_config_rel_path: None,
        velite_output_rel_paths: Vec::new(),
        legacy_generated_state_rel_paths: Vec::new(),
        forbidden_state_rel_paths: Vec::new(),
    };
    let input = G3TsAstroFileTreeChecksInput {
        app_roots: vec![app_root.clone()],
        build_collection_roots: vec![app_root],
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
    };

    assertions::assert_runtime_check_exact_ids(&input, &["TS-ASTRO-CONTENT-FILETREE-02"]);
}
