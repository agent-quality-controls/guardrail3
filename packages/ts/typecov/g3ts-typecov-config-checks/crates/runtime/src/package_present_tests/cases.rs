use g3ts_typecov_types::{
    G3TsTypecovConfigChecksInput, G3TsTypecovContractInput, G3TsTypecovPackageSurfaceSnapshot,
    G3TsTypecovPackageSurfaceState, G3TsTypecovSyncpackSurfaceState,
};

#[test]
fn missing_type_coverage_package_reports_package_rule() {
    let input = G3TsTypecovConfigChecksInput {
        contracts: vec![G3TsTypecovContractInput {
            app_root_rel_path: ".".to_owned(),
            package: G3TsTypecovPackageSurfaceState::Parsed {
                snapshot: G3TsTypecovPackageSurfaceSnapshot {
                    rel_path: "package.json".to_owned(),
                    dependencies: Vec::new(),
                    dev_dependencies: Vec::new(),
                    script_names: Vec::new(),
                    script_tool_invocations: Vec::new(),
                    script_parse_blockers: Vec::new(),
                },
            },
            syncpack_config: G3TsTypecovSyncpackSurfaceState::Missing {
                rel_path: ".syncpackrc".to_owned(),
            },
        }],
    };

    g3ts_typecov_config_checks_assertions::package_present::assert_error(
        &input,
        Some("package.json"),
    );
}
