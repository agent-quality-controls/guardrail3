use g3ts_astro_setup_file_tree_checks_assertions::run as assertions;
use g3ts_astro_setup_types::{G3TsAstroSetupAppRootInput, G3TsAstroSetupFileTreeChecksInput};

#[test]
fn golden_setup_file_tree_package_reports_owned_id() {
    let input = G3TsAstroSetupFileTreeChecksInput {
        app_roots: vec![G3TsAstroSetupAppRootInput {
            app_root_rel_path: ".".to_owned(),
            astro_config_rel_path: Some("astro.config.mjs".to_owned()),
        }],
    };

    assertions::assert_runtime_check_exact_ids(&input, &["TS-ASTRO-SETUP-FILETREE-01"]);
}
