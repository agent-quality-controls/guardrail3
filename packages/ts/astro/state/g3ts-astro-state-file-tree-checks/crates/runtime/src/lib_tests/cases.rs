use g3ts_astro_state_file_tree_checks_assertions::run as assertions;
use g3ts_astro_types::{G3TsAstroAppRootInput, G3TsAstroStateFileTreeChecksInput};

#[test]
fn state_file_tree_package_reports_owned_ids_for_forbidden_state() {
    let app_root = G3TsAstroAppRootInput {
        app_root_rel_path: ".".to_owned(),
        astro_config_rel_path: Some("astro.config.mjs".to_owned()),
        content_config_rel_path: Some("src/content.config.ts".to_owned()),
        live_config_rel_path: None,
        velite_config_rel_path: None,
        velite_output_rel_paths: Vec::new(),
        legacy_generated_state_rel_paths: vec![".next/server/app/page.js".to_owned()],
        forbidden_state_rel_paths: vec![".velite/landing.js".to_owned()],
    };
    let input = G3TsAstroStateFileTreeChecksInput {
        build_collection_roots: vec![app_root],
        live_collection_roots: Vec::new(),
    };

    assertions::assert_runtime_check_exact_ids(
        &input,
        &[
            "TS-ASTRO-STATE-FILETREE-11",
            "TS-ASTRO-STATE-FILETREE-12"
        ],
    );
}
