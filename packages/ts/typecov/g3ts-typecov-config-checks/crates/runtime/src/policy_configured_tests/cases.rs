use g3ts_typecov_types::{
    G3TsTypecovConfigChecksInput, G3TsTypecovContractInput, G3TsTypecovPackageSurfaceState,
    G3TsTypecovSyncpackSurfaceState,
};

#[test]
fn missing_package_root_reports_policy_configured_error() {
    let input = G3TsTypecovConfigChecksInput {
        contracts: vec![G3TsTypecovContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsTypecovPackageSurfaceState::Missing {
                rel_path: "package.json".to_owned(),
            },
            syncpack_config: G3TsTypecovSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    };

    g3ts_typecov_config_checks_assertions::policy_configured::assert_error(
        &input,
        Some("package.json"),
    );
}

#[test]
fn empty_input_reports_policy_configured_error() {
    g3ts_typecov_config_checks_assertions::policy_configured::assert_error(
        &G3TsTypecovConfigChecksInput {
            contracts: Vec::new(),
        },
        None,
    );
}
