use g3ts_astro_setup_file_tree_checks_assertions::run as assertions;
use g3ts_astro_types::{G3TsAstroAppRootInput, G3TsAstroSetupFileTreeChecksInput};

#[test]
fn golden_setup_file_tree_package_reports_owned_id() {
    let input = G3TsAstroSetupFileTreeChecksInput {
        app_roots: vec![G3TsAstroAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
            content_config_rel_path: Some("src/content.config.ts".to_owned()),
            live_config_rel_path: None,
            velite_config_rel_path: None,
            velite_output_rel_paths: Vec::new(),
            legacy_generated_state_rel_paths: Vec::new(),
            forbidden_state_rel_paths: Vec::new(),
        }],
        live_collection_roots: Vec::new(),
    };

    assertions::assert_runtime_check_exact_ids(&input, &["TS-ASTRO-SETUP-FILETREE-01"]);
}
