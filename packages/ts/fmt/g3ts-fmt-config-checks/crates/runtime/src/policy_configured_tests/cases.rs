use g3ts_fmt_types::{
    G3TsFmtConfigChecksInput, G3TsFmtConfigSurfaceState, G3TsFmtContractInput,
    G3TsFmtPackageSurfaceState, G3TsFmtSyncpackSurfaceState,
};

#[test]
fn missing_package_root_reports_policy_configured_error() {
    let input = G3TsFmtConfigChecksInput {
        contracts: vec![G3TsFmtContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsFmtPackageSurfaceState::Missing {
                rel_path: "package.json".to_owned(),
            },
            prettier_config: G3TsFmtConfigSurfaceState::Parsed {
                rel_path: "prettier.config.js".to_owned(),
            },
            syncpack_config: G3TsFmtSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    };

    g3ts_fmt_config_checks_assertions::policy_configured::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn empty_input_reports_policy_configured_error() {
    g3ts_fmt_config_checks_assertions::policy_configured::assert_error(
        &G3TsFmtConfigChecksInput {
            contracts: Vec::new(),
        },
        None,
    );
}
