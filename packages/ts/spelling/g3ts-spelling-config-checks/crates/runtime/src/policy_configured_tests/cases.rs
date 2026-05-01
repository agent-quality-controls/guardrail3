use g3ts_spelling_types::{
    G3TsSpellingConfigChecksInput, G3TsSpellingConfigSurfaceState, G3TsSpellingContractInput,
    G3TsSpellingPackageSurfaceState, G3TsSpellingSyncpackSurfaceState,
};

#[test]
fn missing_package_root_reports_policy_configured_error() {
    let input = G3TsSpellingConfigChecksInput {
        contracts: vec![G3TsSpellingContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsSpellingPackageSurfaceState::Missing {
                rel_path: "package.json".to_owned(),
            },
            cspell_config: G3TsSpellingConfigSurfaceState::Parsed {
                rel_path: "cspell.config.js".to_owned(),
            },
            syncpack_config: G3TsSpellingSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    };

    g3ts_spelling_config_checks_assertions::policy_configured::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn empty_input_reports_policy_configured_error() {
    g3ts_spelling_config_checks_assertions::policy_configured::assert_error(
        &G3TsSpellingConfigChecksInput {
            contracts: Vec::new(),
        },
        None,
    );
}
