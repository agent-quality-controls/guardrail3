use g3ts_astro_content_file_tree_checks_assertions::run as assertions;
use g3ts_astro_content_types::{
    G3TsAstroContentAppRootInput, G3TsAstroContentFileTreeChecksInput,
    G3TsAstroContentVeliteOutputInput,
};

#[test]
fn golden_content_file_tree_package_reports_owned_id() {
    let app_root = G3TsAstroContentAppRootInput {
        app_root_rel_path: ".".to_owned(),
        content_config_rel_path: Some("src/content.config.ts".to_owned()),
        live_config_rel_path: None,
        velite_config_rel_path: None,
    };
    let input = G3TsAstroContentFileTreeChecksInput {
        app_roots: vec![app_root.clone()],
        build_collection_roots: vec![app_root],
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
        velite_output_paths: Vec::new(),
    };

    assertions::assert_runtime_check_exact_ids(&input, &["TS-ASTRO-CONTENT-FILETREE-02"]);
}

#[test]
fn content_file_tree_reports_exact_velite_output_paths() {
    let app_root = G3TsAstroContentAppRootInput {
        app_root_rel_path: ".".to_owned(),
        content_config_rel_path: Some("src/content.config.ts".to_owned()),
        live_config_rel_path: None,
        velite_config_rel_path: None,
    };
    let input = G3TsAstroContentFileTreeChecksInput {
        app_roots: vec![app_root.clone()],
        build_collection_roots: vec![app_root],
        live_collection_roots: Vec::new(),
        route_markdown_pages: Vec::new(),
        velite_output_paths: vec![G3TsAstroContentVeliteOutputInput {
            app_root_rel_path: ".".to_owned(),
            rel_path: ".velite/content.js".to_owned(),
        }],
    };

    assertions::assert_runtime_check_exact_ids(
        &input,
        &[
            "TS-ASTRO-CONTENT-FILETREE-02",
            "TS-ASTRO-CONTENT-FILETREE-06",
        ],
    );
}
