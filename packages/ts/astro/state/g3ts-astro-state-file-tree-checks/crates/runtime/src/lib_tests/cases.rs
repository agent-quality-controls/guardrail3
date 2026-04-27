use g3ts_astro_state_file_tree_checks_assertions::run as assertions;
use g3ts_astro_state_types::{
    G3TsAstroStateAppRootInput, G3TsAstroStateFileTreeChecksInput,
    G3TsAstroStateForbiddenPathInput, G3TsAstroStateLegacyGeneratedPathInput,
    G3TsAstroStateStrictAppRootInput,
};

#[test]
fn state_file_tree_package_reports_owned_ids_for_forbidden_state() {
    let app_root = G3TsAstroStateAppRootInput {
        app_root_rel_path: ".".to_owned(),
    };
    let input = G3TsAstroStateFileTreeChecksInput {
        strict_app_roots: vec![G3TsAstroStateStrictAppRootInput {
            app_root_rel_path: app_root.app_root_rel_path,
        }],
        legacy_generated_paths: vec![G3TsAstroStateLegacyGeneratedPathInput {
            app_root_rel_path: ".".to_owned(),
            rel_path: ".next/server/app/page.js".to_owned(),
        }],
        forbidden_state_paths: vec![G3TsAstroStateForbiddenPathInput {
            app_root_rel_path: ".".to_owned(),
            rel_path: ".velite/landing.js".to_owned(),
        }],
    };

    assertions::assert_runtime_check_exact_ids(
        &input,
        &[
            "g3ts-astro-state/no-legacy-parallel-state",
            "g3ts-astro-state/configured-forbidden-state",
        ],
    );
}
